//! The backtracking machine that runs a compiled pattern.
//!
//! It keeps its own stack of alternatives and of the changes to undo.
//! A pattern then never recurses on the call stack, except for the nesting of
//! lookarounds and atomic groups the pattern itself has.
//! Every instruction costs a step from the caller's budget.

use super::ast::{Anchor, Case, is_word, orbit};
use super::program::{ClassTable, Inst, Program};

/// A capture slot that holds no position.
pub(super) const UNSET: u32 = u32::MAX;

/// How deep subroutine calls may nest before the match fails.
const MAX_CALLS: usize = 256;

/// What the machine can undo when it backtracks.
#[derive(Clone, Copy, Debug)]
enum Entry {
    /// An alternative: resume here.
    Branch { pc: u32, pos: u32 },
    /// A capture slot's old value.
    Slot { index: u32, old: u32 },
    /// A register's old value.
    Reg { index: u32, old: u32 },
    /// A subroutine call was made: drop it.
    Called,
    /// A subroutine returned: call it again.
    Returned { group: u32, ret: u32 },
}

/// What a search is given besides the text.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Context {
    /// Where `\G` matches.
    pub(crate) search: Option<usize>,
    /// Whether `\A` matches at the start of the text.
    pub(crate) text_start: bool,
}

/// The buffers a search uses, kept between searches so that a search
/// allocates nothing once they have grown.
#[derive(Debug, Default)]
pub(crate) struct Scratch {
    pub(super) slots: Vec<u32>,
    regs: Vec<u32>,
    stack: Vec<Entry>,
    calls: Vec<(u32, u32)>,
}

impl Scratch {
    /// How many groups the last pattern run had, the whole match included.
    pub(crate) fn group_count(&self) -> usize {
        self.slots.len() / 2
    }

    /// The bytes group `index` matched in the last successful search.
    pub(crate) fn group(&self, index: usize) -> Option<(usize, usize)> {
        let start = *self.slots.get(index * 2)?;
        let end = *self.slots.get(index * 2 + 1)?;
        (start != UNSET && end != UNSET).then(|| {
            (
                usize::try_from(start).unwrap_or(usize::MAX),
                usize::try_from(end).unwrap_or(usize::MAX),
            )
        })
    }

    /// Readies the buffers for a pattern.
    pub(super) fn reset(&mut self, program: &Program) {
        self.slots.clear();
        self.slots.resize((program.groups + 1) * 2, UNSET);
        self.regs.clear();
        self.regs.resize(program.regs, UNSET);
        self.stack.clear();
        self.calls.clear();
    }

    /// Whether the buffers are as [`Scratch::reset`] leaves them: what an
    /// attempt at one position needs, and what a failed one restores.
    pub(super) fn is_clear(&self) -> bool {
        self.slots.iter().all(|&slot| slot == UNSET)
            && self.regs.iter().all(|&reg| reg == UNSET)
            && self.stack.is_empty()
            && self.calls.is_empty()
    }
}

/// What an instruction decided.
enum Flow {
    /// Continue at this instruction and position.
    Go(usize, usize),
    /// Backtrack.
    Fail,
    /// The run reached its end here.
    Done(usize),
    /// The step budget ran out.
    Exhausted,
}

/// One run of a program over a text.
pub(super) struct Machine<'a> {
    pub(super) program: &'a Program,
    pub(super) text: &'a [u8],
    pub(super) context: Context,
    pub(super) steps: &'a mut u64,
    pub(super) scratch: &'a mut Scratch,
}

