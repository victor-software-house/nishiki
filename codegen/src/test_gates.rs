//! The builds of `mise-tasks/check/test-gates.sh`: one for each set of
//! features that some test code of `src/` compiles under.
//!
//! `check:features` builds each language alone.
//! Test code behind two or more features goes unbuilt there.
//! Only `--all-features` compiles it, so a narrower gate inside it can break
//! unseen (openspec rust-codegen, task 1.6).
//!
//! This module reads the tokens of `src/`, from `src/lib.rs` down the module
//! tree, and parses each `#[cfg]` with syn.
//! A piece of test code needs the features of its own gates and of its
//! modules' gates.
//!
//! Of those features:
//!
//! - one that another of them turns on is dropped;
//! - a private `_…` one is replaced by the public feature that turns it on with
//!   the fewest features;
//! - each set of two or more that remains gets one build.
//!
//! How the walk reads a gate:
//!
//! - A `#[cfg]` inside an item, such as one on a statement, counts with the
//!   gates of that item.
//! - Before a comma that parts fields or match arms, a gate covers what follows
//!   to the next `;`, `{…}` or attribute. Such a build then turns on more
//!   features than the code needs, and still compiles it.
//! - A `not(…)` counts as met, since a build turns on only the features it
//!   names.
//! - A gate that needs `nishiki_unstable` or `docsrs` gets no build here.
//!   `check:unstable` and the doc build compile that code.

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;
use std::{env, fs};

use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use serde_json::Value;
use syn::punctuated::Punctuated;
use syn::{Expr, Lit, Meta, Token};

use crate::File;
use crate::error::Error;

/// The file this module writes, from the repository root.
const PATH: &str = "mise-tasks/check/test-gates.sh";

/// The longest line of the file; a longer build breaks before `--features`
/// (`CODING_STANDARDS.md` 3.10).
const WIDTH: usize = 100;

/// The flags a build line cannot set, so a gate that needs one gets no line.
const UNBUILT_FLAGS: [&str; 2] = ["nishiki_unstable", "docsrs"];

/// A `cfg` predicate.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Cfg {
    /// `feature = "…"`.
    Feature(String),
    /// A bare name, such as `test`, or a key with a value other than
    /// `feature`, such as `target_os`.
    Flag(String),
    /// `all(…)`.
    All(Vec<Cfg>),
    /// `any(…)`.
    Any(Vec<Cfg>),
    /// `not(…)`.
    Not(Box<Cfg>),
}

/// One way to meet a gate: the features and flags it needs on, and those it
/// needs off.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Term {
    /// The features that must be on.
    features: BTreeSet<String>,
    /// The flags that must be on.
    flags: BTreeSet<String>,
    /// The features and flags that must be off.
    off: BTreeSet<String>,
}

impl Term {
    /// The term that needs what `self` and `other` both need.
    fn and(&self, other: &Term) -> Term {
        let mut both = self.clone();
        both.features.extend(other.features.iter().cloned());
        both.flags.extend(other.flags.iter().cloned());
        both.off.extend(other.off.iter().cloned());
        both
    }

    /// Whether a build of the test code can meet the term with features
    /// alone.
    fn buildable(&self) -> bool {
        self.flags.contains("test")
            && !UNBUILT_FLAGS.iter().any(|flag| self.flags.contains(*flag))
            && self.features.is_disjoint(&self.off)
            && self.flags.is_disjoint(&self.off)
    }
}

/// The ways to meet `cfg`, each a term.
fn terms(cfg: &Cfg) -> Vec<Term> {
    match cfg {
        Cfg::Feature(name) => vec![Term {
            features: BTreeSet::from([name.clone()]),
            ..Term::default()
        }],
        Cfg::Flag(name) => vec![Term {
            flags: BTreeSet::from([name.clone()]),
            ..Term::default()
        }],
        Cfg::All(parts) => parts.iter().fold(vec![Term::default()], |met, part| {
            let ways = terms(part);
            met.iter()
                .flat_map(|term| ways.iter().map(move |way| term.and(way)))
                .collect()
        }),
        Cfg::Any(parts) => parts.iter().flat_map(terms).collect(),
        Cfg::Not(inner) => match inner.as_ref() {
            Cfg::Feature(name) | Cfg::Flag(name) => vec![Term {
                off: BTreeSet::from([name.clone()]),
                ..Term::default()
            }],
            Cfg::All(_) | Cfg::Any(_) | Cfg::Not(_) => vec![Term::default()],
        },
    }
}

