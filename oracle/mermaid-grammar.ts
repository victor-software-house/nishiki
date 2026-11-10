// Writes src/mermaid/scope.rs and src/mermaid/grammar.rs from the `mermaid`
// grammar that Shiki 4.6.0 bundles
// (bpruitt-goddard/vscode-mermaid-syntax-highlight, MIT).
//
// grammar.rs holds the scopes each rule and pattern adds, how each rule ends
// and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// The diagrams repeat many patterns word for word, such as the C4 diagrams'
// `Person` and `Rel`.
// A pattern or rule that repeats another, with the same scopes, is the same
// `Pat` or `Rule`, so it has one matcher.
//
// The matchers that decide whether a pattern matches are written by hand in
// src/mermaid/matchers.
//
// Run with `bun run oracle/mermaid-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import mermaid from "@shikijs/langs@4.6.0/mermaid";
import { arms, groupsCheck, oneBlankTable, ruleCodes, scopeFile, startOf, startTable, tooLong, writeRust } from "./rule-grammar.ts";

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
  applyEndPatternLast?: boolean | number;
};

const grammar = (mermaid as unknown as { name: string; scopeName: string; repository: Record<string, Node>; patterns: Node[] }[]).find(
  (lang) => lang.name === "mermaid",
)!;
const repository = grammar.repository;

const root_dir = new URL("..", import.meta.url).pathname;

// Scope names.

const abbreviations: Record<string, string> = {
  punctuation: "P",
  keyword: "Kw",
  constant: "Const",
  entity: "Ent",
  variable: "Var",
  support: "Supp",
  string: "Str",
  comment: "Cmt",
  meta: "Meta",
  storage: "Stor",
};

function scopeIdent(name: string): string {
  const parts = name.split(".");
  // A name without the language's suffix, such as `variable`, ends in `Bare`.
  const suffix = parts[parts.length - 1] === "mermaid" && parts.length > 1 ? "" : "Bare";
  if (!suffix) parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
    }
  });
  return out + suffix;
}

const scopes = new Map<string, string>([["Source", grammar.scopeName]]);
function scope(name: string): string {
  const ident = scopeIdent(name);
  const known = scopes.get(ident);
  if (known !== undefined && known !== name) throw new Error(`${ident}: ${known} and ${name}`);
  scopes.set(ident, name);
  return ident;
}
function scopeList(name: string | undefined): string[] {
  return name === undefined ? [] : name.split(" ").map(scope);
}

// Rules and patterns.

const pascal = (key: string) =>
  key
    .split(/[-_.]/)
    .map((piece) => piece.charAt(0).toUpperCase() + piece.slice(1))
    .join("");

/** The diagrams, by the position of their rule in `#mermaid`. */
const diagrams = [
  "Architecture",
  "C4Component",
  "C4Container",
  "C4Deployment",
  "C4Dynamic",
  "ClassDiagram",
  "ErDiagram",
  "Gantt",
  "GitGraph",
  "Flowchart",
  "Mindmap",
  "Pie",
  "QuadrantChart",
  "RequirementDiagram",
  "SequenceDiagram",
  "StateDiagram",
  "Journey",
  "XyChart",
];

type Pat = {
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
  patterns: string[];
};

const pats = new Map<string, Pat>();
const rules = new Map<string, Rule>();
/** The id each node registered under, by the node's text. */
const seen = new Map<string, string>();

