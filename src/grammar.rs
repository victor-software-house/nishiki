//! Grammars a caller brings: `TextMate` grammars in Shiki's JSON form, compiled
//! at run time or loaded from bytes a build script wrote.
//!
//! - [`Grammar`] is a handle to one compiled grammar, interned for the process.
//! - [`Grammars`] is the set an include can name.
//! - [`Highlighter::for_grammar`] links a grammar with a set into the engine's
//!   tables, interned for the process too.

mod bytes;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, LazyLock, Mutex, MutexGuard, PoisonError};
use std::{error, fmt};

use crate::highlighter::{Sealed, Syntax, lex_line};
use crate::language::Language;
use crate::textmate::compile::link::{LinkError, link};
use crate::textmate::compile::{CompileError, Compiled, compile};
use crate::{Highlighter, Limits, LineReport, State, Token, textmate};

/// A compiled `TextMate` grammar.
///
/// It is a handle: copying it copies no tables.
/// Two handles are equal when they name one compiled grammar.
///
/// Loading the same text twice returns the first handle.
/// A grammar loaded once from JSON and once from bytes gets two handles,
/// with the same output.
///
/// Every distinct grammar stays in memory until the process ends.
/// A service that loads grammars from untrusted input needs its own bound on
/// how many it loads.
#[derive(Clone, Copy)]
pub struct Grammar(&'static Compiled);

impl Grammar {
    /// Compiles `json`, a grammar in the form Shiki and VS Code use.
    ///
    /// A pattern the engine cannot compile is not an error.
    /// It never matches, as in vscode-textmate.
    ///
    /// # Errors
    ///
    /// When `json` is not JSON, or has no string `scopeName`.
    pub fn from_json(json: &str) -> Result<Grammar, GrammarError> {
        if let Some(&compiled) = loaded_grammars().json.get(json) {
            return Ok(Grammar(compiled));
        }
        let compiled = compile(json).map_err(|error| GrammarError(Reason::Compile(error)))?;
        let mut grammars = loaded_grammars();
        let interned = grammars
            .json
            .entry(json.into())
            .or_insert_with(|| Box::leak(Box::new(compiled)));
        Ok(Grammar(interned))
    }

    /// Loads a grammar from the bytes [`Grammar::to_bytes`] wrote.
    ///
    /// It parses no JSON.
    /// It checks every length and index before it trusts the tables.
    ///
    /// # Errors
    ///
    /// When the bytes come from another format version, are cut short, or
    /// hold a length or index out of range.
    pub fn from_bytes(bytes: &[u8]) -> Result<Grammar, GrammarError> {
        if let Some(&compiled) = loaded_grammars().bytes.get(bytes) {
            return Ok(Grammar(compiled));
        }
        let compiled = bytes::read(bytes).map_err(GrammarError)?;
        let mut grammars = loaded_grammars();
        let interned = grammars
            .bytes
            .entry(bytes.into())
            .or_insert_with(|| Box::leak(Box::new(compiled)));
        Ok(Grammar(interned))
    }

    /// The grammar's compiled form as bytes, for a build script to write and
    /// [`Grammar::from_bytes`] to load.
    ///
    /// The bytes hold no other grammar it includes.
    #[must_use]
    pub fn to_bytes(self) -> Vec<u8> {
        bytes::write(self.0)
    }

    /// The grammar's root scope, such as `source.zig`.
    #[must_use]
    pub fn scope(self) -> &'static str {
        &self.0.scope
    }
}

impl PartialEq for Grammar {
    fn eq(&self, other: &Grammar) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl Eq for Grammar {}

impl Hash for Grammar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.0, state);
    }
}

impl fmt::Debug for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Grammar").field(&self.0.scope).finish()
    }
}

/// The grammars an include can name, by root scope.
///
/// An include of a whole grammar resolves in this order:
///
/// 1. a language named with [`Grammars::hand_off`], whose lexer takes the
///    region, even when the set holds its grammar;
/// 2. a grammar in the set;
/// 3. with no language named, a language nishiki lexes by hand, such as
///    [`Language::Css`] for `source.css`, when its feature is on;
/// 4. else nothing: the include matches nothing, as in vscode-textmate.
///
/// An include of one rule of a grammar, such as `source.ruby#escaped_char`,
/// always resolves in the set.
#[derive(Clone, Debug, Default)]
pub struct Grammars {
    /// The grammars, in the order they were first inserted.
    grammars: Vec<Grammar>,
    /// The languages named with [`Grammars::hand_off`], in the order named;
    /// `None` before the first.
    handed: Option<Vec<Language>>,
}

