# Design

This change gives every pattern of a rule-engine grammar a matcher written from its regular expression.
[Its research][research] holds the prototype's measurements.
[The proposal][proposal] links the four changes split from `rust-codegen` with it.

## 1. Default matchers

Today every pattern of a rule-engine grammar has a matcher written by hand.
A caller's grammar has none.

`unstable::codegen` gives every pattern a default matcher (lead, 2026-11-10):

- The default is Rust code, written from the pattern's parsed tree (section 2).
- A pattern the writer cannot express exactly falls back to the crate's TextMate regex engine, `src/textmate/regex`.
  - The engine compiles the pattern once, on first use.
  - It is built on `regex-automata`, and it adds what Oniguruma has and `regex-automata` lacks: back-references and lookbehind.
  - A back-reference, or a lookbehind across the line, takes this path.
  - So does a repeat whose turn the analysis of section 2.1 cannot prove unique.
- The regex engine stays the fallback for a caller's grammar, for every pattern the writer refuses.
- A language can name a hand-written matcher for any pattern.
  The generated `matcher` calls it for that pattern and the default for the rest.
- nishiki's own languages keep their hand matchers until the generated code measures as fast.

The aim is to replace the hand matchers.

- The target is a lexing count at or below 1.000 of the hand matchers'.
  It is counted at one codegen unit (`bench/AGENTS.md` rule 12).
- A language that reaches it deletes its `matchers/` directory in its move pull request.
- Today generated code runs 1.17 to 1.54 times the hand matchers ([research][research]).
  The regex engine runs 2.56 to 4.54 times.

## 2. Exact generated matchers

Shiki-exact output gates the release, so a generated matcher matches exactly what Oniguruma matches.
The code never recurses once per turn of a repeat: it loops, or keeps an explicit stack.

### 2.1 Repeats

| Repeat | Written as |
|:--|:--|
| one character a turn | a loop that gives back one character at a time |
| at most one turn | the body, then the continuation, in the regex's order |
| possessive | a loop that keeps each turn's first match |
| greedy or lazy, with a turn proven unique | a loop that keeps each turn's first match |
| the same, when what follows can start like the body | a loop, then turns given back from an explicit stack |

Any other repeat sends the pattern to the regex engine.

The analysis proves a node unique given what can follow it:

- one character, an anchor, a lookaround or an atomic group has one match;
- a sequence is unique when each part is, given what follows that part;
- an alternation is unique when its arms are, and no two arms can start with the same character;
- a repeat that can give back a turn is unique when what follows must read a character its body cannot start with.

The sets hold code points, not UTF-8 lead bytes.
`\w` and `\s` share every non-ASCII lead byte but no character.

### 2.2 The work budget

A generated matcher spends the line's work budget, `Limits::work`, as a hand matcher does.
`Work` in `src/limits.rs` defines a step as one turn of a lexer's main loop or one byte a lookahead reads.
The `line-limits` spec counts limits in work and bytes, never in time.

The matcher spends in its loops, not at each byte it reads (lead, 2026-11-10):

- A loop that takes turns spends the bytes it took when it ends, on every exit, a failed one included.
- A lazy loop spends each turn's bytes as it takes the turn.
- Each turn given back, or popped from the stack, spends one step.
- A loop that gives back or tries the continuation first checks the budget at each turn.
- A repeat inside a lookahead, or nested in another repeat, is a loop of its own and follows the same rules.

Spending at each byte read kept the `\b` test out of line and cost Zig and Lua about 16% more instructions.
Spending in loops costs Protocol Buffers 1.5% and Zig and Lua about 3% over step 1 ([research][research]).

The steps bound the work of each try of a pattern:

- Every byte the matcher reads lies inside a loop, or inside straight-line code between loops.
- Straight-line code reads a number of bytes the pattern fixes: its characters, classes, anchors and fixed-width lookbehinds.
- Each loop turn takes at least one byte, which the loop spends, or gives back a turn, which spends one step.
- So a try that spends `s` steps does at most `c × (s + 1)` work, where `c` depends only on the pattern.
- The scanner tries patterns at each start, and each try spends its own steps.

