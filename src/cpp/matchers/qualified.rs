//! Qualified type names: attributes, specifiers, a scope chain of names and
//! templates, and the name itself.
//! Most of the large expressions of the grammar begin with this block.

use super::super::engine::Lexer;
use super::super::few::Few;
use super::gaps::{Gap, GapForm};
use super::heads::Pointers;
use crate::cpp::keys::{NOT_0C8F_KEYS, NOT_C3E4_KEYS, Words};
use crate::rules::Chars;

/// Where the groups of a qualified type sit in an expression.
#[derive(Clone, Copy)]
pub(crate) struct Layout {
    /// The attribute's group, if the expression captures it.
    pub(crate) attribute: Option<usize>,
    /// The first gap's groups, then the gap after each specifier.
    pub(crate) gap_a: usize,
    pub(crate) gap_b: usize,
    /// The scope chain's group, and in it the group of the last template with
    /// the white space after it, and the template.
    pub(crate) scope: usize,
    pub(crate) scope_template: (usize, usize),
    /// The gap after the scope.
    pub(crate) gap_c: usize,
    /// The template after the name.
    pub(crate) template: usize,
    /// The form of the gaps.
    pub(crate) gap_form: GapForm,
}

/// A template argument list in a scope chain: where it starts, where it ends,
/// and where the white space after it ends.
pub(crate) type Template = (usize, usize, usize);

/// One name of a scope chain: where it ends and the template it had, if any.
pub(crate) type Step = (usize, Option<Template>);

/// A scope with the gap after it: where the name starts, the scope's last
/// template, and the scope's end with the gap.
type ScopeGap = (usize, Option<Template>, Option<(usize, Gap)>);

/// One way a qualified type matches: where it ends and its groups, at most
/// the attribute, the scope, the scope's template twice and the template,
/// and three gaps.
#[derive(Clone, Copy)]
pub(crate) struct TypeMatch {
    pub(crate) end: usize,
    pub(crate) caps: Few<(usize, usize, usize), 5>,
    pub(crate) gaps: Few<(usize, usize, Gap), 3>,
}

/// What a qualified type has matched before its name: the attribute, the
/// first gap, the specifiers and the scope.
#[derive(Clone, Copy)]
struct TypeStart {
    attribute: Option<usize>,
    start: usize,
    gap_a: Gap,
    loop_end: usize,
    last: Option<(usize, Gap)>,
}

/// The specifiers before a type.
const SPECIFIERS: &[&[u8]] = &[
    b"unsigned",
    b"signed",
    b"short",
    b"long",
    b"struct",
    b"class",
    b"union",
    b"enum",
];

