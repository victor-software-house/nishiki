// Writes src/jinja/scope.rs and src/jinja/grammar.rs from the `jinja` grammar
// that Shiki 4.6.0 bundles (samuelcolvin/jinjahtml-vscode, MIT).
// It writes the scope names.
// For every rule it writes the scopes the rule adds, how it ends and the
// patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// The matchers that decide whether a pattern matches are written by hand in
// src/jinja/matchers.
//
// Run with `bun run oracle/jinja-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import jinja from "@shikijs/langs@4.6.0/jinja";
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

const grammar = (jinja as unknown as Grammar[]).find((lang) => lang.scopeName === "source.jinja")!;
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
  if (parts[parts.length - 1] === "jinja") parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const capture = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? capture) : capture;
    }
  });
  return out || "Plain";
}

const scopes = new Map<string, string>([["Source", "source.jinja"]]);
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

type Pattern = {
  id: string;
  regex: string;
  act: { role: "match"; name: string[] } | { role: "begin"; rule: string } | { role: "end" };
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

const patternById = new Map<string, Pattern>();
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
  const clash = patternById.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (patternById.has(id)) return id;
  if (node.match !== undefined) {
    patternById.set(id, {
      id,
      regex: node.match,
      act: { role: "match", name: scopeList(node.name) },
      captures: captureList(node.captures),
    });
    return id;
  }
  if (node.begin === undefined || node.end === undefined)
    throw new Error(`${id}: neither a match nor a begin/end rule`);
  patternById.set(id, {
    id,
    regex: node.begin,
    act: { role: "begin", rule: id },
    captures: captureList(node.beginCaptures ?? node.captures),
  });
  patternById.set(`${id}End`, {
    id: `${id}End`,
    regex: node.end,
    act: { role: "end" },
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
      out.push(...expandNode(node, listName(context, index), false));
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

/** The names of the root's own patterns and of `#expression`'s, by index. */
const listNames: Record<string, string[]> = {
  Root: ["Raw", "", "Output", "Statement"],
  Expression: [
    "Block",
    "Filter",
    "Is",
    "TagName",
    "Keywords",
    "Constants",
    "Builtins",
    "Name",
    "Arithmetic",
    "FilterCall",
    "Member",
    "Brackets",
    "Parens",
    "Braces",
    "Punctuation",
    "Comparison",
    "Assignment",
    "DoubleString",
    "SingleString",
    "Regex",
  ],
};
for (const [list, names] of Object.entries(listNames)) {
  const nodes = list === "Root" ? root.patterns! : repository[list.toLowerCase()]!.patterns!;
  if (names.length !== nodes.length) throw new Error(`the patterns of ${list} changed`);
}
function listName(context: string, index: number): string {
  const name = listNames[context]?.[index];
  return name ? name : `${context}At${index}`;
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

let g = `//! The grammar's rules as tables. Generated by \`oracle/jinja-grammar.ts\` from
//! the \`jinja\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::Jinja;
use super::scope::S;
use crate::rules;
use crate::scope::scopes;

pub(crate) type Act = rules::Act<Jinja>;
pub(crate) type PatternDefinition = rules::PatternDefinition<Jinja>;
pub(crate) type RuleDefinition = rules::RuleDefinition<Jinja>;
pub(crate) use crate::rules::Start;

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
    `RuleDefinition {\n                name: ${scText(r.name)},\n                content: ${scText(r.content)},\n                end: ${r.end ? `Some(Pattern::${r.end})` : "None"},\n                backref: ${r.backref === null ? "None" : `Some(${r.backref})`},\n                patterns: &[${r.patterns.map((p) => `Pattern::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n\n    /// What a rule carries from line to line: the begin capture group its\n    /// end repeats.\n    #[inline]\n    pub(crate) fn carried_state(self) -> rules::CarriedState {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    r.backref === null ? "rules::CarriedState::None" : `rules::CarriedState::Backref(${r.backref})`,
  ]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pattern {\n`;
// A long regex, or one with characters past ASCII, is elided.
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 || /[^\x20-\x7e]/.test(one) ? `…` : one;
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
    const captures = p.captures.map(([n, names]) => `(${n}, ${scText(names)})`).join(", ");
    return [
      p.id,
      `PatternDefinition {\n                act: ${act},\n                captures: &[${captures}],\n                nested: &[],\n            }`,
    ];
  }),
  "Pattern",
);
g += `        }\n    }\n}\n`;

g += startTable(patternList, "any");
g += ruleCodes("Jinja", "JINJA");
g += groupsCheck(
  "Jinja",
  patternList.flatMap((p) => p.captures.map(([n]) => n)),
);

await writeRust(`${root_dir}src/jinja/grammar.rs`, g);

const s = scopeFile("jinja-grammar.ts", "JINJA", scopes);
await writeRust(`${root_dir}src/jinja/scope.rs`, s);
// With RX_DUMP set to a file, writes each pattern for oracle/onig.ts.
if (Bun.env["RX_DUMP"]) {
  await Bun.write(
    Bun.env["RX_DUMP"],
    patternList
      .map((p) => JSON.stringify({ id: p.id, regex: p.regex, groups: p.captures.map(([n]) => n) }))
      .join("\n") + "\n",
  );
}
console.log(`${ruleList.length} rules, ${patternList.length} patterns, ${scopes.size} scopes`);
