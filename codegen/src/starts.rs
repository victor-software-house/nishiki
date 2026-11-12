//! The start tables of the rule-engine grammars, from
//! `nishiki::unstable::codegen::starts`.
//!
//! Each `src/<language>/grammar.rs` holds `Pattern::start`, which its
//! `oracle/<language>-grammar.ts` writes.
//! This module computes each pattern's start from `codegen/patterns/`.
//!
//! - It writes the starts to `codegen/starts/<language>.json`, which the script
//!   reads.
//! - It lists each start that differs from the table.
//!
//! The grammars of [`REWRITTEN`] get no start file.

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use nishiki::unstable::codegen::starts::{BackrefStart, Start, start, takes_one_blank};
use serde_json::{Map, Value};
use syn::{Expr, ImplItem, Item, Lit};

use crate::File;
use crate::blank_runs::{PATTERNS, names};
use crate::error::Error;
use crate::facts::read;

/// The directory of the start files, from the repository root.
const STARTS: &str = "codegen/starts";

/// The grammars whose scripts rewrite each expression before they read its
/// start.
///
/// - PHP's, Blade's and Hack's read a word table's first list as the whole
///   table.
/// - Julia's reads each identifier class as one the TypeScript reader takes.
///
/// Their scripts keep the TypeScript reader until their family slices port the
/// rewrites.
const REWRITTEN: [&str; 4] = ["blade", "hack", "julia", "php"];

/// The grammars whose back references start at any byte.
/// The others start at a quote.
const ANY_BACKREF: [&str; 20] = [
    "blade", "elixir", "gdscript", "haml", "hack", "haskell", "jinja", "latex", "lua", "nix",
    "perl", "php", "postcss", "proto", "r", "ruby", "sass", "tex", "vb", "zig",
];

/// A pattern's start as its grammar's table holds it.
#[derive(Debug, PartialEq, Eq)]
struct Row {
    /// The bytes, four words.
    bytes: [u64; 4],
    /// The places, by name.
    places: BTreeSet<String>,
    /// Whether a match can start where no byte follows.
    matches_at_end: bool,
}

impl From<Start> for Row {
    fn from(start: Start) -> Row {
        let places = [
            (start.places.away, "Away"),
            (start.places.line_start, "LineStart"),
            (start.places.in_word, "InWord"),
        ]
        .into_iter()
        .filter(|&(holds, _)| holds)
        .map(|(_, name)| name.to_owned())
        .collect();
        Row {
            bytes: start.bytes,
            places,
            matches_at_end: start.matches_at_end,
        }
    }
}

/// The starts `nishiki::unstable::codegen::starts` computes for one grammar.
#[derive(Debug)]
struct Computed {
    /// Each pattern's start, by id.
    starts: BTreeMap<String, Start>,
    /// Whether some pattern is tried inside a word at a scan's first position.
    words_at_start: bool,
    /// The patterns that take one blank, then a character that is not a
    /// blank.
    one_blank: BTreeSet<String>,
}

impl Computed {
    /// The starts of the patterns in `codegen/patterns/<language>.json`.
    fn read(root: &Path, language: &str) -> Result<Computed, Error> {
        let backref = if ANY_BACKREF.contains(&language) {
            BackrefStart::Any
        } else {
            BackrefStart::Quote
        };
        let path = format!("{PATTERNS}/{language}.json");
        let patterns = read(root, &path)?;
        let patterns = patterns
            .as_object()
            .ok_or_else(|| Error::Shape(format!("{path}: no object")))?;
        let mut computed = Computed {
            starts: BTreeMap::new(),
            words_at_start: false,
            one_blank: BTreeSet::new(),
        };
        for (id, pattern) in patterns {
            let regex = pattern
                .get("regex")
                .and_then(Value::as_str)
                .ok_or_else(|| Error::Shape(format!("{path}: {id} has no regex")))?;
            let start = start(regex, backref);
            computed.words_at_start |= start.in_word_first;
            if takes_one_blank(regex, backref) {
                computed.one_blank.insert(id.clone());
            }
            computed.starts.insert(id.clone(), start);
        }
        Ok(computed)
    }
}

