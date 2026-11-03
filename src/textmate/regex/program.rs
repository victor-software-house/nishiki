//! The instructions a pattern compiles to, and the compiler.
//!
//! A pattern becomes a flat list of instructions that a backtracking machine
//! runs with an explicit stack, so no pattern can overflow the call stack and
//! every step can be counted.

use std::collections::HashMap;

use super::analysis::width;
use super::ast::{Anchor, Case, Class, Kind, Node, fold};
use super::parse::{Error, Parsed};

/// The most instructions a pattern may compile to, which bounds the cost of
/// unrolling counted repeats.
const MAX_INSTS: usize = 40_000;

/// One instruction. Jump targets are indexes into the program.
#[derive(Clone, Debug)]
pub(super) enum Inst {
    /// These bytes, exactly.
    Lit(Box<[u8]>),
    /// One character, in either case.
    Fold(char),
    /// Any character; a newline too when the flag is set.
    Any(bool),
    /// One character of the class at this index.
    Class(u32),
    /// Try `primary`, and on failure `secondary`.
    Split { primary: u32, secondary: u32 },
    /// Go to the target.
    Jump(u32),
    /// Store the position in a capture slot.
    Save(u32),
    /// Fail unless the position satisfies the anchor.
    Assert(Anchor),
    /// Store the position in a register, to see whether a loop advanced.
    Mark(u32),
    /// Leave the loop for `exit` when the position is still the register's.
    Progress { reg: u32, exit: u32 },
    /// A lookaround whose body runs from the next instruction to `end`.
    Look {
        ahead: bool,
        negative: bool,
        end: u32,
        min: u32,
        max: Option<u32>,
    },
    /// An atomic group whose body runs from the next instruction to `end`.
    Atomic { end: u32 },
    /// The end of a lookaround or atomic body.
    BodyEnd,
    /// The text a capture group matched.
    Backref { group: u32, case: Case },
    /// Run the group as a subroutine and come back.
    Call { group: u32 },
    /// The end of a group, where a call returns.
    GroupEnd { group: u32 },
    /// The pattern matched.
    Match,
}

/// A character class with its answer for ASCII precomputed.
#[derive(Debug)]
pub(super) struct ClassTable {
    ascii: u128,
    class: Class,
    case: Case,
}

impl ClassTable {
    fn new(class: Class, case: Case) -> ClassTable {
        let mut ascii = 0_u128;
        for b in 0..128_u8 {
            if class.matches(char::from(b), case) {
                ascii |= 1 << b;
            }
        }
        ClassTable { ascii, class, case }
    }

    /// Whether the code point is in the class.
    pub(super) fn matches(&self, c: u32) -> bool {
        if c < 128 {
            self.ascii >> c & 1 == 1
        } else {
            char::from_u32(c).is_some_and(|c| self.class.matches(c, self.case))
        }
    }
}

/// A compiled pattern.
#[derive(Debug)]
pub(super) struct Program {
    pub(super) insts: Vec<Inst>,
    pub(super) classes: Vec<ClassTable>,
    /// How many capture groups the pattern has.
    pub(super) groups: usize,
    /// How many loop registers it needs.
    pub(super) regs: usize,
    /// Where each group starts, for subroutine calls.
    pub(super) group_start: Vec<u32>,
}

/// Compiles a parsed pattern.
pub(super) fn compile(parsed: &Parsed) -> Result<Program, Error> {
    let mut compiler = Compiler {
        insts: Vec::new(),
        classes: Vec::new(),
        regs: 0,
        group_start: vec![0; parsed.groups + 1],
        names: &parsed.names,
        groups: parsed.groups,
        calls: has_calls(&parsed.root),
    };
    compiler.node(&parsed.root)?;
    compiler.push(Inst::Match)?;
    Ok(Program {
        insts: compiler.insts,
        classes: compiler.classes,
        groups: parsed.groups,
        regs: compiler.regs,
        group_start: compiler.group_start,
    })
}

/// Whether the pattern calls a group, so that groups need their return.
fn has_calls(node: &Node) -> bool {
    match node {
        Node::Call(_) | Node::NamedCall(_) => true,
        Node::Seq(items) | Node::Alt(items) => items.iter().any(has_calls),
        Node::Repeat { node, .. }
        | Node::Look { node, .. }
        | Node::Group { node, .. }
        | Node::Atomic(node) => has_calls(node),
        Node::Empty
        | Node::Char(..)
        | Node::Any(_)
        | Node::Class(..)
        | Node::Backref(..)
        | Node::NamedBackref(..)
        | Node::Anchor(_) => false,
    }
}

/// Whether the node can match without consuming a character, so that a loop
/// around it must check that it advances.
fn nullable(node: &Node) -> bool {
    width(node).0 == 0
}

struct Compiler<'a> {
    insts: Vec<Inst>,
    classes: Vec<ClassTable>,
    regs: usize,
    group_start: Vec<u32>,
    names: &'a HashMap<String, usize>,
    groups: usize,
    calls: bool,
}

