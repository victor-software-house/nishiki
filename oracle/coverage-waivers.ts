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

/** The reason for a group that holds only blanks. */
const blanks = "the group matches only blanks, and the oracle compares no blank";

/** The reason for a group that holds only the line feed that ends a line. */
const lineFeed = "the group matches only the line feed at the end of the line, and the oracle compares no blank";

/** The reason for a group that is a lookahead. */
const lookahead = "the group is a lookahead, so it never holds text";

/** The reason for a group that the grammar names but its expression, with
 * `groups` groups, does not have. */
const missing = (groups: number) =>
  `the grammar names the group, but the expression has ${["no groups", "one group", "two groups", "three groups"][groups]}`;

/** Markdown's fences whose only pattern includes a grammar the oracle does
 * not load, because nishiki does not lex its language, with that grammar's
 * scope. vscode-textmate drops such a rule, and then the fence around it, so
 * a fence in one of these languages is a plain fence in the oracle. */
const unloadedFences = [
  ["abap", "source.abap"],
  ["bibtex", "text.bibtex"],
  ["clojure", "source.clojure"],
  ["coffee", "source.coffee"],
  ["elixir", "source.elixir"],
  ["erlang", "source.erlang"],
  ["fsharp", "source.fsharp"],
  ["git_commit", "text.git-commit"],
  ["git_rebase", "text.git-rebase"],
  ["groovy", "source.groovy"],
  ["handlebars", "text.html.handlebars"],
  ["ignore", "source.ignore"],
  ["js_regexp", "source.js.regexp"],
  ["latex", "text.tex.latex"],
  ["log", "text.log"],
  ["objc", "source.objc"],
  ["perl", "source.perl"],
  ["perl6", "source.perl.6"],
  ["pug", "text.pug"],
  ["r", "source.r"],
  ["regexp_python", "source.regexp.python"],
  ["restructuredtext", "source.rst"],
  ["twig", "source.twig"],
  ["vs_net", "source.asp.vb.net"],
  ["xsl", "text.xml.xsl"],
  ["yang", "source.yang"],
] as const;

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
  { grammar: "css", path: "#at-rules.patterns[0].patterns[0].match", group: 2, reason: blanks },
  { grammar: "css", path: "#at-rules.patterns[0].patterns[0].match", group: 4, reason: blanks },
  {
    grammar: "css",
    path: "#at-rules.patterns[0].patterns[2].patterns[0].begin",
    reason: "group 6 of `#at-rules.patterns[0].patterns[0]`, earlier in the `@charset` rule, takes an unclosed string from the same `\"`, and a closed one fails the lookahead",
  },
  {
    grammar: "css",
    path: "#at-rules.patterns[0].patterns[2].patterns[0].end",
    reason: "its begin never matches",
  },
  {
    grammar: "css",
    path: "#at-rules.patterns[3].end",
    group: 0,
    reason: "the expression is a lookahead, so the match holds no text",
  },
  {
    grammar: "css",
    path: "#escapes.patterns[1].end",
    reason: "the begin, `\\\\$\\s*`, takes the line feed, so `\\G` stands at the start of the next line and `^(?<!\\G)` never holds",
  },
  { grammar: "less", path: "#at-container.patterns[0].begin", group: 3, reason: missing(2) },
  {
    grammar: "less",
    path: "#at-container.patterns[0].patterns[0].patterns[1].patterns[5].match",
    reason: "`(([<>])=?)|[/=]`, earlier in the same list, matches every `/`",
  },
  {
    grammar: "less",
    path: "#at-custom-media.patterns[0].match",
    reason: "the rule's end, `\\s*(?=;)`, matches at the same position first",
  },
  { grammar: "less", path: "#at-custom-media.patterns[1].match", group: 3, reason: missing(2) },
  { grammar: "less", path: "#at-keyframes.begin", group: 4, reason: missing(2) },
  { grammar: "less", path: "#at-media.patterns[0].begin", group: 3, reason: missing(2) },
  { grammar: "less", path: "#at-supports.patterns[0].begin", group: 3, reason: missing(2) },
  {
    grammar: "less",
    path: "#less-namespace-accessors.patterns[0].patterns[0].match",
    group: 2,
    reason: missing(1),
  },
  {
    grammar: "less",
    path: "#less-namespace-accessors.patterns[0].patterns[3].begin",
    reason: "the rule's end, `\\s*(?:(,)|(?=[;{]))`, matches at the same position first",
  },
  {
    grammar: "less",
    path: "#less-namespace-accessors.patterns[0].patterns[3].end",
    reason: "its begin never matches",
  },
  {
    grammar: "less",
    path: "#less-namespace-accessors.patterns[1].end",
    group: 2,
    reason: missing(1),
  },
  { grammar: "less", path: "#less-strings.patterns[0].end", group: 2, reason: lineFeed },
  {
    grammar: "less",
    path: "#less-variable-assignment.patterns[0].patterns[0].match",
    group: 4,
    reason: blanks,
  },
  {
    grammar: "less",
    path: "#less-variable-comparison.patterns[0].end",
    group: 1,
    reason: missing(0),
  },
  {
    grammar: "less",
    path: "#less-variable-comparison.patterns[0].patterns[5].match",
    reason: "`#unquoted-string`, earlier in the same list, matches every `,`",
  },
  { grammar: "less", path: "#literal-string.patterns[0].end", group: 2, reason: lineFeed },
  { grammar: "less", path: "#literal-string.patterns[1].end", group: 2, reason: lineFeed },
  {
    grammar: "less",
    path: "#media-query.patterns[2].patterns[1].match",
    reason: "the media-feature name, the begin before it in the same list, matches every word this does",
  },
  {
    grammar: "less",
    path: "#rule-list-body.patterns[5].patterns[5].patterns[0].match",
    group: 4,
    reason: blanks,
  },
  {
    grammar: "less",
    path: "#rule-list-body.patterns[5].patterns[5].patterns[3].match",
    reason: "`#property-values`, earlier in the same list, takes every `,` through `#comma-delimiter`",
  },
  {
    grammar: "less",
    path: "#rule-list-body.patterns[5].patterns[9].patterns[0].match",
    group: 4,
    reason: blanks,
  },
  {
    grammar: "less",
    path: "#rule-list-body.patterns[5].patterns[10].patterns[0].match",
    group: 4,
    reason: blanks,
  },
  {
    grammar: "less",
    path: "#rule-list-body.patterns[5].patterns[11].patterns[0].match",
    group: 4,
    reason: blanks,
  },
  {
    grammar: "less",
    path: "#rule-list-body.patterns[5].patterns[11].patterns[3].match",
    reason: "the name pattern, earlier in the same list, matches every word this does",
  },
  { grammar: "less", path: "#rule-list-body.patterns[6].begin", group: 2, reason: blanks },
  {
    grammar: "less",
    path: "#selectors.patterns[10].patterns[3].patterns[3].match",
    group: 1,
    reason: "the enclosing rule's end, `(?=([]\\s]))`, matches at the blank before the flag first",
  },
  {
    grammar: "less",
    path: "#selectors.patterns[10].patterns[3].patterns[4].match",
    reason: "the enclosing rule's end, `(?=([]\\s]))`, matches at every `]` first",
  },
  { grammar: "scss", path: "#at_rule_charset.end", group: 1, reason: lookahead },
  { grammar: "scss", path: "#at_rule_charset.end", group: 2, reason: missing(1) },
  { grammar: "scss", path: "#at_rule_content.end", group: 1, reason: lookahead },
  { grammar: "scss", path: "#at_rule_each.end", group: 1, reason: lookahead },
  { grammar: "scss", path: "#at_rule_each.end", group: 2, reason: missing(1) },
  { grammar: "scss", path: "#at_rule_else.end", group: 1, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_else.end", group: 2, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_extend.end", group: 1, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_extend.end", group: 2, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_for.end", group: 1, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_for.end", group: 2, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_forward.end", group: 1, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_forward.end", group: 2, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_function.patterns[0].end", group: 1, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_function.patterns[0].end", group: 2, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_function.patterns[0].end", group: 3, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_if.end", group: 1, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_if.end", group: 2, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_import.end", group: 1, reason: lookahead },
  { grammar: "scss", path: "#at_rule_import.end", group: 2, reason: missing(1) },
  { grammar: "scss", path: "#at_rule_namespace.patterns[1].end", group: 1, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_page.patterns[0].end", group: 1, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_page.patterns[0].end", group: 2, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_page.patterns[0].end", group: 3, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_return.end", group: 1, reason: lookahead },
  { grammar: "scss", path: "#at_rule_return.end", group: 2, reason: missing(1) },
  { grammar: "scss", path: "#at_rule_use.end", group: 1, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_use.end", group: 2, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_warn.end", group: 1, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_warn.end", group: 2, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_while.end", group: 1, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_while.end", group: 2, reason: missing(0) },
  { grammar: "scss", path: "#at_rule_function.patterns[0].begin", group: 3, reason: missing(2) },
  {
    grammar: "scss",
    path: "#at_rule_function.patterns[1].match",
    reason: "`#at_rule_function.patterns[0]`, before it, begins with the same expression",
  },
  {
    grammar: "scss",
    path: "#at_rule_supports.end",
    group: 0,
    reason: "the expression is a lookahead or the end of the line, so the match holds no text",
  },
  { grammar: "scss", path: "#at_rule_supports.end", group: 1, reason: missing(0) },
  {
    grammar: "scss",
    path: "#comment_docblock.begin",
    reason: "the rule's only pattern includes `source.sassdoc`, which Shiki does not load, so vscode-textmate drops the rule",
  },
  {
    grammar: "scss",
    path: "#comment_docblock.end",
    reason: "the rule's only pattern includes `source.sassdoc`, which Shiki does not load, so vscode-textmate drops the rule",
  },
  {
    grammar: "scss",
    path: "#constant_sass_functions.begin",
    reason: "`#constant_functions`, before it in `#property_values`, the only rule that includes it, matches every `name(` this does",
  },
  { grammar: "scss", path: "#constant_sass_functions.end", reason: "its begin never matches" },
  { grammar: "scss", path: "#parent_selector_suffix.match", group: 2, reason: missing(1) },
  {
    grammar: "scss",
    path: "#parent_selector_suffix.captures[2].patterns[1].match",
    reason: "it applies to group 2, which the expression does not have",
  },
  {
    grammar: "scss",
    path: "#parent_selector_suffix.captures[2].patterns[2].match",
    reason: "it applies to group 2, which the expression does not have",
  },
  { grammar: "python", path: "#docstring.patterns[2].end", group: 2, reason: lineFeed },
  { grammar: "python", path: "#docstring.patterns[3].end", group: 2, reason: lineFeed },
  {
    grammar: "python",
    path: "#double-one-regexp-character-set.patterns[1].end",
    group: 2,
    reason: lookahead,
  },
  { grammar: "python", path: "#double-one-regexp-comments.end", group: 2, reason: lookahead },
  { grammar: "python", path: "#double-one-regexp-conditional.end", group: 2, reason: lookahead },
  {
    grammar: "python",
    path: "#double-one-regexp-lookahead-negative.end",
    group: 2,
    reason: lookahead,
  },
  { grammar: "python", path: "#double-one-regexp-lookahead.end", group: 2, reason: lookahead },
  {
    grammar: "python",
    path: "#double-one-regexp-lookbehind-negative.end",
    group: 2,
    reason: lookahead,
  },
  { grammar: "python", path: "#double-one-regexp-lookbehind.end", group: 2, reason: lookahead },
  { grammar: "python", path: "#double-one-regexp-named-group.end", group: 2, reason: lookahead },
  {
    grammar: "python",
    path: "#double-one-regexp-parentheses-non-capturing.end",
    group: 2,
    reason: lookahead,
  },
  { grammar: "python", path: "#double-one-regexp-parentheses.end", group: 2, reason: lookahead },
  {
    grammar: "python",
    path: "#double-three-regexp-character-set.patterns[1].end",
    group: 2,
    reason: missing(1),
  },
  { grammar: "python", path: "#double-three-regexp-comments.end", group: 2, reason: missing(1) },
  { grammar: "python", path: "#double-three-regexp-conditional.end", group: 2, reason: missing(1) },
  {
    grammar: "python",
    path: "#double-three-regexp-lookahead-negative.end",
    group: 2,
    reason: missing(1),
  },
  { grammar: "python", path: "#double-three-regexp-lookahead.end", group: 2, reason: missing(1) },
  {
    grammar: "python",
    path: "#double-three-regexp-lookbehind-negative.end",
    group: 2,
    reason: missing(1),
  },
  { grammar: "python", path: "#double-three-regexp-lookbehind.end", group: 2, reason: missing(1) },
  { grammar: "python", path: "#double-three-regexp-named-group.end", group: 2, reason: missing(1) },
  {
    grammar: "python",
    path: "#double-three-regexp-parentheses-non-capturing.end",
    group: 2,
    reason: missing(1),
  },
  { grammar: "python", path: "#double-three-regexp-parentheses.end", group: 2, reason: missing(1) },
  { grammar: "python", path: "#fstring-fnorm-quoted-multi-line.end", group: 2, reason: missing(1) },
  { grammar: "python", path: "#fstring-fnorm-quoted-single-line.end", group: 2, reason: lineFeed },
  {
    grammar: "python",
    path: "#fstring-formatting-braces.patterns[0].match",
    group: 2,
    reason: blanks,
  },
  { grammar: "python", path: "#fstring-normf-quoted-multi-line.end", group: 2, reason: missing(1) },
  { grammar: "python", path: "#fstring-normf-quoted-single-line.end", group: 2, reason: lineFeed },
  { grammar: "python", path: "#fstring-raw-quoted-multi-line.end", group: 2, reason: missing(1) },
  { grammar: "python", path: "#fstring-raw-quoted-single-line.end", group: 2, reason: lineFeed },
  {
    grammar: "python",
    path: "#impossible.match",
    reason: "`$.^` needs a character between the end of a line and the start of one, and `.` takes no line feed",
  },
  { grammar: "python", path: "#regexp-double-one-line.end", group: 2, reason: lineFeed },
  { grammar: "python", path: "#regexp-double-three-line.end", group: 2, reason: missing(1) },
  { grammar: "python", path: "#regexp-single-one-line.end", group: 2, reason: lineFeed },
  { grammar: "python", path: "#regexp-single-three-line.end", group: 2, reason: missing(1) },
  {
    grammar: "python",
    path: "#single-one-regexp-character-set.patterns[1].end",
    group: 2,
    reason: lookahead,
  },
  { grammar: "python", path: "#single-one-regexp-comments.end", group: 2, reason: lookahead },
  { grammar: "python", path: "#single-one-regexp-conditional.end", group: 2, reason: lookahead },
  {
    grammar: "python",
    path: "#single-one-regexp-lookahead-negative.end",
    group: 2,
    reason: lookahead,
  },
  { grammar: "python", path: "#single-one-regexp-lookahead.end", group: 2, reason: lookahead },
  {
    grammar: "python",
    path: "#single-one-regexp-lookbehind-negative.end",
    group: 2,
    reason: lookahead,
  },
  { grammar: "python", path: "#single-one-regexp-lookbehind.end", group: 2, reason: lookahead },
  { grammar: "python", path: "#single-one-regexp-named-group.end", group: 2, reason: lookahead },
  {
    grammar: "python",
    path: "#single-one-regexp-parentheses-non-capturing.end",
    group: 2,
    reason: lookahead,
  },
  { grammar: "python", path: "#single-one-regexp-parentheses.end", group: 2, reason: lookahead },
  {
    grammar: "python",
    path: "#single-three-regexp-character-set.patterns[1].end",
    group: 2,
    reason: missing(1),
  },
  { grammar: "python", path: "#single-three-regexp-comments.end", group: 2, reason: missing(1) },
  { grammar: "python", path: "#single-three-regexp-conditional.end", group: 2, reason: missing(1) },
  {
    grammar: "python",
    path: "#single-three-regexp-lookahead-negative.end",
    group: 2,
    reason: missing(1),
  },
  { grammar: "python", path: "#single-three-regexp-lookahead.end", group: 2, reason: missing(1) },
  {
    grammar: "python",
    path: "#single-three-regexp-lookbehind-negative.end",
    group: 2,
    reason: missing(1),
  },
  { grammar: "python", path: "#single-three-regexp-lookbehind.end", group: 2, reason: missing(1) },
  { grammar: "python", path: "#single-three-regexp-named-group.end", group: 2, reason: missing(1) },
  {
    grammar: "python",
    path: "#single-three-regexp-parentheses-non-capturing.end",
    group: 2,
    reason: missing(1),
  },
  { grammar: "python", path: "#single-three-regexp-parentheses.end", group: 2, reason: missing(1) },
  { grammar: "python", path: "#string-bin-quoted-multi-line.end", group: 2, reason: missing(1) },
  { grammar: "python", path: "#string-bin-quoted-single-line.end", group: 2, reason: lineFeed },
  { grammar: "python", path: "#string-quoted-multi-line.end", group: 2, reason: missing(1) },
  { grammar: "python", path: "#string-quoted-single-line.end", group: 2, reason: lineFeed },
  {
    grammar: "python",
    path: "#string-raw-bin-quoted-multi-line.end",
    group: 2,
    reason: missing(1),
  },
  { grammar: "python", path: "#string-raw-bin-quoted-single-line.end", group: 2, reason: lineFeed },
  { grammar: "python", path: "#string-raw-quoted-multi-line.end", group: 2, reason: missing(1) },
  { grammar: "python", path: "#string-raw-quoted-single-line.end", group: 2, reason: lineFeed },
  ...unloadedFences.flatMap(([name, scope]): Waiver[] =>
    ["begin", "end", "patterns[0].begin", "patterns[0].while"].map((part) => ({
      grammar: "markdown",
      path: `#fenced_code_block_${name}.${part}`,
      reason: `the fence's only pattern includes \`${scope}\`, which the oracle does not load, so vscode-textmate drops the rule`,
    })),
  ),
  {
    grammar: "markdown",
    path: "#html.patterns[0].begin",
    group: 1,
    reason: "the group matches only the empty start of a line, so it never holds text",
  },
  { grammar: "markdown", path: "#html.patterns[0].end", group: 2, reason: missing(1) },
  { grammar: "markdown", path: "#link-email.match", group: 4, reason: missing(3) },
  {
    grammar: "markdown",
    path: "#list_paragraph.begin",
    reason: "`#paragraph`, which `#block` includes before it in both list rules, begins at every position this does",
  },
  { grammar: "markdown", path: "#list_paragraph.while", reason: "its begin never matches" },
  { grammar: "toml", path: "#comments.begin", group: 1, reason: blanks },
  ...(
    [
      ["#key_pair.patterns[0].end", [1, 2]],
      ["#key_pair.patterns[1].end", [1, 2, 3, 4, 5]],
      ["#key_pair.patterns[2].end", [1, 2, 4, 5]],
      ["#key_pair.patterns[3].end", [1, 3]],
    ] as const
  ).flatMap(([path, groups]) =>
    groups.map((group): Waiver => ({
      grammar: "toml",
      path,
      group,
      reason: missing(0),
    })),
  ),
  { grammar: "yaml", path: "#block-pair.patterns[0].begin", group: 1, reason: missing(0) },
  { grammar: "yaml", path: "#comment.begin", group: 1, reason: blanks },
  {
    grammar: "yaml",
    path: "#flow-pair.patterns[0].patterns[3].begin",
    reason: "`#flow-pair`, earlier in the same list, includes `#flow-pair.patterns[3]`, whose begin is the same expression",
  },
  {
    grammar: "yaml",
    path: "#flow-pair.patterns[0].patterns[3].begin",
    group: 0,
    reason: "its begin never matches",
  },
  { grammar: "yaml", path: "#flow-pair.patterns[0].patterns[3].end", reason: "its begin never matches" },
  { grammar: "yaml", path: "#flow-pair.patterns[3].end", group: 0, reason: lookahead },
];

/** The grammars that are complete, by Shiki's name. A grammar joins in the
 * pull request that closes its last gap. */
export const complete: string[] = [
  "csharp",
  "css",
  "javascript",
  "jsx",
  "less",
  "markdown",
  "python",
  "rust",
  "scss",
  "toml",
  "tsx",
  "typescript",
  "yaml",
];
