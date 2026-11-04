//! Values: variables, maps, function calls, escapes, operators and the
//! parts of a property.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The functions of `constant_sass_functions`, its alternatives spelled out.
/// The expression takes the first alternative that a `(` follows.
/// No word of it holds a `(`.
/// The text before the next `(` therefore decides.
const SASS_FUNCTIONS: &[&[u8]] = &[
    b"headings",
    b"stylesheet-url",
    b"rgb",
    b"rgba",
    b"hsl",
    b"hsla",
    b"ie-hex-str",
    b"red",
    b"green",
    b"blue",
    b"alpha",
    b"opacity",
    b"hue",
    b"saturation",
    b"lightness",
    b"prefixed",
    b"prefix",
    b"-moz",
    b"-svg",
    b"-css2",
    b"-pie",
    b"-webkit",
    b"-ms",
    b"font-files",
    b"font-url",
    b"grid-image",
    b"image-width",
    b"image-height",
    b"image-url",
    b"image-color",
    b"sprite",
    b"sprites",
    b"sprite-map",
    b"sprite-map-name",
    b"sprite-file",
    b"sprite-url",
    b"sprite-position",
    b"inline-font-files",
    b"inline-image",
    b"opposite-position",
    b"grad-point",
    b"grad-end-position",
    b"color-stops",
    b"color-stops-in-percentages",
    b"grad-color-stops",
    b"radial-gradient",
    b"radial-svg-gradient",
    b"linear-gradient",
    b"linear-svg-gradient",
    b"opacify",
    b"fadein",
    b"fade-in",
    b"transparentize",
    b"fadeout",
    b"fade-out",
    b"lighten",
    b"darken",
    b"saturate",
    b"desaturate",
    b"grayscale",
    b"adjust-hue",
    b"adjust-lightness",
    b"adjust-saturation",
    b"adjust-color",
    b"scale-lightness",
    b"scale-saturation",
    b"scale-color",
    b"change-color",
    b"spin",
    b"complement",
    b"invert",
    b"mix",
    b"-compass-list",
    b"-compass-space-list",
    b"-compass-slice",
    b"-compass-nth",
    b"-compass-list-size",
    b"blank",
    b"compact",
    b"nth",
    b"first-value-of",
    b"join",
    b"length",
    b"append",
    b"nest",
    b"append-selector",
    b"headers",
    b"enumerate",
    b"range",
    b"percentage",
    b"unitless",
    b"unit",
    b"if",
    b"type-of",
    b"comparable",
    b"elements-of-type",
    b"quote",
    b"unquote",
    b"escape",
    b"e",
    b"sin",
    b"cos",
    b"tan",
    b"abs",
    b"round",
    b"ceil",
    b"floor",
    b"pi",
    b"translateX",
    b"translateY",
];

/// The longest word of [`SASS_FUNCTIONS`], in bytes.
const LONGEST_SASS_FUNCTION: usize = 26;

/// Whether `b` is in `[-*+/]`.
fn arithmetic(b: u8) -> bool {
    matches!(b, b'-' | b'*' | b'+' | b'/')
}

