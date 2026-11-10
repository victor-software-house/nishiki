//! Scope chains in declarations: namespaces, aliases, constructors and
//! function calls.

use super::super::engine::Lexer;
use super::super::few::Few;
use super::gaps::GapForm;
use crate::cpp::keys::{BEHIND_KEYS, NOT_C3E4_KEYS};
use crate::rules::Chars;

/// A way a scope chain matches: where it ends (after its white space), and
/// its groups.
#[derive(Clone, Copy)]
pub(crate) struct Chain {
    pub(crate) end: usize,
    /// The leading `::`.
    pub(crate) colons: Option<usize>,
    /// The last template with its trailing white space, and the template.
    pub(crate) template: Option<(usize, usize, usize)>,
}

/// The groups of the last comment in a template argument list.
type CommentParts = [Option<(usize, usize)>; 3];

/// A template argument list of a call: where it ends, where its white space
/// ends, and the groups of its last comment.
type CallTemplate = (usize, usize, CommentParts);

/// A template argument list with the white space after it.
#[derive(Clone, Copy)]
struct Tpl {
    start: usize,
    end: usize,
    spaced: usize,
    parts: [Option<(usize, usize)>; 3],
}

/// One way `(CHAIN)(NAME)\s*+(TEMPLATE\s*+)?(::)` matches.
struct ScopeResolution {
    chain: Chain,
    first: usize,
    name_at: usize,
    name: usize,
    template: Option<Tpl>,
    colons: usize,
}

/// The names of the functions a call expression takes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Callee {
    /// `[A-Z][0-9A-Z_]*`.
    UpperCase,
    /// Any identifier.
    Any,
}

