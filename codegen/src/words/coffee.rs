//! `src/coffee/words.rs`: the words of the `coffee` grammar's big
//! alternations, sorted, so a matcher can test a whole word with a binary
//! search.
//!
//! `keyword.control` loses its `(?<=for)\s+own` branch, which a matcher
//! tests apart.
//! The methods' `support.function` loses its `plugins.refresh` branch, whose
//! `.` takes any character.

use std::path::Path;

use nishiki::unstable::grammar::RuleId;

use super::{Cut, Source, bytes, replace_branch, sorted, table};
use crate::File;
use crate::error::Error;
use crate::header::header;

/// The pattern list a table's rule sits in.
#[derive(Clone, Copy, Debug)]
enum Patterns {
    /// The grammar's top-level patterns.
    Root,
    /// The patterns of repository entry `method_names`.
    Methods,
    /// The patterns of repository entry `function_names`.
    Functions,
}

/// One table of the file.
#[derive(Debug)]
struct List {
    /// The table's name.
    name: &'static str,
    /// The table's doc line.
    doc: &'static str,
    /// The pattern list that holds its rule.
    patterns: Patterns,
    /// The rule's scope, which no other rule of the list has.
    scope: &'static str,
    /// Where the list sits in the rule's pattern.
    cut: Cut,
    /// The branches the table leaves out.
    dropped: &'static [&'static str],
    /// The text each word of the table starts with, which the list leaves out.
    lead: &'static str,
}

/// A list that is the whole of `\b(…)\b`.
const PLAIN: Cut = Cut::Inside(r"\b(", r")\b");

/// The tables, in the order the file holds them.
const LISTS: &[List] = &[
    List {
        name: "KEYWORD_CONTROL",
        doc: "The words of `keyword.control`.",
        patterns: Patterns::Root,
        scope: "keyword.control.coffee",
        cut: Cut::Inside(r"\b(?<![$.])(", r")(?!\s*:)\b"),
        dropped: &[r"(?<=for)\s+own"],
        lead: "",
    },
    List {
        name: "RESERVED",
        doc: "The words of `keyword.reserved`.",
        patterns: Patterns::Root,
        scope: "keyword.reserved.coffee",
        cut: Cut::Inside(r"\b(?<![$.])(", r")(?!\s*:)\b"),
        dropped: &[],
        lead: "",
    },
    List {
        name: "SUPPORT_CLASSES",
        doc: "The words of `support.class`.",
        patterns: Patterns::Root,
        scope: "support.class.coffee",
        cut: PLAIN,
        dropped: &[],
        lead: "",
    },
    List {
        name: "CONSOLE_METHODS",
        doc: "The words of `support.function.console`.",
        patterns: Patterns::Root,
        scope: "support.function.console.coffee",
        cut: Cut::Inside(r"((?<=console\.)(", r"))\b"),
        dropped: &[],
        lead: "",
    },
    List {
        name: "ARRAY_METHODS",
        doc: "The words of `support.function.method.array`.",
        patterns: Patterns::Root,
        scope: "support.function.method.array.coffee",
        cut: Cut::Inside(r"((?<=\.)(", r"))\b"),
        dropped: &[],
        lead: "",
    },
    List {
        name: "OBJECT_STATICS",
        doc: "The words of `support.function.static.object`.",
        patterns: Patterns::Root,
        scope: "support.function.static.object.coffee",
        cut: Cut::Inside(r"((?<=Object\.)(", r"))\b"),
        dropped: &[],
        lead: "",
    },
    List {
        name: "MATH_STATICS",
        doc: "The words of `support.function.static.math`.",
        patterns: Patterns::Root,
        scope: "support.function.static.math.coffee",
        cut: Cut::Inside(r"((?<=Math\.)(", r"))\b"),
        dropped: &[],
        lead: "",
    },
    List {
        name: "NUMBER_STATICS",
        doc: "The words of `support.function.static.number`.",
        patterns: Patterns::Root,
        scope: "support.function.static.number.coffee",
        cut: Cut::Inside(r"((?<=Number\.)(", r"))\b"),
        dropped: &[],
        lead: "",
    },
    List {
        name: "EVENT_HANDLERS",
        doc: "The words of `support.function.event-handler`, with their `on`.",
        patterns: Patterns::Methods,
        scope: "support.function.event-handler.coffee",
        cut: Cut::Inside(r"\bon(", r")\b"),
        dropped: &[],
        lead: "on",
    },
    List {
        name: "SUPPORT_METHODS",
        doc: "The words of the methods' `support.function`, without `plugins.refresh`.",
        patterns: Patterns::Methods,
        scope: "support.function.coffee",
        cut: PLAIN,
        dropped: &["plugins.refresh"],
        lead: "",
    },
    List {
        name: "DOM_METHODS",
        doc: "The words of `support.function.dom`.",
        patterns: Patterns::Methods,
        scope: "support.function.dom.coffee",
        cut: PLAIN,
        dropped: &[],
        lead: "",
    },
    List {
        name: "SUPPORT_FUNCTIONS",
        doc: "The words of the functions' `support.function`.",
        patterns: Patterns::Functions,
        scope: "support.function.coffee",
        cut: PLAIN,
        dropped: &[],
        lead: "",
    },
];

/// Generates the file.
pub(super) fn file(root: &Path) -> Result<File, Error> {
    let source = Source::load(root, "coffee")?;
    let methods: &[RuleId] = &source.entry("method_names")?.patterns;
    let functions: &[RuleId] = &source.entry("function_names")?.patterns;
    let mut text = header("codegen/src/words/coffee.rs", &source.path());
    text.push_str("//! The words the grammar's alternations spell out, sorted.\n");
    for list in LISTS {
        let patterns = match list.patterns {
            Patterns::Root => source.patterns(),
            Patterns::Methods => methods,
            Patterns::Functions => functions,
        };
        let rule = source.named(patterns, list.scope)?;
        let mut body = list.cut.apply(source.text(rule)?)?.to_owned();
        for branch in list.dropped {
            body = replace_branch(&body, branch, "")?;
        }
        let words: Vec<String> = sorted(&body)?
            .into_iter()
            .map(|word| format!("{}{word}", list.lead))
            .collect();
        if let Some(word) = words.iter().find(|word| !is_word(word)) {
            return Err(Error::Shape(format!("{}: {word} is not \\w+", list.name)));
        }
        let declaration = format!("pub(super) const {}: &[&[u8]]", list.name);
        table(&mut text, &[list.doc], &declaration, bytes(&words));
    }
    Ok(File {
        path: "src/coffee/words.rs",
        text,
    })
}

/// Whether `word` is one or more ASCII word characters.
fn is_word(word: &str) -> bool {
    !word.is_empty() && word.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}
