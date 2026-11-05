// Writes src/vb/scope.rs and src/vb/grammar.rs from the `vb` grammar that
// Shiki 4.6.0 bundles (textmate/asp.vb.net.tmbundle, through VS Code, under
// the TextMate bundle licence).
// It writes the scope names.
// For every rule it writes the scopes the rule adds, how it ends and the
// patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// The matchers that decide whether a pattern matches are written by hand in
// src/vb/matchers.
//
// Run with `bun run oracle/vb-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import vb from "@shikijs/langs@4.6.0/vb";
import {
  arms,
  groupsCheck,
  ruleCodes,
  scopeFile,
  startTable,
  tooLong,
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

const grammar = (vb as unknown as Grammar[]).find((lang) => lang.scopeName === "source.asp.vb.net")!;
const repository = grammar.repository;
const root = { patterns: grammar.patterns } as Node;

const root_dir = new URL("..", import.meta.url).pathname;

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
  if (parts[parts.length - 1] === "asp") parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
    }
  });
  return out || "Plain";
}

const scopes = new Map<string, string>([["Source", "source.asp.vb.net"]]);
/** The identifier of scope `name`. */
function scope(name: string): string {
  const ident = scopeIdent(name);
  const known = scopes.get(ident);
  if (known !== undefined && known !== name) throw new Error(`${ident}: ${known} and ${name}`);
  scopes.set(ident, name);
  return ident;
}

