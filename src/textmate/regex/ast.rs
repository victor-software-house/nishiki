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

/// A named set of characters: ranges of code points, ascending.
#[derive(Clone, Copy, Debug)]
pub(super) struct Prop(&'static [(u32, u32)]);

impl Prop {
    /// Every character.
    pub(super) const ANY: Prop = Prop(&[(0, 0x10_ffff)]);

    /// Whether `c` is in the set.
    pub(super) fn matches(self, c: char) -> bool {
        let code = u32::from(c);
        let at = self.0.partition_point(|&(_, last)| last < code);
        self.0.get(at).is_some_and(|&(first, _)| first <= code)
    }
}

/// The set a `\p{..}` or `[:..:]` name stands for, as Oniguruma measures it:
/// names ignore case, `_`, `-` and spaces.
pub(super) fn prop_named(name: &str) -> Option<Prop> {
    let lower = name.to_ascii_lowercase().replace(['_', ' ', '-'], "");
    let key = match lower.as_str() {
        "any" => return Some(Prop::ANY),
        "letter" => "l",
        "uppercaseletter" => "lu",
        "lowercaseletter" => "ll",
        "titlecaseletter" => "lt",
        "modifierletter" => "lm",
        "otherletter" => "lo",
        "mark" => "m",
        "nonspacingmark" => "mn",
        "spacingmark" => "mc",
        "enclosingmark" => "me",
        "number" => "n",
        "decimalnumber" => "nd",
        "letternumber" => "nl",
        "othernumber" => "no",
        "punctuation" => "p",
        "connectorpunctuation" => "pc",
        "dashpunctuation" => "pd",
        "openpunctuation" => "ps",
        "closepunctuation" => "pe",
        "initialpunctuation" => "pi",
        "finalpunctuation" => "pf",
        "otherpunctuation" => "po",
        "symbol" => "s",
        "mathsymbol" => "sm",
        "currencysymbol" => "sc",
        "modifiersymbol" => "sk",
        "othersymbol" => "so",
        "separator" => "z",
        "spaceseparator" => "zs",
        "lineseparator" => "zl",
        "paragraphseparator" => "zp",
        "other" => "c",
        "control" => "cc",
        "format" => "cf",
        "privateuse" => "co",
        "unassigned" => "cn",
        "surrogate" => "cs",
        "xids" => "xidstart",
        "xidc" => "xidcontinue",
        "ids" => "idstart",
        "idc" => "idcontinue",
        "wspace" => "whitespace",
        "hex" | "hexdigit" | "asciihexdigit" => "xdigit",
        other => other,
    };
    super::unicode::lookup(key).map(Prop)
}

/// A set `prop_named` is known to have.
pub(super) fn builtin(name: &str) -> Prop {
    prop_named(name).unwrap_or(Prop::ANY)
}

/// `\w`, and what `\b` tests.
pub(super) fn is_word(c: char) -> bool {
    if c.is_ascii() {
        return c.is_ascii_alphanumeric() || c == '_';
    }
    builtin("word").matches(c)
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
                        prop.matches(c) == (*polarity == Polarity::Positive)
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
