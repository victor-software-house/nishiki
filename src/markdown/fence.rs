//! Which fenced code blocks embed a language, and which language.
//!
//! The grammar has a rule for each of some sixty languages, but Shiki drops a
//! rule whose language grammar is not loaded, and then the fence is an
//! ordinary one with no embedded scope. So a language is listed here only when
//! nishiki lexes it. To add one, add its names to [`NAMES`] and its embedded
//! scope to `rules::content_name`. A rule that embeds a language under another
//! language's name, as `php` embeds HTML, is an [`Embed`] of its own, with
//! fence and body kinds of its own.

use super::state::Kind;
use super::text::Cx;
use crate::{Language, case_fold};

/// What a fence rule embeds: the language whose lexer takes the body, under
/// the rule's `contentName`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Embed {
    /// A language, under the scope its own rule names.
    Lang(Language),
    /// The `php` rule, whose patterns include `text.html.basic` and then
    /// `source.php`. nishiki lexes no PHP, so HTML takes the body under
    /// `meta.embedded.block.php`, as in Shiki without its PHP grammar.
    #[cfg(feature = "lang-html")]
    Php,
}

impl Embed {
    /// The container the fence opens. The rule is resolved here, once per
    /// fence, so the kinds the lexer meets on every line stay keyed by a
    /// [`Language`].
    pub(crate) fn fence(self) -> Kind {
        match self {
            Embed::Lang(lang) => Kind::Fence(lang),
            #[cfg(feature = "lang-html")]
            Embed::Php => Kind::FencePhp,
        }
    }
}

/// How a fence's info string names a language, as the grammar spells it: each
/// word, and each `{.word ...}` attribute form, matched without regard to
/// case.
/// A dot in a word matches any one character but a line feed, as the
/// grammar's unescaped dot does.
struct Name {
    words: &'static [&'static str],
    /// The text after `{.` in the attribute form, as in `{.rust .numberLines}`.
    brace: Option<&'static str>,
    embed: Embed,
}

