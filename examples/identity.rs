//! Prints a hash of nishiki's tokens for each oracle fixture and for edited
//! copies of it, one line per document, so that two commits can be compared
//! for identical output: `mise run identity <base>` runs it on both and
//! diffs the lines.
//!
//! The edits put the lexers into states the fixtures alone do not reach: a
//! byte that opens or closes something is inserted, a run of bytes is cut,
//! and lines are swapped. Each fixture's edits are drawn from a seed of its
//! language and name, so every run makes the same documents, and a tree with
//! more fixtures makes the same edits of the ones both trees share.
//!
//! Usage: `identity [fixtures] [--dump <language> <fixture> <edit>]`. The
//! fixtures default to this tree's `tests/oracle`; `--dump` prints the tokens
//! of one document, to see where two commits differ.

#![expect(clippy::print_stdout, reason = "the tool prints its results")]

use std::fs;
use std::path::{Path, PathBuf};

use nishiki::{Highlighter, Language, State};

/// Edited copies of each fixture.
const EDITS: u64 = 48;

/// Bytes that open or close something in one of the languages.
const MARKS: &[u8] = b"\"'`()[]{}<>/*#\\$@:;,=-\n \t";

/// One fixture: its language, its name and its source text.
struct Fixture {
    language: Language,
    name: String,
    text: String,
}

impl Fixture {
    /// The seed of its edits, from its language and name alone, so that adding
    /// or removing another fixture leaves its edits as they are.
    fn seed(&self) -> u64 {
        let mut seed = 0xcbf2_9ce4_8422_2325_u64;
        hash(&mut seed, self.language.id().as_bytes());
        hash(&mut seed, b"/");
        hash(&mut seed, self.name.as_bytes());
        seed
    }
}

fn fixtures(root: &Path) -> Vec<Fixture> {
    let mut found = Vec::new();
    let Ok(dirs) = fs::read_dir(root) else {
        return found;
    };
    let mut dirs: Vec<_> = dirs.filter_map(Result::ok).map(|e| e.path()).collect();
    dirs.sort();
    for dir in dirs {
        let Some(language) = dir
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(Language::from_name)
        else {
            continue;
        };
        let Ok(files) = fs::read_dir(&dir) else {
            continue;
        };
        let mut files: Vec<_> = files
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "txt"))
            .collect();
        files.sort();
        for file in files {
            let Ok(content) = fs::read_to_string(&file) else {
                continue;
            };
            let lines: Vec<&str> = content
                .lines()
                .filter_map(|record| record.strip_prefix("L "))
                .collect();
            let name = file
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or_default()
                .to_owned();
            found.push(Fixture {
                language,
                name,
                text: lines.join("\n"),
            });
        }
    }
    found
}

/// A xorshift generator, so that the edits are the same on every run.
struct Rng(u64);

impl Rng {
    fn below(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        let n = u64::try_from(n.max(1)).unwrap_or(u64::MAX);
        usize::try_from(self.0 % n).unwrap_or(0)
    }
}

/// The nearest character boundary of `text` at or before `i`.
fn boundary(text: &str, mut i: usize) -> usize {
    i = i.min(text.len());
    while !text.is_char_boundary(i) {
        i -= 1;
    }
    i
}

/// The `edit`th edited copy of `text`; the 0th is the text itself.
fn edited(text: &str, seed: u64, edit: u64) -> String {
    if edit == 0 || text.is_empty() {
        return text.to_owned();
    }
    let mut rng = Rng((seed ^ edit.wrapping_mul(0x9e37_79b9_7f4a_7c15)) | 1);
    let mut out = text.to_owned();
    for _ in 0..=rng.below(3) {
        let at = boundary(&out, rng.below(out.len()));
        match rng.below(3) {
            0 => {
                let mark = MARKS[rng.below(MARKS.len())];
                out.insert(at, char::from(mark));
            }
            1 => {
                let end = boundary(&out, at + rng.below(40));
                out.replace_range(at..end, "");
            }
            _ => {
                let lines: Vec<&str> = out.split('\n').collect();
                let (a, b) = (rng.below(lines.len()), rng.below(lines.len()));
                let mut lines: Vec<String> = lines.into_iter().map(str::to_owned).collect();
                lines.swap(a, b);
                out = lines.join("\n");
            }
        }
    }
    out
}

/// Lexes `text` line by line, calling `token` with each line number, token
/// text and scope names, innermost last.
fn lex(language: Language, text: &str, mut token: impl FnMut(usize, &str, &[&str])) {
    let mut highlighter = Highlighter::new(language);
    let mut state = State::default();
    let mut tokens = Vec::new();
    for (number, line) in text.split('\n').enumerate() {
        tokens.clear();
        let _ = highlighter.line(line, &mut state, &mut tokens);
        let mut start = 0;
        for t in &tokens {
            let mut names: Vec<&str> = highlighter.scopes(t.stack()).collect();
            names.reverse();
            token(number, line.get(start..t.end()).unwrap_or_default(), &names);
            start = t.end();
        }
    }
}

/// FNV-1a over the bytes.
fn hash(state: &mut u64, bytes: &[u8]) {
    for &b in bytes {
        *state ^= u64::from(b);
        *state = state.wrapping_mul(0x0100_0000_01b3);
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let root = match args.first() {
        Some(first) if first != "--dump" => PathBuf::from(args.remove(0)),
        _ => Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/oracle"),
    };
    let fixtures = fixtures(&root);
    if let [flag, language, name, edit] = args.as_slice()
        && flag == "--dump"
    {
        let edit: u64 = edit.parse().unwrap_or(0);
        for fixture in &fixtures {
            let seed = fixture.seed();
            if fixture.language.id() == language.as_str() && &fixture.name == name {
                let text = edited(&fixture.text, seed, edit);
                lex(fixture.language, &text, |number, piece, names| {
                    println!("{number:>5} {piece:?} {}", names.join(" > "));
                });
            }
        }
        return;
    }
    let mut documents = 0;
    for fixture in &fixtures {
        let seed = fixture.seed();
        for edit in 0..=EDITS {
            let text = edited(&fixture.text, seed, edit);
            let mut state = 0xcbf2_9ce4_8422_2325_u64;
            lex(fixture.language, &text, |number, piece, names| {
                hash(&mut state, &number.to_le_bytes());
                hash(&mut state, piece.as_bytes());
                for name in names {
                    hash(&mut state, name.as_bytes());
                    hash(&mut state, b" ");
                }
            });
            println!(
                "{}/{} {edit} {state:016x}",
                fixture.language.id(),
                fixture.name
            );
            documents += 1;
        }
    }
    println!("{documents} documents");
}
