//! `src/html/words.rs` and `src/html_words.rs`: the words of the `html`
//! grammar's big alternations, sorted.
//!
//! `src/html/words.rs` holds the tag names and attribute names.
//! `src/html_words.rs` holds the event handler names and the character
//! entity names, which other grammars spell the same way.
//! Tag names are matched without regard to case, so they are in lower case.
//! The other names are matched as written.

use std::collections::BTreeSet;
use std::path::Path;

use nishiki::unstable::grammar::UnlinkedRule;

use super::{Cut, Source, quoted, sorted, table};
use crate::File;
use crate::error::Error;
use crate::header::header;

/// The tag names of a rule that opens with `(?i)(<)(`.
const TAGS: Cut = Cut::After("(?i)(<)(", ")(?=");

/// An attribute's name, which ends where the grammar's `(?![-:\w])` follows.
const ATTRIBUTE_END: &str = r")(?![-:\w])";

/// A table of tag names, read from the patterns of `tags-valid`.
#[derive(Debug)]
struct Tags {
    /// The table's name.
    name: &'static str,
    /// The category the rules' scopes name, such as `metadata`.
    category: &'static str,
    /// The index of the rule of a void element, or of the start rule.
    first: usize,
    /// The index of the end rule, whose names the start rule's must equal;
    /// none for a void element.
    end: Option<usize>,
}

/// The tables of `tags-valid`, in the order the file holds them.
const VALID: &[Tags] = &[
    Tags {
        name: "METADATA_VOID",
        category: "metadata",
        first: 2,
        end: None,
    },
    Tags {
        name: "METADATA",
        category: "metadata",
        first: 3,
        end: Some(4),
    },
    Tags {
        name: "STRUCTURE_VOID",
        category: "structure",
        first: 5,
        end: None,
    },
    Tags {
        name: "STRUCTURE",
        category: "structure",
        first: 6,
        end: Some(7),
    },
    Tags {
        name: "INLINE_VOID",
        category: "inline",
        first: 8,
        end: None,
    },
    Tags {
        name: "INLINE",
        category: "inline",
        first: 9,
        end: Some(10),
    },
    Tags {
        name: "OBJECT_VOID",
        category: "object",
        first: 11,
        end: None,
    },
    Tags {
        name: "OBJECT",
        category: "object",
        first: 12,
        end: Some(13),
    },
    Tags {
        name: "DEPRECATED_METADATA_VOID",
        category: "metadata",
        first: 14,
        end: None,
    },
    Tags {
        name: "DEPRECATED_STRUCTURE",
        category: "structure",
        first: 15,
        end: Some(16),
    },
    Tags {
        name: "DEPRECATED_INLINE",
        category: "inline",
        first: 17,
        end: Some(18),
    },
    Tags {
        name: "DEPRECATED_OBJECT_VOID",
        category: "object",
        first: 19,
        end: None,
    },
    Tags {
        name: "DEPRECATED_OBJECT",
        category: "object",
        first: 20,
        end: Some(21),
    },
    Tags {
        name: "UNSUPPORTED",
        category: "other",
        first: 22,
        end: Some(23),
    },
];

/// The `svg` and `math` sub-grammars: the prefix of their tables, their
/// repository entry, and the categories of their elements in the order
/// their `tags` patterns list them.
const FOREIGN: &[(&str, &str, &[&str])] = &[
    (
        "SVG",
        "svg",
        &["metadata", "structure", "inline", "object", "other"],
    ),
    ("MATH", "math", &["structure", "inline", "object"]),
];