const NAMES: &[Name] = &[
    #[cfg(feature = "lang-rust")]
    Name {
        words: &["rust", "rs"],
        brace: Some("rust"),
        embed: Embed::Lang(Language::Rust),
    },
    #[cfg(feature = "lang-typescript")]
    Name {
        words: &["typescript", "ts"],
        brace: None,
        embed: Embed::Lang(Language::TypeScript),
    },
    // The grammar sends `jsx` to the JavaScript grammar, not the JSX one.
    #[cfg(feature = "lang-javascript")]
    Name {
        words: &["javascript", "jsx", "js", "es6", "mjs", "cjs", "dataviewjs"],
        brace: Some("js"),
        embed: Embed::Lang(Language::JavaScript),
    },
    #[cfg(feature = "lang-tsx")]
    Name {
        words: &["tsx"],
        brace: None,
        embed: Embed::Lang(Language::Tsx),
    },
    #[cfg(feature = "lang-markdown")]
    Name {
        words: &["markdown", "md"],
        brace: None,
        embed: Embed::Lang(Language::Markdown),
    },
    // The dot in `.textmate_init` is unescaped in the grammar.
    // It matches any one character, as in Shiki.
    #[cfg(feature = "lang-shell")]
    Name {
        words: &[
            "shell",
            "sh",
            "bash",
            "zsh",
            "bashrc",
            "bash_profile",
            "bash_login",
            "profile",
            "bash_logout",
            ".textmate_init",
        ],
        brace: Some("bash"),
        embed: Embed::Lang(Language::Shell),
    },
    #[cfg(feature = "lang-json")]
    Name {
        words: &[
            "json",
            "json5",
            "sublime-settings",
            "sublime-menu",
            "sublime-keymap",
            "sublime-mousemap",
            "sublime-theme",
            "sublime-build",
            "sublime-project",
            "sublime-completions",
        ],
        brace: None,
        embed: Embed::Lang(Language::Json),
    },
    #[cfg(feature = "lang-json")]
    Name {
        words: &["jsonc"],
        brace: None,
        embed: Embed::Lang(Language::Jsonc),
    },
    #[cfg(feature = "lang-json")]
    Name {
        words: &["jsonl", "jsonlines"],
        brace: None,
        embed: Embed::Lang(Language::Jsonl),
    },
    #[cfg(feature = "lang-python")]
    Name {
        words: &[
            "python",
            "py",
            "py3",
            "rpy",
            "pyw",
            // The grammar's word, which the spelling check reads as a typo.
            "c\x70y",
            "SConstruct",
            "Sconstruct",
            "sconstruct",
            "SConscript",
            "gyp",
            "gypi",
        ],
        brace: Some("python"),
        embed: Embed::Lang(Language::Python),
    },
    #[cfg(feature = "lang-yaml")]
    Name {
        words: &["yaml", "yml"],
        brace: None,
        embed: Embed::Lang(Language::Yaml),
    },
    #[cfg(feature = "lang-make")]
    Name {
        words: &["Makefile", "makefile", "GNUmakefile", "OCamlMakefile"],
        brace: None,
        embed: Embed::Lang(Language::Make),
    },
    #[cfg(feature = "lang-diff")]
    Name {
        words: &["patch", "diff", "rej"],
        brace: None,
        embed: Embed::Lang(Language::Diff),
    },
    #[cfg(feature = "lang-dockerfile")]
    Name {
        words: &["dockerfile"],
        brace: None,
        embed: Embed::Lang(Language::Dockerfile),
    },
    #[cfg(feature = "lang-java")]
    Name {
        words: &["java"],
        brace: None,
        embed: Embed::Lang(Language::Java),
    },
    #[cfg(feature = "lang-c")]
    Name {
        words: &["c", "h"],
        brace: None,
        embed: Embed::Lang(Language::C),
    },
    #[cfg(feature = "lang-cpp")]
    Name {
        words: &["cpp", "c++", "cxx"],
        brace: None,
        embed: Embed::Lang(Language::Cpp),
    },
    #[cfg(feature = "lang-sql")]
    Name {
        words: &["sql", "ddl", "dml"],
        brace: None,
        embed: Embed::Lang(Language::Sql),
    },
    #[cfg(feature = "lang-ini")]
    Name {
        words: &["ini", "conf"],
        brace: None,
        embed: Embed::Lang(Language::Ini),
    },
    // The dot in the grammar's `css(?:|.erb)` is unescaped.
    // It matches any one character, as in Shiki.
    #[cfg(feature = "lang-css")]
    Name {
        words: &["css", "css.erb"],
        brace: None,
        embed: Embed::Lang(Language::Css),
    },
    #[cfg(feature = "lang-go")]
    Name {
        words: &["go", "golang"],
        brace: None,
        embed: Embed::Lang(Language::Go),
    },
    // The dot in `Berksfile.lock` is unescaped in the grammar.
    // It matches any one character, as in Shiki.
    #[cfg(feature = "lang-ruby")]
    Name {
        words: &[
            "ruby",
            "rb",
            "rbx",
            "rjs",
            "Rakefile",
            "rake",
            "cgi",
            "fcgi",
            "gemspec",
            "irbrc",
            "Capfile",
            "ru",
            "prawn",
            "Cheffile",
            "Gemfile",
            "Guardfile",
            "Hobofile",
            "Vagrantfile",
            "Appraisals",
            "Rantfile",
            "Berksfile",
            "Berksfile.lock",
            "Thorfile",
            "Puppetfile",
        ],
        brace: None,
        embed: Embed::Lang(Language::Ruby),
    },
    #[cfg(feature = "lang-csharp")]
    Name {
        words: &["cs", "csharp", "c#"],
        brace: None,
        embed: Embed::Lang(Language::CSharp),
    },
    // The grammar's `xsl` rule embeds the XSL grammar, which nishiki does not
    // lex, so an `xsl` or `xslt` fence is plain.
    #[cfg(feature = "lang-xml")]
    Name {
        words: &[
            "xml", "xsd", "tld", "jsp", "pt", "cpt", "dtml", "rss", "opml",
        ],
        brace: None,
        embed: Embed::Lang(Language::Xml),
    },
    #[cfg(feature = "lang-swift")]
    Name {
        words: &["swift"],
        brace: None,
        embed: Embed::Lang(Language::Swift),
    },
    #[cfg(feature = "lang-powershell")]
    Name {
        words: &["powershell", "ps1", "psm1", "psd1", "pwsh"],
        brace: None,
        embed: Embed::Lang(Language::PowerShell),
    },
    #[cfg(feature = "lang-scala")]
    Name {
        words: &["scala", "sbt"],
        brace: None,
        embed: Embed::Lang(Language::Scala),
    },
    // The grammar's `html?|shtml|xhtml|inc|tmpl|tpl`.
    #[cfg(feature = "lang-html")]
    Name {
        words: &["html", "htm", "shtml", "xhtml", "inc", "tmpl", "tpl"],
        brace: None,
        embed: Embed::Lang(Language::Html),
    },
    #[cfg(feature = "lang-lua")]
    Name {
        words: &["lua"],
        brace: None,
        embed: Embed::Lang(Language::Lua),
    },
    #[cfg(feature = "lang-dart")]
    Name {
        words: &["dart"],
        brace: None,
        embed: Embed::Lang(Language::Dart),
    },
    #[cfg(feature = "lang-julia")]
    Name {
        words: &["julia"],
        brace: Some("julia"),
        embed: Embed::Lang(Language::Julia),
    },
    #[cfg(feature = "lang-scss")]
    Name {
        words: &["scss"],
        brace: None,
        embed: Embed::Lang(Language::Scss),
    },
    #[cfg(feature = "lang-less")]
    Name {
        words: &["less"],
        brace: None,
        embed: Embed::Lang(Language::Less),
    },
    // The grammar's `bat(?:|ch)`.
    #[cfg(feature = "lang-bat")]
    Name {
        words: &["bat", "batch"],
        brace: None,
        embed: Embed::Lang(Language::Bat),
    },
    // The grammar's `php3??|php4|php5|phpt|phtml|aw|ctp`.
    #[cfg(feature = "lang-html")]
    Name {
        words: &["php", "php3", "php4", "php5", "phpt", "phtml", "aw", "ctp"],
        brace: None,
        embed: Embed::Php,
    },
    // The dot in the grammar's `coffee|Cakefile|coffee.erb` is unescaped.
    // It matches any one character, as in Shiki.
    #[cfg(feature = "lang-coffee")]
    Name {
        words: &["coffee", "Cakefile", "coffee.erb"],
        brace: None,
        embed: Embed::Lang(Language::CoffeeScript),
    },
];

