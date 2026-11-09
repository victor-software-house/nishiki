// Writes src/gdscript/scope.rs and src/gdscript/grammar.rs from the
// `gdscript` grammar that Shiki 4.6.0 bundles (godotengine/godot-vscode-plugin,
// MIT).
// It writes the scope names.
// For every rule it writes the scopes the rule adds, how it ends and the
// patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// The matchers that decide whether a pattern matches are written by hand in
// src/gdscript/matchers.
//
// Run with `bun run oracle/gdscript-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import gdscript from "@shikijs/langs@4.6.0/gdscript";
import { expand as spell } from "./alternation.ts";
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

const grammar = (gdscript as unknown as Grammar[]).find((lang) => lang.scopeName === "source.gdscript")!;
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
  if (parts[parts.length - 1] === "gdscript") parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
    }
  });
  return out || "Plain";
}

const scopes = new Map<string, string>([["Source", "source.gdscript"]]);
/** The identifier of scope `name`.
 * The grammar has `keyword.control.flow` beside
 * `keyword.control.flow.gdscript`, so a name without the language suffix
 * that would take another's identifier ends in `Bare`. */
function scope(name: string): string {
  let ident = scopeIdent(name);
  const taken = scopes.get(ident);
  if (taken !== undefined && taken !== name && !name.endsWith(".gdscript")) ident += "Bare";
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
    if (node.include === "$self") {
      out.push(...expandNode(root, "Root", true));
    } else if (node.include !== undefined) {
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

let g = `//! The grammar's rules as tables. Generated by \`oracle/gdscript-grammar.ts\` from
//! the \`gdscript\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pat\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::Gdscript;
use super::scope::S;
use crate::rules;
use crate::scope::scopes;

pub(crate) type Act = rules::Act<Gdscript>;
pub(crate) type PatData = rules::PatData<Gdscript>;
pub(crate) type RuleData = rules::RuleData<Gdscript>;
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

// The grammar's word alternations, as lists the matchers look a word up in.
// Each is followed by `\b`, so a word matches only as a whole.
/** The words of the alternation that `inner` captures from pattern `id`. */
function words(id: string, inner: RegExp): string[] {
  const regex = pats.get(id)!.regex;
  const found = inner.exec(regex)?.[1];
  if (found === undefined) throw new Error(`${id} changed shape`);
  return [...new Set(spell(found.replaceAll("??", "?")))];
}
const bytes = (w: string) => `b"${w}"`;
const wordList = (name: string, doc: string, list: string[]) =>
  `\n/// ${doc}\npub(crate) const ${name}: &[&[u8]] = &[${list.map(bytes).join(", ")}];\n`;
g += wordList(
  "ANNOTATIONS",
  "The annotations after `@` in `Annotations`.",
  words("Annotations", /^\(@\)\(([a-z_0-9|]*)\)\\b$/),
);
g += wordList(
  "BUILTIN_CLASSES",
  "The builtin classes of `BuiltinClasses`, with each lazy `i??` written out.",
  words("BuiltinClasses", /^\(\?<!\[\^\.\]\\\.\|:\)\\b\(([A-Za-z0-9|?]*)\)\\b$/),
);

g += wordList("KEYWORDS", "The keywords of `Keywords`.", words("Keywords", /^\\b\(\?:([a-z_|]*)\)\\b$/));
g += wordList("CONTROL_FLOW", "The keywords of `ControlFlow`.", words("ControlFlow", /^\\b\(\?:([a-z_|]*)\)\\b$/));
g += wordList(
  "STATEMENT_KEYWORDS",
  "The keywords of `StatementKeywordAt0`.",
  words("StatementKeywordAt0", /^\\b\(\?<!\\\.\)\(([a-z_|]*)\)\\b$/),
);

g += startTable(patList, "any");
g += ruleCodes("Gdscript", "GDSCRIPT");
g += groupsCheck("Gdscript", patList.flatMap((p) => p.captures.map(([n]) => n)));

await writeRust(`${root_dir}src/gdscript/grammar.rs`, g);

const s = scopeFile("gdscript-grammar.ts", "GDSCRIPT", scopes);
await writeRust(`${root_dir}src/gdscript/scope.rs`, s);
console.log(`${ruleList.length} rules, ${patList.length} patterns, ${scopes.size} scopes`);