When the budget runs out, the matcher stops and reports no match, as the regex engine does.
The lexer then ends the line at the work limit.
The rest of the line is one token under the current scopes, and the line's report records `Limit::Work`.

The regex engine, as the fallback, runs on the steps the line has left, and stops the same way.
The two count in different units: a generated matcher counts loop work, and the engine counts instructions.
On a line near the limit they can stop at different points.

The default limits still never cut an oracle fixture line, as the `line-limits` spec requires.
A language that moves to generated matchers keeps that check: its oracle test runs under the default limits.

### 2.3 The differential test

A differential test guards every generated language:

- It runs each generated matcher and the regex engine on the same expression.
- It tries every character of every oracle fixture line, with `\G` holding too where the expression holds it.
- It adds fuzzed lines: random ones from the fixtures' characters, and fixture lines with ranges deleted, repeated or reversed.
- It runs with no work limit, since the two count steps in different units.
- It fails at the first differing end or group.
- It runs in CI for every language `codegen/` writes.

### 2.4 Hostile lines

Three kinds of hostile line run in CI beside it, in the language's `tests/it/limits/` file (lead, 2026-11-10):

| Line | Length | Drives | Must |
|:--|:--|:--|:--|
| bound line | about 12,000 bytes, past every fixture line | a give-back loop or a stack, such as a long run of a repeat's body with no continuation | finish under the default work limit, with tokens that cover it |
| retry line | the language's longest oracle fixture line | a repeat that takes a long run and fails, so the scanner tries it again at each later start | end without reaching the work limit |
| limit line | about 12,000 bytes | a pattern whose work grows faster than the line, found by the other two | end at the work limit, with tokens that cover it |

A bound line drives linear work, so it ends under the budget.
A bound line that reaches the limit shows a pattern whose work grows faster: a finding.
The prototype's tests still accept `Limit::Work` on a bound line, and the tests of [rule-table-codegen][rule-table-codegen] task 1.1 do not.

Each pattern whose work grows faster than the line gets a limit line of its own.
That line shows the limit cuts real superlinear work, such as the Protocol Buffers field line below.

A retry line holds only at fixture length:

- A retry at every start of an `L`-byte run takes about `L − p` bytes at start `p`, then gives them back: about `L²` steps.
- The default budget is `64L + 4,096` steps, so such a retry fits only while `L² ≤ 64L + 4,096`, up to about 100 bytes.
- The longest oracle fixture lines are 102 bytes for Protocol Buffers, 242 for Zig and 974 for Lua.
- So a retry line of fixture length passes only when the scan does not retry at every start.
  A `\b` fails at once inside a word, which the scanner skips, or another pattern takes the run first.
- On step 2b, Zig's and Lua's retry lines pass at fixture length.
  Protocol Buffers' field line ends at the work limit at 101 bytes, one short of its longest fixture line (below).
- This is a measured fact about the fixtures, not a bound.
  The hand lexers and Shiki have no linear scan either.

A retry line that reaches the limit at fixture length is a finding, because Shiki would not cut that line.
It is recorded with the pattern, the line and a plan to fix it.

### 2.5 The field-pattern finding

The first finding is Protocol Buffers' field pattern (lead, 2026-11-10):

- The pattern is `\s*(optional|repeated|required)?\s*(\.?[.\w]+)\s+(\w+)\s*(=)\s*(0[Xx]\h+|[0-9]+)`.
- The line is a type with no name after it: `a.` repeated and a space, inside a `message`.
- At each start the pattern takes the rest of the `[.\w]` run, gives it back, finds no name, and fails.
- At a start on a `.`, it does that twice: once with `\.?` taking the dot, once without.
  So the line costs more than `L²` steps.
- Shiki 4.6.0 lexes the line uncut, as one token, in under a millisecond at 101 and at 12,001 bytes.

The generated matcher cuts a 101-byte line that the hand matcher lexes whole.
The 12,001-byte line is the pattern's limit line.
[The research][research], section 4, has the measurements.

Two fixes follow, in order.
Both keep the match exact (lead, 2026-11-10).

#### Fix 1: the arm drop

`codegen/` drops a repeat's arm when the arm it keeps gives the same match.
The unit compared is an arm followed by the rest of the pattern, up to the end of the innermost group around the repeat.

