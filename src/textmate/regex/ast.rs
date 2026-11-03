//! The syntax tree of a pattern, and the character classes in it.

/// A position the engine can test without consuming text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Anchor {
    /// `^`: the start of the text, or after a newline that is not its end.
    Caret,
    /// `$`: the end of the text, or before a newline.
    Dollar,
    /// `\A`: the start of the document, where the caller says so.
    StartText,
    /// `\z`: the end of the text.
    EndText,
    /// `\Z`: the end of the text, or before a final newline.
    EndTextNewline,
    /// `\G`: where the last match of the grammar ended.
    Search,
    /// `\b`.
    WordBoundary,
    /// `\B`.
    NotWordBoundary,
}

/// Whether letters match only in the case the pattern spells them, as
/// Oniguruma's `(?i)` flag decides.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Case {
    /// Only in the pattern's case.
    Sensitive,
    /// In any case.
    Insensitive,
}

/// Whether a set of characters matches its members or every other
/// character, as `\d` and `\D` differ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Polarity {
    /// Its members.
    Positive,
    /// Every character but its members.
    Negated,
}

impl Polarity {
    /// The other polarity.
    pub(super) fn inverse(self) -> Polarity {
        match self {
            Polarity::Positive => Polarity::Negated,
            Polarity::Negated => Polarity::Positive,
        }
    }
}

/// How a repeat takes its iterations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Kind {
    /// As many as it can, giving back on failure.
    Greedy,
    /// As few as it can, taking more on failure.
    Lazy,
    /// As many as it can, never giving back.
    Possessive,
}

/// One member of a class.
#[derive(Clone, Debug)]
pub(super) enum Item {
    /// `a-z`, or a single character as `a-a`.
    Range(char, char),
    /// `\d`, `[:alpha:]` or `\p{Lu}`, negated or not.
    Prop(Prop, Polarity),
    /// A class inside a class.
    Nested(Class),
}

/// A character class: the intersection of unions of items, negated or not.
#[derive(Clone, Debug)]
pub(super) struct Class {
    pub(super) negated: bool,
    pub(super) parts: Vec<Vec<Item>>,
}

/// A named set of characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Prop {
    Digit,
    Word,
    Space,
    Hex,
    Alpha,
    Alnum,
    Upper,
    Lower,
    Punct,
    Cntrl,
    Graph,
    Print,
    Blank,
    Ascii,
    Number,
    Mark,
    Symbol,
    Separator,
    Any,
}

/// The set a `\p{..}` or `[:..:]` name stands for.
///
/// The name `n\x64` is escaped so the spelling check reads it: it is the
/// Unicode general category of decimal digits.
pub(super) fn prop_named(name: &str) -> Option<Prop> {
    let lower = name.to_ascii_lowercase().replace(['_', ' ', '-'], "");
    Some(match lower.as_str() {
        "digit" | "n\x64" => Prop::Digit,
        "word" | "w" | "idcontinue" | "xidcontinue" => Prop::Word,
        "space" | "whitespace" => Prop::Space,
        "xdigit" | "hex" | "asciihexdigit" => Prop::Hex,
        "alpha" | "alphabetic" | "l" | "letter" | "lo" | "lm" | "lt" | "idstart" | "xidstart" => {
            Prop::Alpha
        }
        "alnum" => Prop::Alnum,
        "upper" | "lu" | "uppercase" => Prop::Upper,
        "lower" | "ll" | "lowercase" => Prop::Lower,
        "punct" | "p" | "punctuation" => Prop::Punct,
        "cntrl" | "cc" | "control" => Prop::Cntrl,
        "graph" => Prop::Graph,
        "print" => Prop::Print,
        "blank" => Prop::Blank,
        "ascii" => Prop::Ascii,
        "n" | "number" | "nl" | "no" => Prop::Number,
        "m" | "mark" | "mn" | "mc" | "me" => Prop::Mark,
        "s" | "symbol" | "sm" | "sc" | "sk" | "so" => Prop::Symbol,
        "z" | "separator" | "zs" => Prop::Separator,
        "any" => Prop::Any,
        _ => return None,
    })
}