impl Lexer<'_> {
    /// `(?!\b(?:LIST)\b)` at `p`: no word of `list` stands there alone.
    ///
    /// A word of the list that ends at a boundary is the whole run of word
    /// characters from `p`.
    /// The lists are sorted.
    pub(super) fn not_listed(&self, p: usize, list: &Words) -> bool {
        !self.boundary(p) || self.not_listed_word(p, list)
    }

    /// `(?!(?:LIST)\b)` at `p`, without the leading boundary: the run of
    /// word characters from `p` is not a word of `list`.
    fn not_listed_word(&self, p: usize, list: &Words) -> bool {
        let end = self.word_end(p);
        !list.contains(self.text(p, end))
    }

    /// One name of a scope chain:
    /// `(?!\b(?:LIST)\b)(?<!\w)ID(?!\w)\s*+(TEMPLATE\s*+)?::`: where it ends,
    /// the template and where it ends, if any.
    pub(super) fn scope_step_pub(&self, p: usize) -> Option<Step> {
        self.scope_step(p)
    }

    fn scope_step(&self, p: usize) -> Option<Step> {
        if !self.not_listed(p, &NOT_C3E4_KEYS) || self.word_before(p) {
            return None;
        }
        let name = self.ident_bounded(p)?;
        let after = self.skip_space(name);
        if let Some((end, _)) = self.template_args(after) {
            let next = self.skip_space(end);
            if self.starts(next, b"::") {
                return Some((next + 2, Some((after, end, next))));
            }
        }
        self.starts(after, b"::").then_some((after + 2, None))
    }

    /// `(?:::)?(?:STEP)*+`: the scope chain at `p`, as its end and the last
    /// template in it.
    /// The loop is possessive, and the leading `::` is optional.
    pub(super) fn scope_chain(&self, p: usize) -> Few<Step, 2> {
        let mut out = Few::new();
        let mut starts: Few<usize, 2> = Few::new();
        if self.starts(p, b"::") {
            starts.push(p + 2);
        }
        starts.push(p);
        for start in starts {
            let mut pos = start;
            let mut template = None;
            while let Some((end, tpl)) = self.scope_step(pos) {
                self.spend(1);
                if tpl.is_some() {
                    template = tpl;
                }
                pos = end;
            }
            out.push((pos, template));
        }
        out
    }

    /// `(?:(?:unsigned|…)GAP)*` from `pos`, every way, the longest first.
    /// It finds the first way for which `rest` matches what follows, given the
    /// end and the last specifier's end and gap.
    /// It returns what `rest` found.
    /// `last` is the iteration before `pos`.
    fn find_specifier_chain<R>(
        &self,
        pos: usize,
        last: Option<(usize, Gap)>,
        rest: &mut impl FnMut(&Self, usize, Option<(usize, Gap)>) -> Option<R>,
    ) -> Option<R> {
        let Some(_depth) = self.deeper() else {
            return rest(self, pos, last);
        };
        for word in SPECIFIERS {
            if let Some(end) = self.lit(pos, word) {
                for gap in self.gaps(end) {
                    if let Some(found) = self.find_specifier_chain(gap.end, Some((end, gap)), rest)
                    {
                        return Some(found);
                    }
                }
            }
        }
        rest(self, pos, last)
    }

    /// `ATTRIBUTE?GAP(?:(?:unsigned|…)GAP)*(?:(SCOPE)GAP)?(?!(?:LIST)\b)ID\
    /// b(TEMPLATE)?(?![.:<\w])`: the first way a qualified type matches at
    /// `p`, in the order the expression tries them, for which `rest` matches
    /// what follows, with what `rest` found.
    ///
    /// The scope and its gap are tried first, then left out.
    pub(super) fn find_type<R>(
        &self,
        p: usize,
        layout: &Layout,
        mut rest: impl FnMut(&Self, &TypeMatch) -> Option<R>,
    ) -> Option<R> {
        for attribute in self.attribute_options(p) {
            let start = attribute.unwrap_or(p);
            for gap_a in self.gaps(start) {
                let found =
                    self.find_specifier_chain(gap_a.end, None, &mut |lexer, loop_end, last| {
                        let before = TypeStart {
                            attribute,
                            start,
                            gap_a,
                            loop_end,
                            last,
                        };
                        for (end, template) in lexer.scope_chain(loop_end) {
                            for gap in lexer.gaps(end) {
                                let scope = (gap.end, template, Some((end, gap)));
                                if let Some(found) =
                                    lexer.find_type_name(p, layout, &before, scope, &mut rest)
                                {
                                    return Some(found);
                                }
                            }
                        }
                        lexer.find_type_name(p, layout, &before, (loop_end, None, None), &mut rest)
                    });
                if found.is_some() {
                    return found;
                }
            }
        }
        None
    }

    /// The first way the name and template of a qualified type match after
    /// `before` and the scope `scope`, for which `rest` matches what follows.
    fn find_type_name<R>(
        &self,
        p: usize,
        layout: &Layout,
        before: &TypeStart,
        (name_at, template, scope): ScopeGap,
        rest: &mut impl FnMut(&Self, &TypeMatch) -> Option<R>,
    ) -> Option<R> {
        if !self.not_listed_word(name_at, &NOT_0C8F_KEYS) {
            return None;
        }
        for name in self.ident_ends(name_at) {
            if !self.boundary(name) {
                continue;
            }
            let mut finals: Few<(usize, Option<usize>), 2> = Few::new();
            if let Some((end, _)) = self.template_args(name) {
                finals.push((end, Some(end)));
            }
            finals.push((name, None));
            for (end, tpl) in finals {
                let blocked = self
                    .ch(end)
                    .is_some_and(|(c, _)| matches!(c, '.' | ':' | '<') || super::chars::is_word(c));
                if blocked {
                    continue;
                }
                let mut caps = Few::new();
                let mut gaps = Few::new();
                if let (Some(group), Some(a)) = (layout.attribute, before.attribute) {
                    caps.push((group, p, a));
                }
                gaps.push((layout.gap_a, before.start, before.gap_a));
                if let Some((at, gap)) = before.last {
                    gaps.push((layout.gap_b, at, gap));
                }
                if let Some((scope_end, gap)) = scope {
                    caps.push((layout.scope, before.loop_end, scope_end));
                    gaps.push((layout.gap_c, scope_end, gap));
                    if let Some((a, tpl_end, spaced)) = template {
                        caps.push((layout.scope_template.0, a, spaced));
                        caps.push((layout.scope_template.1, a, tpl_end));
                    }
                }
                if tpl.is_some() {
                    caps.push((layout.template, name, end));
                }
                if let Some(found) = rest(self, &TypeMatch { end, caps, gaps }) {
                    return Some(found);
                }
            }
        }
        None
    }

    /// The first way a qualified type matches at `p`.
    pub(super) fn first_type(&self, p: usize, layout: &Layout) -> Option<TypeMatch> {
        self.find_type(p, layout, |_, found| Some(*found))
    }

    /// Records a qualified type match.
    pub(super) fn put_type(&mut self, layout: &Layout, found: &TypeMatch) {
        for &(group, a, b) in found.caps.iter() {
            self.cap(group, a, b);
        }
        for (group, from, gap) in found.gaps.iter() {
            self.put_gap(*group, layout.gap_form, *from, gap);
        }
    }
}

