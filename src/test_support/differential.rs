//! What the tests of the generated matchers share.
//!
//! - [`Expression`]: each pattern's expression, as `codegen/` writes it into
//!   the language's `EXPRESSIONS` table.
//! - [`group_readers`]: the groups the lexer reads after a pattern matches.
//! - [`differential`]: the test that compares each generated matcher with the
//!   regex engine.

use crate::rules::{Act, CarriedState, Grammar, ScopeName};

/// One pattern's expression and the groups its generated matcher records.
#[derive(Debug)]
pub(crate) struct Expression<P: 'static> {
    /// The pattern.
    pub(crate) pattern: P,
    /// The expression, cut into pieces that fit the line width.
    #[cfg_attr(
        not(feature = "_textmate"),
        expect(
            dead_code,
            reason = "only the differential test reads it, with the regex engine"
        )
    )]
    pub(crate) pieces: &'static [&'static str],
    /// The groups past the whole match that the matcher records, sorted.
    pub(crate) records: &'static [usize],
}

#[cfg(feature = "_textmate")]
impl<P> Expression<P> {
    /// The expression, as the grammar writes it.
    pub(crate) fn regex(&self) -> String {
        self.pieces.concat()
    }
}

/// Where the lexer reads a group of a match.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum GroupReader {
    /// `emit_captures` names the group with the capture table's scopes.
    Table,
    /// `emit_captures` lexes the group's text with patterns of its own.
    Tokenized,
    /// `dynamic_path` puts the group's text in a scope: in the table, in the
    /// act's names, or in the names of the rule a begin opens.
    ScopeText,
    /// `begin_rule` keeps the group's text as the begin text of the rule it
    /// opens.
    /// The rule's end or while pattern matches that text.
    /// So does the end of a language the rule embeds, as its parent's begin
    /// text.
    /// Each reads the kept text, never the group again.
    BeginText,
}

/// Each group past the whole match that the lexer reads after `pattern`
/// matches, with where it reads it, sorted.
///
/// The lexer reads a group of a match in these places, and nowhere else.
/// A generated matcher must record each of these groups.
pub(crate) fn group_readers<G: Grammar>(pattern: G::Pattern) -> Vec<(usize, GroupReader)> {
    let (act, (table, tokenized)) = G::pattern_definition(pattern);
    let mut readers = table
        .iter()
        .map(|&(group, _)| (group, GroupReader::Table))
        .chain(
            tokenized
                .iter()
                .map(|&(group, _)| (group, GroupReader::Tokenized)),
        )
        .collect::<Vec<_>>();
    let mut lists = table.iter().map(|&(_, scopes)| scopes).collect::<Vec<_>>();
    match act {
        Act::Match(scopes) => lists.push(scopes),
        Act::Begin(rule) => {
            lists.extend([
                G::rule_name(rule),
                G::rule_name_after(rule),
                G::rule_content(rule),
            ]);
            if let CarriedState::Backref(group) = G::rule_carried_state(rule) {
                readers.push((group, GroupReader::BeginText));
            }
        }
        Act::End => {}
    }
    for scopes in lists {
        for &scope in &scopes.list {
            if let ScopeName::Capture { group, .. } = G::scope(scope) {
                readers.push((group, GroupReader::ScopeText));
            }
        }
    }
    let mut readers = readers
        .into_iter()
        .filter(|&(group, _)| group != 0)
        .map(|(group, reader)| (usize::from(group), reader))
        .collect::<Vec<_>>();
    readers.sort_unstable();
    readers.dedup();
    readers
}

/// The groups past the whole match that the lexer reads after `pattern`
/// matches ([`group_readers`]), sorted.
pub(crate) fn read_groups<G: Grammar>(pattern: G::Pattern) -> Vec<usize> {
    let mut groups = group_readers::<G>(pattern)
        .into_iter()
        .map(|(group, _)| group)
        .collect::<Vec<_>>();
    groups.dedup();
    groups
}

/// The lines of the language's oracle fixtures, each with whether a line feed
/// follows it, then fuzzed lines:
///
/// - lines whose characters are drawn from the fixtures' characters;
/// - fixture lines with ranges deleted, repeated or reversed.
#[cfg(feature = "_textmate")]
fn lines(language: &str) -> Vec<(String, bool)> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = std::fs::read_dir(root.join("tests/oracle").join(language))
        .expect("the language's oracle fixtures")
        .filter_map(|entry| Some(entry.ok()?.path()))
        .filter(|path| path.extension().is_some_and(|extension| extension == "txt"))
        .collect::<Vec<_>>();
    files.sort();
    let mut lines = Vec::new();
    for file in &files {
        let text = std::fs::read_to_string(file).expect("a fixture");
        let mut pieces = text.split('\n').collect::<Vec<_>>();
        let last = pieces.pop().unwrap_or_default();
        lines.extend(pieces.into_iter().map(|line| (line.to_owned(), true)));
        if !last.is_empty() {
            lines.push((last.to_owned(), false));
        }
    }
    let fixture_lines = lines.len();
    assert!(fixture_lines > 0, "{language} has no oracle fixture lines");
    let mut alphabet = lines
        .iter()
        .flat_map(|(line, _)| line.chars())
        .collect::<Vec<_>>();
    alphabet.sort_unstable();
    alphabet.dedup();
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
    let mut random = move |bound: usize| {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        usize::try_from(seed % u64::try_from(bound.max(1)).unwrap_or(1)).unwrap_or(0)
    };
    for round in 0..2_000 {
        let line = if round % 2 == 0 {
            (0..random(80))
                .map(|_| alphabet[random(alphabet.len())])
                .collect::<String>()
        } else {
            let mut chars = lines[random(fixture_lines)].0.chars().collect::<Vec<_>>();
            for _ in 0..=random(3) {
                let len = chars.len();
                let (a, b) = (random(len + 1), random(len + 1));
                let (start, end) = (a.min(b), a.max(b));
                match random(3) {
                    0 => {
                        chars.drain(start..end);
                    }
                    1 => {
                        let copy = chars[start..end].to_vec();
                        chars.splice(end..end, copy);
                    }
                    _ => chars[start..end].reverse(),
                }
            }
            chars.into_iter().collect()
        };
        lines.push((line, random(4) != 0));
    }
    lines
}

