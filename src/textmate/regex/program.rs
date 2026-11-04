//! The instructions a pattern compiles to, and the compiler.
//!
//! A pattern becomes a flat list of instructions that a backtracking machine
//! runs with an explicit stack, so no pattern can overflow the call stack and
//! every step can be counted.

use std::collections::HashMap;

use super::analysis::{first_bytes, second_bytes, width};
use super::ast::{Anchor, Case, Class, Item, Kind, Node};
use super::parse::{Error, Parsed};

/// The most instructions a pattern may compile to, which bounds the cost of
/// unrolling counted repeats.
const MAX_INSTS: usize = 40_000;

/// One instruction. Jump targets are indexes into the program.
#[derive(Clone, Debug)]
pub(super) enum Inst {
    /// These bytes, exactly.
    Lit(Box<[u8]>),
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
    /// One character of the class at `class`, as [`Inst::Class`], after
    /// leaving `secondary` to try on failure when the lead at `lead` says it
    /// can start here: the alternatives a case fold adds, the first inline
    /// and the rest out of line.
    ClassOr {
        class: u32,
        secondary: u32,
        lead: u32,
    },
}

/// A character class with its answer for ASCII precomputed.
#[derive(Debug)]
pub(super) struct ClassTable {
    ascii: u128,
    class: Class,
}

impl ClassTable {
    fn new(class: Class) -> ClassTable {
        let mut ascii = 0_u128;
        for b in 0..128_u8 {
            if class.matches(char::from(b)) {
                ascii |= 1 << b;
            }
        }
        ClassTable { ascii, class }
    }

    /// Whether the code point is in the class.
    pub(super) fn matches(&self, c: u32) -> bool {
        if c < 128 {
            self.ascii >> c & 1 == 1
        } else {
            char::from_u32(c).is_some_and(|c| self.class.matches(c))
        }
    }
}

/// The bytes a branch can start with, and when it is known, the lead bytes
/// of its second character.
#[derive(Clone, Debug)]
pub(super) struct Lead {
    first: [u64; 4],
    /// Whether the branch can match empty, and so start anywhere.
    empty: bool,
    second: Option<[u64; 4]>,
}

impl Lead {
    /// The bytes `node` can start with, and the lead bytes of its second
    /// character when [`second_bytes`] knows them.
    fn of(node: &Node) -> Lead {
        let mut first = [false; 256];
        let empty = first_bytes(node, &mut first);
        let mut second = [false; 256];
        let known = second_bytes(node, &mut second);
        Lead {
            first: bits(&first),
            empty,
            second: known.then(|| bits(&second)),
        }
    }

    /// The bytes either branch can start with.
    fn union(&self, other: &Lead) -> Lead {
        let or = |a: &[u64; 4], b: &[u64; 4]| std::array::from_fn(|i| a[i] | b[i]);
        Lead {
            first: or(&self.first, &other.first),
            empty: self.empty || other.empty,
            second: self.second.zip(other.second).map(|(a, b)| or(&a, &b)),
        }
    }

    /// Whether the branch can start where the text holds `byte`, or at its
    /// end when there is none; `next` gives the lead byte of the character
    /// after, read only when the second character is known.
    #[inline]
    pub(super) fn admits(&self, byte: Option<u8>, next: impl FnOnce() -> Option<u8>) -> bool {
        if self.empty {
            return true;
        }
        byte.is_some_and(|byte| holds(&self.first, byte))
            && self
                .second
                .is_none_or(|second| next().is_some_and(|next| holds(&second, next)))
    }
}

/// The lead of each suffix of `alts`: the first covers them all, the last
/// only the last.
fn suffix_leads(alts: &[Node]) -> Vec<Lead> {
    let mut leads: Vec<Lead> = Vec::with_capacity(alts.len());
    for alt in alts.iter().rev() {
        let lead = Lead::of(alt);
        leads.push(match leads.last() {
            Some(after) => lead.union(after),
            None => lead,
        });
    }
    leads.reverse();
    leads
}

/// The class a branch starts with, as one character of it, and the nodes
/// after it.
fn leading_class(node: &Node) -> Option<(Class, &[Node])> {
    let one = |node: &Node| match node {
        Node::Class(class) => Some((**class).clone()),
        Node::Char(c) => Some(Class {
            negated: false,
            parts: vec![vec![Item::Range(*c, *c)]],
        }),
        Node::Empty
        | Node::Any(_)
        | Node::Seq(_)
        | Node::Alt(_)
        | Node::Folds(_)
        | Node::Repeat { .. }
        | Node::Group { .. }
        | Node::Look { .. }
        | Node::Atomic(_)
        | Node::Backref(..)
        | Node::NamedBackref(..)
        | Node::Call(_)
        | Node::NamedCall(_)
        | Node::Anchor(_) => None,
    };
    match node {
        Node::Seq(items) => {
            let (first, rest) = items.split_first()?;
            one(first).map(|class| (class, rest))
        }
        Node::Empty
        | Node::Char(_)
        | Node::Any(_)
        | Node::Class(_)
        | Node::Alt(_)
        | Node::Folds(_)
        | Node::Repeat { .. }
        | Node::Group { .. }
        | Node::Look { .. }
        | Node::Atomic(_)
        | Node::Backref(..)
        | Node::NamedBackref(..)
        | Node::Call(_)
        | Node::NamedCall(_)
        | Node::Anchor(_) => one(node).map(|class| (class, &[][..])),
    }
}