/// Generates both files.
pub(super) fn files(root: &Path) -> Result<[File; 2], Error> {
    let source = Source::load(root, "html")?;
    let mut lists: Vec<(String, Vec<String>)> = Vec::new();
    let valid = source.entry("tags-valid")?;
    for tags in VALID {
        let kind = if tags.end.is_some() { "start" } else { "void" };
        let first = scoped(source, valid, tags.first, tags.category, kind)?;
        let names = tag_names(first)?;
        if let Some(end) = tags.end {
            let end = scoped(source, valid, end, tags.category, "end")?;
            if tag_names(&end.replacen("(</)", "(<)", 1))? != names {
                return Err(Error::Shape(format!(
                    "{}: the start and end lists differ",
                    tags.name
                )));
            }
        }
        lists.push((tags.name.to_owned(), names));
    }
    for &(prefix, key, categories) in FOREIGN {
        let entry = source.entry(key)?;
        let first = entry
            .patterns
            .first()
            .map(|&id| source.rule(id))
            .transpose()?;
        let holder = first
            .filter(|rule| rule.repository.is_some())
            .unwrap_or(entry);
        let patterns = source.nested_entry(holder, "tags")?;
        for (index, category) in categories.iter().enumerate() {
            let scope = format!("meta.element.{category}.{key}.$2.html");
            for (offset, suffix) in [(2, "_VOID"), (3, "")] {
                let rule = source.pattern(patterns, 2 * index + offset)?;
                if !super::has_name(rule, &scope) {
                    return Err(Error::Shape(format!("{key}: pattern is not {scope}")));
                }
                let pattern = rule.begin.as_deref().or(rule.matched.as_deref());
                let pattern = pattern
                    .ok_or_else(|| Error::Shape(format!("{key}: {scope} has no pattern")))?;
                let name = format!("{prefix}_{}{suffix}", category.to_ascii_uppercase());
                lists.push((name, tag_names(pattern)?));
            }
        }
    }
    let attribute = source.entry("attribute")?;
    lists.push(("HTML_ATTRIBUTES".to_owned(), attributes(source, attribute)?));
    let events = begin(source, source.pattern(attribute, 2)?)?;
    let mut shared = vec![(
        "EVENT_HANDLERS".to_owned(),
        sorted(Cut::After("on(", ATTRIBUTE_END).apply(events)?)?,
    )];
    for &(prefix, key, _) in FOREIGN {
        let entry = source.entry(key)?;
        let attribute = source.nested_entry(entry, "attribute")?;
        lists.push((
            format!("{prefix}_ATTRIBUTES"),
            attributes(source, attribute)?,
        ));
    }
    let entities = source.text(source.pattern(source.entry("entities")?, 0)?)?;
    shared.push((
        "ENTITIES".to_owned(),
        sorted(Cut::After("(&)(?=[A-Za-z])(", ")(;)").apply(entities)?)?,
    ));
    let input = source.path();
    let mut own = header("codegen/src/words/html.rs", &input);
    own.push_str("//! The tag and attribute names that the big alternations of the `html`\n");
    own.push_str("//! grammar spell out, sorted. Tag names are lower case.\n");
    tables(&mut own, "pub(super)", &lists);
    let mut common = header("codegen/src/words/html.rs", &input);
    common.push_str("//! The event handler and character entity names that the big alternations\n");
    common.push_str("//! of the `html` grammar spell out, sorted.\n");
    common.push_str("//! Every grammar that spells them the same way shares them.\n");
    tables(&mut common, "pub(crate)", &shared);
    Ok([
        File {
            path: "src/html/words.rs",
            text: own,
        },
        File {
            path: "src/html_words.rs",
            text: common,
        },
    ])
}

/// The pattern of the `index`th pattern of `valid`, after checking that its
/// scope is `meta.tag.<category>.$2.<kind>.html`.
fn scoped(
    source: Source,
    valid: &UnlinkedRule,
    index: usize,
    category: &str,
    kind: &str,
) -> Result<&'static str, Error> {
    let rule = source.pattern(valid, index)?;
    let scope = format!("meta.tag.{category}.$2.{kind}.html");
    if !super::has_name(rule, &scope) {
        return Err(Error::Shape(format!(
            "tags-valid: pattern {index} is not {scope}"
        )));
    }
    source.text(rule)
}

/// The tag names of a rule's pattern, in lower case, sorted.
fn tag_names(pattern: &str) -> Result<Vec<String>, Error> {
    let names: BTreeSet<String> = sorted(TAGS.apply(pattern)?)?
        .into_iter()
        .map(|name| name.to_ascii_lowercase())
        .collect();
    Ok(names.into_iter().collect())
}

/// The attribute names of `attribute`'s first pattern, sorted.
fn attributes(source: Source, attribute: &UnlinkedRule) -> Result<Vec<String>, Error> {
    let first = begin(source, source.pattern(attribute, 0)?)?;
    sorted(Cut::After("(", ATTRIBUTE_END).apply(first)?)
}

/// `rule`'s `begin`.
fn begin(source: Source, rule: &'static UnlinkedRule) -> Result<&'static str, Error> {
    rule.begin
        .as_deref()
        .ok_or_else(|| Error::Shape(format!("{}: a rule with no begin", source.name)))
}

/// Appends each of `lists` as a table of `visibility`.
fn tables(text: &mut String, visibility: &str, lists: &[(String, Vec<String>)]) {
    for (name, words) in lists {
        let declaration = format!("{visibility} const {name}: &[&str]");
        table(text, &[], &declaration, quoted(words));
    }
}
