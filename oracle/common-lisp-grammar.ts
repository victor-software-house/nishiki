// Writes src/common_lisp/scope.rs, src/common_lisp/grammar.rs and
// src/common_lisp/words.rs from the `common-lisp` grammar that Shiki 4.6.0
// bundles (qingpeng9802/vscode-common-lisp, MIT).
// grammar.rs holds the scopes each rule and pattern adds, how each rule ends
// and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// A capture group that the grammar tokenizes with patterns of its own gets a
// rule with no end, named in `Nested`.
// words.rs holds the words of the grammar's word alternations.
// The matchers that decide whether a pattern matches are written by hand in
// src/common_lisp/matchers.rs.
//
// Run with `bun run oracle/common-lisp-grammar.ts` and commit the result.
// The output is generated; do not edit it.
// `RX_DUMP=<file>` also writes every pattern's regular expression and capture
// groups, one JSON object per line, for oracle/onig.ts.
import commonLisp from "@shikijs/langs@4.6.0/common-lisp";
import { expand } from "./alternation.ts";
import {
  arms,
  groupsCheck,
  ruleCodes,
  scopeFile,
  startTable,
  takesOneBlank,
  tooLong,
  writePatterns,
  writeRust,
} from "./rule-grammar.ts";

type Captures = Record<string, { name?: string; patterns?: Node[] }>;
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
type Grammar = { scopeName: string; repository: Record<string, Node>; patterns: Node[] };

const grammar = (commonLisp as unknown as Grammar[]).find(
  (lang) => lang.scopeName === "source.commonlisp",
)!;
const repository = grammar.repository;
const root = { patterns: grammar.patterns } as Node;
const root_dir = new URL("..", import.meta.url).pathname;

// Scope names.

const abbreviations: Record<string, string> = {
  punctuation: "P",
  keyword: "Kw",
  constant: "Const",
  storage: "Stor",
  entity: "Ent",
  variable: "Var",
  support: "Supp",
  string: "Str",
  comment: "Cmt",
};

/** The identifier of a scope name: its parts in camel case, without the
 * `commonlisp` suffix, and ending in `Bare` when the name has no suffix. */
function scopeIdent(name: string): string {
  const parts = name.split(".");
  const bare = parts[parts.length - 1] !== "commonlisp";
  if (!bare) parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const capture = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? capture) : capture;
    }
  });
  return out + (bare ? "Bare" : "");
}

const scopes = new Map<string, string>([["Source", "source.commonlisp"]]);
function scopeList(name: string | undefined): string[] {
  if (name === undefined) return [];
  return name.split(" ").map((one) => {
    if (one.includes("$")) throw new Error(`${name}: a name built from a capture`);
    const ident = one === "source.commonlisp" ? "Source" : scopeIdent(one);
    const known = scopes.get(ident);
    if (known !== undefined && known !== one) throw new Error(`${ident}: ${known} and ${one}`);
    scopes.set(ident, one);
    return ident;
  });
}
const scText = (list: string[]) =>
  list.length ? `scopes![${list.map((s) => `S::${s}`).join(", ")}]` : "S::NO_SCOPES";

// Rules and patterns.

const pascal = (key: string) =>
  key
    .split(/[-_.]/)
    .map((piece) => piece.charAt(0).toUpperCase() + piece.slice(1))
    .join("");

type Pattern = {
  id: string;
  regex: string;
  act: { role: "match"; name: string[] } | { role: "begin"; rule: string } | { role: "end" };
  captures: [number, string[]][];
  /** Capture groups that the grammar tokenizes with patterns, with the rule that does. */
  nested: [number, string][];
};
type Rule = { id: string; name: string[]; content: string[]; end: string; patterns: string[] };

const patternById = new Map<string, Pattern>();
const rules = new Map<string, Rule>();

/** The rule that tokenizes a capture group's text with a pattern list, one
 * per distinct list. */
const nestedRules = new Map<string, string>();
const nestedSources: { id: string; list: Node[] }[] = [];

