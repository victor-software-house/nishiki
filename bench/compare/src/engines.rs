//! The engines the comparison runs, each behind one interface: set up for
//! one language, colour a document as fast as it can, and report the same
//! document's colours per byte for the fidelity score.
//!
//! Every engine colours with Shiki's github-dark, from the file the oracle
//! pins (bench/compare/README.md, fairness rule 2).

use std::hint::black_box;

use nishiki::{Highlighter, Language, State, Theme};
use syntect::easy::HighlightLines;
use syntect::highlighting::{
    Color as SynColor, ScopeSelectors, StyleModifier, Theme as SynTheme, ThemeItem, ThemeSettings,
};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter as TsHighlighter};

/// Shiki's github-dark, as the oracle pins it.
pub const GITHUB_DARK: &str = include_str!("../../../tests/oracle/themes/github-dark.json");

/// A document's colours: for each line, each span's byte end within the
/// line and its foreground, as lowercase `#rrggbb`.
pub type Colours = Vec<Vec<(usize, String)>>;

/// One engine, set up for one language.
pub trait Engine {
    /// Colours `text` and returns how many tokens or spans it made. This
    /// is the timed call: it does the engine's own work and nothing else.
    fn run(&mut self, text: &str) -> usize;

    /// The colours of `text`, for the fidelity score; never timed.
    fn colours(&mut self, text: &str) -> Colours;
}

/// The engines by name, in the order reports list them.
pub const NAMES: &[&str] = &[
    "nishiki",
    "syntect",
    "tree-sitter",
    #[cfg(feature = "giallo")]
    "giallo",
];

/// Sets up `name` for `language`, whose corpus files have extension
/// `extension`; `None` when the engine has no grammar for it.
pub fn setup(name: &str, language: Language, extension: &str) -> Option<Box<dyn Engine>> {
    match name {
        "nishiki" => Some(Box::new(Nishiki::new(language)?)),
        "syntect" => Some(Box::new(Syntect::new(extension)?)),
        "tree-sitter" => Some(Box::new(TreeSitter::new(language)?)),
        #[cfg(feature = "giallo")]
        "giallo" => Some(Box::new(crate::giallo::Giallo::new(language)?)),
        _ => None,
    }
}

/// `#rrggbb`, lowercase, from any of the forms the engines print.
pub fn hex(r: u8, g: u8, b: u8) -> String {
    format!("#{r:02x}{g:02x}{b:02x}")
}

/// A document's lines as Shiki splits them: a `\r` before `\n` belongs to
/// the line end.
pub fn lines(text: &str) -> Vec<&str> {
    text.lines()
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
        .collect()
}

/// nishiki: each line lexed, then each token styled.
struct Nishiki {
    language: Language,
    theme: Theme,
}

impl Nishiki {
    fn new(language: Language) -> Option<Nishiki> {
        let theme = Theme::from_json(GITHUB_DARK).ok()?;
        Some(Nishiki { language, theme })
    }
}

impl Engine for Nishiki {
    fn run(&mut self, text: &str) -> usize {
        let mut highlighter = Highlighter::new(self.language);
        let mut state = State::default();
        let mut tokens = Vec::new();
        let mut count = 0;
        for line in lines(text) {
            tokens.clear();
            let report = highlighter.line(line, &mut state, &mut tokens);
            black_box(&report);
            for token in &tokens {
                black_box(highlighter.style(&self.theme, token.stack()));
            }
            count += tokens.len();
        }
        count
    }

    fn colours(&mut self, text: &str) -> Colours {
        let mut highlighter = Highlighter::new(self.language);
        let mut state = State::default();
        let mut tokens = Vec::new();
        lines(text)
            .into_iter()
            .map(|line| {
                tokens.clear();
                let _ = highlighter.line(line, &mut state, &mut tokens);
                tokens
                    .iter()
                    .map(|token| {
                        let style = highlighter.style(&self.theme, token.stack());
                        (token.end(), style.foreground.to_string().to_lowercase())
                    })
                    .collect()
            })
            .collect()
    }
}

/// syntect with two-face's syntaxes (bat's set), lines with their endings.
struct Syntect {
    syntaxes: SyntaxSet,
    syntax: usize,
    theme: SynTheme,
}

