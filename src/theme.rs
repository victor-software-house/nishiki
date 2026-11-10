//! `TextMate` themes in Shiki's format, and the style a theme gives a scope
//! stack.
//!
//! A theme is a list of rules, each a scope selector with a colour or font
//! style. A token's style folds over its scope stack from the outermost scope
//! in. At each scope, the most specific rule that matches the stack so far
//! sets the attributes it names. It leaves the rest as they were.
//!
//! This is the resolution VS Code's `TextMate` engine applies and Shiki
//! inherits. A theme therefore colours nishiki's scopes as it colours
//! Shiki's.

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(feature = "json")]
mod json;
mod trie;

#[cfg(feature = "json")]
pub use json::ThemeError;

/// An sRGB colour with alpha.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Color {
    /// Red.
    pub r: u8,
    /// Green.
    pub g: u8,
    /// Blue.
    pub b: u8,
    /// Alpha, 255 for opaque.
    pub a: u8,
}

impl Color {
    /// The colour a hex string names: `#rgb`, `#rgba`, `#rrggbb` or
    /// `#rrggbbaa`, in either case. `None` for anything else, which a theme
    /// treats as no colour, as VS Code does.
    ///
    /// ```
    /// use nishiki::Color;
    ///
    /// assert_eq!(
    ///     Color::from_hex("#f97583"),
    ///     Some(Color {
    ///         r: 0xf9,
    ///         g: 0x75,
    ///         b: 0x83,
    ///         a: 0xff
    ///     })
    /// );
    /// assert_eq!(
    ///     Color::from_hex("#0f08"),
    ///     Some(Color {
    ///         r: 0,
    ///         g: 0xff,
    ///         b: 0,
    ///         a: 0x88
    ///     })
    /// );
    /// assert_eq!(Color::from_hex("red"), None);
    /// ```
    #[must_use]
    pub fn from_hex(text: &str) -> Option<Color> {
        let digits = text.strip_prefix('#')?.as_bytes();
        let nibble = |byte: u8| {
            char::from(byte)
                .to_digit(16)
                .and_then(|value| u8::try_from(value).ok())
        };
        let short = |index: usize| nibble(digits[index]).map(|value| value * 17);
        let long = |index: usize| Some(nibble(digits[index])? * 16 + nibble(digits[index + 1])?);
        let [r, g, b, a] = match digits.len() {
            3 => [short(0)?, short(1)?, short(2)?, 255],
            4 => [short(0)?, short(1)?, short(2)?, short(3)?],
            6 => [long(0)?, long(2)?, long(4)?, 255],
            8 => [long(0)?, long(2)?, long(4)?, long(6)?],
            _ => return None,
        };
        Some(Color { r, g, b, a })
    }
}

impl fmt::Display for Color {
    /// `#RRGGBB`, or `#RRGGBBAA` when the colour is not opaque.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)?;
        if self.a != 255 {
            write!(f, "{:02X}", self.a)?;
        }
        Ok(())
    }
}

/// A set of font styles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FontStyle(u8);

impl FontStyle {
    /// No style.
    pub const NONE: FontStyle = FontStyle(0);
    /// Italic.
    pub const ITALIC: FontStyle = FontStyle(1);
    /// Bold.
    pub const BOLD: FontStyle = FontStyle(2);
    /// Underline.
    pub const UNDERLINE: FontStyle = FontStyle(4);
    /// Strikethrough.
    pub const STRIKETHROUGH: FontStyle = FontStyle(8);

    /// The styles a theme's `fontStyle` names, such as `"italic bold"`;
    /// unknown words are ignored and `""` is no style.
    #[must_use]
    pub fn parse(text: &str) -> FontStyle {
        text.split(' ')
            .fold(FontStyle::NONE, |style, word| match word {
                "italic" => style | FontStyle::ITALIC,
                "bold" => style | FontStyle::BOLD,
                "underline" => style | FontStyle::UNDERLINE,
                "strikethrough" => style | FontStyle::STRIKETHROUGH,
                _ => style,
            })
    }

    /// Whether every style in `other` is in `self`.
    #[must_use]
    pub fn contains(self, other: FontStyle) -> bool {
        self.0 & other.0 == other.0
    }

