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

/** The TypeScript family's grammars, which share their units' paths. */
const typescriptFamily = ["javascript", "jsx", "tsx", "typescript"];

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
  ...typescriptFamily.flatMap((grammar): Waiver[] => [
    { grammar, path: "#comment.patterns[2].begin", group: 1, reason: "the group matches only the blanks that lead the line, and the oracle compares no blank" },
    { grammar, path: "#single-line-comment-consuming-line-ending.begin", group: 1, reason: "the group matches only the blanks that lead the line, and the oracle compares no blank" },
    {
      grammar,
      path: "#expression-operators.patterns[6].match",
      reason:
        "`#instanceof-expr` begins with the same expression and comes before `#expression-operators` in `#expressionWithoutIdentifiers`, the only rule that includes it",
    },
    {
      grammar,
      path: "#expression-operators.patterns[7].match",
      reason:
        "`#new-expr` begins with the same expression and comes before `#expression-operators` in `#expressionWithoutIdentifiers`, the only rule that includes it",
    },
    {
      grammar,
      path: "#support-objects.patterns[4].match",
      group: 4,
      reason: "the group sits inside the negative lookahead after the property names, so it never holds text; the constant names are group 53",
    },
    { grammar, path: "#template.patterns[1].begin", group: 1, reason: "`#template-call`, the pattern before it, starts at the same name wherever this group would match one" },
    { grammar, path: "#template-type.patterns[1].begin", group: 1, reason: "`#template-call`, the pattern before it, starts at the same name wherever this group would match one" },
  ]),
];

/** The grammars that are complete, by Shiki's name. A grammar joins in the
 * pull request that closes its last gap. */
export const complete: string[] = ["csharp", "javascript", "jsx", "rust", "tsx", "typescript"];