/// A language found in an info string.
pub(crate) struct Found {
    pub(crate) embed: Embed,
    /// Where the language ends, which is where its attributes start.
    pub(crate) name_end: usize,
}

/// How a word of [`NAMES`] is spelled, which picks how it is matched.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Spelling {
    /// Letters alone, matched by [`fold_end`].
    Letters,
    /// With a dot, matched by [`word_end`].
    Dotted,
}

/// A word of [`NAMES`]: its name's position, its own, and its spelling.
#[derive(Clone, Copy, Debug)]
struct Entry {
    name: usize,
    word: usize,
    spelling: Spelling,
}

impl Entry {
    const EMPTY: Entry = Entry {
        name: 0,
        word: 0,
        spelling: Spelling::Letters,
    };

    const fn of(name: usize, word: usize) -> Entry {
        let bytes = NAMES[name].words[word].as_bytes();
        let mut k = 0;
        let mut spelling = Spelling::Letters;
        while k < bytes.len() {
            if bytes[k] == b'.' {
                spelling = Spelling::Dotted;
            }
            k += 1;
        }
        Entry {
            name,
            word,
            spelling,
        }
    }

    /// The word's bytes.
    fn bytes(self) -> Option<&'static [u8]> {
        Some(NAMES.get(self.name)?.words.get(self.word)?.as_bytes())
    }
}

/// Whether a word starts with a dot, which matches any first character.
const fn leads_with_dot(word: &str) -> bool {
    word.as_bytes()[0] == b'.'
}

/// How many words [`NAMES`] holds that start with a letter, and how many
/// start with a dot.
const COUNTS: (usize, usize) = {
    let mut counts = (0, 0);
    let mut n = 0;
    while n < NAMES.len() {
        let mut w = 0;
        while w < NAMES[n].words.len() {
            if leads_with_dot(NAMES[n].words[w]) {
                counts.1 += 1;
            } else {
                counts.0 += 1;
            }
            w += 1;
        }
        n += 1;
    }
    counts
};

/// The bucket of a word's first byte, lowercased: every word starts with an
/// ASCII byte, so a word that does not fails the build.
#[expect(
    clippy::as_conversions,
    reason = "`usize::from` is not const; a `u8` always fits a `usize`"
)]
const fn bucket(byte: u8) -> usize {
    byte.to_ascii_lowercase() as usize
}

