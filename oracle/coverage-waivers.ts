// The oracle-coverage gate's inputs, which oracle/coverage.ts reads.
//
// A grammar is complete when each unit it reaches is exercised by a fixture
// or waived here. Once complete, it stays so: a change that leaves one of its
// units unexercised and unwaived fails `mise run check:oracle-coverage`.
// openspec/changes/oracle-coverage/design.md, section 5.

/** A unit no document can reach, such as a branch that an earlier pattern
 * always takes first. */
export type Waiver = {
  /** Shiki's name for the grammar. */
  grammar: string;
  /** The expression's path, as tests/oracle/COVERAGE.md prints it. */
  path: string;
  /** The capture group, when the waiver is for one group of the expression. */
  group?: number;
  /** Why no document reaches the unit. */
  reason: string;
  /** The oracle/onig.ts expression id whose probe cases check the matcher,
   * required where oracle/fuzz/<grammar>.ts has probe support. */
  probe?: string;
};

export const waivers: Waiver[] = [
  {
    grammar: "rust",
    path: "#functions.patterns[1].match",
    reason: "`\\b(pub)\\b`, the pattern before it, matches wherever `\\b(pub)(\\()` does",
  },
  {
    grammar: "csharp",
    path: "#comment.patterns[0].begin",
    group: 1,
    reason: "the group matches only the blanks that lead the line, and the oracle compares no blank",
    probe: "CommentAt0",
  },
  {
    grammar: "csharp",
    path: "#comment.patterns[0].while",
    group: 1,
    reason: "the group matches only the blanks that lead the line, and the oracle compares no blank",
    probe: "CommentAt0While",
  },
  {
    grammar: "csharp",
    path: "#comment.patterns[1].begin",
    group: 1,
    reason: "the group matches only the blanks that lead the line, and the oracle compares no blank",
    probe: "CommentAt1",
  },
  {
    grammar: "csharp",
    path: "#comment.patterns[1].end",
    group: 1,
    reason: "the group matches only the blanks that lead the line, and the oracle compares no blank",
    probe: "CommentAt1End",
  },
  {
    grammar: "csharp",
    path: "#comment.patterns[1].patterns[0].while",
    group: 1,
    reason: "the group matches only the blanks that lead the line, and the oracle compares no blank",
    probe: "CommentAt1At0While",
  },
  {
    grammar: "csharp",
    path: "#comment.patterns[2].begin",
    group: 1,
    reason: "the group matches only the blanks that lead the line, and the oracle compares no blank",
    probe: "CommentAt2",
  },
  {
    grammar: "csharp",
    path: "#comment.patterns[2].while",
    group: 1,
    reason: "the group matches only the blanks that lead the line, and the oracle compares no blank",
    probe: "CommentAt2While",
  },
  {
    grammar: "csharp",
    path: "#numeric-literal.captures[0].patterns[0].patterns[1].match",
    group: 3,
    reason: "`[01_]`, the first alternative of the repetition, takes every `_`, so the separator group never takes part",
    probe: "NumericLiteral",
  },
  {
    grammar: "csharp",
    path: "#preprocessor-line.patterns[0].match",
    group: 2,
    reason: "the regex `\\\\b(default|hidden)` has one group, so group 2 never exists",
    probe: "PreprocessorLineAt0",
  },
  {
    grammar: "csharp",
    path: "#relational-pattern.begin",
    reason: "`#constant-pattern`, earlier in `#pattern`, includes `#expression-operators`, whose `<=|>=|[<>]` and `\\\\.\\\\.` match first at every position this does",
    probe: "RelationalPattern",
  },
  {
    grammar: "csharp",
    path: "#relational-pattern.end",
    reason: "its begin never matches",
    probe: "RelationalPatternEnd",
  },
  {
    grammar: "csharp",
    path: "#slice-pattern.match",
    reason: "`#constant-pattern`, earlier in `#pattern`, includes `#expression-operators`, whose `<=|>=|[<>]` and `\\\\.\\\\.` match first at every position this does",
    probe: "SlicePattern",
  },
];

/** The grammars that are complete, by Shiki's name. A grammar joins in the
 * pull request that closes its last gap. */
export const complete: string[] = ["csharp", "rust"];