impl Grammars {
    /// An empty set.
    #[must_use]
    pub fn new() -> Grammars {
        Grammars::default()
    }

    /// Adds `grammar` to the set.
    /// It returns the grammar with the same scope that it replaced, if any.
    pub fn insert(&mut self, grammar: Grammar) -> Option<Grammar> {
        if let Some(held) = self
            .grammars
            .iter_mut()
            .find(|held| held.scope() == grammar.scope())
        {
            Some(std::mem::replace(held, grammar))
        } else {
            self.grammars.push(grammar);
            None
        }
    }

    /// The grammar whose root scope is `scope`, if the set holds one.
    #[must_use]
    pub fn get(&self, scope: &str) -> Option<Grammar> {
        self.grammars
            .iter()
            .copied()
            .find(|grammar| grammar.scope() == scope)
    }

    /// Names `language` to lex by hand where a grammar includes its whole
    /// grammar.
    ///
    /// Once any language is named, only named languages are lexed by hand.
    /// [`Highlighter::for_grammar`] fails when nishiki has no grammar scope
    /// for `language`.
    pub fn hand_off(&mut self, language: Language) {
        let handed = self.handed.get_or_insert_with(Vec::new);
        if !handed.contains(&language) {
            handed.push(language);
        }
    }
}

impl FromIterator<Grammar> for Grammars {
    fn from_iter<I: IntoIterator<Item = Grammar>>(grammars: I) -> Grammars {
        let mut set = Grammars::new();
        set.extend(grammars);
        set
    }
}

impl Extend<Grammar> for Grammars {
    fn extend<I: IntoIterator<Item = Grammar>>(&mut self, grammars: I) {
        for grammar in grammars {
            let _ = self.insert(grammar);
        }
    }
}

/// A grammar that could not be loaded or linked.
#[derive(Debug)]
pub struct GrammarError(Reason);

/// What went wrong, kept private so new reasons break no caller.
#[derive(Debug)]
enum Reason {
    /// The JSON did not compile.
    Compile(CompileError),
    /// The bytes do not start with the format's magic.
    NotGrammarBytes,
    /// The bytes come from another format version.
    Version { found: u32, expected: u32 },
    /// The bytes are cut short, or hold a value out of range, at this offset.
    Malformed { at: usize },
    /// The grammar and the set did not link.
    Link(LinkError),
}

impl fmt::Display for GrammarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Reason::Compile(error) => write!(f, "the grammar did not compile: {error}"),
            Reason::NotGrammarBytes => f.write_str("the bytes are not a compiled grammar"),
            Reason::Version { found, expected } => write!(
                f,
                "the bytes are format version {found}, and this nishiki reads version {expected}"
            ),
            Reason::Malformed { at } => write!(f, "the bytes are malformed at offset {at}"),
            Reason::Link(error) => write!(f, "the grammar did not link: {error}"),
        }
    }
}

impl error::Error for GrammarError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.0 {
            Reason::Compile(error) => Some(error),
            Reason::Link(error) => Some(error),
            Reason::NotGrammarBytes | Reason::Version { .. } | Reason::Malformed { .. } => None,
        }
    }
}

/// The grammars loaded in this process, by the text they were loaded from.
#[derive(Default)]
struct LoadedGrammars {
    json: HashMap<Box<str>, &'static Compiled>,
    bytes: HashMap<Box<[u8]>, &'static Compiled>,
}

/// Every grammar this process loaded.
static LOADED: LazyLock<Mutex<LoadedGrammars>> = LazyLock::new(Mutex::default);

/// The loaded grammars, locked.
///
/// Each change to the maps is one insert.
/// A thread that panicked while it held the lock therefore left them whole.
/// A poisoned lock is still read.
fn loaded_grammars() -> MutexGuard<'static, LoadedGrammars> {
    LOADED.lock().unwrap_or_else(PoisonError::into_inner)
}

