// Writes src/coffee/scope.rs and src/coffee/grammar.rs from the `coffee` grammar that
// Shiki 4.6.0 bundles (atom/language-coffee-script through microsoft/vscode, MIT).
// It writes the scope names.
// For every rule it writes the scopes the rule adds, how it ends and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// The matchers that decide whether a pattern matches are written by hand in src/coffee/matchers.
//
// Two changes keep Shiki's output:
// - A pattern named after its first group (`$1`) becomes one pattern per word of the group.
// - The regular expression rule includes only `source.js.regexp`, which no grammar Shiki loads provides.
//   vscode-textmate skips such a rule, so it is left out.
//
// Svelte's grammar injects two patterns into the CoffeeScript it embeds:
// `L:(source.ts, source.js, source.coffee)`, its `$` prefixes.
// They become `Pattern::SvelteVariable` and `Pattern::SvelteProps`, listed in
// `Pattern::SVELTE`. No rule lists them. The tables of the dialect that Svelte
// embeds put them first in every rule (src/coffee/svelte.rs).
//
// The script string `` (`)(.*)(`) `` lexes its middle group as JavaScript.
// That group's nested list is `Rule::Javascript`, a rule with no patterns.
// The engine defers the group, and JavaScript lexes it (`Embeds::lex_nested`).
//
// Run with `bun run oracle/coffee-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import coffee from "@shikijs/langs@4.6.0/coffee";
import svelte from "@shikijs/langs@4.6.0/svelte";
import { arms, groupsCheck, oneBlankTable, ruleCodes, scopeFile, startTable, tooLong, writeRust } from "./rule-grammar.ts";

type CaptureNode = { name?: string; patterns?: Node[] };
type Captures = Record<string, CaptureNode>;
type Node = {
  name?: string;
  contentName?: string;
  match?: string;
  begin?: string;
  end?: string;
  captures?: Captures;
  beginCaptures?: Captures;
  endCaptures?: Captures;
  patterns?: Node[];
  include?: string;
};

const grammar = (coffee as unknown as { scopeName: string; repository: Record<string, Node>; patterns: Node[] }[]).find(
  (lang) => lang.scopeName === "source.coffee",
)!;
const repository = grammar.repository;

const root_dir = new URL("..", import.meta.url).pathname;

/** The first capturing group of `regex`, outside character classes: where it starts and ends, and what is inside. */
function firstGroup(regex: string): { start: number; end: number; body: string } {
  let inClass = false;
  for (let i = 0; i < regex.length; i++) {
    const c = regex[i]!;
    if (c === "\\") i++;
    else if (inClass) inClass = c !== "]";
    else if (c === "[") inClass = true;
    else if (c === "(" && regex[i + 1] !== "?") {
      const end = regex.indexOf(")", i);
      return { start: i, end: end + 1, body: regex.slice(i + 1, end) };
    }
  }
  throw new Error(`no group in ${regex}`);
}

const REGEXP_INCLUDE = "source.js.regexp";

/** The grammar's pattern list with the two changes the header names. */
function rewrite(list: Node[]): Node[] {
  return list.flatMap((node): Node[] => {
    if (node.patterns) node.patterns = rewrite(node.patterns);
    if (node.patterns?.length === 1 && node.patterns[0]!.include === REGEXP_INCLUDE) return [];
    if (node.match === undefined || !node.name?.includes("$1")) return [node];
    const group = firstGroup(node.match);
    return group.body.split("|").map((word) => ({
      ...node,
      match: `${node.match!.slice(0, group.start)}(${word})${node.match!.slice(group.end)}`,
      name: node.name!.replace("$1", word),
    }));
  });
}
const root = { patterns: rewrite(grammar.patterns) } as Node;
for (const node of Object.values(repository)) if (node.patterns) node.patterns = rewrite(node.patterns);

// Scope names.

const abbreviations: Record<string, string> = {
  punctuation: "P",
  keyword: "Kw",
  constant: "Const",
  invalid: "Inv",
  storage: "Stor",
  entity: "Ent",
  variable: "Var",
  support: "Supp",
  string: "Str",
  comment: "Cmt",
  meta: "Meta",
};