/// One expression compiled by the regex engine for both kinds of line.
#[cfg(feature = "_textmate")]
struct Engine {
    lean: crate::textmate::regex::Regex,
    full: crate::textmate::regex::Regex,
}

#[cfg(feature = "_textmate")]
impl Engine {
    /// `regex` of the pattern `pattern`, compiled for both kinds of line.
    fn new(pattern: &dyn std::fmt::Debug, regex: &str) -> Engine {
        use crate::textmate::regex::{Programs, Regex};

        let compile = |programs| {
            Regex::for_grammar(regex, programs).unwrap_or_else(|error| {
                panic!("the regex engine refuses {pattern:?}, {regex:?}: {error:?}")
            })
        };
        Engine {
            lean: compile(Programs::Lean),
            full: compile(Programs::Full),
        }
    }
}

/// What a matcher answers at one position: the end, and its groups.
#[cfg(feature = "_textmate")]
type Answer = Option<(usize, Vec<Option<(usize, usize)>>)>;

/// Compares each generated matcher of `G` with the regex engine on the
/// pattern's expression, and panics at the first difference.
///
/// It tries every pattern at each character of the lines [`lines`] gives.
/// A pattern whose expression holds `\G` is tried with `\G` holding and not.
/// The match end and every group the lexer reads ([`read_groups`]) must be
/// equal.
/// The lexer has no work limit, because the two sides count steps in
/// different units.
///
/// It returns how many tries it compared.
#[cfg(feature = "_textmate")]
pub(crate) fn differential<G: Grammar>(
    language: &str,
    expressions: &[Expression<G::Pattern>],
) -> u64 {
    use super::probe::{After, Anchor, Case, lexer_under};
    use crate::Limits;
    use crate::arena::Arena;
    use crate::textmate::regex::{Context, Programs, Scratch};

    let lines = lines(language);
    let tried = expressions
        .iter()
        .map(|expression| {
            let regex = expression.regex();
            (
                expression.pattern,
                Engine::new(&expression.pattern, &regex),
                regex.contains("\\G"),
                read_groups::<G>(expression.pattern),
            )
        })
        .collect::<Vec<_>>();
    let mut arena = Arena::new(None);
    let unlimited = Limits {
        work: None,
        ..Limits::default()
    };
    let mut scratch = Scratch::default();
    let mut compared = 0;
    for (text, nl) in &lines {
        let mut bytes = text.as_bytes().to_vec();
        if *nl {
            bytes.push(b'\n');
        }
        let programs = Programs::of(&bytes);
        for anchor in [Anchor::Free, Anchor::G] {
            let g = matches!(anchor, Anchor::G);
            let case = Case {
                text,
                p: 0,
                anchor,
                after: if *nl { After::LineFeed } else { After::Nothing },
            };
            lexer_under::<G, _>(&mut arena, &case, &unlimited, |lexer| {
                for p in (0..=text.len()).filter(|&p| text.is_char_boundary(p)) {
                    for (pattern, engine, holds_g, reads) in &tried {
                        if g && !holds_g {
                            continue;
                        }
                        let keep = |found: Answer| -> Answer {
                            found.map(|(end, groups)| {
                                let kept = reads
                                    .iter()
                                    .map(|&group| groups.get(group).copied().flatten())
                                    .collect();
                                (end, kept)
                            })
                        };
                        let mine = keep(
                            lexer.probe_matcher(p, g, |lexer, p| G::matcher(lexer, *pattern, p)),
                        );
                        let regex = match programs {
                            Programs::Lean => &engine.lean,
                            Programs::Full => &engine.full,
                        };
                        let context = Context {
                            search: g.then_some(p),
                            text_start: false,
                        };
                        let mut steps = u64::MAX;
                        let matched = regex.match_at(&mut scratch, &bytes, p, context, &mut steps);
                        let theirs = keep(matched.then(|| {
                            let end = scratch.group(0).map_or(p, |(_, end)| end);
                            let groups = (0..scratch.group_count())
                                .map(|group| scratch.group(group))
                                .collect();
                            (end, groups)
                        }));
                        compared += 1;
                        assert_eq!(
                            mine, theirs,
                            "{language} {pattern:?} at {p}, \\G {g}, line feed {nl}: {text:?}"
                        );
                    }
                }
            });
        }
    }
    compared
}