/// Alternatives a case fold adds that wait to be compiled after the program:
/// they start at the `secondary` of the instruction at `at`, carry the lead
/// of each of their suffixes, and go back to `back` when one matches.
struct Deferred {
    at: u32,
    alts: Vec<Node>,
    leads: Vec<Lead>,
    back: u32,
}

/// A set of bytes as four words of bits.
fn bits(set: &[bool; 256]) -> [u64; 4] {
    let mut words = [0_u64; 4];
    for (byte, _) in set.iter().enumerate().filter(|&(_, &member)| member) {
        if let Some(word) = words.get_mut(byte >> 6) {
            *word |= 1 << (byte & 63);
        }
    }
    words
}

/// Whether `byte` is in the set.
fn holds(words: &[u64; 4], byte: u8) -> bool {
    words
        .get(usize::from(byte >> 6))
        .is_some_and(|word| word >> (byte & 63) & 1 == 1)
}

/// A compiled pattern.
#[derive(Debug)]
pub(super) struct Program {
    pub(super) insts: Vec<Inst>,
    pub(super) classes: Vec<ClassTable>,
    /// The leads of the `ClassOr` instructions.
    pub(super) leads: Vec<Lead>,
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
        leads: Vec::new(),
        deferred: Vec::new(),
        regs: 0,
        group_start: vec![0; parsed.groups + 1],
        names: &parsed.names,
        groups: parsed.groups,
        calls: has_calls(&parsed.root),
    };
    compiler.node(&parsed.root)?;
    compiler.push(Inst::Match)?;
    while let Some(deferred) = compiler.deferred.pop() {
        let start = compiler.pc();
        compiler.patch(deferred.at, start);
        compiler.fold_chain(&deferred.alts, &deferred.leads)?;
        compiler.push(Inst::Jump(deferred.back))?;
    }
    Ok(Program {
        insts: compiler.insts,
        classes: compiler.classes,
        leads: compiler.leads,
        groups: parsed.groups,
        regs: compiler.regs,
        group_start: compiler.group_start,
    })
}

/// Whether the pattern calls a group, so that groups need their return.
fn has_calls(node: &Node) -> bool {
    match node {
        Node::Call(_) | Node::NamedCall(_) => true,
        Node::Seq(items) | Node::Alt(items) | Node::Folds(items) => items.iter().any(has_calls),
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
    leads: Vec<Lead>,
    deferred: Vec<Deferred>,
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
            Inst::Split { secondary, .. } | Inst::ClassOr { secondary, .. } => {
                *secondary = target;
            }
            Inst::Jump(to) => *to = target,
            Inst::Progress { exit, .. } => *exit = target,
            Inst::Look { end, .. } | Inst::Atomic { end } => *end = target,
            Inst::Lit(_)
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
            Node::Class(class) => {
                let id = self.class_table((**class).clone());
                self.push(Inst::Class(id)).map(drop)
            }
            Node::Seq(items) => self.seq(items),
            Node::Alt(alts) => self.alt(alts),
            Node::Folds(alts) => self.fold_chain(alts, &suffix_leads(alts)),
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
            if let Node::Char(c) = item {
                let mut buffer = [0; 4];
                literal.extend_from_slice(c.encode_utf8(&mut buffer).as_bytes());
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

    /// The index of a new table for `class`.
    fn class_table(&mut self, class: Class) -> u32 {
        let id = u32::try_from(self.classes.len()).unwrap_or(u32::MAX);
        self.classes.push(ClassTable::new(class));
        id
    }

    /// The alternatives a case fold adds, whose suffixes have the leads in
    /// `leads`. The first starts with a [`Inst::ClassOr`] and runs inline;
    /// the others are compiled after the program and entered only where
    /// their lead admits the text, so a fold the text does not hold costs
    /// nothing over the first alternative alone. A first alternative that
    /// does not start with a class compiles as a plain alternation.
    fn fold_chain(&mut self, alts: &[Node], leads: &[Lead]) -> Result<(), Error> {
        let (Some((first, rest)), Some(rest_leads)) = (alts.split_first(), leads.get(1..)) else {
            return Ok(());
        };
        let Some(lead) = rest_leads.first() else {
            return self.node(first);
        };
        let Some((class, tail)) = leading_class(first) else {
            return self.alt(alts);
        };
        let class = self.class_table(class);
        let index = u32::try_from(self.leads.len()).unwrap_or(u32::MAX);
        self.leads.push(lead.clone());
        let at = self.push(Inst::ClassOr {
            class,
            secondary: 0,
            lead: index,
        })?;
        self.seq(tail)?;
        let back = self.pc();
        self.deferred.push(Deferred {
            at,
            alts: rest.to_vec(),
            leads: rest_leads.to_vec(),
            back,
        });
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
