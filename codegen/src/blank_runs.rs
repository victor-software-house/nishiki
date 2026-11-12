//! Which rule-engine grammars pass a mixed blank run in one step:
//! `src/rules/blank_runs.rs`.
//!
//! `nishiki::unstable::codegen::blank_runs` decides it for each pattern.
//! A grammar passes a mixed run when every one of its patterns treats a wide
//! space as a space.
//!
//! Each `oracle/<language>-grammar.ts` writes its grammar's patterns to
//! `codegen/patterns/<language>.json`.
//! This module reads them, and checks that each file lists the variants of
//! its grammar's `Pattern` enum.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use nishiki::unstable::codegen::blank_runs::treats_wide_spaces_as_spaces;
use serde_json::Value;

use crate::File;
use crate::error::Error;
use crate::facts::read;
use crate::header::header;

/// The file this module writes, from the repository root.
const PATH: &str = "src/rules/blank_runs.rs";

/// The directory of the pattern files it reads, from the repository root.
pub(crate) const PATTERNS: &str = "codegen/patterns";

/// The blank-run file, generated from the pattern files under `root`.
///
/// It fails when a pattern file has no grammar, or a grammar no pattern file,
/// or when a file's ids differ from its grammar's `Pattern` variants.
pub(crate) fn file(root: &Path) -> Result<File, Error> {
    let files = names(&root.join(PATTERNS), ".json")?;
    let grammars = grammars(root)?;
    if files != grammars {
        return Err(Error::Check(format!(
            "{PATTERNS}/ has no file for {:?}, and a file with no grammar for {:?}",
            grammars.difference(&files).collect::<Vec<_>>(),
            files.difference(&grammars).collect::<Vec<_>>(),
        )));
    }
    let mut text = header("codegen/src/blank_runs.rs", &format!("{PATTERNS}/"));
    for line in [
        "//! Whether each grammar's scan passes a run of spaces, tabs and wide",
        "//! spaces in one step ([`super::Tabled::MIXED_BLANK_RUNS`]).",
        "//! It holds where every pattern treats a wide space as a space.",
    ] {
        text.push_str(line);
        text.push('\n');
    }
    for language in &files {
        let path = format!("{PATTERNS}/{language}.json");
        let patterns = read(root, &path)?;
        let patterns = patterns
            .as_object()
            .ok_or_else(|| Error::Shape(format!("{path}: no object")))?;
        let ids = patterns.keys().cloned().collect::<BTreeSet<_>>();
        let variants = variants(root, language)?;
        if ids != variants {
            let rerun = "rerun the oracle generators with `mise run check:rule-grammars`";
            return Err(Error::Check(format!(
                "{path} lists other ids than src/{language}/grammar.rs's Pattern; {rerun}"
            )));
        }
        let mut mixed = true;
        for (id, pattern) in patterns {
            let field = |key: &str| {
                pattern
                    .get(key)
                    .and_then(Value::as_str)
                    .ok_or_else(|| Error::Shape(format!("{path}: {id} has no {key}")))
            };
            let repeats_begin = matches!(field("role")?, "end" | "while");
            mixed &= treats_wide_spaces_as_spaces(field("regex")?, repeats_begin);
        }
        let feature = language.replace('_', "-");
        let name = language.to_uppercase();
        for line in [
            String::new(),
            format!("#[cfg(feature = \"lang-{feature}\")]"),
            format!("pub(crate) const {name}: bool = {mixed};"),
        ] {
            text.push_str(&line);
            text.push('\n');
        }
    }
    Ok(File {
        path: PATH.to_owned(),
        text,
    })
}

/// The names in `dir` that end in `suffix`, without it.
pub(crate) fn names(dir: &Path, suffix: &str) -> Result<BTreeSet<String>, Error> {
    let io = |source| Error::Io {
        path: dir.to_path_buf(),
        source,
    };
    let mut names = BTreeSet::new();
    for entry in fs::read_dir(dir).map_err(io)? {
        let file = entry.map_err(io)?.file_name();
        if let Some(name) = file.to_str().and_then(|file| file.strip_suffix(suffix)) {
            names.insert(name.to_owned());
        }
    }
    Ok(names)
}

/// The languages whose `src/<language>/grammar.rs` implements `rules::Tabled`.
pub(crate) fn grammars(root: &Path) -> Result<BTreeSet<String>, Error> {
    let src = root.join("src");
    let mut found = BTreeSet::new();
    for language in names(&src, "")? {
        let path = src.join(&language).join("grammar.rs");
        if let Ok(text) = fs::read_to_string(&path)
            && text.contains("impl rules::Tabled for")
        {
            found.insert(language);
        }
    }
    Ok(found)
}

/// The variants of the `Pattern` enum in `src/<language>/grammar.rs`.
fn variants(root: &Path, language: &str) -> Result<BTreeSet<String>, Error> {
    let path = root.join("src").join(language).join("grammar.rs");
    let text = fs::read_to_string(&path).map_err(|source| Error::Io {
        path: path.clone(),
        source,
    })?;
    let file = syn::parse_file(&text)
        .map_err(|error| Error::Shape(format!("{}: {error}", path.display())))?;
    for item in &file.items {
        if let syn::Item::Enum(pattern) = item
            && pattern.ident == "Pattern"
        {
            return Ok(pattern
                .variants
                .iter()
                .map(|variant| variant.ident.to_string())
                .collect());
        }
    }
    Err(Error::Shape(format!("{}: no enum Pattern", path.display())))
}
