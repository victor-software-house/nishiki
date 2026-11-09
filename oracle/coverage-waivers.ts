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
   * required where oracle/fuzz/<grammar>.ts has probe support, unless the
   * waiver is for a rule vscode-textmate drops. */
  probe?: string;
  /** Set for a rule that vscode-textmate drops, such as one whose only
   * patterns include a grammar or a rule that is never loaded.
   * No matcher answers for such a unit, so the waiver names no probe. */
  dropped?: true;
};

/** The TypeScript family's grammars. They share their units' paths. */
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
  ["erlang", "source.erlang"],
  ["fsharp", "source.fsharp"],
  ["git_commit", "text.git-commit"],
  ["git_rebase", "text.git-rebase"],
  ["groovy", "source.groovy"],
  ["handlebars", "text.html.handlebars"],
  ["ignore", "source.ignore"],
  ["js_regexp", "source.js.regexp"],
  ["log", "text.log"],
  ["perl6", "source.perl.6"],
  ["pug", "text.pug"],
  ["regexp_python", "source.regexp.python"],
  ["restructuredtext", "source.rst"],
  ["twig", "source.twig"],
  ["xsl", "text.xml.xsl"],
  ["yang", "source.yang"],
] as const;

/** A group of a gap that opens its expression.
 * Such a gap holds a comment only when the match starts at the comment.
 * `#block_comment` or `#inline_comment` also starts there.
 * One of them comes first in every list that holds the expression. */
const leadingGap = "the gap opens the expression, so a comment there starts a comment rule at the same place, and such a rule comes before the expression in every list that holds it";

/** A group of the gap that opens `#member_access` or `#method_access`.
 * A comment rule comes first in every list that holds them, with one
 * exception.
 * The exception is their own chain captures, `#member_access.captures[11]`
 * and `#method_access.beginCaptures[13]`.
 * The text of those captures is names, dots, arrows and blanks only. */
const leadingGapChain = "the gap opens the expression, and a comment rule comes before it in every list but the chain captures of `#member_access` and `#method_access`, whose text holds no comment";

/** A group of the gap that opens `#typename`. A comment rule comes first in
 * every list that holds it except the patterns of type captures. In a type
 * capture, names and scopes exclude the keyword, and template arguments go to
 * `#template_call_range`, which starts at the `<` and lists a comment rule
 * first. */
const leadingGapTypename = "the gap opens the expression; a comment rule comes before it in every list but those of type captures, where names and scopes exclude `typename` and template arguments go to `#template_call_range`, which starts at the `<` and lists a comment rule first";

/** An expression that only scans one group of its owner.
 * That group is a gap that opens the owner's expression.
 * The group never holds a comment for it to scan. */
const scansLeading = "it scans only a gap that opens its owner's expression, and that gap never holds a comment";

/** An expression that only scans group 9 of `#over_qualified_types`.
 * Group 9 is a gap that follows the gap of group 5 with nothing between
 * them. */
const scansAdjacent = "it scans only group 9, a gap that the gap just before it leaves without a comment";

/** An expression that scans group 3 of a C inline comment expression that
 * never matches. */
const scansNeverMatching = "it scans a group of an expression that never matches";

/** An expression that scans group 16 of `#simple_constructor_call`.
 * Group 16 sits in a lookahead after the end of the match. */
const pastMatchEnd = "it scans group 16, which sits in a lookahead past the end of the match, and vscode-textmate applies no capture that starts after the match ends";

/** The begin of a block's tail rule, `patterns[2]`.
 * It starts right after the body's closing brace.
 * The block's own end matches there too. */
const tailBegin = "the rule's own end, `(?<=}|%>|\\?\\?>)`, matches at the same place wherever this begin's lookbehind holds, and an end is tried before the patterns";

/** The end of a block's tail rule, whose begin never matches. */
const tailEnd = "its begin never matches";

/** A group of a gap that directly follows another gap.
 * Both gaps try comments first.
 * The earlier gap takes every comment.
 * The later gap can match empty after it. */
const adjacentGap = "the gap just before it, with nothing between them, takes every comment first";

/** A group of the gap inside the negative lookahead that ends `#lambdas`'
 * capture list. */
const negativeLookahead = "the group sits inside a negative lookahead, so it never holds text";

/** A group of the gap that follows the specifiers of a function pointer, after
 * a lookahead that allows only blanks before `;`, `=`, `{` or the line end. */
const blanksLookahead = "the lookahead before it allows only blanks up to `;`, `=`, `{` or the line end, so the gap never holds a comment";

/** A group of a gap in the pattern that scans a member chain, the names before
 * the last `.` or `->` of `#member_access` and `#method_access`. */
const chainNoComment = "the chain group it scans holds only names, dots, arrows and blanks, never a comment";

/** The second pattern of a member chain's capture, the first pattern without
 * its lookbehind for `.` or `->`. */
const chainLookbehind = "`patterns[0]` is the same expression behind a lookbehind for `.` or `->`, which holds at the start of every element of the chain, so it starts at the same place and comes first";

/** `#exception_keywords`, whose keywords `#control_flow_keywords` also lists. */
const exceptionShadow = "`#control_flow_keywords` has the same gap, lists `throw`, `catch` and `try` too, and comes before it in every list that holds it";

/** `#string_escaped_char`, a copy of `#backslash_escapes`. */
const escapeShadow = "it is the same expression as `#backslash_escapes`, which comes before it in its only list";

/** Group 2 of `#operators.patterns[12]`, the compound bitwise assignments. */
const operatorShadow = "`patterns[8]` (`<<|>>`) and `patterns[11]` (`[&^|~]`) match the first characters of each of its alternatives at the same place, earlier in the same list";

/** The pattern of `#assembly` for a line that holds only a gap, in a macro
 * body. */
const macroAsmLine = "in a macro body a `\\` or the macro's end follows `asm`, so the begin never reaches the line end, the next line starts without `\\G`, and `(?!\\G)` ends the rule before this `^` pattern";

/** A type list of C's root list: `anon_pattern_8` (36 words), `_9` (10) or
 * `_10` (28). */
const cTypeShadow = "every word of it is in `#storage_types.patterns[1]`, which matches it at the same place and comes earlier in the only list that includes it";

/** The second rule of C's `#block_comment` or `#line_comment`, a copy of the
 * first. */
const repeatsFirst = "it repeats `patterns[0]`, which comes first in the same list";

/** The end of a C comment rule whose begin never matches. */
const beginNeverMatches = "its begin never matches";

/** A copy of C's inline comment expression. */
const inlineEquivalent = "it is `#inline_comment.patterns[0].patterns[0]` with possessive quantifiers in place of atomic groups, so it matches the same text, and that pattern comes first";

/** One C or C++ waiver row: the groups waived, or `null` for the whole
 * expression, and the oracle/onig.ts probe where the grammar has one. */
type CRow = [grammar: string, path: string, groups: number[] | null, probe?: string];