function scopeList(name: string | undefined): string[] {
  if (name === undefined) return [];
  return name.split(" ").map((part) => {
    if (part.includes("$")) throw new Error(`${name}: a name built from a capture`);
    return scope(part);
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

type Pat = {
  id: string;
  regex: string;
  act: { kind: "match"; name: string[] } | { kind: "begin"; rule: string } | { kind: "end" };
  captures: [number, string[]][];
};
type Rule = {
  id: string;
  name: string[];
  content: string[];
  end: string;
  /** The begin capture group that the end pattern repeats, when it does. */
  backref: number | null;
  patterns: string[];
};

const pats = new Map<string, Pat>();
const rules = new Map<string, Rule>();

function captureList(captures: Captures | undefined): [number, string[]][] {
  if (!captures) return [];
  for (const [group, value] of Object.entries(captures)) {
    if (value.patterns !== undefined) throw new Error(`group ${group}: patterns of its own`);
  }
  return Object.entries(captures)
    .map(([group, value]): [number, string[]] => [Number(group), scopeList(value.name)])
    .filter(([, names]) => names.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** Registers a match or begin/end rule under `id`; returns its pattern id. */
function register(node: Node, id: string): string {
  const clash = pats.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (pats.has(id)) return id;
  if (node.match !== undefined) {
    pats.set(id, { id, regex: node.match, act: { kind: "match", name: scopeList(node.name) }, captures: captureList(node.captures) });
    return id;
  }
  if (node.begin === undefined || node.end === undefined) throw new Error(`${id}: neither a match nor a begin/end rule`);
  pats.set(id, {
    id,
    regex: node.begin,
    act: { kind: "begin", rule: id },
    captures: captureList(node.beginCaptures ?? node.captures),
  });
  pats.set(`${id}End`, {
    id: `${id}End`,
    regex: node.end,
    act: { kind: "end" },
    captures: captureList(node.endCaptures ?? node.captures),
  });
  const backref = /\\([1-9])/.exec(node.end);
  const rule: Rule = {
    id,
    name: scopeList(node.name),
    content: scopeList(node.contentName),
    end: `${id}End`,
    backref: backref ? Number(backref[1]) : null,
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

/** The pattern ids a pattern list expands to, `include`s flattened. */
function flatten(list: Node[], context: string): string[] {
  const out: string[] = [];
  list.forEach((node, index) => {
    if (node.include === "$self" || node.include === grammar.scopeName) {
      out.push(...expandNode(root, "Root", true));
    } else if (node.include !== undefined) {
      if (!node.include.startsWith("#")) throw new Error(`external include ${node.include}`);
      const key = node.include.slice(1);
      const target = repository[key];
      if (!target) throw new Error(`unknown include ${key}`);
      out.push(...expandNode(target, pascal(key), true));
    } else {
      out.push(...expandNode(node, context === "Root" ? rootName(index) : `${context}At${index}`, false));
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

/** The names of the root's own patterns, by index. */
const rootNames = [
  "LineFeed", "", "Tabs", "Spaces", "FunctionHead", "Comment", "Control", "Operators", "DimName", "Storage",
  "Modifiers", "Constants", "String", "DollarVariable", "AspClasses", "AspCollections", "AspConstants",
  "AspFunctions", "AspEvents", "AsType", "Functions", "Numbers", "VbConstants", "FunctionCall", "Variable", "JsOperators",
];
if (rootNames.length !== root.patterns!.length) throw new Error("the root's patterns changed");
function rootName(index: number): string {
  const name = rootNames[index];
  if (!name) throw new Error(`root pattern ${index} has no name`);
  return name;
}

const rootRule: Rule = { id: "Root", name: [], content: [], end: "", backref: null, patterns: [] };
rules.set("Root", rootRule);
rootRule.patterns = flatten(root.patterns!, "Root");

// The first pass stopped a bare list where it reached itself, which left a
// rule short of patterns it includes.
// Every pattern has its id by now, so a second pass gives each list in full.
for (const { id, node } of ruleSources) {
  expanding.clear();
  rules.get(id)!.patterns = flatten(node.patterns ?? [], id);
}
expanding.clear();
rootRule.patterns = flatten(root.patterns!, "Root");

// Output.

let g = `//! The grammar's rules as tables. Generated by \`oracle/vb-grammar.ts\` from
//! the \`vb\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pat\`] is one regular expression of the grammar; its doc comment says
//! which. Each rule lists the patterns it tries, in the grammar's order with
//! its \`include\`s flattened, so the first that matches at the earliest
//! position wins as in a \`TextMate\` engine.

use super::engine::Vb;
use super::scope::S;
use crate::rules;
use crate::scope::scopes;

pub(crate) type Act = rules::Act<Vb>;
pub(crate) type PatData = rules::PatData<Vb>;
pub(crate) type RuleData = rules::RuleData<Vb>;
pub(crate) use crate::rules::Start;

`;

const ruleList = [...rules.values()];
const patList = [...pats.values()];
if (ruleList.length > 256) throw new Error(`${ruleList.length} rules do not fit one byte`);

g += `/// A rule of the grammar that can be open at the end of a line.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub(crate) enum Rule {\n`;
for (const r of ruleList) g += `    ${r.id},\n`;
g += `}\n\nimpl Rule {\n    /// Every rule, in code order.\n    pub(crate) const ALL: [Rule; ${ruleList.length}] = [\n`;
for (const r of ruleList) g += `        Rule::${r.id},\n`;
g += `    ];\n\n${tooLong}    pub(crate) fn data(self) -> RuleData {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    `RuleData {\n                name: ${scText(r.name)},\n                content: ${scText(r.content)},\n                end: ${r.end ? `Some(Pat::${r.end})` : "None"},\n                backref: ${r.backref === null ? "None" : `Some(${r.backref})`},\n                patterns: &[${r.patterns.map((p) => `Pat::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n\n    /// What a rule carries from line to line: the begin capture group its\n    /// end repeats.\n    #[inline]\n    pub(crate) fn payload(self) -> rules::Payload {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [r.id, r.backref === null ? "rules::Payload::None" : `rules::Payload::Backref(${r.backref})`]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pat {\n`;
// A long regex, or one with characters past ASCII, is elided.
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 || /[^\x20-\x7e]/.test(one) ? `…` : one;
};
for (const p of patList) {
  g += `    /// The ${p.act.kind} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pat {\n${tooLong}    pub(crate) fn data(self) -> PatData {\n        match self {\n`;
g += arms(
  patList.map((p): [string, string] => {
    const act =
      p.act.kind === "match"
        ? `Act::Match(${scText(p.act.name)})`
        : p.act.kind === "begin"
          ? `Act::Begin(Rule::${p.act.rule})`
          : "Act::End";
    const caps = p.captures.map(([n, names]) => `(${n}, ${scText(names)})`).join(", ");
    return [p.id, `PatData {\n                act: ${act},\n                captures: &[${caps}],\n                nested: &[],\n            }`];
  }),
  "Pat",
);
g += `        }\n    }\n}\n`;

// The grammar's word alternations, `(?i:\b(a|b|…)\b)`, as tables of
// buckets, one per first letter.
// An alternation takes its first word in order that `\b` follows, not its
// longest.
// So each bucket keeps the grammar's order, and a word after an earlier one
// that is its prefix up to a blank never wins.
/** The words of the alternation in pattern `id`, in the grammar's order. */
function ordered(id: string): string[] {
  const regex = pats.get(id)!.regex;
  const found = /^\(\?i:\\s\*\\b\(([A-Za-z0-9_ |?]*)\)\\b(?:\\s\*)?\)$|^\(\?i:\\b\(([A-Za-z0-9_ |?]*)\)\\b\)$/.exec(regex);
  const body = found?.[1] ?? found?.[2];
  if (body === undefined) throw new Error(`${id} changed shape`);
  // A lazy `x??` tries the word without its last letter first.
  return body
    .split("|")
    .flatMap((w) => (w.endsWith("??") ? [w.slice(0, -3), w.slice(0, -2)] : [w]))
    .map((w) => w.toLowerCase());
}
/** The words of `list` that can win, each after no earlier word that is its
 * prefix up to a blank, and no repeat. */
function winners(id: string, list: string[]): string[] {
  const out: string[] = [];
  for (const word of list) {
    const hider = out.find((w) => w === word || word.startsWith(`${w} `));
    if (hider !== undefined) {
      console.log(`${id}: \`${word}\` never wins after \`${hider}\``);
      continue;
    }
    out.push(word);
  }
  return out;
}
const bytes = (w: string) => `b"${w}"`;
g += `
/// A word list split by the word's first letter, \`a\` to \`z\`: each bucket
/// holds the words in the grammar's order, lowercase.
pub(crate) type Buckets = [&'static [&'static [u8]]; 26];
`;
const lists: [string, string, string][] = [
  ["CONTROL", "Control", "The control keywords of `Control`."],
  ["OPERATORS", "Operators", "The word operators of `Operators`."],
  ["STORAGE", "Storage", "The storage words of `Storage`."],
  ["MODIFIERS", "Modifiers", "The modifiers of `Modifiers`."],
  ["CONSTANTS", "Constants", "The constants of `Constants`."],
  ["ASP_CLASSES", "AspClasses", "The ASP objects of `AspClasses`."],
  ["ASP_COLLECTIONS", "AspCollections", "The ASP collections of `AspCollections`."],
  ["ASP_CONSTANTS", "AspConstants", "The ASP properties of `AspConstants`."],
  ["ASP_FUNCTIONS", "AspFunctions", "The ASP methods of `AspFunctions`."],
  ["ASP_EVENTS", "AspEvents", "The ASP events of `AspEvents`."],
  ["FUNCTIONS", "Functions", "The functions of `Functions`."],
  ["VB_CONSTANTS", "VbConstants", "The constants of `VbConstants`."],
];
for (const [name, id, doc] of lists) {
  const list = winners(id, ordered(id));
  const buckets: string[][] = Array.from({ length: 26 }, () => []);
  for (const word of list) {
    const first = word.charCodeAt(0) - 97;
    if (first < 0 || first > 25) throw new Error(`${id}: ${word} starts with no letter`);
    buckets[first]!.push(word);
  }
  g += `\n/// ${doc}\npub(crate) const ${name}: Buckets = [\n${buckets.map((b) => `    &[${b.map(bytes).join(", ")}],`).join("\n")}\n];\n`;
}

g += startTable(patList, "any");
g += ruleCodes("Vb", "VB");
g += groupsCheck("Vb", patList.flatMap((p) => p.captures.map(([n]) => n)));

await writeRust(`${root_dir}src/vb/grammar.rs`, g);

const s = scopeFile("vb-grammar.ts", "VB", scopes);
await writeRust(`${root_dir}src/vb/scope.rs`, s);
console.log(`${ruleList.length} rules, ${patList.length} patterns, ${scopes.size} scopes`);