function nestedRule(list: Node[]): string {
  const key = JSON.stringify(list);
  const known = nestedRules.get(key);
  if (known !== undefined) return known;
  const id = `Nested${nestedRules.size}`;
  nestedRules.set(key, id);
  rules.set(id, { id, name: [], content: [], end: "", patterns: [] });
  nestedSources.push({ id, list });
  rules.get(id)!.patterns = flatten(list, id);
  return id;
}

function nestedList(captures: Captures | undefined): [number, string][] {
  return Object.entries(captures ?? {})
    .filter(([, value]) => value.patterns !== undefined)
    .map(([group, value]): [number, string] => [Number(group), nestedRule(value.patterns!)])
    .sort((a, b) => a[0] - b[0]);
}

function captureList(captures: Captures | undefined): [number, string[]][] {
  return Object.entries(captures ?? {})
    .map(([group, value]): [number, string[]] => [Number(group), scopeList(value.name)])
    .filter(([, list]) => list.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** Registers a match or begin/end rule under `id`; returns its pattern id. */
function register(node: Node, id: string): string {
  const clash = patternById.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (patternById.has(id)) return id;
  if (node.match !== undefined) {
    patternById.set(id, {
      id,
      regex: node.match,
      act: { role: "match", name: scopeList(node.name) },
      captures: captureList(node.captures),
      nested: nestedList(node.captures),
    });
    return id;
  }
  if (node.begin === undefined || node.end === undefined)
    throw new Error(`${id}: neither a match nor a begin/end rule`);
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
  ruleSources.push({ id, node });
  rule.patterns = flatten(node.patterns ?? [], id);
  return id;
}

/** The begin/end rules registered, to expand their patterns a second time. */
const ruleSources: { id: string; node: Node }[] = [];
/** The bare lists being expanded, to stop a list that includes itself. */
const expanding = new Set<string>();

/** The pattern ids a pattern list expands to, `include`s flattened.
 * The block comment's include carries a `name`, which vscode-textmate
 * ignores on an include, and so does this. */
function flatten(list: Node[], context: string): string[] {
  const out: string[] = [];
  list.forEach((node, index) => {
    if (node.include !== undefined) {
      if (!node.include.startsWith("#")) throw new Error(`external include ${node.include}`);
      const key = node.include.slice(1);
      const target = repository[key];
      if (!target) throw new Error(`unknown include ${key}`);
      out.push(...expandNode(target, pascal(key), true));
    } else {
      out.push(...expandNode(node, `${context}At${index}`, false));
    }
  });
  return out;
}

function expandNode(node: Node, id: string, shared: boolean): string[] {
  if (node.match !== undefined || node.begin !== undefined) return [register(node, id)];
  if (node.patterns) {
    if (shared) {
      if (expanding.has(id)) return [];
      expanding.add(id);
    }
    const out = flatten(node.patterns, id);
    if (shared) expanding.delete(id);
    return out;
  }
  return [];
}

const rootRule: Rule = { id: "Root", name: [], content: [], end: "", patterns: [] };
rules.set("Root", rootRule);
rootRule.patterns = flatten(root.patterns!, "Root");

// The first pass stopped a bare list where it reached itself, which left a
// rule short of patterns it includes.
// Every pattern has its id by now, so a second pass gives each list in full.
for (const { id, node } of ruleSources) {
  expanding.clear();
  rules.get(id)!.patterns = flatten(node.patterns ?? [], id);
}
for (const { id, list } of nestedSources) {
  expanding.clear();
  rules.get(id)!.patterns = flatten(list, id);
}

// No pattern may take a blank after the first of a run: the lexer skips the
// rest of a run (`BlankRuns::Skip`).
for (const p of patternById.values()) {
  if (takesOneBlank(p.regex))
    throw new Error(`${p.id} takes one blank: the lexer needs BlankRuns::ToLast`);
}

// Words.

/** The lookbehinds the word patterns start with. */
const lookbehinds = ["(?<=^|[(\\s])", "(?<=^|[(\\s]|,@|,\\.?)", "(?<=^|[(\\s]|#')"];
/** The lookahead every word pattern ends with. */
const delimiter = "(?=([()\\s]))";
/** The characters no word holds: the delimiters, and what ends a symbol. */
const outside = /[()\s"';`,|\\]/;

/** The lowercase words of a `(?i)`, a lookbehind, an alternation and the
 * delimiter lookahead, sorted.
 * No word holds a delimiter, so the alternation matches only the whole run of
 * characters up to the next delimiter. */
function wordList(id: string, regex: string): string[] {
  const head = lookbehinds.find((lb) => regex.startsWith(`(?i)${lb}`));
  if (head === undefined || !regex.endsWith(delimiter))
    throw new Error(`${id}: not a word pattern`);
  return spelled(id, regex.slice(`(?i)${head}`.length, -delimiter.length));
}

/** The lowercase words the alternation `source` spells, sorted. */
function spelled(id: string, source: string): string[] {
  const all = expand(source).map((word) => word.toLowerCase());
  if (all.some((word) => word === "" || outside.test(word)))
    throw new Error(`${id}: a word with a delimiter`);
  return [...new Set(all)].sort();
}

/** The source of the first capture group of `regex`. */
function firstGroup(id: string, regex: string): string {
  const open = regex.indexOf("(def");
  const word = /^\(\?i\)\(\?<=\^\|\[\(\\s\]\)\(([a-z|()?:-]*?)\)\\s\+/.exec(regex)?.[1];
  if (word === undefined) throw new Error(`${id}: its head changed shape`);
  return word;
}

const wordPats: [string, string][] = [
  ["CONSTANTS", "ConstantAt0"],
  ["ACCESSORS", "FunctionAt0"],
  ["SIDE_EFFECTS", "FunctionAt1"],
  ["FUNCTIONS", "FunctionAt2"],
  ["GENERICS", "FunctionAt3"],
  ["REINITIALIZE", "FunctionAt4"],
  ["TYPE_SPECIFIERS", "FunctionAt5"],
  ["PURE_MACROS", "MacroAt0"],
  ["MACROS", "MacroAt1"],
  ["SPECIAL_FORMS", "MacroAt2"],
  ["SYMBOLS", "Symbol"],
  ["DECLARATIONS", "Declaration"],
  ["TYPES", "Type"],
  ["CLASSES", "Class"],
  ["CONDITIONS", "ConditionType"],
  ["GLOBALS", "VariableAt0"],
  ["REPL", "VariableAt1"],
];
const words = new Map<string, [string, string[]]>();
for (const [name, id] of wordPats) words.set(name, [id, wordList(id, patternById.get(id)!.regex)]);
const special = patternById.get("SpecialOperator")!.regex;
const specialWords = /^\(\?i\)\(\\\(\\s\*\)\((.*)\)\(\?=\(\[\(\)\\s\]\)\)$/.exec(special)?.[1];
if (specialWords === undefined) throw new Error("SpecialOperator changed shape");
words.set("SPECIAL_OPERATORS", ["SpecialOperator", spelled("SpecialOperator", specialWords)]);
const heads: [string, string][] = [
  ["DEF_FUNCTIONS", "DefNameAt0"],
  ["DEF_TYPES", "DefNameAt1"],
  ["DEF_CONSTANT", "DefNameAt2"],
  ["DEF_VARIABLES", "DefNameAt3"],
  ["DEF_STRUCT", "DefNameAt4"],
  ["LOCAL_FUNCTIONS", "DefNameAt5"],
];
for (const [name, id] of heads)
  words.set(name, [id, spelled(id, firstGroup(id, patternById.get(id)!.regex))]);
/** The longest word, which sizes the matchers' lowered copy of a run. */
const longest = Math.max(
  ...[...words.values()].flatMap(([, list]) => list.map((word) => word.length)),
);

// Output.

let g = `//! The grammar's rules as tables. Generated by \`oracle/common-lisp-grammar.ts\`
//! from the \`common-lisp\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::CommonLisp;
use super::scope::S;
use crate::rules;
use crate::scope::scopes;

pub(crate) type Act = rules::Act<CommonLisp>;
pub(crate) type PatternDefinition = rules::PatternDefinition<CommonLisp>;
pub(crate) type RuleDefinition = rules::RuleDefinition<CommonLisp>;
pub(crate) use crate::rules::Start;

/// The rule that tokenizes a capture group.
pub(crate) type Nested = Rule;

`;

const ruleList = [...rules.values()];
const patternList = [...patternById.values()];

g += `/// A rule of the grammar that can be open at the end of a line.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub(crate) enum Rule {\n`;
for (const r of ruleList) g += `    ${r.id},\n`;
g += `}\n\nimpl Rule {\n    /// Every rule, in code order.\n    pub(crate) const ALL: [Rule; ${ruleList.length}] = [\n`;
for (const r of ruleList) g += `        Rule::${r.id},\n`;
g += `    ];\n\n${tooLong}    pub(crate) fn definition(self) -> RuleDefinition {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    `RuleDefinition {\n                name: ${scText(r.name)},\n                content: ${scText(r.content)},\n                end: ${r.end ? `Some(Pattern::${r.end})` : "None"},\n                backref: None,\n                patterns: &[${r.patterns.map((p) => `Pattern::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pattern {\n`;
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 || /[^\x20-\x7e]/.test(one) ? "…" : one;
};
for (const p of patternList) {
  g += `    /// The ${p.act.role} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pattern {\n    /// Every pattern, for the tool that compares the matchers with Oniguruma.\n    #[cfg(all(test, feature = "json"))]\n    pub(crate) const ALL: [Pattern; ${patternList.length}] = [\n${patternList.map((p) => `        Pattern::${p.id},\n`).join("")}    ];\n\n${tooLong}    pub(crate) fn definition(self) -> PatternDefinition {\n        match self {\n`;
g += arms(
  patternList.map((p): [string, string] => {
    const act =
      p.act.role === "match"
        ? `Act::Match(${scText(p.act.name)})`
        : p.act.role === "begin"
          ? `Act::Begin(Rule::${p.act.rule})`
          : "Act::End";
    const captures = p.captures.map(([n, list]) => `(${n}, ${scText(list)})`).join(", ");
    const nested = p.nested.map(([n, rule]) => `(${n}, Rule::${rule})`).join(", ");
    return [
      p.id,
      `PatternDefinition {\n                act: ${act},\n                captures: &[${captures}],\n                nested: &[${nested}],\n            }`,
    ];
  }),
  "Pattern",
);
g += `        }\n    }\n}\n`;
g += startTable(patternList);
g += ruleCodes("CommonLisp", "COMMON_LISP");
g += groupsCheck(
  "CommonLisp",
  patternList.flatMap((p) => [...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]),
);
await writePatterns("common_lisp", patternList);
await writeRust(`${root_dir}src/common_lisp/grammar.rs`, g);

const bytes = (word: string) => `b"${word.replaceAll("\\", "\\\\")}"`;
let w = `//! The words of the grammar's word alternations, lowercase and sorted.
//! Generated by \`oracle/common-lisp-grammar.ts\` from the \`common-lisp\`
//! grammar Shiki 4.6.0 bundles; do not edit.

/// The longest word of the lists, which sizes a lowered run.
pub(super) const LONGEST: usize = ${longest};
`;
for (const [name, [id, list]] of words) {
  w += `\n/// The words of the \`${id}\` pattern.\npub(super) const ${name}: &[&[u8]] = &[${list.map(bytes).join(", ")}];\n`;
}
await writeRust(`${root_dir}src/common_lisp/words.rs`, w);

await writeRust(
  `${root_dir}src/common_lisp/scope.rs`,
  scopeFile("common-lisp-grammar.ts", "COMMON_LISP", scopes),
);

if (Bun.env["RX_DUMP"]) {
  await Bun.write(
    Bun.env["RX_DUMP"],
    patternList
      .map((p) =>
        JSON.stringify({
          id: p.id,
          regex: p.regex,
          groups: [...new Set([...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)])].sort(
            (a, b) => a - b,
          ),
        }),
      )
      .join("\n") + "\n",
  );
}
console.log(
  `${ruleList.length} rules, ${patternList.length} patterns, ${scopes.size} scopes, ${words.size} word lists`,
);
