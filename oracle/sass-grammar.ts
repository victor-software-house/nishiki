// Writes src/sass/scope.rs and src/sass/grammar.rs from the `sass` grammar that
// Shiki 4.6.0 bundles (TheRealSyler/vscode-sass-indented, MIT).
// It writes the scope names.
// For every rule it writes the scopes the rule adds, how it ends and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// The matchers that decide whether a pattern matches are written by hand in src/sass/matchers.
//
// Run with `bun run oracle/sass-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import sass from "@shikijs/langs@4.6.0/sass";
import { arms, groupsCheck, oneBlankTable, ruleCodes, scopeFile, startTable, tooLong, writeRust } from "./rule-grammar.ts";

type Captures = Record<string, { name?: string; patterns?: unknown[] }>;
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

const grammar = (sass as unknown as { scopeName: string; repository: Record<string, Node>; patterns: Node[] }[]).find(
  (lang) => lang.scopeName === "source.sass",
)!;
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
  if (parts[parts.length - 1] === "sass") parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
    }
  });
  return out;
}

const scopes = new Map<string, string>([["Source", "source.sass"]]);
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
  /** The begin capture group that the end pattern repeats, when it does. */
  backref: number | null;
  patterns: string[];
};

const pats = new Map<string, Pat>();
const rules = new Map<string, Rule>();

function captureList(captures: Captures | undefined): [number, string[]][] {
  if (!captures) return [];
  return Object.entries(captures)
    .map(([group, value]): [number, string[]] => {
      if (value.patterns !== undefined) throw new Error(`capture ${group}: patterns of its own`);
      return [Number(group), scopeList(value.name)];
    })
    .filter(([, names]) => names.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** Registers a match or begin/end rule under `id`; returns the pattern id. */
function register(node: Node, id: string): string {
  const clash = pats.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (node.match !== undefined) {
    if (!pats.has(id)) {
      pats.set(id, {
        id,
        regex: node.match,
        act: { role: "match", name: scopeList(node.name) },
        captures: captureList(node.captures),
      });
    }
    return id;
  }
  if (node.begin !== undefined && node.end !== undefined) {
    if (!pats.has(id)) {
      pats.set(id, {
        id,
        regex: node.begin,
        act: { role: "begin", rule: id },
        captures: captureList(node.beginCaptures ?? node.captures),
      });
      pats.set(`${id}End`, {
        id: `${id}End`,
        regex: node.end,
        act: { role: "end" },
        captures: captureList(node.endCaptures ?? node.captures),
      });
      const backrefs = [...new Set([...node.end.matchAll(/\\([1-9])/g)].map((m) => Number(m[1])))];
      if (backrefs.length > 1) throw new Error(`${id}: the end repeats more than one group`);
      const rule: Rule = {
        id,
        name: scopeList(node.name),
        content: scopeList(node.contentName),
        end: `${id}End`,
        backref: backrefs[0] ?? null,
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
      out.push(...expand(node, `${context}At${index}`));
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
rules.set("Root", { id: "Root", name: [], content: [], end: "", backref: null, patterns: rootPatterns });

// Output.

const list = (names: string[]) => (names.length ? `scopes![${names.map((n) => `S::${n}`).join(", ")}]` : "S::NO_SCOPES");

let g = `//! The grammar's rules as tables. Generated by \`oracle/sass-grammar.ts\` from
//! the \`sass\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pat\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::Sass;
use super::scope::S;
use crate::scope::scopes;
use crate::rules;

pub(crate) type Act = rules::Act<Sass>;
pub(crate) type PatData = rules::PatData<Sass>;
pub(crate) type RuleData = rules::RuleData<Sass>;
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
    `RuleData {\n                name: ${list(r.name)},\n                content: ${list(r.content)},\n                end: ${r.end ? `Some(Pat::${r.end})` : "None"},\n                backref: ${r.backref === null ? "None" : `Some(${r.backref})`},\n                patterns: &[${r.patterns.map((p) => `Pat::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n\n    /// What a rule carries from line to line.\n    /// A rule whose end repeats a begin capture group carries that group.\n    #[inline]\n    pub(crate) fn payload(self) -> rules::Payload {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    r.backref === null
        ? "rules::Payload::None"
        : `rules::Payload::Backref(${r.backref})`,
  ]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pat {\n`;
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 ? `…` : one;
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
    const caps = p.captures.map(([n, names]) => `(${n}, ${list(names)})`).join(", ");
    return [p.id, `PatData {\n                act: ${act},\n                captures: &[${caps}],\n                nested: &[],\n            }`];
  }),
  "Pat",
);
g += `        }\n    }\n}\n`;

g += startTable(patList, "any");
g += oneBlankTable(patList, "any");
g += groupsCheck("Sass", patList.flatMap((p) => p.captures.map(([n]) => n)));
g += ruleCodes("Sass", "SASS");

await writeRust(`${root_dir}src/sass/grammar.rs`, g);

const s = scopeFile("sass-grammar.ts", "SASS", scopes);
await writeRust(`${root_dir}src/sass/scope.rs`, s);

// With RX_DUMP set to a file, writes each pattern for oracle/onig.ts. An end
// that repeats a group of its begin, as `]\1]` does, is left out: its text
// comes from the line that opened the rule, which a probe case does not have.
if (Bun.env["RX_DUMP"]) {
  await Bun.write(
    Bun.env["RX_DUMP"],
    patList
      .filter((p) => !(p.act.role === "end" && /\\[1-9]/.test(p.regex)))
      .map((p) => JSON.stringify({ id: p.id, regex: p.regex, groups: p.captures.map(([n]) => n) }))
      .join("\n") + "\n",
  );
}
console.log(`${ruleList.length} rules, ${patList.length} patterns, ${scopes.size} scopes`);