/** The C and C++ waivers, grouped by their reason. */
const cFamily: [reason: string, rows: CRow[]][] = [
  [cTypeShadow, [
    ["c", "#anon_pattern_10.match", null],
    ["c", "#anon_pattern_8.match", null],
    ["c", "#anon_pattern_9.match", null],
  ]],
  [leadingGap, [
    ["c", "#anon_pattern_range_1.begin", [1, 2, 3, 4]],
    ["c", "#storage_types.patterns[3].patterns[2].begin", [2, 3, 4, 5]],
    ["c", "#switch_statement.begin", [2, 3, 4, 5]],
    ["cpp-macro", "#assembly.patterns[2].begin", [1, 2, 3, 4], "AssemblyAt2"],
    ["cpp-macro", "#destructor_root.begin", [1, 2, 3, 4], "DestructorRoot"],
    ["cpp-macro", "#extern_block.begin", [1, 2, 3, 4], "ExternBlock"],
    ["cpp-macro", "#parameter.begin", [1, 2, 3, 4], "Parameter"],
    ["cpp-macro", "#parameter.patterns[11].match", [1, 2, 3, 4], "ParameterAt11"],
    ["cpp-macro", "#parameter.patterns[16].match", [1, 2, 3, 4], "ParameterAt16"],
    ["cpp-macro", "#parameter_or_maybe_value.begin", [1, 2, 3, 4], "Parameter"],
    ["cpp-macro", "#parameter_or_maybe_value.patterns[13].match", [1, 2, 3, 4], "ParameterOrMaybeValueAt13"],
    ["cpp-macro", "#parameter_or_maybe_value.patterns[18].match", [1, 2, 3, 4], "ParameterAt16"],
    ["cpp-macro", "#switch_statement.begin", [1, 2, 3, 4], "SwitchStatement"],
    ["cpp", "#access_control_keywords.match", [1, 2], "AccessControlKeywords"],
    ["cpp", "#assembly.patterns[2].begin", [1, 2, 3, 4], "AssemblyAt2"],
    ["cpp", "#control_flow_keywords.match", [1, 2], "ControlFlowKeywords"],
    ["cpp", "#destructor_root.begin", [1, 2, 3, 4], "DestructorRoot"],
    ["cpp", "#extern_block.begin", [1, 2, 3, 4], "ExternBlock"],
    ["cpp", "#goto_statement.match", [1, 2], "GotoStatement"],
    ["cpp", "#label.match", [1, 2], "Label"],
    ["cpp", "#memory_operators.match", [1, 2], "MemoryOperators"],
    ["cpp", "#misc_keywords.match", [1, 2], "MiscKeywords"],
    ["cpp", "#parameter.begin", [1, 2, 3, 4], "Parameter"],
    ["cpp", "#parameter.patterns[11].match", [1, 2, 3, 4], "ParameterAt11"],
    ["cpp", "#parameter.patterns[16].match", [1, 2, 3, 4], "ParameterAt16"],
    ["cpp", "#parameter_or_maybe_value.begin", [1, 2, 3, 4], "Parameter"],
    ["cpp", "#parameter_or_maybe_value.patterns[13].match", [1, 2, 3, 4], "ParameterOrMaybeValueAt13"],
    ["cpp", "#parameter_or_maybe_value.patterns[18].match", [1, 2, 3, 4], "ParameterAt16"],
    ["cpp", "#switch_statement.begin", [1, 2, 3, 4], "SwitchStatement"],
    ["cpp", "#type_casting_operators.match", [1, 2], "TypeCastingOperators"],
  ]],
  [scansLeading, [
    ["c", "#anon_pattern_range_1.beginCaptures[4].patterns[0].match", null],
    ["c", "#anon_pattern_range_1.beginCaptures[4].patterns[1].match", null],
    ["c", "#storage_types.patterns[3].patterns[2].beginCaptures[5].patterns[0].match", null],
    ["c", "#storage_types.patterns[3].patterns[2].beginCaptures[5].patterns[1].match", null],
    ["c", "#switch_statement.beginCaptures[5].patterns[0].match", null],
    ["c", "#switch_statement.beginCaptures[5].patterns[1].match", null],
    ["cpp", "#access_control_keywords.captures[2].patterns[0].match", null, "PragmaMarkCap3At0"],
    ["cpp", "#control_flow_keywords.captures[2].patterns[0].match", null, "PragmaMarkCap3At0"],
    ["cpp", "#exception_keywords.captures[2].patterns[0].match", null, "PragmaMarkCap3At0"],
    ["cpp", "#goto_statement.captures[2].patterns[0].match", null, "PragmaMarkCap3At0"],
    ["cpp", "#label.captures[2].patterns[0].match", null, "PragmaMarkCap3At0"],
    ["cpp", "#member_access.captures[2].patterns[0].match", null, "PragmaMarkCap3At0"],
    ["cpp", "#memory_operators.captures[2].patterns[0].match", null, "PragmaMarkCap3At0"],
    ["cpp", "#misc_keywords.captures[2].patterns[0].match", null, "PragmaMarkCap3At0"],
    ["cpp", "#type_casting_operators.captures[2].patterns[0].match", null, "PragmaMarkCap3At0"],
    ["cpp", "#typename.captures[3].patterns[0].match", null, "PragmaMarkCap3At0"],
  ]],
  [repeatsFirst, [
    ["c", "#block_comment.patterns[1].begin", null],
    ["c", "#line_comment.patterns[1].begin", null],
  ]],
  [beginNeverMatches, [
    ["c", "#block_comment.patterns[1].end", null],
    ["c", "#line_comment.patterns[1].end", null],
  ]],
  [blanks, [
    ["c", "#comments.patterns[0].patterns[0].patterns[6].begin", [1]],
  ]],
  [inlineEquivalent, [
    ["c", "#inline_comment.patterns[0].patterns[1].match", null],
    ["c", "#inline_comment.patterns[1].match", null],
  ]],
  [scansNeverMatching, [
    ["c", "#inline_comment.patterns[0].patterns[1].captures[3].patterns[0].match", null],
    ["c", "#inline_comment.patterns[0].patterns[1].captures[3].patterns[1].match", null],
    ["c", "#inline_comment.patterns[1].captures[3].patterns[0].match", null],
    ["c", "#inline_comment.patterns[1].captures[3].patterns[1].match", null],
  ]],
  [missing(2), [
    ["c", "#preprocessor-rule-enabled.patterns[0].begin", [3]],
  ]],
  [tailBegin, [
    ["c", "#switch_statement.patterns[2].begin", null],
    ["cpp-macro", "#constructor_inline.patterns[2].begin", null, "ConstructorRootAt2"],
    ["cpp-macro", "#constructor_root.patterns[2].begin", null, "ConstructorRootAt2"],
    ["cpp-macro", "#destructor_inline.patterns[2].begin", null, "ConstructorRootAt2"],
    ["cpp-macro", "#destructor_root.patterns[2].begin", null, "ConstructorRootAt2"],
    ["cpp-macro", "#function_definition.patterns[2].begin", null, "ConstructorRootAt2"],
    ["cpp-macro", "#namespace_block.patterns[2].begin", null, "ConstructorRootAt2"],
    ["cpp-macro", "#operator_overload.patterns[2].begin", null, "ConstructorRootAt2"],
    ["cpp-macro", "#switch_statement.patterns[2].begin", null, "ConstructorRootAt2"],
    ["cpp", "#constructor_inline.patterns[2].begin", null, "ConstructorRootAt2"],
    ["cpp", "#constructor_root.patterns[2].begin", null, "ConstructorRootAt2"],
    ["cpp", "#destructor_inline.patterns[2].begin", null, "ConstructorRootAt2"],
    ["cpp", "#destructor_root.patterns[2].begin", null, "ConstructorRootAt2"],
    ["cpp", "#function_definition.patterns[2].begin", null, "ConstructorRootAt2"],
    ["cpp", "#namespace_block.patterns[2].begin", null, "ConstructorRootAt2"],
    ["cpp", "#operator_overload.patterns[2].begin", null, "ConstructorRootAt2"],
    ["cpp", "#switch_statement.patterns[2].begin", null, "ConstructorRootAt2"],
  ]],
  [tailEnd, [
    ["c", "#switch_statement.patterns[2].end", null],
    ["cpp-macro", "#constructor_inline.patterns[2].end", null, "MacroConstructorInlineAt2End"],
    ["cpp-macro", "#constructor_root.patterns[2].end", null, "MacroConstructorInlineAt2End"],
    ["cpp-macro", "#destructor_inline.patterns[2].end", null, "MacroConstructorInlineAt2End"],
    ["cpp-macro", "#destructor_root.patterns[2].end", null, "MacroConstructorInlineAt2End"],
    ["cpp-macro", "#function_definition.patterns[2].end", null, "MacroConstructorInlineAt2End"],
    ["cpp-macro", "#namespace_block.patterns[2].end", null, "MacroConstructorInlineAt2End"],
    ["cpp-macro", "#operator_overload.patterns[2].end", null, "MacroConstructorInlineAt2End"],
    ["cpp-macro", "#switch_statement.patterns[2].end", null, "MacroConstructorInlineAt2End"],
    ["cpp", "#constructor_inline.patterns[2].end", null, "ConstructorRootAt2End"],
    ["cpp", "#constructor_root.patterns[2].end", null, "ConstructorRootAt2End"],
    ["cpp", "#destructor_inline.patterns[2].end", null, "ConstructorRootAt2End"],
    ["cpp", "#destructor_root.patterns[2].end", null, "ConstructorRootAt2End"],
    ["cpp", "#function_definition.patterns[2].end", null, "ConstructorRootAt2End"],
    ["cpp", "#namespace_block.patterns[2].end", null, "ConstructorRootAt2End"],
    ["cpp", "#operator_overload.patterns[2].end", null, "ConstructorRootAt2End"],
    ["cpp", "#switch_statement.patterns[2].end", null, "ConstructorRootAt2End"],
  ]],
  [macroAsmLine, [
    ["cpp-macro", "#assembly.patterns[0].match", null, "AssemblyAt0"],
  ]],
  [adjacentGap, [
    ["cpp-macro", "#class_block.begin", [16, 17, 18, 19], "TypedefClassAt0"],
    ["cpp-macro", "#operator_overload.begin", [41, 42, 43, 44], "OperatorOverload"],
    ["cpp-macro", "#parameter.patterns[5].match", [7, 8, 9, 10], "ParameterAt5"],
    ["cpp-macro", "#parameter_or_maybe_value.patterns[7].match", [7, 8, 9, 10], "ParameterAt5"],
    ["cpp-macro", "#struct_block.begin", [16, 17, 18, 19], "TypedefStructAt0"],
    ["cpp-macro", "#typedef_class.patterns[0].begin", [16, 17, 18, 19], "TypedefClassAt0"],
    ["cpp-macro", "#typedef_struct.patterns[0].begin", [16, 17, 18, 19], "TypedefStructAt0"],
    ["cpp-macro", "#typedef_union.patterns[0].begin", [16, 17, 18, 19], "TypedefUnionAt0"],
    ["cpp-macro", "#union_block.begin", [16, 17, 18, 19], "TypedefUnionAt0"],
    ["cpp", "#class_block.begin", [16, 17, 18, 19], "TypedefClassAt0"],
    ["cpp", "#operator_overload.begin", [41, 42, 43, 44], "OperatorOverload"],
    ["cpp", "#over_qualified_types.patterns[0].match", [8, 9], "OverQualifiedTypesAt0"],
    ["cpp", "#over_qualified_types.patterns[1].match", [8, 9], "OverQualifiedTypesAt1"],
    ["cpp", "#over_qualified_types.patterns[2].match", [8, 9], "OverQualifiedTypesAt2"],
    ["cpp", "#over_qualified_types.patterns[3].match", [8, 9], "OverQualifiedTypesAt3"],
    ["cpp", "#parameter.patterns[5].match", [7, 8, 9, 10], "ParameterAt5"],
    ["cpp", "#parameter_or_maybe_value.patterns[7].match", [7, 8, 9, 10], "ParameterAt5"],
    ["cpp", "#struct_block.begin", [16, 17, 18, 19], "TypedefStructAt0"],
    ["cpp", "#typedef_class.patterns[0].begin", [16, 17, 18, 19], "TypedefClassAt0"],
    ["cpp", "#typedef_struct.patterns[0].begin", [16, 17, 18, 19], "TypedefStructAt0"],
    ["cpp", "#typedef_union.patterns[0].begin", [16, 17, 18, 19], "TypedefUnionAt0"],
    ["cpp", "#union_block.begin", [16, 17, 18, 19], "TypedefUnionAt0"],
  ]],
  [blanksLookahead, [
    ["cpp-macro", "#function_pointer.end", [11, 12, 13, 14], "MacroFunctionPointerParameterEnd"],
    ["cpp-macro", "#function_pointer_parameter.end", [11, 12, 13, 14], "MacroFunctionPointerParameterEnd"],
    ["cpp", "#function_pointer.end", [11, 12, 13, 14], "FunctionPointerParameterEnd"],
    ["cpp", "#function_pointer_parameter.end", [11, 12, 13, 14], "FunctionPointerParameterEnd"],
  ]],
  [negativeLookahead, [
    ["cpp-macro", "#lambdas.begin", [5, 6, 7, 8], "Lambdas"],
    ["cpp", "#lambdas.begin", [5, 6, 7, 8], "Lambdas"],
  ]],
  [leadingGapChain, [
    ["cpp-macro", "#method_access.begin", [1, 2, 3, 4], "MethodAccess"],
    ["cpp", "#member_access.match", [1, 2], "MemberAccess"],
    ["cpp", "#method_access.begin", [1, 2, 3, 4], "MethodAccess"],
  ]],
  [chainNoComment, [
    ["cpp-macro", "#method_access.beginCaptures[13].patterns[0].match", [1, 2, 3, 4], "MethodAccessCap13At0"],
    ["cpp", "#member_access.captures[11].patterns[0].match", [1, 2, 3, 4], "MethodAccessCap13At0"],
    ["cpp", "#method_access.beginCaptures[13].patterns[0].match", [1, 2, 3, 4], "MethodAccessCap13At0"],
  ]],
  [chainLookbehind, [
    ["cpp-macro", "#method_access.beginCaptures[13].patterns[1].match", null, "MethodAccessCap13At1"],
    ["cpp", "#member_access.captures[11].patterns[1].match", null, "MethodAccessCap13At1"],
    ["cpp", "#method_access.beginCaptures[13].patterns[1].match", null, "MethodAccessCap13At1"],
  ]],
  [operatorShadow, [
    ["cpp-macro", "#operators.patterns[12].match", [2], "OperatorsAt12"],
    ["cpp", "#operators.patterns[12].match", [2], "OperatorsAt12"],
  ]],
  [exceptionShadow, [
    ["cpp", "#exception_keywords.match", null, "ExceptionKeywords"],
  ]],
  [scansAdjacent, [
    ["cpp", "#over_qualified_types.patterns[0].captures[9].patterns[0].match", null, "PragmaMarkCap3At0"],
    ["cpp", "#over_qualified_types.patterns[1].captures[9].patterns[0].match", null, "PragmaMarkCap3At0"],
    ["cpp", "#over_qualified_types.patterns[2].captures[9].patterns[0].match", null, "PragmaMarkCap3At0"],
    ["cpp", "#over_qualified_types.patterns[3].captures[9].patterns[0].match", null, "PragmaMarkCap3At0"],
  ]],
  [pastMatchEnd, [
    ["cpp", "#simple_constructor_call.captures[16].patterns[0].match", null, "PragmaMarkCap3At0"],
  ]],
  [escapeShadow, [
    ["cpp", "#string_escaped_char.match", null, "StringEscapesContextCAt0"],
  ]],
  [leadingGapTypename, [
    ["cpp", "#typename.match", [2, 3], "Typename"],
  ]],
];

