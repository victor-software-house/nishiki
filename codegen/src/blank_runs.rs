//! How each rule-engine grammar's scan passes a blank run:
//! `src/rules/blank_runs.rs`.
//!
//! `nishiki::unstable::codegen::blank_runs` decides two facts for each
//! pattern.
//!
//! - A grammar passes a mixed run in one step when every one of its patterns
//!   treats a wide space as a space.
//! - A grammar tries each wide space of a run when one of its patterns has a
//!   lookbehind that can pass after a wide space and a start that can be one.
//!   This file writes that fact as an impl of `rules::WideSpaceLookbehind` for
//!   the grammar's type.
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

use nishiki::unstable::codegen::blank_runs::{
    looks_behind_at_a_wide_space, treats_wide_spaces_as_spaces,
};
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
        "//!",
        "//! Each grammar's impl of [`super::WideSpaceLookbehind`] says whether its",
        "//! scan tries each wide space of a run.",
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
        let mut looks_behind = false;
        for (id, pattern) in patterns {
            let field = |key: &str| {
                pattern
                    .get(key)
                    .and_then(Value::as_str)
                    .ok_or_else(|| Error::Shape(format!("{path}: {id} has no {key}")))
            };
            let repeats_begin = matches!(field("role")?, "end" | "while");
            let regex = field("regex")?;
            mixed &= treats_wide_spaces_as_spaces(regex, repeats_begin);
            looks_behind |= looks_behind_at_a_wide_space(regex);
        }
        let grammar = grammar_type(root, language)?;
        let feature = language.replace('_', "-");
        let name = language.to_uppercase();
        for line in [
            String::new(),
            format!("#[cfg(feature = \"lang-{feature}\")]"),
            format!("pub(crate) const {name}: bool = {mixed};"),
            String::new(),
            format!("#[cfg(feature = \"lang-{feature}\")]"),
            format!(
                "impl{} super::WideSpaceLookbehind for {} {{",
                grammar.0, grammar.1
            ),
            format!("    const LOOKS_BEHIND_AT_A_WIDE_SPACE: bool = {looks_behind};"),
            "}".to_owned(),
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
    let path = format!("src/{language}/grammar.rs");
    for item in &parse(root, &path)?.items {
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
    Err(Error::Shape(format!("{path}: no enum Pattern")))
}

/// The generic parameters and the path of the type that
/// `src/<language>/grammar.rs` implements `rules::Tabled` for.
///
/// The type is defined in `src/<language>/engine.rs`.
/// A type with const parameters, as `Coffee<const D: u8>`, gets them in both
/// parts, so one impl covers each of its dialects.
fn grammar_type(root: &Path, language: &str) -> Result<(String, String), Error> {
    let path = format!("src/{language}/grammar.rs");
    let name = parse(root, &path)?
        .items
        .iter()
        .find_map(|item| {
            if let syn::Item::Impl(tabled) = item
                && let Some((name, _)) = &tabled.trait_
                && name
                    .segments
                    .last()
                    .is_some_and(|last| last.ident == "Tabled")
                && let syn::Type::Path(ty) = &*tabled.self_ty
            {
                ty.path.segments.last().map(|last| last.ident.to_string())
            } else {
                None
            }
        })
        .ok_or_else(|| Error::Shape(format!("{path}: no impl of rules::Tabled")))?;
    let path = format!("src/{language}/engine.rs");
    let generics = parse(root, &path)?
        .items
        .into_iter()
        .find_map(|item| {
            if let syn::Item::Struct(grammar) = item
                && grammar.ident == name
            {
                Some(grammar.generics)
            } else {
                None
            }
        })
        .ok_or_else(|| Error::Shape(format!("{path}: no struct {name}")))?;
    let mut params = Vec::new();
    for param in generics.const_params() {
        let syn::Type::Path(ty) = &param.ty else {
            return Err(Error::Shape(format!(
                "{path}: {name} takes a const of no path type"
            )));
        };
        let ty = ty.path.segments.last().map(|last| last.ident.to_string());
        let ty =
            ty.ok_or_else(|| Error::Shape(format!("{path}: {name} takes a const of no type")))?;
        params.push((param.ident.to_string(), ty));
    }
    let ty = format!("crate::{language}::engine::{name}");
    if params.is_empty() {
        return Ok((String::new(), ty));
    }
    let declared = params
        .iter()
        .map(|(ident, ty)| format!("const {ident}: {ty}"))
        .collect::<Vec<_>>();
    let used = params
        .iter()
        .map(|(ident, _)| ident.as_str())
        .collect::<Vec<_>>();
    Ok((
        format!("<{}>", declared.join(", ")),
        format!("{ty}<{}>", used.join(", ")),
    ))
}

/// The Rust file at `path`, from the repository root.
fn parse(root: &Path, path: &str) -> Result<syn::File, Error> {
    let text = fs::read_to_string(root.join(path)).map_err(|source| Error::Io {
        path: root.join(path),
        source,
    })?;
    syn::parse_file(&text).map_err(|error| Error::Shape(format!("{path}: {error}")))
}