impl Lexer<'_> {
    /// `\s*+(ATTRIBUTE)?GAP(…)`: the whole expression of a qualified type.
    pub(super) fn qualified_type(&mut self, p: usize) -> Option<usize> {
        let layout = Layout {
            attribute: Some(1),
            gap_a: 2,
            gap_b: 4,
            scope: 6,
            scope_template: (7, 8),
            gap_c: 9,
            template: 11,
            gap_form: GapForm::Plain,
        };
        let start = self.skip_space(p);
        let found = self.first_type(start, &layout)?;
        self.put_type(&layout, &found);
        Some(found.end)
    }
}

impl Lexer<'_> {
    /// `(?!class|struct|…)\b` at `p`: no such word starts there, and a word
    /// boundary does.
    fn not_keyword_start(&self, p: usize) -> bool {
        const WORDS: &[&[u8]] = &[
            b"class",
            b"struct",
            b"union",
            b"enum",
            b"explicit",
            b"new",
            b"delete",
            b"operator",
            b"template",
            b"throw",
            b"decltype",
            b"typename",
            b"override",
            b"final",
        ];
        !WORDS.iter().any(|word| self.starts(p, word)) && self.boundary(p)
    }

    /// A gap, `typename`, a gap and a qualified type: a dependent type name.
    pub(super) fn typename_type(&mut self, p: usize) -> Option<usize> {
        let layout = Layout {
            attribute: Some(7),
            gap_a: 8,
            gap_b: 10,
            scope: 12,
            scope_template: (13, 14),
            gap_c: 15,
            template: 17,
            gap_form: GapForm::Plain,
        };
        for first in self.gaps(p) {
            let Some(word) = self.word_lit(first.end, b"typename") else {
                continue;
            };
            for second in self.gaps(word) {
                let start = self.skip_space(second.end);
                if let Some(found) = self.first_type(start, &layout) {
                    self.cap(1, p, word);
                    self.put_gap(2, GapForm::Plain, p, &first);
                    self.put_gap(4, GapForm::Plain, word, &second);
                    self.cap(6, second.end, found.end);
                    self.put_type(&layout, &found);
                    return Some(found.end);
                }
            }
        }
        None
    }

    /// After `protected`, `virtual`, `private`, `public`, a `,` or a `:`, white
    /// space and a qualified type that is none of those words: a base class.
    pub(super) fn base_class(&mut self, p: usize) -> Option<usize> {
        let behind = self.prev(p).is_some_and(|b| b == b',' || b == b':')
            || [&b"protected"[..], b"virtual", b"private", b"public"]
                .iter()
                .any(|w| self.ends_with_at(p, w));
        if !behind {
            return None;
        }
        let spaces = self.skip_space(p);
        let excluded = self.starts(p, b"protected")
            || self.starts(p, b"private")
            || self.starts(p, b"public")
            || self.starts(p, b"virtual");
        if spaces == p && excluded {
            return None;
        }
        let layout = Layout {
            attribute: Some(2),
            gap_a: 3,
            gap_b: 5,
            scope: 7,
            scope_template: (8, 9),
            gap_c: 10,
            template: 12,
            gap_form: GapForm::Plain,
        };
        let found = self.first_type(spaces, &layout)?;
        self.cap(1, spaces, found.end);
        self.put_type(&layout, &found);
        Some(found.end)
    }

    /// `(\s*+(ATTR)?GAP…)GAP(\{)`: a type followed by a brace.
    pub(super) fn type_brace(&mut self, p: usize) -> Option<usize> {
        let layout = Layout {
            attribute: Some(2),
            gap_a: 3,
            gap_b: 7,
            scope: 11,
            scope_template: (12, 13),
            gap_c: 14,
            template: 18,
            gap_form: GapForm::Parts,
        };
        let start = self.skip_space(p);
        let (found, gap) = self.find_type(start, &layout, |lexer, found| {
            let gap = lexer
                .gaps(found.end)
                .find(|gap| lexer.at(gap.end) == b'{' && gap.end < lexer.lim())?;
            Some((*found, gap))
        })?;
        self.cap(1, p, found.end);
        self.put_type(&layout, &found);
        self.put_gap(19, GapForm::Parts, found.end, &gap);
        self.cap(23, gap.end, gap.end + 1);
        Some(gap.end + 1)
    }

    /// The layout of the types of the constructor-call expressions.
    fn constructor_layout() -> Layout {
        Layout {
            attribute: Some(2),
            gap_a: 3,
            gap_b: 5,
            scope: 7,
            scope_template: (8, 9),
            gap_c: 10,
            template: 12,
            gap_form: GapForm::Plain,
        }
    }

    /// `(?!class|…)\b(\s*+(TYPE)(?=GAP(?=(?<!\w)ID(?!\w)(?=GAP(?=[({])))))`.
    pub(super) fn simple_constructor_call(&mut self, p: usize) -> Option<usize> {
        if !self.not_keyword_start(p) || !self.may_start_declaration(p, Pointers::Ignore) {
            return None;
        }
        let layout = Self::constructor_layout();
        let start = self.skip_space(p);
        let (found, first, name, second) = self.find_type(start, &layout, |lexer, found| {
            lexer.gaps(found.end).find_map(|first| {
                if lexer.word_before(first.end) {
                    return None;
                }
                let name = lexer.ident_bounded(first.end)?;
                let second = lexer.gaps(name).find(|second| {
                    matches!(lexer.at(second.end), b'(' | b'{') && second.end < lexer.lim()
                })?;
                Some((*found, first, name, second))
            })
        })?;
        self.cap(1, p, found.end);
        self.put_type(&layout, &found);
        self.put_gap(13, GapForm::Plain, found.end, &first);
        self.put_gap(15, GapForm::Plain, name, &second);
        Some(found.end)
    }

    /// `(?!class|…)\b(\s*+(TYPE)GAP(?:\b(?:STYLES)\b|(ID))GAP(?=\{)`.
    pub(super) fn constructor_bracket_call(&mut self, p: usize) -> Option<usize> {
        if !self.not_keyword_start(p) || !self.may_start_declaration(p, Pointers::Ignore) {
            return None;
        }
        let layout = Self::constructor_layout();
        let start = self.skip_space(p);
        let (found, first, group, end, second) =
            self.find_type(start, &layout, |lexer, found| {
                lexer.gaps(found.end).find_map(|first| {
                    let at = first.end;
                    let styles = lexer
                        .style_ends(at)
                        .into_iter()
                        .map(|(style, end)| (15 + style, end));
                    let idents = (!lexer.word_before(at))
                        .then(|| lexer.ident_ends(at))
                        .into_iter()
                        .flatten()
                        .filter(|&end| lexer.word_at(end).is_none())
                        .map(|end| (19, end));
                    styles.chain(idents).find_map(|(group, end)| {
                        let second = lexer.gaps(end).find(|second| {
                            lexer.at(second.end) == b'{' && second.end < lexer.lim()
                        })?;
                        Some((*found, first, group, end, second))
                    })
                })
            })?;
        self.cap(1, p, found.end);
        self.put_type(&layout, &found);
        self.put_gap(13, GapForm::Plain, found.end, &first);
        self.cap(group, first.end, end);
        self.put_gap(20, GapForm::Plain, end, &second);
        Some(second.end)
    }
}