function scopeIdent(name: string): string {
  const parts = name.split(".");
  if (parts[parts.length - 1] === "coffee") parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
    }
  });
  return out;
}

const scopes = new Map<string, string>([["Source", "source.coffee"]]);
function scope(name: string): string {
  const ident = scopeIdent(name);
  const known = scopes.get(ident);
  if (known !== undefined && known !== name) throw new Error(`${ident}: ${known} and ${name}`);
  scopes.set(ident, name);
  return ident;
}
/** The scopes of a rule's name; an empty name adds none, as in vscode-textmate. */
function scopeList(name: string | undefined): string[] {
  return name === undefined ? [] : name.split(" ").filter((part) => part !== "").map(scope);
}

// Rules and patterns.

const pascal = (key: string) =>
  key
    .split(/[-_.]/)
    .map((piece) => piece.charAt(0).toUpperCase() + piece.slice(1))
    .join("");

/** Names for the patterns that no repository key names, by list and place. */
const names: Record<string, string> = {
  RootAt1: "NewInstance",
  RootAt2: "SingleHeredoc",
  RootAt3: "DoubleHeredoc",
  RootAt4: "Script",
  RootAt5: "BlockComment",
  RootAt6: "LineComment",
  RootAt7: "Heregex",
  RootAt8: "KeywordControl",
  RootAt9: "Delete",
  RootAt10: "Instanceof",
  RootAt11: "New",
  RootAt12: "Typeof",
  RootAt13: "Reserved",
  RootAt14: "FunctionAssignment",
  RootAt15: "QuotedFunctionAssignment",
  RootAt16: "InlineFunction",
  RootAt17: "DestructuredObject",
  RootAt18: "DestructuredArray",
  RootAt19: "True",
  RootAt20: "False",
  RootAt21: "Null",
  RootAt22: "Extends",
  RootAt23: "Super",
  RootAt24: "This",
  RootAt25: "Arguments",
  RootAt26: "ClassExtends",
  RootAt27: "Class",
  RootAt28: "Debugger",
  RootAt29: "SupportClass",
  RootAt30: "Console",
  RootAt31: "ConsoleMethod",
  RootAt32: "ArrayMethod",
  RootAt33: "ArrayStatic",
  RootAt34: "ObjectStatic",
  RootAt35: "MathStatic",
  RootAt36: "NumberStatic",
  RootAt37: "SupportVariable",
  RootAt38: "LanguageConstant",
  RootAt45: "Prototype",
  RootAt46: "InvalidIdentifier",
  RootAt47: "Terminator",
  RootAt48: "Delimiter",
  RootAt49: "Brace",
  RootAt50: "Bracket",
  RootAt51: "Paren",
  SingleHeredocAt0: "SingleHeredocEscape",
  DoubleHeredocAt0: "DoubleHeredocEscape",
  BlockCommentAt0: "Annotation",
  DestructuredObjectAt1: "DestructuredObjectName",
  DestructuredArrayAt1: "DestructuredArrayName",
  BracketAt0: "SliceExclusive",
  BracketAt1: "SliceInclusive",
  ArgumentsAt0: "ParenArguments",
  ArgumentsAt1: "BareArguments",
  DoubleQuotedStringAt0: "DoubleString",
  DoubleStringAt0: "DoubleEscape",
  SingleQuotedStringAt0: "SingleString",
  SingleStringAt0: "SingleEscape",
  EmbeddedCommentAt0: "EmbeddedComment",
  FunctionCallsAt0: "FunctionCall",
  FunctionCallsAt1: "BareFunctionCall",
  FunctionNamesAt0: "SupportFunction",
  FunctionNamesAt1: "FunctionName",
  FunctionNamesAt2: "InvalidFunctionName",
  FunctionParamsAt0: "Params",
  ParamsAt0: "Param",
  ParamsAt1: "InstanceParam",
  HeregexpAt0: "Anchor",
  HeregexpAt1: "BackReference",
  HeregexpAt2: "Quantifier",
  HeregexpAt3: "Or",
  HeregexpAt4: "Assertion",
  HeregexpAt5: "Group",
  HeregexpAt6: "CharacterSet",
  CharacterSetAt0: "CharacterRange",
  InstanceVariableAt0: "InstanceVariable",
  InterpolatedCoffeeAt0: "Interpolation",
  JsxAttributeAt0: "JsxAttribute",
  JsxEndTagAt0: "JsxEndTag",
  JsxTagAt0: "JsxTag",
  MethodCallsAt0: "MethodCall",
  MethodCallsAt1: "BareMethodCall",
  MethodNamesAt0: "EventHandler",
  MethodNamesAt1: "SupportMethod",
  MethodNamesAt2: "DomMethod",
  MethodNamesAt3: "MethodName",
  MethodNamesAt4: "InvalidMethodName",
  NumbersAt0: "Hex",
  NumbersAt1: "Binary",
  NumbersAt2: "Octal",
  NumbersAt3: "Decimal",
  ObjectsAt0: "ConstantObject",
  ObjectsAt1: "VariableObject",
  OperatorsAt0: "WordAssignment",
  OperatorsAt1: "CompoundAssignment",
  OperatorsAt2: "BitwiseAssignment",
  OperatorsAt3: "Shift",
  OperatorsAt4: "Comparison",
  OperatorsAt5: "Logical",
  OperatorsAt6: "Bitwise",
  OperatorsAt7: "Assignment",
  OperatorsAt8: "Decrement",
  OperatorsAt9: "Increment",
  OperatorsAt10: "Splat",
  OperatorsAt11: "Existential",
  OperatorsAt12: "Arithmetic",
  OperatorsAt13: "WordOperator",
  PropertiesAt0: "ConstantObjectProperty",
  PropertiesAt1: "VariableObjectProperty",
  PropertiesAt2: "ConstantProperty",
  PropertiesAt3: "VariableProperty",
  PropertiesAt4: "InvalidProperty",
  RegexCharacterClassAt0: "CharacterClass",
  RegexCharacterClassAt1: "NumericEscape",
  RegexCharacterClassAt2: "ControlEscape",
  RegexCharacterClassAt3: "BackslashEscape",
};
const used = new Set<string>();