For `\.?`, that is `\.[.\w]+` against `[.\w]+`, both inside group 2.
The bare arms of `\.?`, a dot and nothing, never give the same end, so a check on them would never drop an arm.

Each unit gives its ends in its order of preference, each with its group spans.
`codegen/` drops an arm only when, at every position:

- the kept arm's unit gives every end the dropped arm's unit gives, with the same group spans;
- when the dropped arm is preferred, its unit's ends are the kept unit's first ends, in the same order.

At a shared end, what follows the group sees the same position and spans, so it succeeds or fails the same way.
So the first end where it succeeds is the same with the dropped arm or without it.
A dropped arm tried second only repeats ends already tried.

`\.?` before `[.\w]+` passes at a dot:

- `\.?` is greedy, so the arm that takes the dot is preferred.
- Its unit gives the ends from the run's end down to two bytes after the dot.
- The other unit gives the same ends in the same order, then one byte after the dot.
- Both units put the whole match in group 2, so their spans are equal.

A check for equal sets of ends would refuse this case, because the preferred unit cannot end one byte after the dot.
Dropping the arm halves the work at a dot.

#### Fix 2: the scan skip

A skip in the generated scan makes the line linear.
The pattern is a prefix `P`, then `R`, the `[.\w]+` that takes a run, then a suffix `S`.
`codegen/` writes the skip only when the expression meets four conditions, and writes none otherwise:

- `P` has an empty path with no assertion on it, so it matches empty at every start.
  An assertion is `\b`, `^`, `$`, `\G` or a lookaround: each holds at some positions only.
  With `P` of `(?!a)`, `P` has no empty match before an `a`, so `R` never starts there.
  The field pattern's `\s*(…)?\s*` has such a path.
- `P` holds no lookbehind.
  With one, the run bytes that begin a match of `P` depend on the bytes before the start, and so does `k`.
- `R` is one class under a `+` that gives back: greedy or lazy, not possessive, and not inside an atomic group.
- `S` holds no back-reference and no conditional `(?(n)…)` on a group of `P` or `R`.

Say the pattern fails at a start `p` in a run that ends at `r`.
The run is the longest run of `R`'s class from `p`.

- `P` matched empty at `p`, so `R` started at `p`.
- `R` gives back, so it was tried at every end up to `r`.
- A later start `q` before `r` puts `R`'s start inside the run when `P`'s match from `q` ends before `r`.
- `R`'s ends are then ends already tried from `p`.
- `S`'s result depends only on where it starts, so it fails from each of them again.

A back-reference and a conditional each read group state, which differs between starts.
They are the only constructs that make `S` depend on more than its start.
A lookbehind and `\b` read only the line, which is the same for every start.
`\G` holds at one fixed place, where the search started, and not at each try's start.

The scan then skips every start before `r` from which no match of `P` reaches `r`.
A start whose `P` can reach `r` is still tried.
The skip follows only a failure.
A try that ended at `Limit::Work` tried only some ends, so no skip follows it.

The general rule takes `k`, the length of the longest string of the run's bytes that begins a match of `P`.
That string need not be a match of `P` itself.
With `P` of `(ab\s)?`, `ab` is no match, yet `P` reaches past `r` from `r − 2`.

After the failure, the scan goes on at `r − k`, or at `p + 1` when that is later.
`codegen/` proves `k` from the expression.
When `k` has no bound, as for a `P` of `\w*`, it writes no skip.

In the field pattern after fix 1, `P` takes nothing or one keyword from the run: `optional`, `repeated` or `required`.
So `k` is 8.
Without fix 1, `\.?` belongs to `P`, and `optional.` makes `k` 9.

On `xoptional foo bar = 1`, start 0 fails and start 1 matches, as on the regex machine.
A skip to the run's end would have missed that match.
With the skip, the 12,001-byte line no longer reaches the limit, and the pattern needs no limit line.

#### Before the fixes

Until both land, a caller that needs such lines uncut raises `Limits::work`.
Protocol Buffers moves to generated matchers only after them, since Identity and the hostile-line tests must hold.

[proposal]: proposal.md
[research]: research.md
[rule-table-codegen]: ../rule-table-codegen/tasks.md