/** A group of the alternative of Go's number pattern that starts with a
 * point, as `.5` does. Its first group is that point. */
const goLeadingPoint =
  "the alternative starts at a `.` before a digit, where `#delimiters`, before `#numeric_literals` in every list that holds it, matches `\\.(?!\\.\\.)` first, and the ellipsis rule of `#operators`, also earlier, takes a `.` that two more follow";

/** Why a GraphQL end lacks the groups its rule names: the rule gives one
 * `captures` for its begin and its end. */
const graphqlSharedCaptures = "the rule's `captures` name the begin's groups for its end too";

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
    dropped: true,
  },
  {
    grammar: "scss",
    path: "#comment_docblock.end",
    reason: "the rule's only pattern includes `source.sassdoc`, which Shiki does not load, so vscode-textmate drops the rule",
    dropped: true,
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
  { grammar: "postcss", path: "patterns[12].end", group: 0, reason: lookahead },
  {
    grammar: "postcss",
    path: "patterns[13].end",
    group: 0,
    reason: "the end matches only the line feed or a lookahead, and the oracle compares no blank",
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
      dropped: true,
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
  {
    grammar: "matlab",
    path: "#blocks.patterns[1].end",
    group: 2,
    reason: "the grammar names the group, but the expression has one group",
    probe: "ClassdefAt0End",
  },
  ...[
    ["#blocks.patterns[1].patterns[0].end", "BlocksAt1At0"],
    ["#blocks.patterns[1].patterns[1].end", "BlocksAt1At1"],
    ["#blocks.patterns[4].patterns[0].end", "BlocksAt4At0"],
    ["#blocks.patterns[4].patterns[1].end", "BlocksAt4At1"],
    ["#blocks.patterns[5].patterns[0].end", "BlocksAt5At0"],
  ].map(
    ([path, probe]): Waiver => ({
      grammar: "matlab",
      path: path as string,
      reason: "the rule has a `match`, so vscode-textmate never reads its `end`",
      probe,
    }),
  ),
  { grammar: "matlab", path: "#comment_block.begin", group: 1, reason: blanks, probe: "CommentBlock" },
  { grammar: "matlab", path: "#comments.patterns[0].begin", group: 1, reason: blanks, probe: "CommentsAt0" },
  { grammar: "matlab", path: "#comments.patterns[2].begin", group: 1, reason: blanks, probe: "CommentsAt2" },
  ...(
    [
      ["#all-balanced-env.patterns[7]", "`text.tex.markdown_latex_combined`"],
      ["#embedded-code-env.patterns[0].patterns[1]", "`source.asy`"],
      ["#embedded-code-env.patterns[0].patterns[3]", "`source.cpp.embedded.latex`"],
      ["#embedded-code-env.patterns[0].patterns[5]", "`source.gnuplot`"],
      ["#embedded-code-env.patterns[1].patterns[2]", "`source.asymptote`"],
      ["#embedded-code-env.patterns[2].patterns[2]", "`source.cpp.embedded.latex`"],
      ["#embedded-code-env.patterns[3].patterns[2]", "`source.dot`"],
      ["#embedded-code-env.patterns[4].patterns[2]", "`source.gnuplot`"],
      ["#embedded-code-env.patterns[16].patterns[0].patterns[1]", "`source.asy`"],
      ["#embedded-code-env.patterns[16].patterns[2].patterns[1]", "`source.cpp.embedded.latex`"],
      ["#embedded-code-env.patterns[16].patterns[4].patterns[1]", "`source.gnuplot`"],
      ["#inline-code-macro.patterns[6].patterns[1]", "`source.asy`"],
      ["#inline-code-macro.patterns[8].patterns[1]", "`source.cpp.embedded.latex`"],
      ["#inline-code-macro.patterns[10].patterns[1]", "`source.gnuplot`"],
    ] as const
  ).flatMap(([rule, included]) =>
    ["begin", "end"].map(
      (part): Waiver => ({
        grammar: "latex",
        path: `${rule}.${part}`,
        reason: `the rule's only pattern includes ${included}, which Shiki does not load with LaTeX, so vscode-textmate drops the rule`,
        dropped: true,
      }),
    ),
  ),
  {
    grammar: "latex",
    path: "#documentclass-usepackage-macro.end",
    group: 0,
    reason: "the end is the lookbehind `(?<=})`, so its match holds no text",
    probe: "DocumentclassUsepackageMacroEnd",
  },
  {
    grammar: "latex",
    path: "#embedded-code-env.patterns[16].patterns[3].begin",
    reason: "`c(?:|pp)`, earlier in the same list, matches the `c` that starts every `css`, so this begin never wins",
    probe: "EmbeddedCodeEnvAt16At3",
  },
  ...(
    [
      ["#embedded-code-env.patterns[16].patterns[3].end", "EmbeddedCodeEnvAt16At0End"],
      ["#embedded-code-env.patterns[16].patterns[3].patterns[0].begin", "EmbeddedCodeEnvAt16At0At0"],
      ["#embedded-code-env.patterns[16].patterns[3].patterns[0].end", "EmbeddedCodeEnvAt16At0At0End"],
      ["#embedded-code-env.patterns[16].patterns[3].patterns[1].begin", "EmbeddedCodeEnvAt16At1At1"],
      ["#embedded-code-env.patterns[16].patterns[3].patterns[1].end", "EmbeddedCodeEnvAt16At1At1End"],
    ] as const
  ).map(
    ([path, probe]): Waiver => ({
      grammar: "latex",
      path,
      reason: "its rule, the `css` one, never opens, so it is never tried",
      probe,
    }),
  ),
  ...[1, 2].map(
    (group): Waiver => ({
      grammar: "latex",
      path: "#hyperref-macro.patterns[0].patterns[2].begin",
      group,
      reason: "the pattern before it matches wherever this optional group can, so the group never takes part",
      probe: "HyperrefMacroAt0At2",
    }),
  ),
  ...(
    [
      ["#songs-env.patterns[1].begin", "SongsEnvAt1"],
      ["#songs-env.patterns[1].end", "SongsEnvAt1End"],
    ] as const
  ).flatMap(([path, probe]) =>
    [3, 4].map((group): Waiver => ({ grammar: "latex", path, group, reason: missing(2), probe })),
  ),
  ...[2, 3, 4, 5, 6].map(
    (group): Waiver => ({
      grammar: "latex",
      path: "#inline-code-macro.patterns[0].end",
      group,
      reason: missing(1),
      probe: "InlineCodeMacroAt0End",
    }),
  ),
  { grammar: "tex", path: "#comment.begin", group: 1, reason: blanks, probe: "Comment" },
  {
    grammar: "tex",
    path: "#comment.patterns[1].begin",
    reason: "`%:?`, earlier in the same list, matches at the same `%`, so this begin never wins",
    probe: "CommentAt1",
  },
  {
    grammar: "tex",
    path: "#comment.patterns[1].end",
    reason: "its rule never opens, so its end is never tried",
    probe: "CommentAt0End",
  },
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
  {
    grammar: "svelte",
    path: "injections[L:meta.style.svelte meta.lang.stylus - (meta source)].patterns[0].begin",
    reason: "the rule's only pattern includes `source.stylus`, which Shiki does not load, so vscode-textmate drops the rule",
    dropped: true,
  },
  {
    grammar: "svelte",
    path: "injections[L:meta.style.svelte meta.lang.stylus - (meta source)].patterns[0].end",
    reason: "the rule's only pattern includes `source.stylus`, which Shiki does not load, so vscode-textmate drops the rule",
    dropped: true,
  },
  {
    grammar: "svelte",
    path: "injections[L:meta.template.svelte meta.lang.pug - (meta source)].patterns[0].begin",
    reason: "the rule's only pattern includes `text.pug`, which Shiki does not load, so vscode-textmate drops the rule",
    dropped: true,
  },
  {
    grammar: "svelte",
    path: "injections[L:meta.template.svelte meta.lang.pug - (meta source)].patterns[0].end",
    reason: "the rule's only pattern includes `text.pug`, which Shiki does not load, so vscode-textmate drops the rule",
    dropped: true,
  },
  { grammar: "swift", path: "#comments.patterns[5].begin", group: 1, reason: blanks, probe: "CommentsAt5" },
  { grammar: "swift", path: "#compiler-control.patterns[1].end", group: 1, reason: missing(0), probe: "CompilerControlAt1End" },
  { grammar: "swift", path: "#compiler-control.patterns[1].end", group: 2, reason: missing(0), probe: "CompilerControlAt1End" },
  {
    grammar: "swift",
    path: "#declarations-import.patterns[0].patterns[2].captures[1].patterns[0].match",
    reason: "group 1 is a run of operator heads and combining marks, none of them `.`, so the pattern never finds one in it",
    probe: "DeclarationsImportAt0At2",
  },
  { grammar: "swift", path: "#keywords.patterns[5].match", group: 2, reason: blanks, probe: "KeywordsAt5" },
  { grammar: "swift", path: "#keywords.patterns[10].match", group: 2, reason: blanks, probe: "KeywordsAt10" },
  { grammar: "swift", path: "#literals-regular-expression-literal.patterns[2].match", group: 5, reason: lineFeed, probe: "LiteralsRegularExpressionLiteralAt2" },
  { grammar: "swift", path: "#literals-regular-expression-literal-quote.end", group: 1, reason: lineFeed, probe: "LiteralsRegularExpressionLiteralQuoteEnd" },
  ...(
    [
      ["literals-string-string-guts", "LiteralsStringStringGutsAt2"],
      ["literals-string-raw-string-guts", "LiteralsStringRawStringGutsAt2"],
    ] as const
  ).flatMap(([rule, probe]): Waiver[] => [
    {
      grammar: "swift",
      path: `#${rule}.patterns[2].patterns[1].begin`,
      reason:
        "`$self`, before it in the same list, holds `#expressions-without-trailing-closures-or-member-references-parenthesized-expression`, whose begin `\\(` matches every `(`",
      probe,
    },
    { grammar: "swift", path: `#${rule}.patterns[2].patterns[1].end`, reason: "its begin never matches", probe },
  ]),
  ...(
    [
      [0, [0, 1, 2, 3]],
      [1, [0, 1]],
      [2, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]],
      [5, [0]],
    ] as const
  ).flatMap(([rule, patterns]) =>
    patterns.map(
      (pattern): Waiver => ({
        grammar: "swift",
        path: `#operators.patterns[1].patterns[${rule}].captures[0].patterns[${pattern}].match`,
        reason:
          "vscode-textmate tokenizes a capture's patterns with no anchor, so `\\G` never matches there and the operator falls to the `.+` pattern after it",
        probe: `OperatorsAt1At${rule}`,
      }),
    ),
  ),
  { grammar: "go", path: "#generic_param_types.patterns[5].match", group: 3, reason: missing(2) },
  {
    grammar: "go",
    path: "#generic_param_types.patterns[5].captures[3].patterns[1].match",
    reason: "it scans group 3, which the expression lacks",
  },
  {
    grammar: "go",
    path: "#map_types.patterns[3].match",
    reason: "`#parameter-variable-types`, before it in its only list, opens `([*.\\w]+)?(\\[)` at every `[`",
  },
  {
    grammar: "go",
    path: "#map_types.patterns[4].match",
    reason: "the rule's end, `(])`, matches at every `]` the rules inside it leave, and an end is tried before the patterns",
  },
  ...[5, 6, 7].map(
    (pattern): Waiver => ({
      grammar: "go",
      path: `#map_types.patterns[${pattern}].match`,
      reason: "`#parameter-variable-types`, before it in its only list, has the same expression",
    }),
  ),
  ...[18, 19, 20, 21, 22, 23, 24, 25].map(
    (group): Waiver => ({
      grammar: "go",
      path: "#numeric_literals.captures[0].patterns[0].patterns[0].match",
      group,
      reason: goLeadingPoint,
    }),
  ),
  ...[19, 24].map(
    (group): Waiver => ({
      grammar: "go",
      path: `#numeric_literals.captures[0].patterns[0].patterns[0].captures[${group}].patterns[0].match`,
      reason: goLeadingPoint,
    }),
  ),
  ...[3, 4, 5, 6, 7, 8, 9].map(
    (pattern): Waiver => ({
      grammar: "go",
      path: `#single_type.patterns[1].patterns[${pattern}].match`,
      reason:
        "the begin stops before the `[` its lookahead requires, where `#parameter-variable-types`, earlier in the list, opens `([*.\\w]+)?(\\[)` and runs to the matching `]`; right after it the end, `(?<=])` with an optional group, matches, and an end is tried before the patterns",
    }),
  ),
  {
    grammar: "go",
    path: "#support_functions.captures[2].patterns[1].match",
    reason: "`#type-declarations`, before it in the list, holds `#numeric_literals`, whose `(?<!\\w)\\.?\\d` matches at every digit that starts the group",
  },
  {
    grammar: "go",
    path: "#switch_types.beginCaptures[3].patterns[2].match",
    reason: "group 3 is `.(type)`, and `#brackets`, before it in the list, opens `\\(` at its `(` and closes after `type`",
  },
  { grammar: "go", path: "#syntax_errors.patterns[0].match", group: 1, reason: blanks },
  ...[1, 2].map(
    (pattern): Waiver => ({
      grammar: "go",
      path: `#type_assertion_inline.captures[2].patterns[${pattern}].match`,
      reason: "group 2 holds only blanks, brackets, `*`, `.`, `<-`, braces, commas and words, never `(` or `)`",
    }),
  ),
  { grammar: "ruby", path: "patterns[21].end", group: 1, reason: missing(0) },
  { grammar: "ruby", path: "patterns[86].begin", group: 1, reason: blanks },
  {
    grammar: "ruby",
    path: "patterns[88].begin",
    reason: "`patterns[29]` (`\\b[A-Z_]\\w*\\b`), earlier in the same list, takes `__END__` as a constant at the same place",
  },
  { grammar: "ruby", path: "patterns[88].end", reason: "its begin never matches" },
  { grammar: "ruby", path: "patterns[88].patterns[0].begin", reason: "the rule that holds it never opens" },
  { grammar: "ruby", path: "patterns[88].patterns[0].end", reason: "the rule that holds it never opens" },
  {
    grammar: "ruby",
    path: "#nest_brackets_r.begin",
    reason: "`#regex_sub`, before it in every list that holds it, opens a character class at every `[`",
  },
  { grammar: "ruby", path: "#nest_brackets_r.end", reason: "its begin never matches" },
  {
    grammar: "ruby",
    path: "#nest_parens_r.begin",
    reason: "`#regex_sub`, before it in every list that holds it, opens a group at every `(`",
  },
  { grammar: "ruby", path: "#nest_parens_r.end", reason: "its begin never matches" },
  { grammar: "ruby", path: "#regex_sub.patterns[6].end", group: 0, reason: lineFeed },
  ...cFamily.flatMap(([reason, rows]) =>
    rows.flatMap(([grammar, path, groups, probe]): Waiver[] => {
      const waiver: Waiver = { grammar, path, reason, ...(probe === undefined ? {} : { probe }) };
      return groups === null ? [waiver] : groups.map((group) => ({ ...waiver, group }));
    }),
  ),
  { grammar: "lua", path: "patterns[3].match", reason: "`patterns[1]`, the hexadecimal float before it, matches every hexadecimal integer this does", probe: "RootAt3" },
  { grammar: "lua", path: "patterns[6].match", reason: "`patterns[4]`, the decimal float before it, matches every integer this does", probe: "RootAt6" },
  {
    grammar: "lua",
    path: "patterns[17].match",
    reason: "`patterns[0]`, the function rule before it, begins with `(function)` under the same lookahead, so it matches wherever this does",
    probe: "RootAt17",
  },
  { grammar: "lua", path: "#comment.patterns[0].begin", group: 1, reason: blanks, probe: "CommentAt0" },
  { grammar: "lua", path: "#comment.patterns[0].end", group: 1, reason: "the group holds only blanks and the line feed, and the oracle compares no blank", probe: "CommentAt0End" },
  {
    grammar: "lua",
    path: "#emmydoc.patterns[18].patterns[1].match",
    reason: "the `@see` rule's end, `(?=[\\n#@])`, matches at every `#` and is tried first",
    probe: "EmmydocAt18At1",
  },
  {
    grammar: "lua",
    path: "#emmydoc.type.patterns[4].patterns[1].match",
    reason: "`[-\\]*,.0-9<>A-\\[_a-z]+`, the pattern before it, takes every `.` of a `...`",
    probe: "EmmydocTypeAt4At1",
  },
  { grammar: "diff", path: "patterns[11].end", group: 1, reason: missing(0) },
  {
    grammar: "docker",
    path: "patterns[2].match",
    reason: "`patterns[1]`, before it, lists `CMD` and `ENTRYPOINT` after the same optional `ONBUILD`, so it matches wherever this does",
  },
  ...["patterns[4].begin", "patterns[4].end", "patterns[5].begin", "patterns[5].end"].map(
    (path): Waiver => ({ grammar: "docker", path, group: 1, reason: missing(0) }),
  ),
  { grammar: "docker", path: "patterns[6].match", group: 1, reason: blanks },
  { grammar: "ini", path: "patterns[0].begin", group: 1, reason: blanks },
  { grammar: "ini", path: "patterns[1].begin", group: 1, reason: blanks },
  { grammar: "make", path: "#comment.begin", group: 1, reason: blanks },
  ...[2, 3, 4].map(
    (group): Waiver => ({ grammar: "make", path: "#directives.patterns[2].end", group, reason: missing(1) }),
  ),
  { grammar: "java", path: "#comments-inline.patterns[1].begin", group: 1, reason: blanks },
  {
    grammar: "dart",
    path: "#operators.patterns[3].match",
    reason: "`#operators.patterns[2]`, `(<<|>>>?|[\\&^|~])`, before it, matches at the start of every one of its operators",
  },
  {
    grammar: "julia",
    path: "#operator.patterns[12].match",
    reason: "`#operator.patterns[11]`, before it, holds `∘` in its class `[…∗-∜…]`",
  },
  { grammar: "nix", path: "#if.patterns[2].end", group: 0, reason: lookahead },
  ...["begin", "end"].map(
    (key): Waiver => ({
      grammar: "nix",
      path: `#attrset-or-function.patterns[3].patterns[3].patterns[1].${key}`,
      reason: "the enclosing rule's end, `(?=([]),;}]|\\b(else|then)\\b))`, matches before every `,`, and an end is tried before the patterns",
    }),
  ),
  {
    grammar: "zig",
    path: "#numbers.patterns[2].match",
    reason: "`#numbers.patterns[1]`, before it, matches every one of its matches, since its fraction and exponent are optional",
  },
  {
    grammar: "zig",
    path: "#operators.patterns[4].match",
    reason: "the comparison and arithmetic rules before it match the first byte of `==`, `++`, `**` and `->`",
  },
  { grammar: "powershell", path: "#commentLine.end", group: 1, reason: missing(0) },
  ...[1, 2, 3].map(
    (index): Waiver => ({
      grammar: "powershell",
      path: `#variableNoProperty.patterns[${index}].match`,
      group: 3,
      reason: missing(2),
    }),
  ),
  ...[4, 6].map(
    (index): Waiver => ({
      grammar: "powershell",
      path: `#variableNoProperty.patterns[${index}].match`,
      group: 4,
      reason: missing(3),
    }),
  ),
  {
    grammar: "kotlin",
    path: "#operators.patterns[5].match",
    reason: "`#operators.patterns[3]`, `([-%*+/])`, before it, takes the first character of every `--` and `++`",
  },
  {
    grammar: "kotlin",
    path: "#type-parameter.patterns[1].match",
    reason: "`#type-parameter.patterns[0]`, `\\b\\w+\\b`, before it, matches every word this does",
  },
  ...[0, 1].flatMap((index): Waiver[] => [
    {
      grammar: "erb",
      path: `injections[text.html.erb - (meta.embedded.block.erb | meta.embedded.line.erb | comment)].patterns[${index}].begin`,
      group: 0,
      reason: "the match is only the blanks that lead the line, before the lookahead, and the oracle compares no blank",
    },
    {
      grammar: "erb",
      path: `injections[text.html.erb - (meta.embedded.block.erb | meta.embedded.line.erb | comment)].patterns[${index}].end`,
      group: 0,
      reason: "the match is only blanks and the line feed that end the line, and the oracle compares no blank",
    },
  ]),
  ...[
    { path: "patterns[0].end", groups: [2] },
    { path: "patterns[1].end", groups: [2, 3] },
    { path: "patterns[4].end", groups: [2, 3, 4, 5] },
    { path: "#EntityDecl.end", groups: [2, 3, 4, 5] },
  ].flatMap(({ path, groups }) =>
    groups.map(
      (group): Waiver => ({
        grammar: "xml",
        path,
        group,
        reason: "the rule's `captures` name the group for its end too, and the end's expression has one group",
      }),
    ),
  ),
  { grammar: "graphql", path: "#graphql-comment.patterns[0].match", group: 1, reason: blanks },
  {
    grammar: "graphql",
    path: "#graphql-description-docstring.begin",
    reason: "`#graphql-comment`, before it in every list that holds it, opens `(\"\"\")` at every `\"\"\"`",
  },
  {
    grammar: "graphql",
    path: "#graphql-description-docstring.end",
    reason: "its begin never matches",
  },
  {
    grammar: "graphql",
    path: "#graphql-description-singleline.match",
    reason: "`#graphql-comment`, before it in every list that holds it, matches `(\\s*)(#).*` at or before every `#`",
  },
  ...[
    {
      path: "#graphql-directive-definition.begin",
      groups: [3, 4],
      why: "`beginCaptures` name the groups of the `on` rule inside it",
      has: 2,
    },
    { path: "#graphql-fragment-definition.end", groups: [1, 2, 3, 4], why: graphqlSharedCaptures, has: 0 },
    { path: "#graphql-fragment-spread.end", groups: [1, 2], why: graphqlSharedCaptures, has: 0 },
    { path: "#graphql-inline-fragment.end", groups: [1, 2, 3], why: graphqlSharedCaptures, has: 0 },
    {
      path: "#graphql-input-types.patterns[2].begin",
      groups: [2],
      why: "the rule's `captures` name its end's `!` group for its begin too",
      has: 1,
    },
    { path: "#graphql-union.end", groups: [1, 2], why: graphqlSharedCaptures, has: 0 },
    { path: "#graphql-union.patterns[0].end", groups: [1, 2], why: graphqlSharedCaptures, has: 0 },
  ].flatMap(({ path, groups, why, has }) =>
    groups.map((group): Waiver => ({ grammar: "graphql", path, group, reason: `${why}, and ${missing(has)}` })),
  ),
  {
    grammar: "graphql",
    path: "#graphql-string-value.end",
    group: 3,
    reason: "the possessive `\\s*+` before it takes the line feed, so `(\\n)` never matches",
  },
  {
    grammar: "graphql",
    path: "#graphql-type-interface.end",
    reason: "the rule's last pattern, `#graphql-ignore-spaces`, matches `\\s*` at every place, and the rule tries its end after its patterns",
  },
  {
    grammar: "sql",
    path: "#comments.patterns[0].begin",
    group: 1,
    reason: blanks,
  },
  {
    grammar: "sql",
    path: "#comments.patterns[1].begin",
    group: 1,
    reason: blanks,
  },
  {
    grammar: "sql",
    path: "#regexps.patterns[0].begin",
    reason: "`patterns[21]`, `[-+/]`, before `#regexps` in the only list that holds it, matches every `/`",
  },
  {
    grammar: "sql",
    path: "#regexps.patterns[0].end",
    reason: "its begin never matches",
  },
  {
    grammar: "sql",
    path: "#regexps.patterns[0].patterns[1].match",
    reason: "its rule's begin never matches",
  },
  ...[
    { path: "#alias_statement.begin", group: 7 },
    { path: "#array_value.begin", group: 4 },
    { path: "#modified_assignment_statement.patterns[2].match", group: 4 },
    { path: "#normal_assignment_statement.begin", group: 4 },
  ].map(
    ({ path, group }): Waiver => ({
      grammar: "shellscript",
      path,
      group,
      reason: "the alternative before it in the index, `\\$?(?<!\\w)[-0-9A-Z_a-z]+(?!\\w)`, matches every text this group's `-?\\d+` can",
    }),
  ),
  { grammar: "shellscript", path: "#interpolation.patterns[2].patterns[1].begin", group: 1, reason: missing(0) },
  {
    grammar: "shellscript",
    path: "#loop.patterns[0].begin",
    reason: "`#for_statement`, before `#loop` in `#initial_context`, matches `\\bfor\\b` wherever this does",
  },
  {
    grammar: "shellscript",
    path: "#loop.patterns[0].end",
    reason: "its begin never matches, since `#for_statement`, before `#loop` in `#initial_context`, takes every `for`",
  },
  {
    grammar: "shellscript",
    path: "#normal_statement_seperator.match",
    group: 3,
    reason: "`#pipeline`, before it in every context that includes both, takes the first `|` of every `||` with `[!|]`",
  },
  { grammar: "shellscript", path: "#option.begin", group: 2, reason: lookahead },
  ...[1, 2, 3].map((group): Waiver => ({ grammar: "sass", path: "patterns[2].end", group, reason: missing(0) })),
  { grammar: "sass", path: "patterns[9].end", group: 0, reason: "the end matches only the line feed, or nothing before a lookahead, and the oracle compares no blank" },
  { grammar: "sass", path: "patterns[17].end", group: 0, reason: lookahead },
  { grammar: "sass", path: "patterns[18].end", group: 0, reason: lookahead },
  ...[1, 2].map((group): Waiver => ({ grammar: "sass", path: "patterns[19].end", group, reason: missing(0) })),
  { grammar: "sass", path: "patterns[20].end", group: 0, reason: "the end matches only the line feed, or nothing before a lookahead, and the oracle compares no blank" },
  { grammar: "scala", path: "#comments.patterns[1].begin", group: 1, reason: blanks },
  ...["#exports.patterns[5].patterns[2].match", "#imports.patterns[6].patterns[2].match"].map(
    (path): Waiver => ({
      grammar: "scala",
      path,
      group: 1,
      reason: "`\\b(given)\\b`, before it in the same list, matches at every `given` that would start the group",
    }),
  ),
  {
    grammar: "scala",
    path: "#meta-bounds.match",
    reason: "`#keywords`, before it in `#code`, takes every run of two or more operator characters, and each bound is such a run",
  },
  {
    grammar: "scala",
    path: "#meta-brackets.patterns[0].match",
    reason: "`#curly-braces`, before it in `#code`, opens at every `{`",
  },
  ...[2, 3, 4, 5].map(
    (group): Waiver => ({
      grammar: "scala",
      path: "#xml-literal.patterns[1].end",
      group,
      reason: "the rule's `captures` name the begin's groups for its end too, and the end's expression has one group",
    }),
  ),
  { grammar: "astro", path: "#entities.patterns[1].match", group: 3, reason: missing(2) },
  { grammar: "astro", path: "#entities.patterns[2].match", group: 3, reason: missing(2) },
  {
    grammar: "astro",
    path: "injections[L:meta.style.astro meta.lang.stylus - (meta source)].patterns[0].begin",
    reason: "the rule's only pattern includes `source.stylus`, which Shiki does not load, so vscode-textmate drops the rule",
    dropped: true,
  },
  {
    grammar: "astro",
    path: "injections[L:meta.style.astro meta.lang.stylus - (meta source)].patterns[0].end",
    reason: "the rule's only pattern includes `source.stylus`, which Shiki does not load, so vscode-textmate drops the rule",
    dropped: true,
  },
  { grammar: "bat", path: "#strings.patterns[0].end", group: 2, reason: lineFeed },
  ...["#mermaid.patterns[1].patterns[9]", "#mermaid.patterns[2].patterns[11]", "#mermaid.patterns[3].patterns[9]", "#mermaid.patterns[4].patterns[11]"].map(
    (rule): Waiver => ({
      grammar: "mermaid",
      path: `${rule}.match`,
      reason: "the boundary's long form, the pattern before it, makes every part after the word optional, so it matches wherever this one does",
    }),
  ),
  {
    grammar: "mermaid",
    path: "#mermaid.patterns[1].patterns[15].match",
    reason: "it is the same expression as `patterns[14]`, the pattern before it, which matches first",
  },
  { grammar: "mermaid", path: "#mermaid.patterns[9].patterns[5].begin", group: 3, reason: missing(2) },
  ...["#mermaid.patterns[9].patterns[5]", "#mermaid.patterns[9].patterns[6]"].map(
    (rule): Waiver => ({
      grammar: "mermaid",
      path: `${rule}.patterns[0].patterns[0].patterns[0].match`,
      group: 1,
      reason:
        "the rule's begin `([^\"]*)` takes the text up to the quote, where its end `(?=\")` holds, or the rest of the line; Shiki lexes the next line outside the rule, so this match only matches empty",
    }),
  ),
  ...["patterns[8].begin", "patterns[8].end"].map(
    (path): Waiver => ({
      grammar: "coffee",
      path,
      reason: "the regular expression rule includes only `source.js.regexp`, which no grammar provides, so vscode-textmate skips it",
    }),
  ),
  ...["patterns[27].match", "patterns[29].match", "patterns[30].match", "patterns[31].match", "patterns[32].match"].map(
    (path): Waiver => ({
      grammar: "coffee",
      path,
      reason:
        "the method must follow `console.`, `Array.`, `Object.`, `Math.` or `Number.`, and a property or call match takes that `.` first, since no match ends at it",
    }),
  ),
  { grammar: "elixir", path: "patterns[7].begin", group: 1, reason: lookahead },
  { grammar: "elixir", path: "#core_syntax.patterns[21].match", group: 1, reason: missing(0) },
  { grammar: "elixir", path: "#core_syntax.patterns[47].begin", group: 1, reason: blanks },
  { grammar: "elixir", path: "#core_syntax.patterns[48].begin", group: 1, reason: blanks },
  {
    grammar: "gdscript",
    path: "#annotated_parameter.patterns[1].match",
    reason: "`#expression`, before it in its only list, holds `#assignment_operator`, whose `=` matches at every `=` this does",
  },
  {
    grammar: "gdscript",
    path: "#any_method.match",
    reason: "`#function_call` has the same test, `(?=\\b[A-Z_a-z]\\w*\\b\\s*\\()`, and `#base_expression` holds it before it in `#expression`, its only list",
  },
  { grammar: "gdscript", path: "#builtin_get_node_shorthand_bare.match", group: 4, reason: missing(3) },
  { grammar: "gdscript", path: "#class_declaration.match", group: 2, reason: missing(1) },
  { grammar: "gdscript", path: "#class_new.match", group: 3, reason: missing(2) },
  { grammar: "gdscript", path: "#in_keyword.patterns[0].end", group: 1, reason: missing(0) },
  { grammar: "gdscript", path: "#lambda_declaration.begin", group: 2, reason: missing(1) },
  {
    grammar: "gdscript",
    path: "#statement_keyword.patterns[1].match",
    reason: "`#keywords`, before it in `#base_expression`, its only list, matches `class` with the same `\\b` on each side",
  },
  { grammar: "pascal", path: "patterns[12].begin", group: 1, reason: blanks },
  {
    grammar: "pascal",
    path: "patterns[15].patterns[0].match",
    reason: "the string's end `'` is tried before its patterns at every quote, so `''` ends the string and the next quote starts another",
  },
  ...[1, 2].flatMap((group): Waiver[] => [
    { grammar: "vb", path: "patterns[2].patterns[0].match", group, reason: blanks },
    { grammar: "vb", path: "patterns[3].patterns[0].match", group, reason: blanks },
  ]),
  { grammar: "vb", path: "patterns[5].begin", group: 1, reason: blanks },
  {
    grammar: "vb",
    path: "patterns[12].patterns[0].match",
    reason: "the string's end `\"` is tried before its patterns at every quote, so `\"\"` ends the string and the next quote starts another",
  },
  ...["patterns[5]", "patterns[10]", "patterns[14]"].flatMap((rule) =>
    ["begin", "end"].map(
      (part): Waiver => ({
        grammar: "haml",
        path: `${rule}.${part}`,
        reason: "the first `:ruby` filter, `^(\\s*):ruby`, needs nothing after the name, so it always matches first",
      }),
    ),
  ),
  ...["patterns[13]", "patterns[17]", "patterns[20]", "patterns[24]"].flatMap((rule) =>
    ["begin", "end"].map(
      (part): Waiver => ({
        grammar: "haml",
        path: `${rule}.${part}`,
        reason: "the rule's only pattern includes a grammar that Shiki's `haml` does not load, so vscode-textmate drops the rule",
      }),
    ),
  ),
  ...["begin", "end"].map(
    (part): Waiver => ({
      grammar: "haml",
      path: `patterns[23].${part}`,
      reason: "the `:sass` filter before it, `^(\\s*):(s(?:tyle|ass))$`, matches the same lines first",
    }),
  ),
  { grammar: "haml", path: "patterns[6].match", group: 1, reason: blanks },
  ...[1, 2, 3].map((group): Waiver => ({ grammar: "haml", path: "patterns[8].end", group, reason: missing(0) })),
  { grammar: "haml", path: "patterns[21].end", group: 2, reason: missing(0) },
  { grammar: "haml", path: "patterns[22].end", group: 2, reason: missing(0) },
  {
    grammar: "haml",
    path: "patterns[18].match",
    reason: "the tag rule before it, `^\\s*(?:((%)([-:\\w]+))|(?=[#.]))`, opens at the same `.` first",
  },
  {
    grammar: "haml",
    path: "#continuation.match",
    reason: "every rule that lists it lists `source.ruby` or the block opener `([,<|]|do|\\{)\\s*(#.*)?$\\n*` first, and those take the `|`",
  },
  {
    grammar: "haml",
    path: "#rubyline.patterns[4].match",
    reason: "the keyword pattern before it, `\\s+((elseif|foreach|switch|declare|default|use))(?=[(\\s])`, takes `use` before `(` or a blank first",
  },
  { grammar: "haskell", path: "#comments.patterns[1].begin", group: 1, reason: blanks },
  { grammar: "haskell", path: "#comments.patterns[3].begin", group: 1, reason: blanks },
  {
    grammar: "haskell",
    path: "patterns[17].end",
    group: 1,
    reason: "the rule's `captures` name the begin's group for its end too, and the end's expression has no group",
  },
  {
    grammar: "haskell",
    path: "#type_signature.patterns[4].match",
    reason: "it repeats `patterns[2]`, which comes earlier in the same list",
  },
  {
    grammar: "objective-c",
    path: "#c_lang.patterns[36].match",
    reason: "the root tries `#bracketed_content`, whose begin is `\\[`, before `#c_lang`, so it takes every `[` first",
    probe: "CLangAt36",
  },
  // Objective-C++ repeats Objective-C's expressions, so these hold for both.
  ...["objective-c", "objective-cpp"].flatMap((grammar): Waiver[] => [
    { grammar, path: "#anonymous_pattern_1.begin", group: 8, reason: blanks, probe: "AnonymousPattern1" },
    {
      grammar,
      path: "#anonymous_pattern_1.begin",
      group: 9,
      reason: "the grammar names the group, but the expression has eight groups",
      probe: "AnonymousPattern1",
    },
    ...[
      ["#anonymous_pattern_1.end", 4],
      ["#anonymous_pattern_1.end", 6],
      ["#anonymous_pattern_1.end", 7],
      ["#anonymous_pattern_1.end", 8],
      ["#anonymous_pattern_1.end", 9],
      ["#anonymous_pattern_2.end", 4],
      ["#anonymous_pattern_2.end", 5],
    ].map(
      ([path, group]): Waiver => ({
        grammar,
        path: path as string,
        group: group as number,
        reason: "the rule's `captures` name the begin's groups for its end too, and the end's expression has two groups",
        probe: "AnonymousPattern1End",
      }),
    ),
    { grammar, path: "#anonymous_pattern_14.patterns[0].match", group: 1, reason: missing(0), probe: "AnonymousPattern14At0" },
    { grammar, path: "#anonymous_pattern_18.match", group: 1, reason: blanks, probe: "AnonymousPattern18" },
    { grammar, path: "#anonymous_pattern_19.match", group: 1, reason: blanks, probe: "AnonymousPattern19" },
    { grammar, path: "#bracketed_content.patterns[0].begin", group: 2, reason: missing(1), probe: "BracketedContentAt0" },
    { grammar, path: "#c_functions.patterns[0].match", group: 1, reason: blanks, probe: "CFunctionsAt0" },
    { grammar, path: "#c_functions.patterns[1].match", group: 1, reason: blanks, probe: "CFunctionsAt1" },
    {
      grammar,
      path: "#c_functions.patterns[1].match",
      group: 3,
      reason: "the group sits inside a negative lookahead, so it never holds text",
      probe: "CFunctionsAt1",
    },
    ...[25, 26, 27].map(
      (index): Waiver => ({
        grammar,
        path: `#c_lang.patterns[${index}].match`,
        reason: "`#storage_types`, earlier in the same list, takes every word of this alternation first",
        probe: `CLangAt${index}`,
      }),
    ),
    { grammar, path: "#c_lang.repository.#comments.patterns[3].begin", group: 1, reason: blanks, probe: "CCommentsAt3" },
    { grammar, path: "#comment.patterns[1].begin", group: 1, reason: blanks, probe: "CCommentsAt3" },
    {
      grammar,
      path: "#c_lang.repository.#preprocessor-rule-enabled.patterns[0].begin",
      group: 3,
      reason: missing(2),
      probe: "CPreprocessorRuleEnabledAt0",
    },
    ...[
      ["begin", "CSwitchStatementAt2"],
      ["end", "CSwitchStatementAt2End"],
    ].map(
      ([part, probe]): Waiver => ({
        grammar,
        path: `#c_lang.repository.#switch_statement.patterns[2].${part}`,
        reason: "the switch's end `(?<=})` holds where this begin could, and a rule's end comes before its patterns",
        probe,
      }),
    ),
    ...["enabled-interface", "disabled-interface", "enabled-implementation", "disabled-implementation"].flatMap((rule): Waiver[] => [
      {
        grammar,
        path: `#preprocessor-rule-${rule}.end`,
        group: 3,
        reason: missing(2),
        probe: "PreprocessorRuleEnabledInterfaceEnd",
      },
      ...[1, 2].map(
        (group): Waiver => ({
          grammar,
          path: `#preprocessor-rule-${rule}.patterns[0].end`,
          group,
          reason: missing(0),
          probe: "PreprocessorRuleEnabledInterfaceAt0End",
        }),
      ),
    ]),
  ]),
  ...[
    "#bracketed_content.begin",
    "#bracketed_content.end",
    "#bracketed_content.patterns[0].begin",
    "#bracketed_content.patterns[0].end",
    "#bracketed_content.patterns[0].patterns[0].match",
    "#bracketed_content.patterns[0].patterns[1].match",
    "#bracketed_content.patterns[0].patterns[2].begin",
    "#bracketed_content.patterns[0].patterns[2].end",
    ...[0, 1, 2, 3, 4, 5, 6, 7].map((index) => `#bracketed_content.patterns[0].patterns[2].patterns[${index}].match`),
    "#bracketed_content.patterns[1].begin",
    "#bracketed_content.patterns[1].end",
    "#bracketed_content.patterns[1].patterns[0].match",
  ].map(
    (path): Waiver => ({
      grammar: "objective-cpp",
      path,
      reason: "the root reaches `#c_lang` first, through `#cpp_lang`, and its `patterns[35]` and `patterns[36]` take every `[` before `#bracketed_content`",
    }),
  ),
  ...["#c_functions.patterns[0].match", "#c_functions.patterns[1].match", "#apple_foundation_functional_macros.begin", "#apple_foundation_functional_macros.end"].map(
    (path): Waiver => ({
      grammar: "objective-cpp",
      path,
      reason: "the root reaches `#c_lang` first, through `#cpp_lang`, and its function rule `patterns[33]` takes a name before `(`, as in `sqrt(4)` and `API_AVAILABLE(ios(13.0))`",
    }),
  ),
  ...["begin", "end"].map(
    (part): Waiver => ({
      grammar: "objective-cpp",
      path: `#cpp_lang.patterns[17].${part}`,
      reason: "it repeats `patterns[16]`, which comes earlier in the same list",
    }),
  ),
  ...["begin", "end"].map(
    (part): Waiver => ({
      grammar: "objective-cpp",
      path: `#cpp_lang.repository.#constructor.patterns[0].${part}`,
      reason: "vscode-textmate drops the rule: its only patterns include `probably_a_parameter` and `function-innards`, which no repository in its reach defines",
      dropped: true,
    }),
  ),
  {
    grammar: "objective-cpp",
    path: "#cpp_lang.repository.#special_block.patterns[2].patterns[1].end",
    group: 2,
    reason: "the group matches only blanks and the line feed that ends the line, and the oracle compares no blank",
  },
  ...[0, 1, 2, 3, 4, 5, 6, 7, 8].flatMap((n): Waiver[] => [
    { grammar: "php", path: `#heredoc_interior.patterns[${n}].begin`, group: 5, reason: blanks },
    { grammar: "php", path: `#nowdoc_interior.patterns[${n}].begin`, group: 3, reason: blanks },
  ]),
  { grammar: "php", path: "#comments.patterns[2].begin", group: 1, reason: blanks },
  { grammar: "php", path: "#comments.patterns[3].begin", group: 1, reason: blanks },
  ...["#heredoc_interior.patterns[6].patterns[5].end", "#nowdoc_interior.patterns[6].patterns[4].end"].map(
    (path): Waiver => ({ grammar: "php", path, group: 0, reason: "the end is `$`, which holds no text" }),
  ),
  { grammar: "hack", path: "#comments.patterns[2].begin", group: 1, reason: blanks, probe: "CommentsAt2" },
  {
    grammar: "hack",
    path: "#language.patterns[19].begin",
    group: 5,
    reason: "the expression has four groups, so no match holds a fifth",
    probe: "LanguageAt19",
  },
  {
    grammar: "hack",
    path: "#support.patterns[101].match",
    reason: "its words, `is_int` and `is_integer`, are in `patterns[93]`, which comes first and takes them",
    probe: "SupportAt0",
  },
  { grammar: "perl", path: "patterns[17].end", reason: "vscode-textmate rewrites `\\z` so that it never matches a line, so the data section runs to the document's end", probe: "RootAt17End" },
  ...["#heredoc.patterns[5].begin", "#heredoc.patterns[5].end", "#heredoc.patterns[5].patterns[0].begin", "#heredoc.patterns[5].patterns[0].end"].map(
    (path): Waiver => ({
      grammar: "perl",
      path,
      reason: "the rule's only pattern includes `source.postscript`, which Shiki lacks, so vscode-textmate drops it, and the quoted heredoc after it takes the label",
      probe: "HeredocAt6",
    }),
  ),
  ...["#heredoc.patterns[22].begin", "#heredoc.patterns[22].end"].map(
    (path): Waiver => ({
      grammar: "perl",
      path,
      reason: "the bare heredoc before it matches first at `<<`, with an empty label, since a bare label stops at the backtick",
      probe: "HeredocAt21",
    }),
  ),
  ...[
    ["patterns[3].patterns[0].end", "RootAt3At0End"],
    ["patterns[3].patterns[1].end", "RootAt3At1End"],
    ["patterns[3].patterns[2].end", "RootAt3At2End"],
    ["patterns[3].patterns[3].end", "RootAt3At3End"],
    ["patterns[3].patterns[4].end", "RootAt3At4End"],
    ["patterns[3].patterns[5].end", "RootAt3At5End"],
    ["patterns[4].patterns[0].end", "RootAt4At0End"],
    ["patterns[4].patterns[1].end", "RootAt4At1End"],
    ["patterns[4].patterns[2].end", "RootAt4At2End"],
    ["patterns[4].patterns[3].end", "RootAt4At3End"],
    ["patterns[4].patterns[4].end", "RootAt4At4End"],
    ["patterns[4].patterns[5].end", "RootAt4At5End"],
    ["patterns[4].patterns[5].patterns[3].begin", "RootAt4At5At3"],
    ["patterns[4].patterns[5].patterns[3].end", "RootAt4At5At3End"],
    ["patterns[5].patterns[0].end", "RootAt5At0End"],
    ["patterns[5].patterns[1].end", "RootAt5At1End"],
    ["patterns[5].patterns[2].end", "RootAt5At2End"],
    ["patterns[5].patterns[3].end", "RootAt5At3End"],
    ["patterns[6].patterns[0].end", "RootAt6At0End"],
    ["patterns[7].patterns[0].end", "RootAt7At0End"],
    ["#line_comment.patterns[0].begin", "LineCommentAt0"],
    ...["braces", "brackets", "ltgt", "parens"].flatMap((kind) =>
      ["", "_interpolated"].flatMap((tail) => {
        const id = `Nested${kind.charAt(0).toUpperCase()}${kind.slice(1)}${tail ? "Interpolated" : ""}`;
        return [
          [`#nested_${kind}${tail}.begin`, id],
          [`#nested_${kind}${tail}.end`, `${id}End`],
        ];
      }),
    ),
  ].map(([path, probe]): Waiver => ({ grammar: "perl", path: path!, group: 1, reason: missing(0), probe: probe! })),
  ...["patterns[6].patterns[0].end", "patterns[7].patterns[0].end"].map(
    (path, k): Waiver => ({ grammar: "perl", path, group: 0, reason: "the end is a lookahead, which holds no text", probe: `RootAt${6 + k}At0End` }),
  ),
  { grammar: "perl", path: "patterns[12].match", group: 3, reason: missing(2), probe: "RootAt12" },
  ...[2, 3].map((group): Waiver => ({ grammar: "perl", path: "patterns[13].match", group, reason: missing(1), probe: "RootAt13" })),
  ...[1, 2].map((group): Waiver => ({ grammar: "perl", path: "patterns[14].patterns[0].match", group, reason: blanks, probe: "RootAt14At0" })),
  { grammar: "perl", path: "patterns[43].end", group: 0, reason: "the end is `$`, which holds no text", probe: "RootAt43End" },
  ...[
    ["#heredoc.patterns[7].begin", "HeredocAt7"],
    ["#heredoc.patterns[15].begin", "HeredocAt15"],
    ["#heredoc.patterns[16].begin", "HeredocAt16"],
    ["#heredoc.patterns[17].begin", "HeredocAt17"],
    ["#heredoc.patterns[18].begin", "HeredocAt18"],
    ["#heredoc.patterns[19].begin", "HeredocAt19"],
    ["#heredoc.patterns[20].begin", "HeredocAt20"],
    ["#heredoc.patterns[21].begin", "HeredocAt21"],
  ].map(([path, probe]): Waiver => ({
    grammar: "perl",
    path: path!,
    group: 7,
    reason: "the group is `()`, the closing quote of a label that has none, so it never holds text",
    probe: probe!,
  })),
  { grammar: "common-lisp", path: "#comment.begin", group: 1, reason: blanks, probe: "Comment" },
  {
    grammar: "common-lisp",
    path: "#variable.patterns[0].match",
    reason:
      "`#style-guide`'s `(\\*[^*\\s]+\\*)`, before it in the root, matches every `*name*` it does, after a lookbehind that holds wherever its own does",
    probe: "VariableAt0",
  },
  {
    grammar: "common-lisp",
    path: "#punctuation.patterns[10].match",
    reason: "`#package`, before it in the root, matches the same `#` before a `:` after the same lookbehind",
    probe: "PunctuationAt10",
  },
  ...[2, 3, 4].map(
    (group): Waiver => ({
      grammar: "common-lisp",
      path: "#punctuation.patterns[11].match",
      group,
      reason:
        "`#punctuation`'s first pattern, before this one, matches every backquote before a `\\S`, after a lookbehind that holds wherever this one's does",
      probe: "PunctuationAt11",
    }),
  ),
  {
    grammar: "r",
    path: "#basic-roxygen-example.patterns[0].match",
    reason: "`#roxygen-example.patterns[0]`, the same expression, comes before it in the one list that holds it",
    probe: "MarkdownAt0At0",
  },
  { grammar: "hcl", path: "#double_slash_line_comments.end", group: 0, reason: lineFeed },
  { grammar: "hcl", path: "#hash_line_comments.end", group: 0, reason: lineFeed },
  {
    grammar: "asm",
    path: "#mnemonics-sgx.patterns[2].match",
    reason: "every word it lists, such as `eadd`, an earlier mnemonic list holds too, and the first list that holds a word wins",
    probe: "MnemonicsSgxAt2",
  },
  ...[0, 1, 2, 3].map(
    (n): Waiver => ({ grammar: "asm", path: `#preprocessor.patterns[${n}].end`, group: 1, reason: missing(0), probe: `PreprocessorAt${n}End` }),
  ),
  { grammar: "asm", path: "#preprocessor-functions.patterns[0].end", group: 3, reason: missing(1), probe: "PreprocessorFunctionsAt0End" },
  ...[0, 1, 2].flatMap((n): Waiver[] =>
    ["begin", "end"].map((side) => ({
      grammar: "asm",
      path: `#strings.patterns[${n}].${side}`,
      reason:
        "vscode-textmate drops the string rule: its only patterns include `#string_escaped_char` and `#string_placeholder`, which the grammar never defines",
      dropped: true,
    })),
  ),
];