impl Syntect {
    fn new(extension: &str) -> Option<Syntect> {
        let syntaxes = two_face::syntax::extra_newlines();
        let syntax = syntaxes.syntaxes().iter().position(|syntax| {
            syntax
                .file_extensions
                .iter()
                .any(|known| known == extension)
        })?;
        Some(Syntect {
            syntaxes,
            syntax,
            theme: syntect_theme()?,
        })
    }
}

/// github-dark as a syntect theme, built from Shiki's JSON rules.
fn syntect_theme() -> Option<SynTheme> {
    let json: serde_json::Value = serde_json::from_str(GITHUB_DARK).ok()?;
    let colour = |value: &serde_json::Value| -> Option<SynColor> {
        let text = value.as_str()?.strip_prefix('#')?;
        let byte = |at: usize| u8::from_str_radix(text.get(at..at + 2)?, 16).ok();
        Some(SynColor {
            r: byte(0)?,
            g: byte(2)?,
            b: byte(4)?,
            a: 0xff,
        })
    };
    let settings = ThemeSettings {
        foreground: colour(&json["colors"]["editor.foreground"]),
        ..ThemeSettings::default()
    };
    let mut scopes = Vec::new();
    for rule in json["tokenColors"].as_array()? {
        let names = match &rule["scope"] {
            serde_json::Value::String(scope) => scope.clone(),
            serde_json::Value::Array(list) => list
                .iter()
                .filter_map(serde_json::Value::as_str)
                .collect::<Vec<_>>()
                .join(", "),
            _ => continue,
        };
        let Ok(scope) = names.parse::<ScopeSelectors>() else {
            continue;
        };
        scopes.push(ThemeItem {
            scope,
            style: StyleModifier {
                foreground: colour(&rule["settings"]["foreground"]),
                background: None,
                font_style: None,
            },
        });
    }
    Some(SynTheme {
        name: Some("github-dark".into()),
        author: None,
        settings,
        scopes,
    })
}

impl Engine for Syntect {
    fn run(&mut self, text: &str) -> usize {
        let syntax = &self.syntaxes.syntaxes()[self.syntax];
        let mut lines = HighlightLines::new(syntax, &self.theme);
        let mut count = 0;
        for line in LinesWithEndings::from(text) {
            if let Ok(regions) = lines.highlight_line(line, &self.syntaxes) {
                count += regions.len();
                black_box(regions);
            }
        }
        count
    }

    fn colours(&mut self, text: &str) -> Colours {
        let syntax = &self.syntaxes.syntaxes()[self.syntax];
        let mut highlight = HighlightLines::new(syntax, &self.theme);
        LinesWithEndings::from(text)
            .map(|line| {
                let content = line.trim_end_matches(['\n', '\r']).len();
                let mut end = 0;
                let mut spans = Vec::new();
                for (style, piece) in highlight
                    .highlight_line(line, &self.syntaxes)
                    .unwrap_or_default()
                {
                    end += piece.len();
                    let f = style.foreground;
                    spans.push((end.min(content), hex(f.r, f.g, f.b)));
                }
                spans
            })
            .collect()
    }
}

/// The capture names tree-sitter's queries use, each with the `TextMate`
/// scope whose github-dark colour stands for it.
const CAPTURES: &[(&str, &str)] = &[
    ("attribute", "entity.other.attribute-name"),
    ("comment", "comment"),
    ("constant", "constant"),
    ("constant.builtin", "constant.language"),
    ("constructor", "entity.name.function"),
    ("embedded", "source"),
    ("escape", "constant.character.escape"),
    ("function", "entity.name.function"),
    ("function.builtin", "support.function"),
    ("function.method", "entity.name.function"),
    ("keyword", "keyword"),
    ("label", "entity.name.label"),
    ("module", "entity.name.namespace"),
    ("number", "constant.numeric"),
    ("operator", "keyword.operator"),
    ("property", "variable.other.property"),
    ("punctuation", "punctuation"),
    ("string", "string"),
    ("string.escape", "constant.character.escape"),
    ("string.special", "string.regexp"),
    ("tag", "entity.name.tag"),
    ("type", "entity.name.type"),
    ("type.builtin", "support.type"),
    ("variable", "variable"),
    ("variable.builtin", "variable.language"),
    ("variable.parameter", "variable.parameter"),
];

