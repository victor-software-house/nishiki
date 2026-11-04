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
use crate::Language;

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
    // The grammar's `.textmate_init` has an unescaped dot, which matches any
    // character; it is matched here as a literal dot.
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
    // The grammar's `css(?:|.erb)` has an unescaped dot, which matches any
    // character; it is matched here as a literal dot.
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
    // The grammar's `Berksfile.lock` has an unescaped dot, which matches any
    // character; it is matched here as a literal dot.
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
];

/// A language found in an info string.
pub(crate) struct Found {
    pub(crate) embed: Embed,
    /// Where the language ends, which is where its attributes start.
    pub(crate) name_end: usize,
}

/// How many words [`NAMES`] holds.
const WORDS: usize = {
    let mut total = 0;
    let mut n = 0;
    while n < NAMES.len() {
        total += NAMES[n].words.len();
        n += 1;
    }
    total
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
            starts[bucket(NAMES[n].words[w].as_bytes()[0]) + 1] += 1;
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

/// Every word of [`NAMES`] as its name's position and its own, grouped by
/// the word's first byte, lowercased, and in table order within a group, so
/// the first match in a group is the first in the table.
const BY_FIRST: [(usize, usize); WORDS] = {
    let mut by_first = [(0, 0); WORDS];
    let mut next = STARTS;
    let mut n = 0;
    while n < NAMES.len() {
        let mut w = 0;
        while w < NAMES[n].words.len() {
            let b = bucket(NAMES[n].words[w].as_bytes()[0]);
            by_first[next[b]] = (n, w);
            next[b] += 1;
            w += 1;
        }
        n += 1;
    }
    by_first
};

/// The language the info string at `i` names, when the attributes after it
/// are allowed: nothing, or a space or one of `,:?{` and then no backtick.
///
/// An ASCII first byte other than `{` can only start a word of its own
/// letter, as the folds that match more than ASCII case all start with a
/// non-ASCII byte, so it tries only the words of its group in [`BY_FIRST`].
/// Any other first byte tries the whole table.
pub(crate) fn language(cx: &Cx<'_>, i: usize) -> Option<Found> {
    let backtick = last_backtick(cx, i);
    if let Some(first) = cx.get(i)
        && first.is_ascii()
        && first != b'{'
    {
        let b = bucket(first);
        let group = STARTS
            .get(b)
            .zip(STARTS.get(b + 1))
            .and_then(|(&start, &end)| BY_FIRST.get(start..end))
            .unwrap_or_default();
        for &(n, w) in group {
            if let Some(lang) = NAMES.get(n)
                && let Some(word) = lang.words.get(w)
                && let Some(end) = fold_end(cx, i, word.as_bytes())
                && attributes_ok(cx, end, backtick)
            {
                return Some(Found {
                    embed: lang.embed,
                    name_end: end,
                });
            }
        }
        return None;
    }
    for lang in NAMES {
        for word in lang.words {
            if let Some(end) = fold_end(cx, i, word.as_bytes())
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

/// The end of `word` matched at `i` the way a case-insensitive Oniguruma
/// pattern matches it, which folds more than ASCII case: the Kelvin sign
/// U+212A reads as `k`, the long s U+017F as `s`, and the ligatures U+FB00 to
/// U+FB06 as the letters they stand for (`ff`, `fi`, `fl`, `ffi`, `ffl`,
/// `st`). Checked against Shiki's Markdown grammar for each of them.
fn fold_end(cx: &Cx<'_>, i: usize, word: &[u8]) -> Option<usize> {
    let (mut at, mut k) = (i, 0);
    while k < word.len() {
        let b = cx.get(at)?;
        if b.eq_ignore_ascii_case(&word[k]) {
            at += 1;
            k += 1;
            continue;
        }
        let (len, folded): (usize, &[u8]) = match (b, cx.get(at + 1), cx.get(at + 2)) {
            (0xe2, Some(0x84), Some(0xaa)) => (3, b"k"),
            (0xc5, Some(0xbf), _) => (2, b"s"),
            (0xef, Some(0xac), Some(0x80)) => (3, b"ff"),
            (0xef, Some(0xac), Some(0x81)) => (3, b"fi"),
            (0xef, Some(0xac), Some(0x82)) => (3, b"fl"),
            (0xef, Some(0xac), Some(0x83)) => (3, b"ffi"),
            (0xef, Some(0xac), Some(0x84)) => (3, b"ffl"),
            (0xef, Some(0xac), Some(0x85 | 0x86)) => (3, b"st"),
            _ => return None,
        };
        let rest = word.get(k..k + folded.len())?;
        if !rest.eq_ignore_ascii_case(folded) {
            return None;
        }
        at += len;
        k += folded.len();
    }
    Some(at)
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
