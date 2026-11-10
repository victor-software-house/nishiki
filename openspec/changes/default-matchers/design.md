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

Under the default limits, on a box:

| Line | Hand matcher, at base | Generated matcher, step 2b |
|:--|:--|:--|
| 101 bytes, one short of the longest fixture line (102) | uncut | ends at `Limit::Work` |
| 12,001 bytes | ends at `Limit::Work` | ends at `Limit::Work` |

So the generated matcher cuts a 101-byte line that the hand matcher lexes whole.
The 12,001-byte line is the pattern's limit line.

A 102-byte line was not measured.
From 101 bytes, its work grows by about `2L`, some 200 steps, and the budget by 64.
So it would end at the limit too.
The tests of [rule-table-codegen][rule-table-codegen] task 1.1 run it at 102 bytes.

Two fixes, in order:

1. `codegen/` drops a repeat's arm when the other arm gives the same match.
   Both fixes keep the match exact (lead, 2026-11-10).
   Before it drops an arm, `codegen/` checks that at every position:
   - both arms give the same set of ends;
   - each end has the same group spans in both;
   - each end comes in the same order of preference, so leftmost-first picks the same one.

   `\.?` before `[.\w]+` passes the check, because `.` is in `[.\w]`.
   The arm that takes the dot and the arm that leaves it to `[.\w]+` give the same ends.
   Both put the whole run in the one group around them.
   Dropping the arm halves the work at a dot.
2. A skip in the generated scan makes the line linear:
   - the pattern fails after taking a run of `[.\w]`;
   - every later start inside that run then fails too, since the run and what follows it stay the same;
   - the scan then goes on from the run's end.

   `codegen/` proves that skip from the expression, so it holds for any grammar.
   With it, the 12,001-byte line no longer reaches the limit, and the pattern needs no limit line.

Until both land, a caller that needs such lines uncut raises `Limits::work`.
Protocol Buffers moves to generated matchers only after them, since Identity and the hostile-line tests must hold.

[proposal]: proposal.md
[research]: research.md
[rule-table-codegen]: ../rule-table-codegen/tasks.md