/// What a link reads: the root, every grammar of the set in order, and the
/// languages named for hand-offs.
///
/// The whole set is the key, not only the grammars the link reached.
/// A scope the link looked up and missed changes the tables too, as does a
/// grammar that injects itself into the root.
#[derive(PartialEq, Eq, Hash)]
struct LinkKey {
    root: Grammar,
    set: Vec<Grammar>,
    handed: Option<Vec<Language>>,
}

/// One key's linked table, empty until a link of that key succeeds.
///
/// A thread holds its lock while it links.
/// Another thread that links the same key waits for that table.
type LinkedTable = Arc<Mutex<Option<&'static textmate::Grammar>>>;

/// Every table this process linked, by what the link read.
static LINKED_TABLES: LazyLock<Mutex<HashMap<LinkKey, LinkedTable>>> =
    LazyLock::new(Mutex::default);

/// The engine's table for `root` linked with `grammars`, linked once per
/// process for each [`LinkKey`].
///
/// The lock on every key is held only to find or add the key's
/// [`LinkedTable`].
/// The link runs under that key's own lock.
/// Links of other keys then never wait for it.
/// Two links of one key end with one table, so no second table leaks.
/// A link that fails leaves the key empty, and the next one tries again.
fn linked_table(
    root: Grammar,
    grammars: &Grammars,
) -> Result<&'static textmate::Grammar, GrammarError> {
    let key = LinkKey {
        root,
        set: grammars.grammars.clone(),
        handed: grammars.handed.clone(),
    };
    let slot = Arc::clone(
        LINKED_TABLES
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .entry(key)
            .or_default(),
    );
    let mut table = slot.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(linked) = *table {
        return Ok(linked);
    }
    let set: Vec<&Compiled> = grammars.grammars.iter().map(|grammar| grammar.0).collect();
    let linked = link(root.0, &set, grammars.handed.as_deref())
        .map_err(|error| GrammarError(Reason::Link(error)))?;
    *table = Some(linked);
    Ok(linked)
}

/// What a grammar highlighter lexes with: its grammar, and the table linked
/// from it.
#[derive(Clone, Copy, Debug)]
pub(crate) struct GrammarLexer {
    grammar: Grammar,
    table: &'static textmate::Grammar,
}

impl Syntax for Grammar {}

impl Sealed for Grammar {
    type Lexer = GrammarLexer;

    fn start(lexer: GrammarLexer, arena: &mut crate::arena::Arena, base: u32) -> u32 {
        textmate::start(arena, lexer.table, base)
    }

    fn line(
        highlighter: &mut Highlighter<Grammar>,
        line: &str,
        state: &mut State,
        out: &mut Vec<Token>,
    ) -> LineReport {
        lex_line(highlighter, line, state, out, |lexer, cx, text, from| {
            textmate::lex_linked(cx, lexer.table, text, from)
        })
    }
}

impl Highlighter<Grammar> {
    /// A highlighter for `grammar` with [`Limits::default`], whose includes
    /// resolve in `grammars`.
    ///
    /// It links the grammar and the set into the engine's tables once per
    /// process.
    /// The set is not kept: a later change to it does not reach the
    /// highlighter.
    ///
    /// # Errors
    ///
    /// When the link fails, for example when [`Grammars::hand_off`] names a
    /// language no grammar can hand a region to.
    pub fn for_grammar(grammar: Grammar, grammars: &Grammars) -> Result<Self, GrammarError> {
        Self::for_grammar_with_limits(grammar, grammars, Limits::default())
    }

    /// A highlighter for `grammar` with `limits`, whose includes resolve in
    /// `grammars`.
    ///
    /// # Errors
    ///
    /// As [`Highlighter::for_grammar`].
    pub fn for_grammar_with_limits(
        grammar: Grammar,
        grammars: &Grammars,
        limits: Limits,
    ) -> Result<Self, GrammarError> {
        let table = linked_table(grammar, grammars)?;
        Ok(Self::with_lexer(GrammarLexer { grammar, table }, limits))
    }

    /// The grammar this highlighter lexes.
    #[must_use]
    pub fn grammar(&self) -> Grammar {
        self.lexer().grammar
    }
}

#[cfg(test)]
mod tests;