function captureList(captures: Captures | undefined): [number, string[]][] {
  if (!captures) return [];
  for (const [group, value] of Object.entries(captures)) {
    if (value.patterns) throw new Error(`capture ${group}: nested patterns`);
  }
  return Object.entries(captures)
    .map(([group, value]): [number, string[]] => [Number(group), scopeList(value.name)])
    .filter(([, list]) => list.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** Registers a match or begin/end rule under `id`, unless the same node is
 * registered already; returns the pattern id. */
function register(node: Node, id: string): string {
  const key = JSON.stringify(node);
  const known = seen.get(key);
  if (known !== undefined) return known;
  if (pats.has(id)) throw new Error(`${id} names two rules`);
  seen.set(key, id);
  if (node.applyEndPatternLast) throw new Error(`${id}: applyEndPatternLast`);
  if (node.match !== undefined) {
    pats.set(id, {
      id,
      regex: node.match,
      act: { role: "match", name: scopeList(node.name) },
      captures: captureList(node.captures),
    });
    return id;
  }
  if (node.begin !== undefined && node.end !== undefined) {
    pats.set(id, {
      id,
      regex: node.begin,
      act: { role: "begin", rule: id },
      captures: captureList(node.beginCaptures ?? node.captures),
    });
    const end = `${id}End`;
    const endNode = { end: node.end, captures: node.endCaptures ?? node.captures };
    const endKey = JSON.stringify(endNode);
    const endKnown = seen.get(endKey);
    if (endKnown === undefined) {
      seen.set(endKey, end);
      pats.set(end, { id: end, regex: node.end, act: { role: "end" }, captures: captureList(endNode.captures) });
    }
    const rule: Rule = {
      id,
      name: scopeList(node.name),
      content: scopeList(node.contentName),
      end: endKnown ?? end,
      patterns: [],
    };
    rules.set(id, rule);
    rule.patterns = flatten(node.patterns ?? [], id);
    return id;
  }
  throw new Error(`${id}: neither a match nor a begin/end rule`);
}

/** The pattern ids a pattern list expands to, `include`s flattened. */
function flatten(list: Node[], context: string): string[] {
  const out: string[] = [];
  list.forEach((node, index) => {
    if (node.include !== undefined) {
      if (!node.include.startsWith("#")) throw new Error(`external include ${node.include}`);
      const key = node.include.slice(1);
      const target = repository[key];
      if (!target) throw new Error(`unknown include ${key}`);
      out.push(...expand(target, pascal(key)));
    } else {
      const id = context === "Mermaid" ? diagrams[index]! : `${context}At${index}`;
      out.push(...expand(node, id));
    }
  });
  return out;
}

/** A repository entry that lists one pattern takes the entry's name. */
function expand(node: Node, id: string): string[] {
  if (node.match !== undefined || node.begin !== undefined) return [register(node, id)];
  if (node.patterns?.length === 1 && node.patterns[0]!.include === undefined) {
    return [register(node.patterns[0]!, id)];
  }
  if (node.patterns) return flatten(node.patterns, id);
  throw new Error(`${id}: empty rule`);
}

const rootRule: Rule = { id: "Root", name: [], content: [], end: "", patterns: [] };
rules.set("Root", rootRule);
rootRule.patterns = flatten(grammar.patterns, "Root");

// Output: the tables.

const list = (ids: string[]) => (ids.length ? `scopes![${ids.map((n) => `S::${n}`).join(", ")}]` : "S::NO_SCOPES");

let g = `//! The grammar's rules as tables. Generated by \`oracle/mermaid-grammar.ts\`
//! from the \`mermaid\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pat\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::Mermaid;
use super::scope::S;
use crate::rules;
use crate::scope::scopes;

pub(crate) type Act = rules::Act<Mermaid>;
pub(crate) type PatData = rules::PatData<Mermaid>;
pub(crate) type RuleData = rules::RuleData<Mermaid>;
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
    `RuleData {\n                name: ${list(r.name)},\n                content: ${list(r.content)},\n                end: ${r.end ? `Some(Pat::${r.end})` : "None"},\n                backref: None,\n                patterns: &[${r.patterns.map((p) => `Pat::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pat {\n`;
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 || /[^\x20-\x7e]/.test(one) ? `…` : one;
};
for (const p of patList) {
  g += `    /// The ${p.act.role} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pat {\n    /// Every pattern, for the tool that compares the matchers with Oniguruma.\n    #[cfg(test)]\n    pub(crate) const ALL: [Pat; ${patList.length}] = [\n${patList.map((p) => `        Pat::${p.id},\n`).join("")}    ];\n\n${tooLong}    pub(crate) fn data(self) -> PatData {\n        match self {\n`;
g += arms(
  patList.map((p): [string, string] => {
    const act =
      p.act.role === "match"
        ? `Act::Match(${list(p.act.name)})`
        : p.act.role === "begin"
          ? `Act::Begin(Rule::${p.act.rule})`
          : "Act::End";
    const caps = p.captures.map(([n, ids]) => `(${n}, ${list(ids)})`).join(", ");
    return [p.id, `PatData {\n                act: ${act},\n                captures: &[${caps}],\n                nested: &[],\n            }`];
  }),
  "Pat",
);
g += `        }\n    }\n}\n`;

// Only an end anchored at `\G` may start anywhere: the anchor can stand at
// any byte.
for (const p of patList) {
  if (startOf(p.regex).any && !p.regex.includes("\\G")) {
    throw new Error(`${p.id}: the first-byte analysis does not read ${p.regex}`);
  }
}
g += startTable(patList);
g += oneBlankTable(patList);
g += ruleCodes("Mermaid", "MERMAID");
g += groupsCheck("Mermaid", patList.flatMap((p) => p.captures.map(([n]) => n)));

await writeRust(`${root_dir}src/mermaid/grammar.rs`, g);
await writeRust(`${root_dir}src/mermaid/scope.rs`, scopeFile("mermaid-grammar.ts", "MERMAID", scopes));

// With RX_DUMP set to a file, writes each pattern for oracle/onig.ts.
if (Bun.env["RX_DUMP"]) {
  await Bun.write(
    Bun.env["RX_DUMP"],
    patList.map((p) => JSON.stringify({ id: p.id, regex: p.regex, groups: p.captures.map(([n]) => n) })).join("\n") + "\n",
  );
}

console.log(`${ruleList.length} rules, ${patList.length} patterns, ${scopes.size} scopes`);