    /// The styles as Shiki numbers them: italic 1, bold 2, underline 4,
    /// strikethrough 8.
    #[must_use]
    pub fn bits(self) -> u8 {
        self.0
    }
}

impl std::ops::BitOr for FontStyle {
    type Output = FontStyle;

    fn bitor(self, other: FontStyle) -> FontStyle {
        FontStyle(self.0 | other.0)
    }
}

/// Whether a theme is meant for a light or a dark background.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ColorScheme {
    /// A light background.
    Light,
    /// A dark background, Shiki's default when a theme names neither.
    #[default]
    Dark,
}

/// The style a theme gives a token.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Style {
    /// The text colour.
    pub foreground: Color,
    /// The background colour.
    pub background: Color,
    /// Italic, bold, underline and strikethrough.
    pub font: FontStyle,
}

/// The scopes a theme rule applies to, as a theme file writes them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum RuleScope {
    /// No scope: the rule sets the theme's defaults.
    #[default]
    Defaults,
    /// One string of comma-separated selectors, such as
    /// `"string, comment.line"`.
    Selectors(String),
    /// A list of selectors, one per entry.
    List(Vec<String>),
}

/// One rule of a theme, as a theme file writes it.
///
/// Colours and font styles stay as text: a colour that is not hex sets no
/// colour, as in VS Code.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ThemeRule {
    /// The rule's name, which Shiki uses to find the defaults rule.
    pub name: Option<String>,
    /// What the rule applies to.
    pub scope: RuleScope,
    /// The text colour.
    pub foreground: Option<String>,
    /// The background colour.
    pub background: Option<String>,
    /// The font styles, such as `"italic bold"`.
    pub font_style: Option<String>,
}

/// A theme as a theme file states it, before its rules are resolved.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ThemeSource {
    /// The theme's name.
    pub name: Option<String>,
    /// Light or dark; dark when unset.
    pub color_scheme: Option<ColorScheme>,
    /// The editor's text colour, `colors["editor.foreground"]` in the file.
    pub editor_foreground: Option<String>,
    /// The editor's background, `colors["editor.background"]` in the file.
    pub editor_background: Option<String>,
    /// The rules, `tokenColors` or `settings` in the file.
    pub rules: Vec<ThemeRule>,
}

/// A resolved theme, ready to style scope stacks.
///
/// ```
/// use nishiki::{RuleScope, Theme, ThemeRule, ThemeSource};
///
/// let theme = Theme::new(ThemeSource {
///     editor_foreground: Some("#e1e4e8".into()),
///     editor_background: Some("#24292e".into()),
///     rules: vec![ThemeRule {
///         scope: RuleScope::Selectors("string, comment".into()),
///         foreground: Some("#9ecbff".into()),
///         ..ThemeRule::default()
///     }],
///     ..ThemeSource::default()
/// });
/// let style = theme.style(["string.quoted.double.rust", "source.rust"]);
/// assert_eq!(style.foreground.to_string(), "#9ECBFF");
/// assert_eq!(
///     theme.style(["source.rust"]).foreground.to_string(),
///     "#E1E4E8"
/// );
/// ```
#[derive(Clone, Debug)]
pub struct Theme {
    name: Option<String>,
    color_scheme: ColorScheme,
    defaults: Style,
    rules: trie::Trie,
    /// Unique to the [`Theme::new`] call that built the theme; a clone keeps
    /// it, since it holds the same rules.
    /// A highlighter's style cache belongs to one id.
    id: u64,
}

/// The id the next theme gets.
static NEXT_THEME: AtomicU64 = AtomicU64::new(0);

/// How many scopes [`Theme::style`] holds on the stack.
/// A deeper stack moves to the heap, so styling a token allocates nothing
/// below this depth.
const INLINE_SCOPES: usize = 64;

/// Shiki's fallback editor colours, VS Code's own, for a theme that names
/// none.
const FALLBACK_LIGHT: (&str, &str) = ("#333333", "#fffffe");
const FALLBACK_DARK: (&str, &str) = ("#bbbbbb", "#1e1e1e");