/// The code point at `pos` and its width in bytes.
/// A byte that does not start a character reads as itself.
fn decode(text: &[u8], pos: usize) -> Option<(u32, usize)> {
    let lead = *text.get(pos)?;
    if lead < 0x80 {
        return Some((u32::from(lead), 1));
    }
    let (width, mut code) = match lead {
        0xc0..=0xdf => (2, u32::from(lead & 0x1f)),
        0xe0..=0xef => (3, u32::from(lead & 0x0f)),
        0xf0..=0xf7 => (4, u32::from(lead & 0x07)),
        _ => return Some((u32::from(lead), 1)),
    };
    for offset in 1..width {
        let byte = *text.get(pos + offset)?;
        if byte & 0xc0 != 0x80 {
            return Some((u32::from(lead), 1));
        }
        code = code << 6 | u32::from(byte & 0x3f);
    }
    Some((code, width))
}

/// The character that ends at `pos`, and where it starts.
fn decode_before(text: &[u8], pos: usize) -> Option<(u32, usize)> {
    let mut start = pos.checked_sub(1)?;
    while start > 0 && text.get(start).is_some_and(|b| b & 0xc0 == 0x80) {
        start -= 1;
    }
    decode(text, start).map(|(code, _)| (code, start))
}

/// Whether `text` holds `bytes` at `pos`.
///
/// The first byte is compared before the rest.
/// Most literals fail on their first byte.
/// Comparing the rest calls `memcmp`, which costs about ten instructions.
fn lit_at(text: &[u8], pos: usize, bytes: &[u8]) -> bool {
    let Some(head) = pos
        .checked_add(bytes.len())
        .and_then(|end| text.get(pos..end))
    else {
        return false;
    };
    match bytes.split_first() {
        None => true,
        Some((first, rest)) => {
            head.first() == Some(first) && (rest.is_empty() || head.get(1..) == Some(rest))
        }
    }
}

fn as_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(UNSET)
}

fn as_usize(value: u32) -> usize {
    usize::try_from(value).unwrap_or(usize::MAX)
}

impl ClassTable {
    /// Whether the code point is in the class.
    ///
    /// It lives in `Machine::run`'s module, so both share a codegen unit.
    /// An `#[inline]` mark instead changed how `run` compiled at one unit.
    fn matches(&self, c: u32) -> bool {
        if c < 128 {
            self.ascii >> c & 1 == 1
        } else {
            char::from_u32(c).is_some_and(|c| self.class.matches(c))
        }
    }
}