/// The start file of each grammar outside [`REWRITTEN`].
///
/// It fails when `codegen/starts/` holds a file for another grammar.
pub(crate) fn files(root: &Path, languages: &BTreeSet<String>) -> Result<Vec<File>, Error> {
    let written = languages
        .iter()
        .filter(|language| !REWRITTEN.contains(&language.as_str()))
        .collect::<BTreeSet<_>>();
    let present = names(&root.join(STARTS), ".json")?;
    let extra = present
        .iter()
        .filter(|name| !written.contains(name))
        .collect::<Vec<_>>();
    if !extra.is_empty() {
        return Err(Error::Check(format!(
            "{STARTS}/ holds a file for {extra:?}, which gets none"
        )));
    }
    written
        .into_iter()
        .map(|language| {
            let path = format!("{STARTS}/{language}.json");
            let text = json(&path, &Computed::read(root, language)?)?;
            Ok(File { path, text })
        })
        .collect()
}

/// A start file's text: each pattern's start by id, then
/// `Pattern::WORDS_AT_START`.
///
/// A start's bytes and places are each one string, as a table lists them, so
/// that a start takes five lines.
/// It is indented by two spaces, as `JSON.stringify` writes the pattern files.
fn json(path: &str, computed: &Computed) -> Result<String, Error> {
    let mut starts = Map::new();
    for (id, start) in &computed.starts {
        let bytes = start.bytes.map(hex).join(", ");
        let places = Row::from(*start).places;
        let mut fields = Map::new();
        fields.insert("bytes".to_owned(), Value::from(bytes));
        fields.insert(
            "matches_at_end".to_owned(),
            Value::from(start.matches_at_end),
        );
        fields.insert(
            "places".to_owned(),
            Value::from(
                PLACE_NAMES
                    .into_iter()
                    .filter(|name| places.contains(*name))
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        );
        starts.insert(id.clone(), Value::Object(fields));
    }
    let mut file = Map::new();
    file.insert("starts".to_owned(), Value::Object(starts));
    file.insert(
        "words_at_start".to_owned(),
        Value::from(computed.words_at_start),
    );
    let mut text =
        serde_json::to_string_pretty(&Value::Object(file)).map_err(|source| Error::Json {
            path: PathBuf::from(path),
            source,
        })?;
    text.push('\n');
    Ok(text)
}

/// A word of a start's bytes as the tables write it, such as
/// `0x0000_0000_ffff_0000`.
fn hex(word: u64) -> String {
    let digits = format!("{word:016x}");
    let groups = digits
        .as_bytes()
        .chunks(4)
        .filter_map(|group| std::str::from_utf8(group).ok())
        .collect::<Vec<_>>();
    format!("0x{}", groups.join("_"))
}

/// The places, in the order a table lists them.
const PLACE_NAMES: [&str; 3] = ["Away", "LineStart", "InWord"];

/// Each start that differs from its grammar's table, one line each.
pub(crate) fn differences(root: &Path, languages: &BTreeSet<String>) -> Result<Vec<String>, Error> {
    let mut found = Vec::new();
    for language in languages {
        let computed = Computed::read(root, language)?;
        let table = Table::read(root, language)?;
        for (id, start) in &computed.starts {
            let ours = Row::from(*start);
            match table.rows.get(id) {
                Some(row) if *row == ours => {}
                Some(row) => found.push(format!(
                    "{language} {id}: the table holds {row:?}, codegen {ours:?}"
                )),
                None => found.push(format!("{language} {id}: no row in the table")),
            }
        }
        if computed.words_at_start != table.words_at_start {
            found.push(format!(
                "{language}: WORDS_AT_START is {}, codegen {}",
                table.words_at_start, computed.words_at_start
            ));
        }
        if let Some(listed) = table.one_blank
            && listed != computed.one_blank
        {
            found.push(format!(
                "{language}: takes_one_blank lists {:?} more and {:?} fewer than codegen",
                listed.difference(&computed.one_blank).collect::<Vec<_>>(),
                computed.one_blank.difference(&listed).collect::<Vec<_>>(),
            ));
        }
    }
    Ok(found)
}

/// What a grammar's `grammar.rs` holds of its starts.
#[derive(Debug)]
struct Table {
    /// Each pattern's row, by id.
    rows: BTreeMap<String, Row>,
    /// `Pattern::WORDS_AT_START`.
    words_at_start: bool,
    /// The patterns `Pattern::takes_one_blank` lists, when the grammar has it.
    one_blank: Option<BTreeSet<String>>,
}

impl Table {
    /// The table in `src/<language>/grammar.rs`.
    fn read(root: &Path, language: &str) -> Result<Table, Error> {
        let path = root.join("src").join(language).join("grammar.rs");
        let shape = |what: &str| Error::Shape(format!("{}: {what}", path.display()));
        let text = fs::read_to_string(&path).map_err(|source| Error::Io {
            path: path.clone(),
            source,
        })?;
        let file = syn::parse_file(&text).map_err(|error| shape(&error.to_string()))?;
        let mut table = Table {
            rows: BTreeMap::new(),
            words_at_start: false,
            one_blank: None,
        };
        let items = file.items.iter().filter_map(|item| {
            if let Item::Impl(block) = item {
                Some(block.items.iter())
            } else {
                None
            }
        });
        for item in items.flatten() {
            if let ImplItem::Fn(function) = item
                && function.sig.ident == "start"
            {
                table.rows = rows(&function.block).ok_or_else(|| shape("an unread start"))?;
            } else if let ImplItem::Fn(function) = item
                && function.sig.ident == "takes_one_blank"
            {
                table.one_blank = Some(listed(&function.block));
            } else if let ImplItem::Const(constant) = item
                && constant.ident == "WORDS_AT_START"
                && let Some(value) = boolean(&constant.expr)
            {
                table.words_at_start = value;
            }
        }
        Ok(table)
    }
}

/// The rows of `Pattern::start`, whose body ends in `match self`.
fn rows(block: &syn::Block) -> Option<BTreeMap<String, Row>> {
    let Some(syn::Stmt::Expr(Expr::Match(arms), None)) = block.stmts.last() else {
        return None;
    };
    let mut rows = BTreeMap::new();
    for arm in &arms.arms {
        let mut body = arm.body.as_ref();
        if let Expr::Block(block) = body
            && let Some(syn::Stmt::Expr(tail, None)) = block.block.stmts.last()
        {
            body = tail;
        }
        let Expr::Struct(body) = body else {
            return None;
        };
        let field = |name: &str| {
            body.fields.iter().find_map(|field| {
                if let syn::Member::Named(ident) = &field.member
                    && ident == name
                {
                    Some(&field.expr)
                } else {
                    None
                }
            })
        };
        let Expr::Array(words) = field("bytes")? else {
            return None;
        };
        let words = words
            .elems
            .iter()
            .map(|word| {
                if let Expr::Lit(syn::ExprLit {
                    lit: Lit::Int(int), ..
                }) = word
                {
                    int.base10_parse::<u64>().ok()
                } else {
                    None
                }
            })
            .collect::<Option<Vec<_>>>()?;
        let bytes = <[u64; 4]>::try_from(words).ok()?;
        let places = places(field("places")?)?;
        let matches_at_end = boolean(field("matches_at_end")?)?;
        for id in alternatives(&arm.pat) {
            rows.insert(
                id,
                Row {
                    bytes,
                    places: places.clone(),
                    matches_at_end,
                },
            );
        }
    }
    Some(rows)
}

/// The places a `PlaceSet::of(&[Place::Away, …])` call names, such as `Away`.
fn places(expr: &Expr) -> Option<BTreeSet<String>> {
    let Expr::Call(call) = expr else {
        return None;
    };
    let Some(Expr::Reference(reference)) = call.args.first() else {
        return None;
    };
    let Expr::Array(array) = reference.expr.as_ref() else {
        return None;
    };
    array
        .elems
        .iter()
        .map(|place| {
            let Expr::Path(path) = place else {
                return None;
            };
            path.path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
        })
        .collect()
}

/// The variant names an arm's pattern lists, such as `A` in `Pattern::A`.
fn alternatives(pat: &syn::Pat) -> Vec<String> {
    if let syn::Pat::Or(or) = pat {
        return or.cases.iter().flat_map(alternatives).collect();
    }
    let syn::Pat::Path(path) = pat else {
        return Vec::new();
    };
    path.path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
        .into_iter()
        .collect()
}

/// The variants a `matches!(self, A | B)` body lists.
fn listed(block: &syn::Block) -> BTreeSet<String> {
    let last = block.stmts.last();
    let text = if let Some(syn::Stmt::Macro(call)) = last {
        call.mac.tokens.to_string()
    } else if let Some(syn::Stmt::Expr(Expr::Macro(call), None)) = last {
        call.mac.tokens.to_string()
    } else {
        String::new()
    };
    text.split("Pattern ::")
        .skip(1)
        .filter_map(|rest| rest.split_whitespace().next())
        .map(str::to_owned)
        .collect()
}

/// The value of a `true` or `false` literal.
fn boolean(expr: &Expr) -> Option<bool> {
    if let Expr::Lit(syn::ExprLit {
        lit: Lit::Bool(value),
        ..
    }) = expr
    {
        Some(value.value)
    } else {
        None
    }
}