/// `@pierre/highlights`' token names that [`CAPTURES`] lacks, each with the
/// `TextMate` scope whose github-dark colour stands for it. Pierre resolves a
/// name it has no entry for from the name's prefix, as `function.method`
/// from `function`. A name is listed here only when that prefix would give
/// the wrong colour or none.
const PIERRE: &[(&str, &str)] = &[
    ("boolean", "constant.language"),
    ("diff.minus", "markup.deleted"),
    ("diff.plus", "markup.inserted"),
    ("emphasis", "markup.italic"),
    ("emphasis.strong", "markup.bold"),
    ("enum", "entity.name.type"),
    ("keyword.operator", "keyword.operator"),
    ("link_text", "string.other.link"),
    ("link_uri", "markup.underline.link"),
    ("namespace", "entity.name.namespace"),
    ("preproc", "keyword.control.directive"),
    ("property.json_key", "support.type.property-name.json"),
    (
        "punctuation.list_marker",
        "punctuation.definition.list.begin.markdown",
    ),
    ("selector", "entity.name.tag.css"),
    ("selector.class", "entity.other.attribute-name.class.css"),
    ("selector.id", "entity.other.attribute-name.id.css"),
    (
        "selector.pseudo",
        "entity.other.attribute-name.pseudo-class.css",
    ),
    ("string.regex", "string.regexp"),
    ("string.special.symbol", "constant.other.symbol"),
    ("text.literal", "markup.inline.raw"),
    ("title", "markup.heading"),
    ("variable.special", "variable.language"),
    ("variant", "variable.other.enummember"),
];

/// Shiki's github-dark as a Zed theme for `@pierre/highlights`, which takes
/// no `TextMate` theme. Each token name of [`CAPTURES`] and [`PIERRE`] gets
/// the colour nishiki's theme gives its scope. Pierre's fidelity to Shiki
/// depends on this map, so a low figure can come from the map, not the
/// engine.
pub fn pierre_theme() -> Option<String> {
    let theme = Theme::from_json(GITHUB_DARK).ok()?;
    let colour = |scope: &str| theme.style([scope]).foreground.to_string().to_lowercase();
    let syntax: serde_json::Map<String, serde_json::Value> = CAPTURES
        .iter()
        .chain(PIERRE)
        .map(|(name, scope)| {
            (
                (*name).to_owned(),
                serde_json::json!({ "color": colour(scope) }),
            )
        })
        .collect();
    let defaults = theme.defaults();
    let json = serde_json::json!({
        "name": "Shiki github-dark",
        "appearance": "dark",
        "style": {
            "editor.foreground": defaults.foreground.to_string().to_lowercase(),
            "editor.background": defaults.background.to_string().to_lowercase(),
            "syntax": syntax,
        },
    });
    serde_json::to_string_pretty(&json).ok()
}

/// tree-sitter-highlight on a whole document, with the grammar crate's own
/// highlight query.
struct TreeSitter {
    config: HighlightConfiguration,
    highlighter: TsHighlighter,
    colours: Vec<String>,
    default: String,
}