impl RuleScope {
    /// Whether Shiki reads the scope as absent: none, or an empty string.
    fn is_absent(&self) -> bool {
        match self {
            RuleScope::Defaults => true,
            RuleScope::Selectors(text) => text.is_empty(),
            RuleScope::List(_) => false,
        }
    }
}

impl Theme {
    /// Resolves `source`'s rules into a theme.
    ///
    /// The theme's own colours come from three places, in order, as in Shiki:
    ///
    /// - its first rule with no name and no scope;
    /// - the editor colours;
    /// - VS Code's fallbacks for its [`ColorScheme`].
    #[must_use]
    pub fn new(source: ThemeSource) -> Theme {
        let color_scheme = source.color_scheme.unwrap_or_default();
        let global = source
            .rules
            .iter()
            .find(|rule| rule.name.as_deref().is_none_or(str::is_empty) && rule.scope.is_absent());
        let fallback = match color_scheme {
            ColorScheme::Light => FALLBACK_LIGHT,
            ColorScheme::Dark => FALLBACK_DARK,
        };
        let pick = |from_rule: Option<&String>, editor: Option<&String>, fallback: &str| {
            from_rule
                .or(editor)
                .cloned()
                .unwrap_or_else(|| fallback.to_owned())
        };
        let foreground = pick(
            global.and_then(|rule| rule.foreground.as_ref()),
            source.editor_foreground.as_ref(),
            fallback.0,
        );
        let background = pick(
            global.and_then(|rule| rule.background.as_ref()),
            source.editor_background.as_ref(),
            fallback.1,
        );
        let mut rules = source.rules;
        if rules.first().is_none_or(|first| !first.scope.is_absent()) {
            rules.insert(
                0,
                ThemeRule {
                    foreground: Some(foreground),
                    background: Some(background),
                    ..ThemeRule::default()
                },
            );
        }
        let (defaults, rules) = trie::Trie::build(&rules);
        Theme {
            name: source.name,
            color_scheme,
            defaults,
            rules,
            id: NEXT_THEME.fetch_add(1, Ordering::Relaxed),
        }
    }

    /// Parses a theme file in Shiki's JSON format, as `@shikijs/themes`
    /// ships them and VS Code reads them.
    ///
    /// # Errors
    ///
    /// When `json` is not valid JSON, or not a JSON object.
    #[cfg(feature = "json")]
    pub fn from_json(json: &str) -> Result<Theme, ThemeError> {
        json::source(json).map(Theme::new)
    }

    /// The theme's name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Whether the theme is light or dark.
    #[must_use]
    pub fn color_scheme(&self) -> ColorScheme {
        self.color_scheme
    }

    /// The style of text no rule matches.
    #[must_use]
    pub fn defaults(&self) -> Style {
        self.defaults
    }

