//! The bounds the C++ matchers put on their own recursion and backtracking.
//! The per-line limits (length, work, depth, nodes) are the highlighter's
//! [`Limits`](crate::Limits).

/// How many iterations of a loop that a matcher writes as recursion it
/// follows: a loop of more is read as ending there, so a hostile line cannot
/// exhaust the stack or make the candidates of a loop grow without bound.
pub(crate) const MAX_LOOPS: u32 = 64;

/// The deepest the groups that call themselves (`\g<3>`) nest, as in
/// Oniguruma: it lets a group call itself 19 times inside the group that
/// defines it, so 20 levels of `<` or `[` match and a 21st fails. A fact of
/// the regex engine Shiki runs, which the lexer must follow, and also what
/// bounds the matchers' recursion.
pub(crate) const MAX_CALL_DEPTH: usize = 20;

/// How many ends of an attribute (`[[…]]`, `__attribute__((…))`,
/// `__declspec(…)`, `alignas(…)`) a matcher tries: the lazy `.*?` of the
/// expression gives the next one each time the rest fails, and a line with
/// more than this many closings is read as having no later ones.
pub(crate) const MAX_ENDS: usize = 16;

/// How many ways a run of comments between two tokens is tried, the longest
/// first: the expression takes every comment and gives them back one at a
/// time when the rest fails, and after this many it gives back no more.
pub(crate) const MAX_COMMENT_RUNS: usize = 8;

/// How many capture groups are lexed again with a rule's patterns inside one
/// another, as a scope chain's (`a::b::c::`) groups are: each level is a call
/// of the lexer, so a hostile line cannot exhaust the stack, even with no
/// depth limit. A deeper group keeps its own scopes and its text is not lexed
/// again.
pub(crate) const MAX_NESTED: u32 = 64;