/// Where each first byte's words start in [`BY_FIRST`]: the words of byte `b`
/// sit at `STARTS[b]..STARTS[b + 1]`.
const STARTS: [usize; 129] = {
    let mut starts = [0; 129];
    let mut n = 0;
    while n < NAMES.len() {
        let mut w = 0;
        while w < NAMES[n].words.len() {
            let word = NAMES[n].words[w];
            if !leads_with_dot(word) {
                starts[bucket(word.as_bytes()[0]) + 1] += 1;
            }
            w += 1;
        }
        n += 1;
    }
    let mut b = 1;
    while b < starts.len() {
        starts[b] += starts[b - 1];
        b += 1;
    }
    starts
};

/// Every word of [`NAMES`] that starts with a letter, grouped by that
/// letter, lowercased, and in table order within a group, so the first match
/// in a group is the first in the table.
const BY_FIRST: [Entry; COUNTS.0] = {
    let mut by_first = [Entry::EMPTY; COUNTS.0];
    let mut next = STARTS;
    let mut n = 0;
    while n < NAMES.len() {
        let mut w = 0;
        while w < NAMES[n].words.len() {
            let word = NAMES[n].words[w];
            if !leads_with_dot(word) {
                let b = bucket(word.as_bytes()[0]);
                by_first[next[b]] = Entry::of(n, w);
                next[b] += 1;
            }
            w += 1;
        }
        n += 1;
    }
    by_first
};

/// Every word of [`NAMES`] that starts with a dot, in table order.
/// Its dot matches any first character, so it belongs to no group.
const AFTER_ANY: [Entry; COUNTS.1] = {
    let mut after_any = [Entry::EMPTY; COUNTS.1];
    let mut next = 0;
    let mut n = 0;
    while n < NAMES.len() {
        let mut w = 0;
        while w < NAMES[n].words.len() {
            if leads_with_dot(NAMES[n].words[w]) {
                after_any[next] = Entry::of(n, w);
                next += 1;
            }
            w += 1;
        }
        n += 1;
    }
    after_any
};

/// The language the info string at `i` names, when the attributes after it
/// are allowed: nothing, or a space or one of `,:?{` and then no backtick.
///
/// An ASCII first byte other than `{` can only start a word of its own
/// letter, as the folds that match more than ASCII case all start with a
/// non-ASCII byte, so it tries only the words of its group in [`BY_FIRST`].
/// Then it tries the words that start with a dot, in [`AFTER_ANY`], when
/// the second byte can match theirs.
/// No word of a group spells a dotted word's letters, and a name's
/// attributes start with a space or one of `,:?{`, which no word holds.
/// So a word that starts with a dot never matches where a group's word does,
/// and trying it last keeps the table's order.
/// Any other first byte tries the whole table.
pub(crate) fn language(cx: &Cx<'_>, i: usize) -> Option<Found> {
    let backtick = last_backtick(cx, i);
    let first = cx.get(i)?;
    if first.is_ascii() && first != b'{' {
        let b = bucket(first);
        let group = STARTS
            .get(b)
            .zip(STARTS.get(b + 1))
            .and_then(|(&start, &end)| BY_FIRST.get(start..end))
            .unwrap_or_default();
        for &entry in group {
            if let Some(lang) = NAMES.get(entry.name)
                && let Some(word) = lang.words.get(entry.word)
                && let Some(end) = match entry.spelling {
                    Spelling::Letters => fold_end(cx, i, word.as_bytes()),
                    Spelling::Dotted => word_end(cx, i, word.as_bytes()),
                }
                && attributes_ok(cx, end, backtick)
            {
                return Some(Found {
                    embed: lang.embed,
                    name_end: end,
                });
            }
        }
        return after_any(cx, i, backtick);
    }
    for lang in NAMES {
        for word in lang.words {
            if let Some(end) = word_end(cx, i, word.as_bytes())
                && attributes_ok(cx, end, backtick)
            {
                return Some(Found {
                    embed: lang.embed,
                    name_end: end,
                });
            }
        }
        if let Some(name) = lang.brace
            && cx.starts(i, b"{.")
            && let Some(end) = fold_end(cx, i + 2, name.as_bytes())
        {
            // `.+?}`: at least one character, then the first `}` that the
            // attributes can follow.
            let mut close = end + 1;
            let found = loop {
                match cx.get(close) {
                    None | Some(b'\n') => break None,
                    Some(b'}') if attributes_ok(cx, close + 1, backtick) => break Some(close),
                    Some(_) => close += 1,
                }
            };
            cx.spend(close - end);
            if let Some(close) = found {
                return Some(Found {
                    embed: lang.embed,
                    name_end: close + 1,
                });
            }
        }
    }
    None
}