    /// The id that tells this theme's styles from another's.
    pub(crate) fn id(&self) -> u64 {
        self.id
    }
    /// The style of a token whose scope stack is `scopes`, innermost first,
    /// as [`Highlighter::scopes`](crate::Highlighter::scopes) lists them.
    ///
    /// A renderer styles many tokens with few distinct stacks.
    /// [`Highlighter::style`](crate::Highlighter::style) keeps each stack's
    /// style once computed, and gives the same style.
    #[must_use]
    pub fn style<'a>(&self, scopes: impl IntoIterator<Item = &'a str>) -> Style {
        let mut inline = [""; INLINE_SCOPES];
        let mut spilled = Vec::new();
        let mut len = 0;
        for scope in scopes {
            if let Some(slot) = inline.get_mut(len) {
                *slot = scope;
            } else {
                if spilled.is_empty() {
                    spilled.extend_from_slice(&inline);
                }
                spilled.push(scope);
            }
            len += 1;
        }
        let path = if spilled.is_empty() {
            inline.get_mut(..len).unwrap_or_default()
        } else {
            spilled.as_mut_slice()
        };
        path.reverse();
        let mut style = self.defaults;
        for depth in 1..=path.len() {
            self.rules.apply(&path[..depth], &mut style);
        }
        style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule(scope: &str, foreground: &str) -> ThemeRule {
        ThemeRule {
            scope: RuleScope::Selectors(scope.into()),
            foreground: Some(foreground.into()),
            ..ThemeRule::default()
        }
    }

    fn theme(rules: Vec<ThemeRule>) -> Theme {
        Theme::new(ThemeSource {
            editor_foreground: Some("#000000".into()),
            editor_background: Some("#ffffff".into()),
            rules,
            ..ThemeSource::default()
        })
    }

    fn fg(theme: &Theme, innermost_first: &[&str]) -> String {
        theme
            .style(innermost_first.iter().copied())
            .foreground
            .to_string()
    }

    #[test]
    fn a_deeper_scope_match_wins() {
        let theme = theme(vec![
            rule("string", "#111111"),
            rule("string.quoted", "#222222"),
        ]);
        assert_eq!(fg(&theme, &["string.quoted.double", "source"]), "#222222");
        assert_eq!(fg(&theme, &["string.other", "source"]), "#111111");
        assert_eq!(
            fg(&theme, &["strings", "source"]),
            "#000000",
            "a prefix must end at a dot"
        );
    }

    #[test]
    fn parent_selectors_narrow_a_rule() {
        let theme = theme(vec![
            rule("string", "#111111"),
            rule("source.rust string", "#222222"),
        ]);
        assert_eq!(fg(&theme, &["string.quoted", "source.rust"]), "#222222");
        assert_eq!(fg(&theme, &["string.quoted", "source.js"]), "#111111");
    }

    #[test]
    fn the_child_combinator_needs_the_direct_parent() {
        let theme = theme(vec![rule("meta > string", "#222222")]);
        assert_eq!(fg(&theme, &["string", "meta"]), "#222222");
        assert_eq!(fg(&theme, &["string", "other", "meta"]), "#000000");
    }

    #[test]
    fn outer_scopes_lend_what_inner_rules_leave_unset() {
        let mut bold = rule("markup.bold", "#111111");
        bold.font_style = Some("bold".into());
        let theme = theme(vec![bold, rule("markup.bold string", "#222222")]);
        let style = theme.style(["string", "markup.bold"]);
        assert_eq!(style.foreground.to_string(), "#222222");
        assert!(
            style.font.contains(FontStyle::BOLD),
            "bold carries in from the outer scope"
        );
    }

    #[test]
    fn a_stack_deeper_than_the_inline_buffer_styles_as_a_shallow_one() {
        let theme = theme(vec![
            rule("string", "#111111"),
            rule("source.rust string", "#222222"),
        ]);
        let mut deep = vec!["string.quoted"];
        deep.extend(std::iter::repeat_n("meta.block", INLINE_SCOPES + 5));
        deep.push("source.rust");
        assert_eq!(fg(&theme, &deep), "#222222");
        assert_eq!(fg(&theme, &deep[..INLINE_SCOPES]), "#111111");
    }

    #[test]
    fn later_rules_for_the_same_selector_override_earlier_ones() {
        let theme = theme(vec![rule("comment", "#111111"), rule("comment", "#222222")]);
        assert_eq!(fg(&theme, &["comment"]), "#222222");
    }

    #[test]
    fn colours_parse_in_every_hex_form() {
        assert_eq!(
            Color::from_hex("#ABC").map(|c| c.to_string()),
            Some("#AABBCC".into())
        );
        assert_eq!(
            Color::from_hex("#aabbcc80").map(|c| c.to_string()),
            Some("#AABBCC80".into())
        );
        assert_eq!(Color::from_hex("#abcd").map(|c| c.a), Some(0xdd));
        assert_eq!(Color::from_hex("#abcde"), None);
        assert_eq!(Color::from_hex("#ggg"), None);
    }

    #[test]
    fn defaults_follow_shiki_when_a_theme_names_no_colours() {
        let light = Theme::new(ThemeSource {
            color_scheme: Some(ColorScheme::Light),
            ..ThemeSource::default()
        });
        assert_eq!(light.defaults().foreground.to_string(), "#333333");
        assert_eq!(light.defaults().background.to_string(), "#FFFFFE");
        let dark = Theme::new(ThemeSource::default());
        assert_eq!(dark.defaults().foreground.to_string(), "#BBBBBB");
    }
}