impl Lexer<'_> {
    /// `(?=\$[-\w]+\s*:)`: the name cannot give back a character that would
    /// be white space or the colon.
    pub(super) fn variable_setting(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let name = self.dash_word_run(p + 1)?;
        let colon = self.spaces(name);
        (self.at(colon) == b':').then_some(p)
    }

    /// `\$[-\w]+(?=\s*:)`.
    pub(super) fn variable_setting_at0(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let name = self.dash_word_run(p + 1)?;
        let colon = self.spaces(name);
        (self.at(colon) == b':').then_some(name)
    }

    /// `\n`: the line feed that ends the line.
    pub(super) fn comment_line_end(&mut self, p: usize) -> Option<usize> {
        (p == self.lim() && self.nl).then_some(p + 1)
    }

    /// `\b([-\w]+)\s*(:)`.
    pub(super) fn map_at3(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let key = self.dash_word_run(p)?;
        let colon = self.spaces(key);
        if self.at(colon) != b':' {
            return None;
        }
        self.cap(1, p, key);
        self.cap(2, colon, colon + 1);
        Some(colon + 1)
    }

    /// `\b([-\w]+)(\.)(\$[-\w]+)\b`.
    /// The module name cannot give back a character that would be the dot.
    /// The variable's name gives characters back until a `\b` follows.
    pub(super) fn variables_at0(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let module = self.dash_word_run(p)?;
        if self.at(module) != b'.' || self.at(module + 1) != b'$' {
            return None;
        }
        let dollar = module + 1;
        let name = self.dash_word_run(dollar + 1)?;
        let end = self.boundary_back(dollar + 1, name)?;
        self.cap(1, p, module);
        self.cap(2, module, module + 1);
        self.cap(3, dollar, end);
        Some(end)
    }

    /// `(\$|--)[-0-9A-Z_a-z]+\b`: the name gives characters back until a
    /// `\b` follows.
    pub(super) fn variables_at1(&mut self, p: usize) -> Option<usize> {
        let start = if self.at(p) == b'$' {
            p + 1
        } else {
            self.lit(p, b"--")?
        };
        if !matches!(self.at(start), b'-' | b'_' | b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z') {
            return None;
        }
        let (from, to, last) = self.x.variable.get();
        if from < start && start < to {
            return last.filter(|&end| end > start);
        }
        let name = self.skip(start, |b| {
            b == b'-' || b == b'_' || b.is_ascii_alphanumeric()
        });
        let last = self.boundary_back(start, name);
        self.x.variable.set((start, name, last));
        last
    }

    /// `\\(\h{1,6}|.)`: a backslash and up to six hex digits.
    /// Otherwise it is a backslash and one character that is not the line
    /// feed.
    pub(super) fn string_single_at0(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' {
            return None;
        }
        let hex = self.skip(p + 1, |b| b.is_ascii_hexdigit()).min(p + 7);
        if hex > p + 1 {
            return Some(hex);
        }
        (p + 1 < self.lim()).then(|| self.char_end(p + 1))
    }

    /// `(?:([-\w]+)(\.))?([-\w]+)(\()`: with the module name, and else
    /// without it.
    /// The module name cannot give back a character that would be the dot.
    pub(super) fn constant_functions(&mut self, p: usize) -> Option<usize> {
        let first = self.dash_word_run(p)?;
        if self.at(first) == b'.'
            && let Some(name) = self.dash_word_run(first + 1)
            && self.at(name) == b'('
        {
            self.cap(1, p, first);
            self.cap(2, first, first + 1);
            self.cap(3, first + 1, name);
            self.cap(4, name, name + 1);
            return Some(name + 1);
        }
        if self.at(first) != b'(' {
            return None;
        }
        self.cap(3, p, first);
        self.cap(4, first, first + 1);
        Some(first + 1)
    }

    /// `(\))`.
    pub(super) fn constant_functions_end(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b")")?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `(headings|stylesheet-url|rgba?|…|translate[XY])(\()`.
    pub(super) fn constant_sass_functions(&mut self, p: usize) -> Option<usize> {
        let window = self.text(p, (p + LONGEST_SASS_FUNCTION + 1).min(self.lim()));
        let paren = p + window.iter().position(|&b| b == b'(')?;
        let name = self.text(p, paren);
        if !SASS_FUNCTIONS.contains(&name) {
            return None;
        }
        self.cap(1, p, paren);
        self.cap(2, paren, paren + 1);
        Some(paren + 1)
    }

    /// `\b([-*+/])\b`.
    pub(super) fn constant_mathematical_symbols(&mut self, p: usize) -> Option<usize> {
        if !arithmetic(self.at(p)) || !self.word_before(p) || self.word_at(p + 1).is_none() {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(p + 1)
    }

    /// `[-*+/](?!\s*[-*+/])`.
    pub(super) fn operators(&mut self, p: usize) -> Option<usize> {
        if !arithmetic(self.at(p)) {
            return None;
        }
        let next = self.spaces(p + 1);
        (!arithmetic(self.at(next))).then_some(p + 1)
    }

    /// `[^\t "'),]+`.
    /// It takes the line feed too.
    pub(super) fn parameters_at4(&mut self, p: usize) -> Option<usize> {
        let mut end = self.skip(p, |b| {
            !matches!(b, b'\t' | b' ' | b'"' | b'\'' | b')' | b',')
        });
        if end == self.lim() && self.nl {
            end += 1;
        }
        (end > p).then_some(end)
    }

    /// `([-\w]+)(\()\s*`.
    pub(super) fn scss_functions_at0(&mut self, p: usize) -> Option<usize> {
        let name = self.dash_word_run(p)?;
        if self.at(name) != b'(' {
            return None;
        }
        self.cap(1, p, name);
        self.cap(2, name, name + 1);
        Some(self.spaces(name + 1))
    }

    /// `(?<![-a-z])(?=[-a-z])`.
    pub(super) fn at_rule_media_at0_at4_at0(&mut self, p: usize) -> Option<usize> {
        let name = |b: u8| b == b'-' || b.is_ascii_lowercase();
        (name(self.at(p)) && !self.prev(p).is_some_and(name)).then_some(p)
    }

    /// `$|(?![-a-z])`.
    pub(super) fn at_rule_media_at0_at4_at0_end(&mut self, p: usize) -> Option<usize> {
        let name = |b: u8| b == b'-' || b.is_ascii_lowercase();
        (self.at_eol(p) || !name(self.at(p))).then_some(p)
    }

    /// `(:)\s*(?!(\s*\{))`: when a `{` follows the white space, giving
    /// some of it back still leaves the `{` after the rest.
    pub(super) fn at_rule_media_at0_at4_at1(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b':' {
            return None;
        }
        let end = self.spaces(p + 1);
        if self.at(end) == b'{' {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(end)
    }

    /// `\s*(;|(?=[)}]))`.
    pub(super) fn at_rule_media_at0_at4_at1_end(&mut self, p: usize) -> Option<usize> {
        let end = self.spaces(p);
        match self.at(end) {
            b';' => {
                self.cap(1, end, end + 1);
                Some(end + 1)
            }
            b')' | b'}' => Some(end),
            _ => None,
        }
    }
}