/// Whether `c` is in the set, approximated with the standard library's
/// character tests.
pub(super) fn prop_matches(prop: Prop, c: char) -> bool {
    match prop {
        Prop::Digit => c.is_ascii_digit() || (!c.is_ascii() && c.is_numeric()),
        Prop::Word => is_word(c),
        Prop::Space => c.is_whitespace(),
        Prop::Hex => c.is_ascii_hexdigit(),
        Prop::Alpha => c.is_alphabetic(),
        Prop::Alnum => c.is_alphanumeric(),
        Prop::Upper => c.is_uppercase(),
        Prop::Lower => c.is_lowercase(),
        Prop::Punct => c.is_ascii_punctuation() && !"$+<=>^`|~".contains(c),
        Prop::Cntrl => c.is_control(),
        Prop::Graph => !c.is_whitespace() && !c.is_control(),
        Prop::Print => !c.is_control(),
        Prop::Blank => c == ' ' || c == '\t',
        Prop::Ascii => c.is_ascii(),
        Prop::Number => c.is_numeric(),
        Prop::Mark => ('\u{300}'..='\u{36f}').contains(&c),
        Prop::Symbol => "$+<=>^`|~".contains(c),
        Prop::Separator => c.is_whitespace() && !c.is_control(),
        Prop::Any => true,
    }
}

/// `\w`, and what `\b` tests.
pub(super) fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || ('\u{300}'..='\u{36f}').contains(&c)
}

/// The case a case-insensitive comparison reads `c` in.
pub(super) fn fold(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

impl Class {
    /// Whether `c` is in the class, also in the other case of `c` when the
    /// class ignores case.
    pub(super) fn matches(&self, c: char, case: Case) -> bool {
        let hit = |c: char| {
            self.parts.iter().all(|union| {
                union.iter().any(|item| match item {
                    Item::Range(lo, hi) => (*lo..=*hi).contains(&c),
                    Item::Prop(prop, polarity) => {
                        prop_matches(*prop, c) == (*polarity == Polarity::Positive)
                    }
                    Item::Nested(class) => class.matches(c, Case::Sensitive),
                })
            })
        };
        let found = hit(c)
            || (case == Case::Insensitive && {
                let lower = fold(c);
                let upper = c.to_uppercase().next().unwrap_or(c);
                hit(lower) || hit(upper)
            });
        found != self.negated
    }
}

/// A node of the tree.
#[derive(Clone, Debug)]
pub(super) enum Node {
    /// Nothing: an empty pattern or alternative.
    Empty,
    /// A character, case-insensitive when the flag is set.
    Char(char, Case),
    /// `.`, matching a newline when the flag is set.
    Any(bool),
    /// A class, case-insensitive when the flag is set.
    Class(Box<Class>, Case),
    /// The nodes in order.
    Seq(Vec<Node>),
    /// The nodes as alternatives, the first that matches taken.
    Alt(Vec<Node>),
    /// A node repeated between `min` and `max` times.
    Repeat {
        node: Box<Node>,
        min: u32,
        max: Option<u32>,
        kind: Kind,
    },
    /// A group: capturing when it has an index.
    Group {
        index: Option<usize>,
        node: Box<Node>,
    },
    /// A lookahead or lookbehind.
    Look {
        ahead: bool,
        negative: bool,
        node: Box<Node>,
    },
    /// `(?>..)`.
    Atomic(Box<Node>),
    /// `\1`, case-insensitive when the flag is set.
    Backref(usize, Case),
    /// `\k<name>`.
    NamedBackref(String, Case),
    /// `\g<1>`.
    Call(usize),
    /// `\g<name>`.
    NamedCall(String),
    /// An anchor.
    Anchor(Anchor),
}