/** The grammars that are complete, by Shiki's name. A grammar joins in the
 * pull request that closes its last gap. */
export const complete: string[] = [
  "asm",
  "astro",
  "bat",
  "c",
  "cmake",
  "coffee",
  "common-lisp",
  "cpp",
  "cpp-macro",
  "csharp",
  "css",
  "dart",
  "diff",
  "docker",
  "elixir",
  "erb",
  "gdscript",
  "go",
  "graphql",
  "hack",
  "haml",
  "haskell",
  "hcl",
  "hlsl",
  "ini",
  "java",
  "javascript",
  "jinja",
  "json",
  "jsonc",
  "jsonl",
  "jsx",
  "julia",
  "kotlin",
  "latex",
  "less",
  "lua",
  "make",
  "markdown",
  "matlab",
  "mermaid",
  "nix",
  "objective-c",
  "objective-cpp",
  "pascal",
  "perl",
  "php",
  "postcss",
  "powershell",
  "proto",
  "python",
  "r",
  "ruby",
  "rust",
  "sass",
  "scala",
  "scss",
  "shaderlab",
  "shellscript",
  "sql",
  "svelte",
  "swift",
  "tex",
  "toml",
  "tsx",
  "typescript",
  "vb",
  "xml",
  "yaml",
  "zig",
];