/// The grammar and highlight query for `language`. TypeScript's query
/// extends JavaScript's, and C++'s extends C's. Each is joined to its base,
/// as tree-sitter's own CLI does.
fn grammar(language: Language) -> Option<(tree_sitter::Language, String)> {
    let joined = |base: &str, own: &str| format!("{base}\n{own}");
    let pair: (tree_sitter::Language, String) = match language.id() {
        "shellscript" => (
            tree_sitter_bash::LANGUAGE.into(),
            tree_sitter_bash::HIGHLIGHT_QUERY.into(),
        ),
        "c" => (
            tree_sitter_c::LANGUAGE.into(),
            tree_sitter_c::HIGHLIGHT_QUERY.into(),
        ),
        "cpp" => (
            tree_sitter_cpp::LANGUAGE.into(),
            joined(
                tree_sitter_c::HIGHLIGHT_QUERY,
                tree_sitter_cpp::HIGHLIGHT_QUERY,
            ),
        ),
        "csharp" => (
            tree_sitter_c_sharp::LANGUAGE.into(),
            tree_sitter_c_sharp::HIGHLIGHTS_QUERY.into(),
        ),
        "css" => (
            tree_sitter_css::LANGUAGE.into(),
            tree_sitter_css::HIGHLIGHTS_QUERY.into(),
        ),
        "go" => (
            tree_sitter_go::LANGUAGE.into(),
            tree_sitter_go::HIGHLIGHTS_QUERY.into(),
        ),
        "html" => (
            tree_sitter_html::LANGUAGE.into(),
            tree_sitter_html::HIGHLIGHTS_QUERY.into(),
        ),
        "java" => (
            tree_sitter_java::LANGUAGE.into(),
            tree_sitter_java::HIGHLIGHTS_QUERY.into(),
        ),
        "javascript" => (
            tree_sitter_javascript::LANGUAGE.into(),
            tree_sitter_javascript::HIGHLIGHT_QUERY.into(),
        ),
        "json" => (
            tree_sitter_json::LANGUAGE.into(),
            tree_sitter_json::HIGHLIGHTS_QUERY.into(),
        ),
        "python" => (
            tree_sitter_python::LANGUAGE.into(),
            tree_sitter_python::HIGHLIGHTS_QUERY.into(),
        ),
        "ruby" => (
            tree_sitter_ruby::LANGUAGE.into(),
            tree_sitter_ruby::HIGHLIGHTS_QUERY.into(),
        ),
        "rust" => (
            tree_sitter_rust::LANGUAGE.into(),
            tree_sitter_rust::HIGHLIGHTS_QUERY.into(),
        ),
        "typescript" => (
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            joined(
                tree_sitter_javascript::HIGHLIGHT_QUERY,
                tree_sitter_typescript::HIGHLIGHTS_QUERY,
            ),
        ),
        "yaml" => (
            tree_sitter_yaml::LANGUAGE.into(),
            tree_sitter_yaml::HIGHLIGHTS_QUERY.into(),
        ),
        _ => return None,
    };
    Some(pair)
}

impl TreeSitter {
    fn new(language: Language) -> Option<TreeSitter> {
        let (grammar, query) = grammar(language)?;
        let mut config =
            HighlightConfiguration::new(grammar, language.id(), &query, "", "").ok()?;
        let names: Vec<&str> = CAPTURES.iter().map(|(name, _)| *name).collect();
        config.configure(&names);
        let theme = Theme::from_json(GITHUB_DARK).ok()?;
        let colours = CAPTURES
            .iter()
            .map(|(_, scope)| theme.style([*scope]).foreground.to_string().to_lowercase())
            .collect();
        let default = theme.defaults().foreground.to_string().to_lowercase();
        Some(TreeSitter {
            config,
            highlighter: TsHighlighter::new(),
            colours,
            default,
        })
    }
}

impl Engine for TreeSitter {
    fn run(&mut self, text: &str) -> usize {
        let Ok(events) =
            self.highlighter
                .highlight(&self.config, text.as_bytes(), None, None, |_| None)
        else {
            return 0;
        };
        let mut count = 0;
        for event in events.flatten() {
            if let HighlightEvent::Source { .. } = event {
                count += 1;
            }
            black_box(event);
        }
        count
    }

    fn colours(&mut self, text: &str) -> Colours {
        // The colour of each byte, then cut into lines.
        let mut per_byte = vec![self.default.as_str(); text.len()];
        if let Ok(events) =
            self.highlighter
                .highlight(&self.config, text.as_bytes(), None, None, |_| None)
        {
            let mut stack: Vec<usize> = Vec::new();
            for event in events.flatten() {
                match event {
                    HighlightEvent::HighlightStart(highlight) => stack.push(highlight.0),
                    HighlightEvent::HighlightEnd => {
                        stack.pop();
                    }
                    HighlightEvent::Source { start, end } => {
                        if let Some(&top) = stack.last() {
                            let colour = self
                                .colours
                                .get(top)
                                .map_or(self.default.as_str(), String::as_str);
                            for slot in per_byte.get_mut(start..end).into_iter().flatten() {
                                *slot = colour;
                            }
                        }
                    }
                }
            }
        }
        let mut colours = Vec::new();
        let mut offset = 0;
        for line in text.split('\n') {
            let content = line.strip_suffix('\r').unwrap_or(line);
            let mut spans: Vec<(usize, String)> = Vec::new();
            for (at, colour) in per_byte[offset..offset + content.len()].iter().enumerate() {
                match spans.last_mut() {
                    Some((end, last)) if last == colour => *end = at + 1,
                    _ => spans.push((at + 1, (*colour).to_owned())),
                }
            }
            colours.push(spans);
            offset += line.len() + 1;
        }
        if text.ends_with('\n') {
            colours.pop();
        }
        colours
    }
}
