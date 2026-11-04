//! The benchmark corpus, as the benchmarks read it.
//!
//! Each file of `bench/corpus/` sits at `<language>/<tier>/<file>`.
//! `mise run check:corpus` holds it to its row in `bench/corpus.toml`. The
//! language and the tier then come from the path. A document's lines are
//! split once, before any timing, as an editor holds them.

use std::path::Path;
use std::{fmt, fs};

use nishiki::Language;

/// What a file of the corpus stands for (`bench/corpus/README.md`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tier {
    /// 1 to 4 KiB: a snippet or a code block in a reply.
    Small,
    /// 16 to 64 KiB: a typical source file.
    Medium,
    /// 200 KiB to 1 MiB: a big real file.
    Large,
    /// A Markdown reply with code blocks in several languages.
    Reply,
}

impl Tier {
    /// Every tier, in the order reports list them.
    pub const ALL: [Tier; 4] = [Tier::Small, Tier::Medium, Tier::Large, Tier::Reply];

    /// The tier's folder name.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Tier::Small => "small",
            Tier::Medium => "medium",
            Tier::Large => "large",
            Tier::Reply => "reply",
        }
    }

    fn from_name(name: &str) -> Option<Tier> {
        Tier::ALL.into_iter().find(|tier| tier.name() == name)
    }
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// One file of the corpus.
#[derive(Debug)]
pub struct Document {
    /// The language its folder names.
    pub language: Language,
    /// The tier its folder names.
    pub tier: Tier,
    /// `<language>/<tier>/<file>`.
    pub name: String,
    text: String,
}

impl Document {
    /// The file's size in bytes.
    #[must_use]
    pub fn bytes(&self) -> usize {
        self.text.len()
    }

    /// The file's lines, without their line ends, as Shiki splits them: a
    /// `\r` before a `\n` is part of the line end.
    #[must_use]
    pub fn lines(&self) -> Vec<&str> {
        self.text
            .lines()
            .map(|line| line.strip_suffix('\r').unwrap_or(line))
            .collect()
    }
}

/// Every file of the corpus whose language is built in, sorted by path.
///
/// # Errors
///
/// When a folder of the corpus cannot be read, or a file is not UTF-8.
pub fn corpus() -> std::io::Result<Vec<Document>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus");
    let mut documents = Vec::new();
    for language_dir in sorted_dirs(&root)? {
        let Some(language) = file_name(&language_dir).and_then(Language::from_name) else {
            continue;
        };
        for tier_dir in sorted_dirs(&language_dir)? {
            let Some(tier) = file_name(&tier_dir).and_then(Tier::from_name) else {
                continue;
            };
            let mut files: Vec<_> = fs::read_dir(&tier_dir)?
                .map(|entry| entry.map(|entry| entry.path()))
                .collect::<Result<_, _>>()?;
            files.retain(|path| path.is_file());
            files.sort();
            for file in files {
                let text = fs::read_to_string(&file)?;
                let name = format!(
                    "{}/{tier}/{}",
                    language.id(),
                    file_name(&file).unwrap_or_default()
                );
                documents.push(Document {
                    language,
                    tier,
                    name,
                    text,
                });
            }
        }
    }
    Ok(documents)
}

fn sorted_dirs(dir: &Path) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut dirs: Vec<_> = fs::read_dir(dir)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<_, _>>()?;
    dirs.retain(|path| path.is_dir());
    dirs.sort();
    Ok(dirs)
}

fn file_name(path: &Path) -> Option<&str> {
    path.file_name().and_then(|name| name.to_str())
}