fn err<T>(message: &str) -> Result<T, Error> {
    Err(Error(message.to_owned()))
}

impl Compiler<'_> {
    fn pc(&self) -> u32 {
        u32::try_from(self.insts.len()).unwrap_or(u32::MAX)
    }

    /// Adds an instruction and returns its index.
    fn push(&mut self, inst: Inst) -> Result<u32, Error> {
        if self.insts.len() >= MAX_INSTS {
            return err("pattern too large");
        }
        let at = self.pc();
        self.insts.push(inst);
        Ok(at)
    }

    /// Points the jump at `at` to `target`.
    fn patch(&mut self, at: u32, target: u32) {
        let Some(inst) = self
            .insts
            .get_mut(usize::try_from(at).unwrap_or(usize::MAX))
        else {
            return;
        };
        match inst {
            Inst::Split { secondary, .. } => *secondary = target,
            Inst::Jump(to) => *to = target,
            Inst::Progress { exit, .. } => *exit = target,
            Inst::Look { end, .. } | Inst::Atomic { end } => *end = target,
            Inst::Lit(_)
            | Inst::Fold(_)
            | Inst::Any(_)
            | Inst::Class(_)
            | Inst::Save(_)
            | Inst::Assert(_)
            | Inst::Mark(_)
            | Inst::BodyEnd
            | Inst::Backref { .. }
            | Inst::Call { .. }
            | Inst::GroupEnd { .. }
            | Inst::Match => {}
        }
    }

    /// Makes the `primary` of the split at `at` the next instruction.
    fn patch_primary(&mut self, at: u32, target: u32) {
        if let Some(Inst::Split { primary, .. }) = self
            .insts
            .get_mut(usize::try_from(at).unwrap_or(usize::MAX))
        {
            *primary = target;
        }
    }

    fn group_number(&self, name: &str) -> Result<u32, Error> {
        match self.names.get(name) {
            Some(&index) => Ok(u32::try_from(index).unwrap_or(u32::MAX)),
            None => err("undefined group name"),
        }
    }

    fn node(&mut self, node: &Node) -> Result<(), Error> {
        match node {
            Node::Empty => Ok(()),
            Node::Char(..) => self.seq(std::slice::from_ref(node)),
            Node::Any(dotall) => self.push(Inst::Any(*dotall)).map(drop),
            Node::Class(class, case) => {
                let id = u32::try_from(self.classes.len()).unwrap_or(u32::MAX);
                self.classes.push(ClassTable::new((**class).clone(), *case));
                self.push(Inst::Class(id)).map(drop)
            }
            Node::Seq(items) => self.seq(items),
            Node::Alt(alts) => self.alt(alts),
            Node::Repeat {
                node,
                min,
                max,
                kind,
            } => self.repeat(node, *min, *max, *kind),
            Node::Group { index, node } => self.group(*index, node),
            Node::Look {
                ahead,
                negative,
                node,
            } => {
                let (min, max) = width(node);
                let at = self.push(Inst::Look {
                    ahead: *ahead,
                    negative: *negative,
                    end: 0,
                    min: u32::try_from(min).unwrap_or(u32::MAX),
                    max: max.map(|m| u32::try_from(m).unwrap_or(u32::MAX)),
                })?;
                self.node(node)?;
                self.push(Inst::BodyEnd)?;
                let end = self.pc();
                self.patch(at, end);
                Ok(())
            }
            Node::Atomic(node) => self.atomic(node),
            Node::Backref(group, case) => {
                let group = u32::try_from(*group).unwrap_or(u32::MAX);
                self.backref(group, *case)
            }
            Node::NamedBackref(name, case) => {
                let group = self.group_number(name)?;
                self.backref(group, *case)
            }
            Node::Call(group) => {
                let group = u32::try_from(*group).unwrap_or(u32::MAX);
                self.call(group)
            }
            Node::NamedCall(name) => {
                let group = self.group_number(name)?;
                self.call(group)
            }
            Node::Anchor(anchor) => self.push(Inst::Assert(*anchor)).map(drop),
        }
    }

    fn backref(&mut self, group: u32, case: Case) -> Result<(), Error> {
        if usize::try_from(group).unwrap_or(usize::MAX) > self.groups {
            return err("invalid backreference");
        }
        self.push(Inst::Backref { group, case }).map(drop)
    }

    fn call(&mut self, group: u32) -> Result<(), Error> {
        if usize::try_from(group).unwrap_or(usize::MAX) > self.groups {
            return err("undefined group");
        }
        self.push(Inst::Call { group }).map(drop)
    }

    /// The nodes in order, with a run of plain characters as one literal.
    fn seq(&mut self, items: &[Node]) -> Result<(), Error> {
        let mut literal: Vec<u8> = Vec::new();
        for item in items {
            if let Node::Char(c, case) = item {
                if *case == Case::Sensitive || c.to_lowercase().eq(c.to_uppercase()) {
                    let mut buffer = [0; 4];
                    literal.extend_from_slice(c.encode_utf8(&mut buffer).as_bytes());
                    continue;
                }
                self.flush(&mut literal)?;
                self.push(Inst::Fold(fold(*c)))?;
                continue;
            }
            self.flush(&mut literal)?;
            self.node(item)?;
        }
        self.flush(&mut literal)
    }

    fn flush(&mut self, literal: &mut Vec<u8>) -> Result<(), Error> {
        if !literal.is_empty() {
            self.push(Inst::Lit(std::mem::take(literal).into_boxed_slice()))?;
        }
        Ok(())
    }

    fn alt(&mut self, alts: &[Node]) -> Result<(), Error> {
        let mut jumps = Vec::new();
        for (index, alt) in alts.iter().enumerate() {
            if index + 1 < alts.len() {
                let split = self.push(Inst::Split {
                    primary: 0,
                    secondary: 0,
                })?;
                let body = self.pc();
                self.patch_primary(split, body);
                self.node(alt)?;
                jumps.push(self.push(Inst::Jump(0))?);
                let next = self.pc();
                self.patch(split, next);
            } else {
                self.node(alt)?;
            }
        }
        let end = self.pc();
        for jump in jumps {
            self.patch(jump, end);
        }
        Ok(())
    }

    fn group(&mut self, index: Option<usize>, node: &Node) -> Result<(), Error> {
        let Some(index) = index else {
            return self.node(node);
        };
        let slot = u32::try_from(index * 2).unwrap_or(u32::MAX);
        let start = self.push(Inst::Save(slot))?;
        if let Some(entry) = self.group_start.get_mut(index) {
            *entry = start;
        }
        self.node(node)?;
        self.push(Inst::Save(slot + 1))?;
        if self.calls {
            self.push(Inst::GroupEnd {
                group: u32::try_from(index).unwrap_or(u32::MAX),
            })?;
        }
        Ok(())
    }

    fn atomic(&mut self, node: &Node) -> Result<(), Error> {
        let at = self.push(Inst::Atomic { end: 0 })?;
        self.node(node)?;
        self.push(Inst::BodyEnd)?;
        let end = self.pc();
        self.patch(at, end);
        Ok(())
    }

    /// A group repeated no times is still there to be called, so its code is
    /// emitted and jumped over.
    fn repeat(&mut self, node: &Node, min: u32, max: Option<u32>, kind: Kind) -> Result<(), Error> {
        if kind == Kind::Possessive {
            let at = self.push(Inst::Atomic { end: 0 })?;
            self.repeat(node, min, max, Kind::Greedy)?;
            self.push(Inst::BodyEnd)?;
            let end = self.pc();
            self.patch(at, end);
            return Ok(());
        }
        if max == Some(0) && self.calls {
            let jump = self.push(Inst::Jump(0))?;
            self.node(node)?;
            let end = self.pc();
            self.patch(jump, end);
            return Ok(());
        }
        let lazy = kind == Kind::Lazy;
        for _ in 0..min {
            self.node(node)?;
        }
        match max {
            None => self.star(node, kind),
            Some(max) => {
                let mut exits = Vec::new();
                for _ in min..max {
                    let split = self.push(Inst::Split {
                        primary: 0,
                        secondary: 0,
                    })?;
                    let body = self.pc();
                    if !lazy {
                        self.patch_primary(split, body);
                    }
                    exits.push((split, body, lazy));
                    self.node(node)?;
                }
                let end = self.pc();
                for (split, body, lazy) in exits {
                    if lazy {
                        if let Some(Inst::Split { primary, secondary }) = self
                            .insts
                            .get_mut(usize::try_from(split).unwrap_or(usize::MAX))
                        {
                            *primary = end;
                            *secondary = body;
                        }
                    } else {
                        self.patch(split, end);
                    }
                }
                Ok(())
            }
        }
    }

    /// `node*`, greedy or lazy.
    fn star(&mut self, node: &Node, kind: Kind) -> Result<(), Error> {
        let lazy = kind == Kind::Lazy;
        let check = nullable(node);
        let head = self.push(Inst::Split {
            primary: 0,
            secondary: 0,
        })?;
        let body = self.pc();
        let reg = if check {
            let reg = u32::try_from(self.regs).unwrap_or(u32::MAX);
            self.regs += 1;
            self.push(Inst::Mark(reg))?;
            Some(reg)
        } else {
            None
        };
        self.node(node)?;
        let progress = match reg {
            Some(reg) => Some(self.push(Inst::Progress { reg, exit: 0 })?),
            None => None,
        };
        self.push(Inst::Jump(head))?;
        let exit = self.pc();
        if let Some(progress) = progress {
            self.patch(progress, exit);
        }
        if let Some(Inst::Split { primary, secondary }) = self
            .insts
            .get_mut(usize::try_from(head).unwrap_or(usize::MAX))
        {
            if lazy {
                *primary = exit;
                *secondary = body;
            } else {
                *primary = body;
                *secondary = exit;
            }
        }
        Ok(())
    }
}
