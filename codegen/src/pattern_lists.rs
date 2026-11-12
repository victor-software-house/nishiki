//! The check that codegen builds each language's pattern list as its script
//! does.
//!
//! Each `oracle/<language>-grammar.ts` writes its patterns to
//! `codegen/patterns/<language>.json`.
//! For each language of [`languages::ALL`], this module builds the list with
//! `nishiki::unstable::codegen::flatten` from the grammar in
//! `tests/engine/grammars/`.
//! It then writes the list as the script does, and compares the bytes.

#[cfg(test)]
mod tests;

use std::fs;
use std::path::Path;

use nishiki::unstable::codegen::flatten::flatten;
use nishiki::unstable::codegen::tables::{External, Tables};

use crate::blank_runs::PATTERNS;
use crate::error::Error;
use crate::languages;
use crate::source::Source;

/// The languages whose pattern list matches their pattern file.
///
/// It fails at the first language whose list differs, and names its first
/// differing line.
pub(crate) fn check(root: &Path) -> Result<Vec<&'static str>, Error> {
    let mut matched = Vec::new();
    for language in languages::ALL {
        let source = Source::load(root, language.grammar)?;
        let others = language
            .externals
            .iter()
            .filter_map(|&(_, external)| match external {
                External::Pattern { grammar, .. } => Some(grammar),
                External::AddsNothing
                | External::Begin { .. }
                | External::Embeds
                | External::Missing => None,
            })
            .map(|name| Ok((name, Source::load(root, name)?.grammar())))
            .collect::<Result<Vec<_>, Error>>()?;
        let tables =
            flatten(source.grammar(), &others, &language).map_err(|source| Error::Generate {
                language: language.language,
                source,
            })?;
        let path = root.join(format!("{PATTERNS}/{}.json", language.language));
        let written = fs::read_to_string(&path).map_err(|source| Error::Io {
            path: path.clone(),
            source,
        })?;
        let built = json(&tables)?;
        if built != written {
            let line = built
                .lines()
                .zip(written.lines())
                .position(|(ours, theirs)| ours != theirs)
                .unwrap_or_else(|| built.lines().count().min(written.lines().count()));
            return Err(Error::Check(format!(
                "{}: line {} of {} is {} there and {} in codegen",
                language.language,
                line + 1,
                path.display(),
                written.lines().nth(line).unwrap_or("the end"),
                built.lines().nth(line).unwrap_or("the end"),
            )));
        }
        matched.push(language.language);
    }
    Ok(matched)
}

/// The pattern file's text, as `writePatterns` in `oracle/rule-grammar.ts`
/// writes it: each pattern's role and expression by id, in table order,
/// indented by two spaces.
fn json(tables: &Tables) -> Result<String, Error> {
    let quote = |text: &str| {
        serde_json::to_string(text).map_err(|source| Error::Json {
            path: PATTERNS.into(),
            source,
        })
    };
    let mut text = String::from("{\n");
    for (index, pattern) in tables.patterns.iter().enumerate() {
        let comma = if index + 1 < tables.patterns.len() {
            ","
        } else {
            ""
        };
        for line in [
            format!("  {}: {{", quote(&pattern.id)?),
            format!("    \"role\": \"{}\",", pattern.act.role()),
            format!("    \"regex\": {}", quote(&pattern.regex)?),
            format!("  }}{comma}"),
        ] {
            text.push_str(&line);
            text.push('\n');
        }
    }
    text.push_str("}\n");
    Ok(text)
}