/// The features each feature of nishiki's `Cargo.toml` turns on, itself
/// included.
#[derive(Debug)]
pub(crate) struct Features {
    /// Each feature, and every feature it turns on through its list.
    turned_on: BTreeMap<String, BTreeSet<String>>,
}

impl Features {
    /// The features from each feature's list, as `cargo metadata` gives them.
    fn new(lists: &BTreeMap<String, Vec<String>>) -> Features {
        let turned_on = lists
            .keys()
            .map(|feature| {
                let mut on = BTreeSet::new();
                let mut next = vec![feature.clone()];
                while let Some(name) = next.pop() {
                    if on.insert(name.clone())
                        && let Some(list) = lists.get(&name)
                    {
                        next.extend(
                            list.iter()
                                .filter(|entry| !entry.contains(['/', ':']))
                                .cloned(),
                        );
                    }
                }
                (feature.clone(), on)
            })
            .collect();
        Features { turned_on }
    }

    /// Reads nishiki's features with `cargo metadata`.
    fn read(root: &Path) -> Result<Features, Error> {
        let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
        let manifest = root.join("Cargo.toml");
        let output = Command::new(cargo)
            .args([
                "metadata",
                "--no-deps",
                "--offline",
                "--format-version",
                "1",
            ])
            .arg("--manifest-path")
            .arg(&manifest)
            .output()
            .map_err(|source| Error::Io {
                path: manifest.clone(),
                source,
            })?;
        if !output.status.success() {
            return Err(Error::Check(format!(
                "cargo metadata failed on {}: {}",
                manifest.display(),
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        let metadata: Value =
            serde_json::from_slice(&output.stdout).map_err(|source| Error::Json {
                path: manifest.clone(),
                source,
            })?;
        let features = metadata["packages"]
            .as_array()
            .and_then(|packages| packages.iter().find(|package| package["name"] == "nishiki"))
            .and_then(|package| package["features"].as_object())
            .ok_or_else(|| Error::Shape("cargo metadata lists no nishiki features".to_owned()))?;
        let lists = features
            .iter()
            .map(|(name, list)| {
                let list = list
                    .as_array()
                    .map(|entries| {
                        entries
                            .iter()
                            .filter_map(Value::as_str)
                            .map(str::to_owned)
                            .collect()
                    })
                    .unwrap_or_default();
                (name.clone(), list)
            })
            .collect();
        Ok(Features::new(&lists))
    }

    /// `features` without each one that another of them turns on.
    fn fewest(&self, features: &BTreeSet<String>) -> BTreeSet<String> {
        features
            .iter()
            .filter(|feature| {
                !features.iter().any(|other| {
                    other != *feature
                        && self
                            .turned_on
                            .get(other)
                            .is_some_and(|on| on.contains(*feature))
                })
            })
            .cloned()
            .collect()
    }

    /// The public features of the fewest build that turns on all of
    /// `features`, or none when a private feature has no public one that
    /// turns it on.
    ///
    /// A private `_…` feature groups code that several languages share.
    /// No caller turns one on alone, so each is replaced by the public
    /// feature that turns it on with the fewest features, as `_textmate` by
    /// `textmate`.
    fn public(&self, features: &BTreeSet<String>) -> Option<BTreeSet<String>> {
        let public = self
            .fewest(features)
            .into_iter()
            .map(|feature| {
                if !feature.starts_with('_') {
                    return Some(feature);
                }
                self.turned_on
                    .iter()
                    .filter(|(name, on)| {
                        !name.starts_with('_') && *name != "default" && on.contains(&feature)
                    })
                    .min_by_key(|(name, on)| (on.len(), *name))
                    .map(|(name, _)| name.clone())
            })
            .collect::<Option<BTreeSet<_>>>()?;
        Some(self.fewest(&public))
    }
}

/// The test-gate build file, from the module tree of `src/` under `root`.
pub(crate) fn file(root: &Path) -> Result<File, Error> {
    let features = Features::read(root)?;
    let read = |path: &Path| {
        fs::read_to_string(root.join(path)).map_err(|source| Error::Io {
            path: root.join(path),
            source,
        })
    };
    let sets = sets(Path::new("src/lib.rs"), &read, &features)?;
    Ok(File {
        path: PATH,
        text: script(&sets),
    })
}

/// The feature sets of the test gates in the crate rooted at `lib`, each of
/// two or more features.
/// `read` gives the text of a source file from its path.
fn sets(
    lib: &Path,
    read: &dyn Fn(&Path) -> Result<String, Error>,
    features: &Features,
) -> Result<BTreeSet<BTreeSet<String>>, Error> {
    let mut walk = Walk {
        read,
        features,
        gates: Vec::new(),
        sets: BTreeSet::new(),
    };
    let dir = lib.parent().map(Path::to_path_buf).unwrap_or_default();
    walk.file(lib, &dir)?;
    Ok(walk.sets)
}

/// A walk over the module tree, with the gates of the modules and items it is
/// in.
struct Walk<'a> {
    /// Gives the text of a source file.
    read: &'a dyn Fn(&Path) -> Result<String, Error>,
    /// The features each feature turns on.
    features: &'a Features,
    /// The gates of the modules and items around the walk, outermost first.
    gates: Vec<Cfg>,
    /// The feature sets found so far.
    sets: BTreeSet<BTreeSet<String>>,
}

impl Walk<'_> {
    /// Walks the tokens of the file at `path`, whose child modules sit in
    /// `dir`.
    fn file(&mut self, path: &Path, dir: &Path) -> Result<(), Error> {
        let text = (self.read)(path)?;
        let tokens = text
            .parse::<TokenStream>()
            .map_err(|error| Error::Shape(format!("{}: {error}", path.display())))?;
        self.tokens(tokens, dir)
    }

    /// Walks `tokens`, whose child modules sit in `dir`.
    ///
    /// The tokens split into pieces at each `;`, at each `{…}` and before each
    /// outer attribute.
    /// The outer `#[cfg]`s that open a piece gate the rest of it, and every
    /// group inside it.
    /// An inner `#![cfg]` gates every piece after it in `tokens`.
    /// A piece is an item, a statement or a field.
    /// Where commas part fields or arms, a piece runs on to the next split.
    fn tokens(&mut self, tokens: TokenStream, dir: &Path) -> Result<(), Error> {
        let trees = tokens.into_iter().collect::<Vec<_>>();
        let outer = self.gates.len();
        let mut at = 0;
        while at < trees.len() {
            let mut pushed = 0;
            let mut found = false;
            while let Some(attribute) = attribute_at(&trees, at) {
                let inner = inner_attribute_at(&trees, at);
                if let Some(gate) = gate(attribute) {
                    if inner {
                        self.gates.insert(outer, gate);
                    } else {
                        self.gates.push(gate);
                        pushed += 1;
                    }
                    found = true;
                }
                at += if inner { 3 } else { 2 };
            }
            if found {
                self.record();
            }
            let start = at;
            while at < trees.len() && attribute_at(&trees, at).is_none() {
                let ends = match &trees[at] {
                    TokenTree::Punct(punct) => punct.as_char() == ';',
                    TokenTree::Group(group) => group.delimiter() == Delimiter::Brace,
                    TokenTree::Ident(_) | TokenTree::Literal(_) => false,
                };
                at += 1;
                if ends {
                    break;
                }
            }
            let piece = &trees[start..at];
            if let Some((name, body)) = module(piece) {
                let child = dir.join(name);
                match body {
                    Some(body) => self.tokens(body, &child)?,
                    None => self.file(&child.with_extension("rs"), &child)?,
                }
            } else {
                for tree in piece {
                    if let TokenTree::Group(group) = tree {
                        self.tokens(group.stream(), dir)?;
                    }
                }
            }
            self.gates.truncate(self.gates.len() - pushed);
        }
        self.gates.truncate(outer);
        Ok(())
    }

    /// Records the feature set of each buildable way to meet the gates around
    /// the walk.
    fn record(&mut self) {
        for term in terms(&Cfg::All(self.gates.clone())) {
            if term.buildable()
                && let Some(public) = self.features.public(&term.features)
                && public.len() >= 2
            {
                self.sets.insert(public);
            }
        }
    }
}

/// The bracketed contents of the attribute that opens at `at`, outer as
/// `#[…]` or inner as `#![…]`.
fn attribute_at(trees: &[TokenTree], at: usize) -> Option<&Group> {
    let TokenTree::Punct(hash) = trees.get(at)? else {
        return None;
    };
    if hash.as_char() != '#' {
        return None;
    }
    let next = if inner_attribute_at(trees, at) {
        at + 2
    } else {
        at + 1
    };
    match trees.get(next)? {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => Some(group),
        TokenTree::Group(_) | TokenTree::Punct(_) | TokenTree::Ident(_) | TokenTree::Literal(_) => {
            None
        }
    }
}

/// Whether the `#` at `at` opens an inner attribute, `#![…]`.
fn inner_attribute_at(trees: &[TokenTree], at: usize) -> bool {
    matches!(trees.get(at + 1), Some(TokenTree::Punct(bang)) if bang.as_char() == '!')
}

/// The gate an attribute's bracketed `contents` spell, if it is a `#[cfg]`.
/// A `#[cfg]` in a macro's template, such as `feature = $feature`, gives
/// none: only the expanded code holds its features.
fn gate(contents: &Group) -> Option<Cfg> {
    let Ok(Meta::List(list)) = syn::parse2::<Meta>(contents.stream()) else {
        return None;
    };
    if !list.path.is_ident("cfg") {
        return None;
    }
    cfg(&syn::parse2::<Meta>(list.tokens.clone()).ok()?)
}

/// The name of the module `piece` declares, with its inline body if it has
/// one, when `piece` is `[pub(…)] mod name;` or `[pub(…)] mod name {…}`.
fn module(piece: &[TokenTree]) -> Option<(String, Option<TokenStream>)> {
    let mut trees = piece.iter().skip_while(|tree| match tree {
        TokenTree::Ident(ident) => ident == "pub",
        TokenTree::Group(group) => group.delimiter() == Delimiter::Parenthesis,
        TokenTree::Punct(_) | TokenTree::Literal(_) => false,
    });
    let (Some(TokenTree::Ident(keyword)), Some(TokenTree::Ident(name))) =
        (trees.next(), trees.next())
    else {
        return None;
    };
    if keyword != "mod" {
        return None;
    }
    match trees.next()? {
        TokenTree::Punct(semi) if semi.as_char() == ';' => Some((name.to_string(), None)),
        TokenTree::Group(body) if body.delimiter() == Delimiter::Brace => {
            Some((name.to_string(), Some(body.stream())))
        }
        TokenTree::Punct(_) | TokenTree::Group(_) | TokenTree::Ident(_) | TokenTree::Literal(_) => {
            None
        }
    }
}

/// The predicate `meta` spells, or none when it is not one rustc reads, such
/// as a macro template's `not(feature = $feature)`.
fn cfg(meta: &Meta) -> Option<Cfg> {
    let name = meta.path().get_ident()?.to_string();
    match meta {
        Meta::Path(_) => Some(Cfg::Flag(name)),
        Meta::NameValue(pair) => {
            if name != "feature" {
                return Some(Cfg::Flag(name));
            }
            if let Expr::Lit(literal) = &pair.value
                && let Lit::Str(feature) = &literal.lit
            {
                Some(Cfg::Feature(feature.value()))
            } else {
                None
            }
        }
        Meta::List(list) => {
            let parts = list
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .ok()?
                .iter()
                .map(cfg)
                .collect::<Option<Vec<_>>>()?;
            match name.as_str() {
                "all" => Some(Cfg::All(parts)),
                "any" => Some(Cfg::Any(parts)),
                "not" => <[Cfg; 1]>::try_from(parts)
                    .ok()
                    .map(|[inner]| Cfg::Not(Box::new(inner))),
                _ => None,
            }
        }
    }
}

/// The text of `mise-tasks/check/test-gates.sh`, one build per set.
fn script(sets: &BTreeSet<BTreeSet<String>>) -> String {
    let mut text = String::from("#!/usr/bin/env bash\n");
    text.push_str(r#"#MISE description="Each test gate of two or more features compiles"#);
    text.push_str(" with only those features\"\n");
    text.push_str(r#"#MISE env={CARGO_TARGET_DIR = "target/features", "#);
    text.push_str(r#"RUSTFLAGS = "-D warnings -A dead_code"}"#);
    text.push('\n');
    for line in [
        "# @generated by codegen/src/test_gates.rs from src/.",
        "# Regenerate with `mise run codegen`; do not edit.",
        "# check:features builds each language alone.",
        "# The test code under these gates compiles only here and with --all-features.",
        "set -euo pipefail",
    ] {
        text.push_str(line);
        text.push('\n');
    }
    for set in sets {
        let features = set.iter().map(String::as_str).collect::<Vec<_>>().join(",");
        let command = "cargo check --locked --tests --no-default-features";
        let line = format!("{command} --features {features}");
        if line.len() <= WIDTH {
            text.push_str(&line);
        } else {
            text.push_str(command);
            text.push_str(" \\");
            text.push('\n');
            text.push_str("  --features ");
            text.push_str(&features);
        }
        text.push('\n');
    }
    text
}