impl Machine<'_> {
    /// Runs the program from `pc` at `pos`.
    /// Returns the end of the match, or `None`.
    /// `target` is where a lookbehind body must end.
    pub(super) fn run(
        &mut self,
        mut pc: usize,
        mut pos: usize,
        target: Option<usize>,
    ) -> Option<usize> {
        let base = self.scratch.stack.len();
        loop {
            if *self.steps == 0 {
                self.unwind(base);
                return None;
            }
            *self.steps -= 1;
            let Some(inst) = self.program.insts.get(pc) else {
                self.unwind(base);
                return None;
            };
            let flow = match inst {
                Inst::Lit(_) | Inst::Any(_) | Inst::Class(_) | Inst::Backref { .. } => {
                    self.consume(inst, pc, pos)
                }
                Inst::Look { .. } | Inst::Atomic { .. } | Inst::BodyEnd | Inst::Match => {
                    self.nested(inst, pc, pos, target)
                }
                Inst::Split { .. }
                | Inst::Jump(_)
                | Inst::Save(_)
                | Inst::Assert(_)
                | Inst::Mark(_)
                | Inst::Progress { .. }
                | Inst::Call { .. }
                | Inst::GroupEnd { .. }
                | Inst::ClassOr { .. } => self.control(inst, pc, pos),
            };
            match flow {
                Flow::Go(next_pc, next_pos) => {
                    pc = next_pc;
                    pos = next_pos;
                }
                Flow::Done(end) => return Some(end),
                Flow::Fail => match self.backtrack(base) {
                    Some((next_pc, next_pos)) => {
                        pc = next_pc;
                        pos = next_pos;
                    }
                    None => return None,
                },
                Flow::Exhausted => {
                    self.unwind(base);
                    return None;
                }
            }
        }
    }

    /// An instruction that reads text.
    ///
    /// The run loop sends every other instruction to the function that takes
    /// it.
    fn consume(&self, inst: &Inst, pc: usize, pos: usize) -> Flow {
        let step = |width: usize| Flow::Go(pc + 1, pos + width);
        match inst {
            Inst::Lit(bytes) => {
                if lit_at(self.text, pos, bytes) {
                    step(bytes.len())
                } else {
                    Flow::Fail
                }
            }
            Inst::Any(dotall) => match decode(self.text, pos) {
                Some((code, width)) if *dotall || code != u32::from(b'\n') => step(width),
                _ => Flow::Fail,
            },
            Inst::Class(id) => {
                let table = self.program.classes.get(as_usize(*id));
                match decode(self.text, pos) {
                    Some((code, width)) if table.is_some_and(|t| t.matches(code)) => step(width),
                    _ => Flow::Fail,
                }
            }
            Inst::Backref { group, case } => match self.backref(as_usize(*group), *case, pos) {
                Some(after) => Flow::Go(pc + 1, after),
                None => Flow::Fail,
            },
            Inst::Split { .. }
            | Inst::Jump(_)
            | Inst::Save(_)
            | Inst::Assert(_)
            | Inst::Mark(_)
            | Inst::Progress { .. }
            | Inst::Look { .. }
            | Inst::Atomic { .. }
            | Inst::BodyEnd
            | Inst::Call { .. }
            | Inst::GroupEnd { .. }
            | Inst::Match
            | Inst::ClassOr { .. } => Flow::Fail,
        }
    }

    /// An instruction that moves within the program or changes what is
    /// remembered.
    ///
    /// The run loop sends every other instruction to the function that takes
    /// it.
    fn control(&mut self, inst: &Inst, pc: usize, pos: usize) -> Flow {
        match inst {
            Inst::Split { primary, secondary } => {
                self.scratch.stack.push(Entry::Branch {
                    pc: *secondary,
                    pos: as_u32(pos),
                });
                Flow::Go(as_usize(*primary), pos)
            }
            Inst::ClassOr {
                class,
                secondary,
                lead,
            } => {
                let decoded = decode(self.text, pos);
                let byte = self.text.get(pos).copied();
                let next = || decoded.and_then(|(_, width)| self.text.get(pos + width).copied());
                if self
                    .program
                    .leads
                    .get(as_usize(*lead))
                    .is_some_and(|lead| lead.admits(byte, next))
                {
                    self.scratch.stack.push(Entry::Branch {
                        pc: *secondary,
                        pos: as_u32(pos),
                    });
                }
                let table = self.program.classes.get(as_usize(*class));
                match decoded {
                    Some((code, width)) if table.is_some_and(|t| t.matches(code)) => {
                        Flow::Go(pc + 1, pos + width)
                    }
                    _ => Flow::Fail,
                }
            }
            Inst::Jump(target) => Flow::Go(as_usize(*target), pos),
            Inst::Save(slot) => {
                let index = as_usize(*slot);
                if let Some(old) = self.scratch.slots.get(index).copied() {
                    self.scratch.stack.push(Entry::Slot { index: *slot, old });
                    self.scratch.slots[index] = as_u32(pos);
                }
                Flow::Go(pc + 1, pos)
            }
            Inst::Assert(anchor) => {
                if self.assert_at(*anchor, pos) {
                    Flow::Go(pc + 1, pos)
                } else {
                    Flow::Fail
                }
            }
            Inst::Mark(reg) => {
                let index = as_usize(*reg);
                if let Some(old) = self.scratch.regs.get(index).copied() {
                    self.scratch.stack.push(Entry::Reg { index: *reg, old });
                    self.scratch.regs[index] = as_u32(pos);
                }
                Flow::Go(pc + 1, pos)
            }
            Inst::Progress { reg, exit } => {
                let same = self.scratch.regs.get(as_usize(*reg)).copied() == Some(as_u32(pos));
                Flow::Go(if same { as_usize(*exit) } else { pc + 1 }, pos)
            }
            Inst::Call { group } => {
                let start = self.program.group_start.get(as_usize(*group)).copied();
                match start {
                    Some(start) if self.scratch.calls.len() < MAX_CALLS => {
                        self.scratch.calls.push((*group, as_u32(pc + 1)));
                        self.scratch.stack.push(Entry::Called);
                        Flow::Go(as_usize(start), pos)
                    }
                    _ => Flow::Fail,
                }
            }
            Inst::GroupEnd { group } => match self.scratch.calls.last().copied() {
                Some((called, ret)) if called == *group => {
                    self.scratch.calls.pop();
                    self.scratch
                        .stack
                        .push(Entry::Returned { group: called, ret });
                    Flow::Go(as_usize(ret), pos)
                }
                _ => Flow::Go(pc + 1, pos),
            },
            Inst::Lit(_)
            | Inst::Any(_)
            | Inst::Class(_)
            | Inst::Look { .. }
            | Inst::Atomic { .. }
            | Inst::BodyEnd
            | Inst::Backref { .. }
            | Inst::Match => Flow::Fail,
        }
    }

    /// A lookaround, an atomic group, the end of one, or the end of the
    /// pattern.
    ///
    /// The run loop sends every other instruction to the function that takes
    /// it.
    fn nested(&mut self, inst: &Inst, pc: usize, pos: usize, target: Option<usize>) -> Flow {
        match inst {
            Inst::Look {
                ahead,
                negative,
                end,
                min,
                max,
            } => {
                let mark = self.scratch.stack.len();
                let found = if *ahead {
                    self.run(pc + 1, pos, None).is_some()
                } else {
                    self.look_behind(pc + 1, pos, *min, *max)
                };
                if *self.steps == 0 {
                    return Flow::Exhausted;
                }
                if found == *negative {
                    if found {
                        self.unwind(mark);
                    }
                    return Flow::Fail;
                }
                if found {
                    self.commit(mark);
                }
                Flow::Go(as_usize(*end), pos)
            }
            Inst::Atomic { end } => {
                let mark = self.scratch.stack.len();
                match self.run(pc + 1, pos, None) {
                    Some(after) => {
                        self.commit(mark);
                        Flow::Go(as_usize(*end), after)
                    }
                    None if *self.steps == 0 => Flow::Exhausted,
                    None => Flow::Fail,
                }
            }
            Inst::BodyEnd => {
                if target.is_none_or(|t| t == pos) {
                    Flow::Done(pos)
                } else {
                    Flow::Fail
                }
            }
            Inst::Match => Flow::Done(pos),
            Inst::Lit(_)
            | Inst::Any(_)
            | Inst::Class(_)
            | Inst::Split { .. }
            | Inst::Jump(_)
            | Inst::Save(_)
            | Inst::Assert(_)
            | Inst::Mark(_)
            | Inst::Progress { .. }
            | Inst::Backref { .. }
            | Inst::Call { .. }
            | Inst::GroupEnd { .. }
            | Inst::ClassOr { .. } => Flow::Fail,
        }
    }

    /// Pops entries above `base` until an alternative is found, undoing the
    /// changes on the way.
    fn backtrack(&mut self, base: usize) -> Option<(usize, usize)> {
        while self.scratch.stack.len() > base {
            let entry = self.scratch.stack.pop()?;
            if let Entry::Branch { pc, pos } = entry {
                return Some((as_usize(pc), as_usize(pos)));
            }
            self.undo(entry);
        }
        None
    }

    fn undo(&mut self, entry: Entry) {
        match entry {
            Entry::Branch { .. } => {}
            Entry::Slot { index, old } => {
                if let Some(slot) = self.scratch.slots.get_mut(as_usize(index)) {
                    *slot = old;
                }
            }
            Entry::Reg { index, old } => {
                if let Some(reg) = self.scratch.regs.get_mut(as_usize(index)) {
                    *reg = old;
                }
            }
            Entry::Called => {
                self.scratch.calls.pop();
            }
            Entry::Returned { group, ret } => self.scratch.calls.push((group, ret)),
        }
    }

    /// Undoes everything above `base`.
    fn unwind(&mut self, base: usize) {
        while self.scratch.stack.len() > base {
            if let Some(entry) = self.scratch.stack.pop() {
                self.undo(entry);
            }
        }
    }

    /// Drops the alternatives above `mark`, keeping what can be undone.
    /// A finished lookaround or atomic group then cannot be backtracked into.
    fn commit(&mut self, mark: usize) {
        let stack = &mut self.scratch.stack;
        let mut write = mark;
        for read in mark..stack.len() {
            let entry = stack[read];
            if !matches!(entry, Entry::Branch { .. }) {
                stack[write] = entry;
                write += 1;
            }
        }
        stack.truncate(write);
    }

    /// A lookbehind: the body, run from each start that puts its end at
    /// `pos`, shortest first.
    fn look_behind(&mut self, body: usize, pos: usize, min: u32, max: Option<u32>) -> bool {
        let mut start = pos;
        let mut chars = 0_u32;
        loop {
            if chars >= min && self.run(body, start, Some(pos)).is_some() {
                return true;
            }
            if *self.steps == 0 || max.is_some_and(|m| chars >= m) {
                return false;
            }
            match decode_before(self.text, start) {
                Some((_, before)) => {
                    start = before;
                    chars += 1;
                }
                None => return false,
            }
        }
    }

    /// Where the text the group matched ends when it repeats at `pos`.
    ///
    /// As in Oniguruma, the repeat is read within as many bytes as the group
    /// took.
    /// Under `(?i)`, each character of the group matches one of its orbit
    /// there.
    /// The repeat ends where the group's characters run out, which may be
    /// before that window ends.
    /// For example, `(?i)(ſ)\1` takes `ſs`.
    /// `(?i)(s)\1` refuses `sſ`, whose `ſ` does not fit in one byte.
    fn backref(&self, group: usize, case: Case, pos: usize) -> Option<usize> {
        let start = *self.scratch.slots.get(group * 2)?;
        let end = *self.scratch.slots.get(group * 2 + 1)?;
        if start == UNSET || end == UNSET {
            return None;
        }
        let word = self.text.get(as_usize(start)..as_usize(end))?;
        let window = self.text.get(pos..pos + word.len())?;
        if word == window {
            return Some(pos + word.len());
        }
        if case == Case::Sensitive {
            return None;
        }
        let mut at = 0;
        for c in std::str::from_utf8(word).ok()?.chars() {
            let (code, width) = decode(window, at)?;
            if !orbit(c).members().contains(&char::from_u32(code)?) {
                return None;
            }
            at += width;
        }
        Some(pos + at)
    }

    /// `^` matches at the start of the text and after a newline.
    /// As in Oniguruma, it does not match at the end of the text after a
    /// final newline.
    fn assert_at(&self, anchor: Anchor, pos: usize) -> bool {
        let len = self.text.len();
        match anchor {
            Anchor::Caret => pos == 0 || (self.text.get(pos - 1) == Some(&b'\n') && pos < len),
            Anchor::Dollar => pos == len || self.text.get(pos) == Some(&b'\n'),
            Anchor::StartText => pos == 0 && self.context.text_start,
            Anchor::EndText => pos == len,
            Anchor::EndTextNewline => {
                pos == len || (pos + 1 == len && self.text.get(pos) == Some(&b'\n'))
            }
            Anchor::Search => self.context.search == Some(pos),
            Anchor::WordBoundary | Anchor::NotWordBoundary => {
                let before = decode_before(self.text, pos)
                    .and_then(|(code, _)| char::from_u32(code))
                    .is_some_and(is_word);
                let after = decode(self.text, pos)
                    .and_then(|(code, _)| char::from_u32(code))
                    .is_some_and(is_word);
                (before != after) == (anchor == Anchor::WordBoundary)
            }
        }
    }
}