impl Lexer<'_> {
    /// `(::)?(?:STEP)*\s*+` at `p`, every way: the leading `::` taken or not,
    /// and the loop given back one step at a time.
    /// It finds the first way for which `rest` matches what follows.
    /// It returns what `rest` found.
    /// The loop is followed forward, and the last end that `rest` accepts is
    /// kept.
    /// That is the end the longest-first search stops at, with no list of
    /// ends.
    pub(super) fn find_scope_chain<R>(
        &self,
        p: usize,
        mut rest: impl FnMut(&Self, &Chain) -> Option<R>,
    ) -> Option<R> {
        let mut starts: Few<(usize, Option<usize>), 2> = Few::new();
        if self.starts(p, b"::") {
            starts.push((p + 2, Some(p)));
        }
        starts.push((p, None));
        for (start, colons) in starts {
            let chain_at = |at: usize, template| Chain {
                end: self.skip_space(at),
                colons,
                template,
            };
            let mut found = rest(self, &chain_at(start, None));
            let mut pos = start;
            let mut template = None;
            while let Some((end, tpl)) = self.scope_step_pub(pos) {
                self.spend(1);
                if tpl.is_some() {
                    template = tpl;
                }
                pos = end;
                if let Some(r) = rest(self, &chain_at(pos, template)) {
                    found = Some(r);
                }
            }
            if found.is_some() {
                return found;
            }
        }
        None
    }

    /// Records a chain whose groups are `colons`, `template_outer` and
    /// `template`.
    fn put_chain(&mut self, chain: &Chain, colons: usize, template: (usize, usize)) {
        if let Some(at) = chain.colons {
            self.capture(colons, at, at + 2);
        }
        if let Some((a, tpl_end, spaced)) = chain.template {
            self.capture(template.0, a, spaced);
            self.capture(template.1, a, tpl_end);
        }
    }

    /// `(?<!\w)(using)\s+(namespace)\s+(CHAIN)?((?<!\w)ID(?!\w))(?=[\n;])`.
    pub(super) fn using_namespace(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let using = self.lit(p, b"using")?;
        let a = self.skip_space(using);
        if a == using {
            return None;
        }
        let namespace = self.lit(a, b"namespace")?;
        let b = self.skip_space(namespace);
        if b == namespace {
            return None;
        }
        let ahead = |lexer: &Self, at: usize| {
            (lexer.at(at) == b'\n' && lexer.nl && at == lexer.lim())
                || (lexer.at(at) == b';' && at < lexer.lim())
        };
        let name_after = |lexer: &Self, at: usize| {
            if lexer.word_before(at) {
                return None;
            }
            lexer
                .ident_ends(at)
                .find(|&name| lexer.word_at(name).is_none() && ahead(lexer, name))
        };
        let (chain, name) = self
            .find_scope_chain(b, |lexer, chain| {
                Some((Some(*chain), name_after(lexer, chain.end)?))
            })
            .or_else(|| Some((None, name_after(self, b)?)))?;
        self.capture(1, p, using);
        self.capture(2, a, namespace);
        let at = chain.map_or(b, |c| c.end);
        if let Some(chain) = &chain {
            self.capture(3, b, chain.end);
            self.put_chain(chain, 4, (5, 6));
        }
        self.capture(7, at, name);
        Some(name)
    }

    /// `(CHAIN)\s*((?<!\w)ID(?!\w))\s*(?:(::)\s*(inline))?`.
    ///
    /// Without the `:: inline` tail the match ends after the spaces.
    pub(super) fn namespace_name(&mut self, p: usize) -> Option<usize> {
        let (chain, at, name) = self.find_scope_chain(p, |lexer, chain| {
            let at = lexer.skip_space(chain.end);
            if lexer.word_before(at) {
                return None;
            }
            let name = lexer
                .ident_ends(at)
                .find(|&name| lexer.word_at(name).is_none())?;
            Some((*chain, at, name))
        })?;
        let after = self.skip_space(name);
        let (end, tail) = if self.starts(after, b"::") {
            let word = self.skip_space(after + 2);
            match self.lit(word, b"inline") {
                Some(e) => (e, Some((after, word, e))),
                None => (self.skip_space(name), None),
            }
        } else {
            (after, None)
        };
        self.capture(1, p, chain.end);
        self.put_chain(&chain, 2, (3, 4));
        self.capture(5, at, name);
        if let Some((colons, word, e)) = tail {
            self.capture(6, colons, colons + 2);
            self.capture(7, word, e);
        }
        Some(end)
    }

    /// `(?<!\w)(namespace)\s+((?<!\w)ID(?!\w))\s*(=)\s*((CHAIN)\s*((?<!\w)ID(?!
    /// \w))\s*(?:(;)|\n))`.
    ///
    /// The white space before the end gives back the line feed, which then
    /// ends the match.
    pub(super) fn namespace_alias(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let word = self.lit(p, b"namespace")?;
        let a = self.skip_space(word);
        if a == word || self.word_before(a) {
            return None;
        }
        let alias = self.ident_bounded(a)?;
        let eq = self.skip_space(alias);
        if self.at(eq) != b'=' || eq >= self.lim() {
            return None;
        }
        let target = self.skip_space(eq + 1);
        let (chain, at, name, end, semicolon) = self.find_scope_chain(target, |lexer, chain| {
            let at = lexer.skip_space(chain.end);
            if lexer.word_before(at) {
                return None;
            }
            lexer.ident_ends(at).find_map(|name| {
                if lexer.word_at(name).is_some() {
                    return None;
                }
                let tail = lexer.skip_space(name);
                let (end, semicolon) = if lexer.at(tail) == b';' && tail < lexer.lim() {
                    (tail + 1, Some(tail))
                } else if lexer.nl && tail > lexer.lim() {
                    (lexer.lim() + 1, None)
                } else if lexer.nl && tail == lexer.lim() {
                    (tail + 1, None)
                } else {
                    return None;
                };
                Some((*chain, at, name, end, semicolon))
            })
        })?;
        self.capture(1, p, word);
        self.capture(2, a, alias);
        self.capture(3, eq, eq + 1);
        self.capture(4, target, end);
        self.capture(5, target, chain.end);
        self.put_chain(&chain, 6, (7, 8));
        self.capture(9, at, name);
        if let Some(s) = semicolon {
            self.capture(10, s, s + 1);
        }
        Some(end)
    }

    /// `\s*+((?:__…l)?)GAP(CHAIN)((?>(?<!\w)ID(?!\w))GAP::GAP\10GAP(?=\())`
    /// for a constructor defined outside its class.
    pub(super) fn constructor_root(&mut self, p: usize) -> Option<usize> {
        let start = self.skip_space(p);
        for convention in self.convention_ends(start) {
            for first in self.gaps(convention) {
                for (chain_end, template) in self.scope_chain(first.end) {
                    let Some(name) = self.word_ident_atomic(chain_end) else {
                        continue;
                    };
                    for second in self.gaps(name) {
                        if !self.starts(second.end, b"::") {
                            continue;
                        }
                        for third in self.gaps(second.end + 2) {
                            let line = self.s;
                            let Some(copy) = self.lit(third.end, &line[chain_end..name]) else {
                                continue;
                            };
                            for fourth in self.gaps(copy) {
                                if self.at(fourth.end) != b'(' || fourth.end >= self.lim() {
                                    continue;
                                }
                                self.capture(1, start, convention);
                                self.put_gap(2, GapForm::Parts, convention, &first);
                                self.capture(6, first.end, chain_end);
                                if let Some((a, tpl_end, spaced)) = template {
                                    self.capture(7, a, spaced);
                                    self.capture(8, a, tpl_end);
                                }
                                self.capture(9, chain_end, fourth.end);
                                self.capture(10, chain_end, name);
                                self.put_gap(11, GapForm::Parts, name, &second);
                                self.put_gap(15, GapForm::Parts, second.end + 2, &third);
                                self.put_gap(19, GapForm::Parts, copy, &fourth);
                                return Some(fourth.end);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// `GAP((?:__…l)?)GAP(CHAIN)((?>(?<!\w)ID(?!\w))GAP::GAP~\14GAP(?=\())`
    /// for a destructor defined outside its class.
    pub(super) fn destructor_root(&mut self, p: usize) -> Option<usize> {
        for first in self.gaps(p) {
            for convention in self.convention_ends(first.end) {
                for second in self.gaps(convention) {
                    for (chain_end, template) in self.scope_chain(second.end) {
                        let Some(name) = self.word_ident_atomic(chain_end) else {
                            continue;
                        };
                        for third in self.gaps(name) {
                            if !self.starts(third.end, b"::") {
                                continue;
                            }
                            for fourth in self.gaps(third.end + 2) {
                                if self.at(fourth.end) != b'~' || fourth.end >= self.lim() {
                                    continue;
                                }
                                let line = self.s;
                                let Some(copy) = self.lit(fourth.end + 1, &line[chain_end..name])
                                else {
                                    continue;
                                };
                                for fifth in self.gaps(copy) {
                                    if self.at(fifth.end) != b'(' || fifth.end >= self.lim() {
                                        continue;
                                    }
                                    self.put_gap(1, GapForm::Parts, p, &first);
                                    self.capture(5, first.end, convention);
                                    self.put_gap(6, GapForm::Parts, convention, &second);
                                    self.capture(10, second.end, chain_end);
                                    if let Some((a, tpl_end, spaced)) = template {
                                        self.capture(11, a, spaced);
                                        self.capture(12, a, tpl_end);
                                    }
                                    self.capture(13, chain_end, fifth.end);
                                    self.capture(14, chain_end, name);
                                    self.put_gap(15, GapForm::Parts, name, &third);
                                    self.put_gap(19, GapForm::Parts, third.end + 2, &fourth);
                                    self.put_gap(23, GapForm::Parts, copy, &fifth);
                                    return Some(fifth.end);
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// `(?<!\w)enum(?!\w)`, then an optional `class` or `struct`, an
    /// attribute or white space, the name, and an optional `: type` base.
    ///
    /// After the keywords comes the first of white space, an attribute and a
    /// `{` ahead.
    pub(super) fn enum_block(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let word = self.lit(p, b"enum")?;
        if self.word_at(word).is_some() {
            return None;
        }
        let mut keyword = None;
        for text in [&b"class"[..], &b"struct"[..]] {
            let a = self.skip_space(word);
            if a > word
                && let Some(e) = self.lit(a, text)
            {
                keyword = Some((a, e));
                break;
            }
        }
        for form in [keyword, None] {
            let after = form.map_or(word, |(_, e)| e);
            let spaced = self.skip_space(after);
            let (sep_end, attribute) = if spaced > after {
                (spaced, None)
            } else if let Some(&end) = self.attribute_ends(after).first() {
                (end, Some(end))
            } else if self.at(after) == b'{' && after < self.lim() {
                (after, None)
            } else {
                continue;
            };
            let name_at = self.skip_space(sep_end);
            let name_end = if self.word_before(name_at) {
                name_at
            } else {
                self.ident_bounded(name_at).unwrap_or(name_at)
            };
            self.capture(1, p, word);
            if let Some((a, e)) = form {
                self.capture(2, a, e);
            }
            if let Some(end) = attribute {
                self.capture(3, after, end);
            }
            self.capture(4, name_at, name_end);
            return Some(self.enum_base(name_end).unwrap_or(name_end));
        }
        None
    }

    /// `(CHAIN)(NAME)\s*+(TEMPLATE\s*+)?(::)` at `p`, every way the chain,
    /// the name and the template are chosen: the first for which `accept`
    /// gives a value.
    fn scope_resolutions<T>(
        &self,
        p: usize,
        mut accept: impl FnMut(&Self, &ScopeResolution) -> Option<T>,
    ) -> Option<(ScopeResolution, T)> {
        self.find_scope_chain(p, |lexer, chain| {
            let name_at = chain.end;
            if !lexer.not_listed(name_at, &NOT_C3E4_KEYS) || lexer.word_before(name_at) {
                return None;
            }
            lexer.ident_ends(name_at).find_map(|name| {
                if lexer.word_at(name).is_some() {
                    return None;
                }
                let after = lexer.skip_space(name);
                let mut templates: Few<(usize, Option<Tpl>), 2> = Few::new();
                if let Some((end, comment)) = lexer.template_args(after) {
                    let spaced = lexer.skip_space(end);
                    let parts = comment.map_or([None; 3], |c| [Some(c[0]), Some(c[1]), Some(c[2])]);
                    templates.push((
                        spaced,
                        Some(Tpl {
                            start: after,
                            end,
                            spaced,
                            parts,
                        }),
                    ));
                }
                templates.push((after, None));
                templates.into_iter().find_map(|(colons, template)| {
                    if !lexer.starts(colons, b"::") {
                        return None;
                    }
                    let found = ScopeResolution {
                        chain: *chain,
                        first: p,
                        name_at,
                        name,
                        template,
                        colons,
                    };
                    let out = accept(lexer, &found)?;
                    Some((found, out))
                })
            })
        })
    }

    /// Records the groups of a scope resolution that starts at group
    /// `chain_group`.
    /// The groups are:
    /// - the chain with its `::` and template (three more groups);
    /// - the name;
    /// - the template with its comment parts;
    /// - the last `::`.
    fn put_resolution(&mut self, found: &ScopeResolution, chain_group: usize) {
        self.capture(chain_group, found.first, found.chain.end);
        self.put_chain(
            &found.chain,
            chain_group + 1,
            (chain_group + 2, chain_group + 3),
        );
        self.capture(chain_group + 4, found.name_at, found.name);
        let template = chain_group + 5;
        if let Some(tpl) = &found.template {
            self.capture(template, tpl.start, tpl.spaced);
            self.capture(template + 1, tpl.start, tpl.end);
            for (i, part) in tpl.parts.iter().enumerate() {
                if let Some((x, y)) = part {
                    self.capture(template + 2 + i, *x, *y);
                }
            }
        }
        self.capture(template + 5, found.colons, found.colons + 2);
    }

    /// `(CHAIN)(NAME)\s*+(TEMPLATE\s*+)?(::)` as a whole expression.
    pub(super) fn scope_resolution_inner(&mut self, p: usize) -> Option<usize> {
        let (found, ()) = self.scope_resolutions(p, |_, _| Some(()))?;
        self.put_resolution(&found, 1);
        Some(found.colons + 2)
    }

    /// The `\s*(:)\s*(?:CHAIN NAME TEMPLATE? ::)?\s*(ID)` tail of an enum:
    /// where it ends, with its groups recorded.
    fn enum_base(&mut self, from: usize) -> Option<usize> {
        let colon = self.skip_space(from);
        if self.at(colon) != b':' || colon >= self.lim() {
            return None;
        }
        let first = self.skip_space(colon + 1);
        let with_chain = self.scope_resolutions(first, |lexer, found| {
            let id_at = lexer.skip_space(found.colons + 2);
            if lexer.word_before(id_at) {
                return None;
            }
            Some((id_at, lexer.ident_bounded(id_at)?))
        });
        let (id_at, id) = if let Some((found, ids)) = with_chain {
            self.put_resolution(&found, 6);
            ids
        } else {
            if self.word_before(first) {
                return None;
            }
            (first, self.ident_bounded(first)?)
        };
        self.capture(5, colon, colon + 1);
        self.capture(17, id_at, id);
        Some(id)
    }

    /// `(?>(?<!\w)ID(?!\w))`: the longest identifier that no word follows.
    fn word_ident_atomic(&self, p: usize) -> Option<usize> {
        self.word_ident(p)
    }

    /// `\b(?<!(?:\W|^)(?:LIST))` at `e`, for function names: the name just
    /// matched is not one of the words.
    ///
    /// A word of the list that has no word character before it is the whole run
    /// of word characters that ends at `e`.
    /// The list is sorted.
    pub(super) fn not_behind_listed(&self, e: usize) -> bool {
        let mut start = e;
        while let Some(c) = self.ch_before(start) {
            if !super::chars::is_word(c) {
                break;
            }
            start -= c.len_utf8();
        }
        !BEHIND_KEYS.contains(self.text(start, e))
    }

    /// `(CHAIN)(NAME)\b(?<!…)GAP((TEMPLATE)\s*+)?(\()` for calls of functions
    /// named as `callee` says.
    pub(super) fn function_call_head(&mut self, p: usize, callee: Callee) -> Option<usize> {
        let (chain, name, gap, open, template) = self.find_scope_chain(p, |lexer, chain| {
            let at = chain.end;
            let upper = (callee == Callee::UpperCase
                && at < lexer.lim()
                && lexer.at(at).is_ascii_uppercase())
            .then(|| {
                let mut end = at + 1;
                while end < lexer.lim()
                    && (lexer.s[end].is_ascii_digit()
                        || lexer.s[end].is_ascii_uppercase()
                        || lexer.s[end] == b'_')
                {
                    end += 1;
                }
                (at + 1..=end).rev()
            });
            let any = (callee == Callee::Any).then(|| lexer.ident_ends(at));
            let names = upper.into_iter().flatten().chain(any.into_iter().flatten());
            for name in names {
                if !lexer.boundary(name) || !lexer.not_behind_listed(name) {
                    continue;
                }
                for gap in lexer.gaps(name) {
                    let mut templates: Few<(usize, Option<CallTemplate>), 2> = Few::new();
                    if let Some((end, comment)) = lexer.template_args(gap.end) {
                        let after = lexer.skip_space(end);
                        let parts =
                            comment.map_or([None; 3], |c| [Some(c[0]), Some(c[1]), Some(c[2])]);
                        templates.push((after, Some((end, after, parts))));
                    }
                    templates.push((gap.end, None));
                    for (open, template) in templates {
                        if lexer.at(open) == b'(' && open < lexer.lim() {
                            return Some((*chain, name, gap, open, template));
                        }
                    }
                }
            }
            None
        })?;
        self.capture(1, p, chain.end);
        self.put_chain(&chain, 2, (3, 4));
        self.capture(5, chain.end, name);
        self.put_gap(6, GapForm::Parts, name, &gap);
        if let Some((tpl_end, after, parts)) = template {
            self.capture(10, gap.end, after);
            self.capture(11, gap.end, tpl_end);
            for (i, part) in parts.iter().enumerate() {
                if let Some((a, b)) = part {
                    self.capture(12 + i, *a, *b);
                }
            }
        }
        self.capture(15, open, open + 1);
        Some(open + 1)
    }
}