type Pattern = {
  id: string;
  regex: string;
  act: { role: "match"; name: string[] } | { role: "begin"; rule: string } | { role: "end" };
  captures: [number, string[]][];
  /** Capture groups that the grammar tokenizes with patterns of their own. */
  nested: [number, string][];
};
type Rule = {
  id: string;
  name: string[];
  content: string[];
  end: string;
  patterns: string[];
};

const patternById = new Map<string, Pattern>();
const rules = new Map<string, Rule>();

/** The rule that tokenizes the text of a capture group with the patterns `list`, one per distinct list. */
const nestedRules = new Map<string, string>();
function nestedRule(list: Node[]): string {
  const key = JSON.stringify(list);
  const known = nestedRules.get(key);
  if (known !== undefined) return known;
  const include = list.length === 1 ? list[0]!.include : undefined;
  if (include === "source.js") {
    // JavaScript lexes the group, so the rule has no patterns of its own.
    nestedRules.set(key, "Javascript");
    rules.set("Javascript", { id: "Javascript", name: [], content: [], end: "", patterns: [] });
    return "Javascript";
  }
  const id = include?.startsWith("#") ? pascal(include.slice(1)) : `Nested${nestedRules.size}`;
  nestedRules.set(key, id);
  const rule: Rule = { id, name: [], content: [], end: "", patterns: [] };
  rules.set(id, rule);
  rule.patterns = flatten(list, id);
  return id;
}

/** The capture groups whose text the grammar tokenizes with patterns, with the rule that does. */
function nestedList(captures: Captures | undefined): [number, string][] {
  if (!captures) return [];
  const out: [number, string][] = [];
  for (const [group, value] of Object.entries(captures)) {
    if (value.patterns === undefined) continue;
    out.push([Number(group), nestedRule(value.patterns)]);
  }
  return out.sort((a, b) => a[0] - b[0]);
}