/// The language that a word of [`AFTER_ANY`] names at `i`, when its
/// attributes are allowed.
/// A word is tried only when the second byte at `i` can match its own.
#[cold]
#[inline(never)]
fn after_any(cx: &Cx<'_>, i: usize, backtick: Option<usize>) -> Option<Found> {
    let second = cx.get(i + 1)?;
    for entry in AFTER_ANY {
        let word = entry.bytes()?;
        let may_match = word
            .get(1)
            .is_some_and(|&want| !second.is_ascii() || second.eq_ignore_ascii_case(&want));
        if may_match
            && let Some(end) = word_end(cx, i, word)
            && attributes_ok(cx, end, backtick)
        {
            return Some(Found {
                embed: NAMES.get(entry.name)?.embed,
                name_end: end,
            });
        }
    }
    None
}

/// The end of a fence name's `word` matched at `i`.
/// The word's letters match as in [`fold_end`].
/// Each dot matches any one character but a line feed, as the grammar's
/// unescaped dot does: `css.erb` also reads `cssXerb` and `css一erb`.
fn word_end(cx: &Cx<'_>, i: usize, word: &[u8]) -> Option<usize> {
    let Some(dot) = memchr::memchr(b'.', word) else {
        return fold_end(cx, i, word);
    };
    let at = fold_end(cx, i, word.get(..dot)?)?;
    if cx.get(at)? == b'\n' {
        return None;
    }
    let after = at + cx.char_len(at);
    word_end(cx, after, word.get(dot + 1..)?)
}

/// The end of `word` matched at `i` the way a case-insensitive Oniguruma
/// pattern matches it, which folds more than ASCII case: the long s reads as
/// `s`, `ß` as `ss` and the ligature `ﬁ` as `fi`.
///
/// The word is compared as ASCII in one pass that stops at the first byte
/// that differs. Up to there the text has spelled the word one ASCII byte a
/// letter, so a fold can start only at that byte, and only when it is above
/// ASCII; only then is the rest read through [`folded_end`].
fn fold_end(cx: &Cx<'_>, i: usize, word: &[u8]) -> Option<usize> {
    let rest = cx.line.get(i..cx.end.min(cx.line.len()))?;
    match rest
        .iter()
        .zip(word)
        .position(|(have, want)| !have.eq_ignore_ascii_case(want))
    {
        None => (rest.len() >= word.len()).then_some(i + word.len()),
        Some(k) if rest.get(k).is_some_and(|b| !b.is_ascii()) => {
            folded_end(cx, i + k, word.get(k..)?)
        }
        Some(_) => None,
    }
}

/// [`fold_end`] from the first byte that differs, at `i`, which is above
/// ASCII, with the rest of the word ([`case_fold::literal_end`]).
#[cold]
fn folded_end(cx: &Cx<'_>, i: usize, rest: &[u8]) -> Option<usize> {
    case_fold::literal_end(cx.line.get(..cx.end.min(cx.line.len()))?, i, rest)
}

/// The attributes that may follow a language name at `i`: nothing, or a space
/// or one of `,:?{` and then anything but a backtick, to the end of the line.
/// `backtick` is the line's last backtick at or after the info string.
fn attributes_ok(cx: &Cx<'_>, i: usize, backtick: Option<usize>) -> bool {
    match cx.get(i) {
        None | Some(b'\n') => true,
        Some(b) if cx.space_at(i) > 0 || matches!(b, b',' | b':' | b'?' | b'{') => {
            backtick.is_none_or(|last| last < i)
        }
        Some(_) => false,
    }
}

/// The last backtick in the text from `i`, found once per info string so
/// that each place the attributes may start costs nothing more to check.
fn last_backtick(cx: &Cx<'_>, i: usize) -> Option<usize> {
    let line_end = cx.end.min(cx.line.len());
    let rest = cx.line.get(i..line_end)?;
    cx.spend(rest.len());
    memchr::memrchr(b'`', rest).map(|at| i + at)
}

/// Whether a backtick is anywhere in the text from `i`.
pub(crate) fn has_backtick(cx: &Cx<'_>, i: usize) -> bool {
    let line_end = cx.end.min(cx.line.len());
    cx.line
        .get(i..line_end)
        .is_some_and(|rest| memchr::memchr(b'`', rest).is_some())
}
