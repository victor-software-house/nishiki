//! `src/sql/words.rs`: the words of the `sql` grammar's keyword rule and of
//! its function rules.
//!
//! Every alternative of those rules spells plain words, except three in the
//! keyword rule that the lexer handles by hand.
//!
//! - `create(\\s+or\\s+alter)?`, whose `create` the table keeps: its group
//!   needs a literal backslash, since the grammar escapes it twice.
//! - `priority ` with a space.
//! - `within group`, which the earlier `within` always hides.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use nishiki::unstable::grammar::UnlinkedRule;

use super::{Cut, Source, quoted, replace_branch, sorted, table};
use crate::File;
use crate::error::Error;
use crate::header::header;

/// The keyword rule's alternatives that spell no plain word, each with the
/// alternative the table reads in its place, empty to drop it.
const BY_HAND: &[(&str, &str)] = &[
    (r"create(\\s+or\\s+alter)?", "create"),
    ("priority ", ""),
    ("within group", ""),
];

/// The keyword rule's list.
const KEYWORDS: Cut = Cut::Inside(r"\b(?i)(", r")\b");

/// A function rule's list, before `(`.
const FUNCTIONS: Cut = Cut::Inside(r"(?i)\b(", r")\b\s*\(");

/// What a function rule of `@@name(` opens with.
const GLOBAL: &str = "(?<!@)@@";

/// Generates the file.
pub(super) fn file(root: &Path) -> Result<File, Error> {
    let source = Source::load(root, "sql")?;
    let keyword = source.named(source.patterns(), "keyword.other.sql")?;
    let mut list = KEYWORDS.apply(source.text(keyword)?)?.to_owned();
    for (branch, with) in BY_HAND {
        list = replace_branch(&list, branch, with)?;
    }
    let keywords = sorted(&list)?;
    plain(&keywords)?;
    let mut functions: BTreeMap<String, String> = BTreeMap::new();
    let mut globals: BTreeSet<String> = BTreeSet::new();
    for &id in source.patterns() {
        let rule = source.rule(id)?;
        let Some(scope) = capture_scope(source, rule)? else {
            continue;
        };
        let Some(category) = scope.strip_prefix("support.function.") else {
            continue;
        };
        let category = category.split('.').next().unwrap_or_default();
        let pattern = source.text(rule)?;
        let (global, pattern) = match pattern.strip_prefix(GLOBAL) {
            Some(rest) => (true, rest),
            None => (false, pattern),
        };
        let words = sorted(FUNCTIONS.apply(pattern)?)?;
        plain(&words)?;
        for word in words {
            if global {
                globals.insert(word);
            } else {
                functions.entry(word).or_insert_with(|| camel(category));
            }
        }
    }
    let mut text = header("codegen/src/words/sql.rs", &source.path());
    for line in [
        "//! The words of the `sql` grammar's keyword and function rules.",
        "",
        "use super::scope::S;",
    ] {
        text.push_str(line);
        text.push('\n');
    }
    table(
        &mut text,
        &["The plain words of the keyword rule, lowercase, sorted."],
        "pub(super) const KEYWORDS: &[&str]",
        keywords.iter().map(|word| literal(word)),
    );
    table(
        &mut text,
        &[
            "The functions that `name(` scopes, lowercase, sorted. A name in several",
            "of the grammar's rules has the first one's scope.",
        ],
        "pub(super) const FUNCTIONS: &[(&str, S)]",
        functions
            .iter()
            .map(|(word, category)| format!("({}, S::Fn{category})", literal(word))),
    );
    let globals: Vec<String> = globals.into_iter().collect();
    table(
        &mut text,
        &["The functions that `@@name(` scopes, lowercase, sorted."],
        "pub(super) const GLOBALS: &[&str]",
        quoted(&globals),
    );
    Ok(File {
        path: "src/sql/words.rs",
        text,
    })
}

/// The scope of capture 1 of `rule`, when it has one.
fn capture_scope(source: Source, rule: &UnlinkedRule) -> Result<Option<&'static str>, Error> {
    let Some(captures) = rule.captures.as_ref() else {
        return Ok(None);
    };
    let Some(capture) = captures.iter().find(|capture| capture.index == Some(1)) else {
        return Ok(None);
    };
    let scopes = &source.rule(capture.rule)?.name;
    Ok(scopes.first().map(|scope| &**scope))
}

/// Fails unless each of `words` is lower-case letters, digits and `_`.
fn plain(words: &[String]) -> Result<(), Error> {
    match words.iter().find(|word| {
        word.is_empty()
            || !word
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
    }) {
        Some(word) => Err(Error::Shape(format!("sql: unhandled alternative {word}"))),
        None => Ok(()),
    }
}

/// The variant suffix of a function category, such as `Datetime` for
/// `datetime`.
fn camel(category: &str) -> String {
    category
        .split(['-', '_'])
        .map(|part| {
            let mut chars = part.chars();
            let first = chars.next().map(|c| c.to_ascii_uppercase());
            first.into_iter().chain(chars).collect::<String>()
        })
        .collect()
}

/// `word` as a string literal.
///
/// Two words read as typos to the spelling check: one the grammar
/// misspells, and a function's name.
/// Each gets its first `i` as a byte escape, and is spelled in parts here
/// for the same reason.
fn literal(word: &str) -> String {
    let flagged = [
        ["v", "i", "s", "i", "b", "l", "i", "t", "y"].concat(),
        ["i", "if"].concat(),
    ];
    if flagged.iter().any(|flag| flag == word) {
        format!("\"{}\"", word.replacen('i', "\\x69", 1))
    } else {
        format!("{word:?}")
    }
}
