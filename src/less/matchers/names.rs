//! Names: variables, properties, keyframes, counters and font families.

use super::super::engine::Lexer;
use super::chars::{Class, Escape};
use crate::rules::Chars;

impl Lexer<'_> {
    /// `(@)(-?(?:[-\w[^\x00-\x7F]]|ESC)(?:[-\w[^\x00-\x7F]]|ESC)*)`.
    pub(super) fn variable_name(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' {
            return None;
        }
        let end = self.name(p + 1, Class::DashWord, Class::DashWord, Escape::Hex)?;
        self.cap(1, p, p + 1);
        self.cap(2, p + 1, end);
        Some(end)
    }

    /// `(@{1,2})(-?([_a-z[^\x00-\x7F]]|ESC)(?:[-\w[^\x00-\x7F]]|ESC)*)`: the
    /// second `@` is no name character, so giving it back cannot help.
    pub(super) fn variable_reference(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' {
            return None;
        }
        let sigil = if self.at(p + 1) == b'@' { p + 2 } else { p + 1 };
        let end = self.name(sigil, Class::Lower, Class::DashWord, Escape::Hex)?;
        self.cap(1, p, sigil);
        self.cap(2, sigil, end);
        Some(end)
    }

    /// `--(?:[-\w[^\x00-\x7F]]|ESC)+|-?(?:[A-Z_a-z[^\x00-\x7F]]|ESC)(?:[-\w[^\
    /// x00-\x7F]]|ESC)*`.
    pub(super) fn counter_name(&self, p: usize) -> Option<usize> {
        if self.starts(p, b"--") {
            let end = self.units_end(p + 2, Class::DashWord, Escape::Hex);
            if end > p + 2 {
                return Some(end);
            }
        }
        self.name(p, Class::Alpha, Class::DashWord, Escape::Hex)
    }

    /// `NAME(\s+NAME)*`, where a name is
    /// `-?(?:[A-Z_a-z[^\x00-\x{9F}]]|ESC)(?:[-\w[^\x00-\x{9F}]]|ESC)*`: a font
    /// family of several words.
    pub(super) fn family_name(&self, p: usize) -> Option<usize> {
        let mut end = self.name(p, Class::AlphaWide, Class::DashWordWide, Escape::Hex)?;
        while let Some(next) = self
            .spaces_run(end)
            .and_then(|word| self.name(word, Class::AlphaWide, Class::DashWordWide, Escape::Hex))
        {
            end = next;
        }
        Some(end)
    }

    /// `-?(?:[A-Z_a-z[^\x00-\x7F]]|ESC)(?:[-0-9A-Z_a-z[^\x00-\x7F]]|ESC)*`,
    /// with the escapes of a keyframe name.
    pub(super) fn animation_name(&self, p: usize) -> Option<usize> {
        self.name_with(
            p,
            (Class::Alpha, Escape::KeyframeStart),
            (Class::AlphaRest, Escape::KeyframeRest),
        )
    }

    /// `\s*(-?(?:[_a-z[^\x00-\x7F]]|ESC)(?:[-0-9_a-z[^\x00-\x7F]]|ESC)*)?`:
    /// white space, and the keyframe name in group 1 when one follows.
    pub(super) fn keyframe_name(&mut self, p: usize) -> usize {
        let at = self.spaces(p);
        let Some(end) = self.name_with(
            at,
            (Class::Lower, Escape::KeyframeStart),
            (Class::LowerRest, Escape::KeyframeRest),
        ) else {
            return at;
        };
        self.cap(1, at, end);
        end
    }

    /// `\s*(--)((?:[-\w[^\x00-\x7F]]|ESC)+)`.
    pub(super) fn custom_property(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        self.lit(at, b"--")?;
        let end = self.units_end(at + 2, Class::DashWord, Escape::Hex);
        if end == at + 2 {
            return None;
        }
        self.cap(1, at, at + 2);
        self.cap(2, at + 2, end);
        Some(end)
    }

    /// `\b(var-)(-?(?:[-\w[^\x00-\x{9F}]]|ESC)(?:[A-Z_a-z[^\x00-\x{9F}]]|ESC)*
    /// )(?=\s)`.
    pub(super) fn var_property(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let name = self.lit(p, b"var-")?;
        let end = self.name(name, Class::DashWordWide, Class::AlphaWide, Escape::Hex)?;
        if self.space_at(end) == 0 {
            return None;
        }
        self.cap(1, p, name);
        self.cap(2, name, end);
        Some(end)
    }

    /// `(-[-\w]+?-)(NAME)\b`, where the name has the CSS name characters: the
    /// vendor prefix is the shortest that a name follows.
    pub(super) fn vendor_property(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'-' {
            return None;
        }
        let run = self.dash_word_end(p + 1);
        let mut dash = p + 2;
        while dash < run {
            if self.at(dash) == b'-'
                && let Some(head) = self.unit_at(dash + 1, Class::NameStart, Escape::Any)
            {
                let end = self.units_end(head, Class::NameRest, Escape::Any);
                if let Some(end) = self.boundary_back(dash + 1, end) {
                    self.cap(1, p, dash + 1);
                    return Some(end);
                }
            }
            dash += 1;
            self.spend(1);
        }
        None
    }

    /// `(--|-?NAME)\s*(?=[):])`, where the name has the CSS name characters.
    /// A name that gives characters back leaves a name character next, never
    /// white space, `)` or `:`.
    pub(super) fn media_feature_name(&self, p: usize) -> Option<usize> {
        let ahead = |end: usize| {
            let next = self.spaces(end);
            matches!(self.at(next), b')' | b':').then_some(next)
        };
        self.lit(p, b"--").and_then(ahead).or_else(|| {
            self.name(p, Class::NameStart, Class::NameRest, Escape::Any)
                .and_then(ahead)
        })
    }

    /// `\b(((contain-intrinsic|max|min)-)?(block|inline)?-size)\b`.
    pub(super) fn size_property(&self, p: usize) -> Option<usize> {
        const NAMES: [&[u8]; 12] = [
            b"contain-intrinsic-block-size",
            b"contain-intrinsic-inline-size",
            b"contain-intrinsic--size",
            b"max-block-size",
            b"max-inline-size",
            b"max--size",
            b"min-block-size",
            b"min-inline-size",
            b"min--size",
            b"block-size",
            b"inline-size",
            b"-size",
        ];
        self.alt(p, &NAMES)
    }

    /// `\b(border((-(bottom|top)-(left|right))|((-(start|end)){2}))?
    /// -radius|(border-image(?!-)))\b`.
    pub(super) fn border_radius(&self, p: usize) -> Option<usize> {
        const RADII: [&[u8]; 9] = [
            b"border-bottom-left-radius",
            b"border-bottom-right-radius",
            b"border-top-left-radius",
            b"border-top-right-radius",
            b"border-start-start-radius",
            b"border-start-end-radius",
            b"border-end-start-radius",
            b"border-end-end-radius",
            b"border-radius",
        ];
        self.alt(p, &RADII).or_else(|| {
            let end = self.lit(p, b"border-image")?;
            (self.boundary(p) && self.at(end) != b'-' && self.boundary(end)).then_some(end)
        })
    }

    /// `\bfont(-family)?(?!-)\b`.
    pub(super) fn font_property(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        [b"font-family".as_slice(), b"font"]
            .into_iter()
            .filter_map(|word| self.lit(p, word))
            .find(|&end| self.at(end) != b'-' && self.boundary(end))
    }

    /// `\banimation(?:-name)?(?=(?:\+_?)?:)\b`.
    pub(super) fn animation_property(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        [b"animation-name".as_slice(), b"animation"]
            .into_iter()
            .filter_map(|word| self.lit(p, word))
            .find(|&end| self.merge_colon_ahead(end).is_some())
    }

    /// `\bcontainer(?:-name)?(?=\s*?:)`.
    pub(super) fn container_property(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        [b"container-name".as_slice(), b"container"]
            .into_iter()
            .filter_map(|word| self.lit(p, word))
            .find(|&end| self.spaces_then(end, b":").is_some())
    }

    /// `(?:(NAME)|(\*))?(\|)(?!=)`, where the name is
    /// `-?(?:[-\w[^\x00-\x7F]]|ESC)(?:[A-Z_a-z[^\x00-\x7F]]|ESC)*`: a name
    /// that gives characters back leaves a name character before the `|`.
    pub(super) fn qualified_name(&mut self, p: usize) -> Option<usize> {
        let bar = |lexer: &Self, at: usize| {
            (lexer.at(at) == b'|' && at < lexer.lim() && lexer.at(at + 1) != b'=').then_some(at)
        };
        if let Some(name) = self.name(p, Class::DashWord, Class::Alpha, Escape::Hex)
            && let Some(at) = bar(self, name)
        {
            self.cap(1, p, name);
            self.cap(3, at, at + 1);
            return Some(at + 1);
        }
        if self.at(p) == b'*'
            && let Some(at) = bar(self, p + 1)
        {
            self.cap(2, p, p + 1);
            self.cap(3, at, at + 1);
            return Some(at + 1);
        }
        let at = bar(self, p)?;
        self.cap(3, at, at + 1);
        Some(at + 1)
    }

    /// `(?=[-\w]*?@\{.*}[-\w]*?\s*:[^(;{]*(?=[);}]))`: a property name with an
    /// interpolation, whose value runs to a `;`, `)` or `}`. `.*}` tries the
    /// last `}` first, and any `}` after the `@{` that the colon and the value
    /// after it accept will do, so the last accepted `}` of the line decides;
    /// it is looked for once.
    pub(super) fn interpolated_property_ahead(&self, p: usize) -> Option<usize> {
        let open = self.dash_word_end(p);
        if !self.starts(open, b"@{") {
            return None;
        }
        let close = *self
            .x
            .interpolated
            .get_or_init(move || self.last_property_close());
        close.filter(|&close| close > open + 2).map(|_| p)
    }

    /// The last `}` of the line that `[-\w]*?\s*:[^(;{]*(?=[);}])` follows.
    /// A value that fails holds no `}`, so the values read do not overlap.
    fn last_property_close(&self) -> Option<usize> {
        let mut close = self.lim();
        while close > 0 {
            close -= 1;
            self.spend(1);
            if self.at(close) != b'}' {
                continue;
            }
            let after = close + 1;
            let colon = [after, self.dash_word_end(after)]
                .into_iter()
                .map(|at| self.spaces(at))
                .find(|&at| self.at(at) == b':');
            if let Some(colon) = colon {
                let stop = self.skip(colon + 1, |b| !matches!(b, b'(' | b';' | b'{'));
                if self.at(stop) == b';' && stop < self.lim()
                    || self
                        .text(colon + 1, stop)
                        .iter()
                        .any(|&b| matches!(b, b')' | b'}'))
                {
                    return Some(close);
                }
            }
        }
        None
    }

    /// `(?=[^:\s])`.
    pub(super) fn not_colon_or_space(&self, p: usize) -> Option<usize> {
        (p < self.lim() && self.at(p) != b':' && self.space_at(p) == 0).then_some(p)
    }
}