function captureList(captures: Captures | undefined): [number, string[]][] {
  if (!captures) return [];
  return Object.entries(captures)
    .map(([group, value]): [number, string[]] => [Number(group), scopeList(value.name)])
    .filter(([, names]) => names.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** Registers a match or begin/end rule under `id`; returns the pattern id. */
function register(node: Node, id: string): string {
  const clash = patternById.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (node.match !== undefined) {
    if (!patternById.has(id)) {
      patternById.set(id, {
        id,
        regex: node.match,
        act: { role: "match", name: scopeList(node.name) },
        captures: captureList(node.captures),
        nested: nestedList(node.captures),
      });
    }
    return id;
  }
  if (node.begin !== undefined && node.end !== undefined) {
    if (!patternById.has(id)) {
      if (/\\[1-9]/.test(node.end)) throw new Error(`${id}: the end repeats a group`);
      patternById.set(id, {
        id,
        regex: node.begin,
        act: { role: "begin", rule: id },
        captures: captureList(node.beginCaptures ?? node.captures),
        nested: nestedList(node.beginCaptures ?? node.captures),
      });
      patternById.set(`${id}End`, {
        id: `${id}End`,
        regex: node.end,
        act: { role: "end" },
        captures: captureList(node.endCaptures ?? node.captures),
        nested: nestedList(node.endCaptures ?? node.captures),
      });
      const rule: Rule = {
        id,
        name: scopeList(node.name),
        content: scopeList(node.contentName),
        end: `${id}End`,
        patterns: [],
      };
      rules.set(id, rule);
      rule.patterns = flatten(node.patterns ?? [], id);
    }
    return id;
  }
  throw new Error(`${id}: neither a match nor a begin/end rule`);
}

/** The pattern ids a pattern list expands to, `include`s flattened. */
function flatten(list: Node[], context: string): string[] {
  const out: string[] = [];
  list.forEach((node, index) => {
    if (node.include === "$self") {
      out.push(...flatten(root.patterns!, "Root"));
    } else if (node.include !== undefined) {
      if (!node.include.startsWith("#")) throw new Error(`external include ${node.include}`);
      const key = node.include.slice(1);
      const target = repository[key];
      if (!target) throw new Error(`unknown include ${key}`);
      out.push(...expand(target, pascal(key)));
    } else {
      const id = `${context}At${index}`;
      if (names[id]) used.add(id);
      out.push(...expand(node, names[id] ?? id));
    }
  });
  return out;
}

function expand(node: Node, id: string): string[] {
  if (node.match !== undefined || node.begin !== undefined) return [register(node, id)];
  if (node.patterns) return flatten(node.patterns, id);
  throw new Error(`${id}: empty rule`);
}

const rootPatterns = flatten(root.patterns!, "Root");
rules.set("Root", { id: "Root", name: [], content: [], end: "", patterns: rootPatterns });
const svelteGrammar = (svelte as unknown as { scopeName: string; injections: Record<string, { patterns: Node[] }> }[]).find(
  (lang) => lang.scopeName === "source.svelte",
)!;
const injected = svelteGrammar.injections["L:(source.ts, source.js, source.coffee)"]!.patterns;
const svelteIds = ["SvelteVariable", "SvelteProps"];
if (injected.length !== svelteIds.length) throw new Error(`Svelte injects ${injected.length} patterns, not ${svelteIds.length}`);
injected.forEach((node, index) => register({ match: node.match, name: node.name }, svelteIds[index]!));
const unused = Object.keys(names).filter((id) => !used.has(id));
if (unused.length) throw new Error(`names for no pattern: ${unused.join(", ")}`);
const unnamed = [...patternById.keys()].filter((id) => /At\d/.test(id));
if (unnamed.length) throw new Error(`patterns without a name: ${unnamed.join(", ")}`);

// Output.

const list = (names: string[]) => (names.length ? `scopes![${names.map((n) => `S::${n}`).join(", ")}]` : "S::NO_SCOPES");

let g = `//! The grammar's rules as tables. Generated by \`oracle/coffee-grammar.ts\` from
//! the \`coffee\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::Coffee;
use super::scope::S;
use crate::scope::scopes;
use crate::rules;

pub(crate) type Act = rules::Act<Coffee>;
pub(crate) type PatternDefinition = rules::PatternDefinition<Coffee>;
pub(crate) type RuleDefinition = rules::RuleDefinition<Coffee>;
pub(crate) use crate::rules::Start;

/// The rule that tokenizes a capture group.
pub(crate) type Nested = Rule;

`;

const ruleList = [...rules.values()];
const patternList = [...patternById.values()];
if (ruleList.length > 256) throw new Error(`${ruleList.length} rules do not fit one byte`);

g += `/// A rule of the grammar that can be open at the end of a line.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub(crate) enum Rule {\n`;
for (const r of ruleList) g += `    ${r.id},\n`;
g += `}\n\nimpl Rule {\n    /// Every rule, in code order.\n    pub(crate) const ALL: [Rule; ${ruleList.length}] = [\n`;
for (const r of ruleList) g += `        Rule::${r.id},\n`;
g += `    ];\n\n${tooLong}    pub(crate) fn definition(self) -> RuleDefinition {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    `RuleDefinition {\n                name: ${list(r.name)},\n                content: ${list(r.content)},\n                end: ${r.end ? `Some(Pattern::${r.end})` : "None"},\n                backref: None,\n                patterns: &[${r.patterns.map((p) => `Pattern::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pattern {\n`;
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 ? `…` : one;
};
for (const p of patternList) {
  g += `    /// The ${p.act.role} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pattern {\n    /// Svelte's \`$\` prefixes, which its grammar injects into the CoffeeScript it\n    /// embeds, in the grammar's order.\n    #[cfg(feature = "lang-svelte")]\n    pub(crate) const SVELTE: [Pattern; ${svelteIds.length}] = [${svelteIds.map((id) => `Pattern::${id}`).join(", ")}];\n\n    /// Every pattern, for the tool that compares the matchers with Oniguruma.\n    #[cfg(test)]\n    pub(crate) const ALL: [Pattern; ${patternList.length}] = [\n${patternList.map((p) => `        Pattern::${p.id},\n`).join("")}    ];\n\n${tooLong}    pub(crate) fn definition(self) -> PatternDefinition {\n        match self {\n`;
g += arms(
  patternList.map((p): [string, string] => {
    const act =
      p.act.role === "match"
        ? `Act::Match(${list(p.act.name)})`
        : p.act.role === "begin"
          ? `Act::Begin(Rule::${p.act.rule})`
          : "Act::End";
    const caps = p.captures.map(([n, names]) => `(${n}, ${list(names)})`).join(", ");
    const nested = p.nested.map(([n, name]) => `(${n}, Rule::${name})`).join(", ");
    return [p.id, `PatternDefinition {\n                act: ${act},\n                captures: &[${caps}],\n                nested: &[${nested}],\n            }`];
  }),
  "Pattern",
);
g += `        }\n    }\n}\n`;

g += oneBlankTable(patternList);
g += startTable(patternList);
g += groupsCheck("Coffee", patternList.flatMap((p) => [...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]));
g += ruleCodes("Coffee", "COFFEE");

await writeRust(`${root_dir}src/coffee/grammar.rs`, g);

const s = scopeFile("coffee-grammar.ts", "COFFEE", scopes);
await writeRust(`${root_dir}src/coffee/scope.rs`, s);

// With RX_DUMP set to a file, writes each pattern for oracle/onig.ts.
if (Bun.env["RX_DUMP"]) {
  await Bun.write(
    Bun.env["RX_DUMP"],
    patternList.map((p) => JSON.stringify({ id: p.id, regex: p.regex, groups: p.captures.map(([n]) => n) })).join("\n") + "\n",
  );
}
console.log(`${ruleList.length} rules, ${patternList.length} patterns, ${scopes.size} scopes`);
