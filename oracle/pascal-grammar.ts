// Writes src/pascal/scope.rs, src/pascal/grammar.rs and src/pascal/words.rs
// from the `pascal` grammar that Shiki 4.6.0 bundles
// (alefragnani/vscode-language-pascal, after textmate/pascal.tmbundle).
// The grammar has no repository: its root lists every pattern.
// grammar.rs holds the scopes each rule and pattern adds, how each rule ends
// and the patterns it tries, in the grammar's order.
// words.rs holds the words of the grammar's `\b(?i:(…))\b` alternations.
// The matchers that decide whether a pattern matches are written by hand in
// src/pascal/matchers.rs.
//
// Run with `bun run oracle/pascal-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import pascal from "@shikijs/langs@4.6.0/pascal";
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

type Node = {
  name?: string;
  match?: string;
  begin?: string;
  end?: string;
  captures?: Record<string, { name?: string }>;
  beginCaptures?: Record<string, { name?: string }>;
  endCaptures?: Record<string, { name?: string }>;
  patterns?: Node[];
};

const grammar = (pascal as unknown as { patterns: Node[] }[])[0]!;
const root_dir = new URL("..", import.meta.url).pathname;

// Scope names.

const abbreviations: Record<string, string> = {
  constant: "Const",
  storage: "Stor",
  entity: "Ent",
  variable: "Var",
  string: "Str",
  comment: "Cmt",
  keyword: "Kw",
  punctuation: "Punct",
};

/** The identifier of a scope name: its parts in camel case, without the
 * `pascal` part, and ending in `Bare` when the name has no `pascal` part. */
function scopeIdent(name: string): string {
  const parts = name.split(".");
  const bare = !parts.includes("pascal");
  let out = "";
  parts
    .filter((part) => part !== "pascal")
    .forEach((part, index) => {
      for (const piece of part.split(/[-_]/)) {
        const capture = piece.charAt(0).toUpperCase() + piece.slice(1);
        out += index === 0 ? (abbreviations[piece] ?? capture) : capture;
      }
    });
  return out + (bare ? "Bare" : "");
}

const scopes = new Map<string, string>([["Source", "source.pascal"]]);
function scopeList(name: string | undefined): string[] {
  if (name === undefined) return [];
  return name.split(" ").map((one) => {
    const ident = scopeIdent(one);
    const known = scopes.get(ident);
    if (known !== undefined && known !== one) throw new Error(`${ident}: ${known} and ${one}`);
    scopes.set(ident, one);
    return ident;
  });
}

// Rules and patterns.

/** The names of the root's patterns, in the grammar's order. */
const names = [
  "Keywords",
  "Prototype",
  "Function",
  "Variable",
  "Operator",
  "Control",
  "Directive",
  "Types",
  "Number",
  "Hex",
  "Constant",
  "Assert",
  "LineComment",
  "ParenComment",
  "BraceComment",
  "String",
  "CharCode",
];
if (names.length !== grammar.patterns.length) throw new Error("the grammar's root changed");

type Pattern = {
  id: string;
  regex: string;
  act: { role: "match"; name: string[] } | { role: "begin"; rule: string } | { role: "end" };
  captures: [number, string[]][];
};
type Rule = { id: string; name: string[]; end: string; patterns: string[] };

const patternById: Pattern[] = [];
const rules: Rule[] = [{ id: "Root", name: [], end: "", patterns: [] }];

function captureList(captures: Node["captures"]): [number, string[]][] {
  return Object.entries(captures ?? {})
    .map(([group, value]): [number, string[]] => [Number(group), scopeList(value.name)])
    .filter(([, list]) => list.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** Registers `node` under `id` and returns the id of the pattern it adds. */
function register(node: Node, id: string): string {
  if (node.match !== undefined) {
    patternById.push({
      id,
      regex: node.match,
      act: { role: "match", name: scopeList(node.name) },
      captures: captureList(node.captures),
    });
    return id;
  }
  if (node.begin === undefined || node.end === undefined)
    throw new Error(`${id}: neither a match nor a begin/end rule`);
  if (/\\[1-9]/.test(node.end)) throw new Error(`${id}: the end repeats a group`);
  patternById.push({
    id,
    regex: node.begin,
    act: { role: "begin", rule: id },
    captures: captureList(node.beginCaptures ?? node.captures),
  });
  patternById.push({
    id: `${id}End`,
    regex: node.end,
    act: { role: "end" },
    captures: captureList(node.endCaptures ?? node.captures),
  });
  const rule: Rule = { id, name: scopeList(node.name), end: `${id}End`, patterns: [] };
  rules.push(rule);
  rule.patterns = (node.patterns ?? []).map((inner, index) => register(inner, `${id}At${index}`));
  return id;
}

rules[0]!.patterns = grammar.patterns.map((node, index) => register(node, names[index]!));

// No pattern may take a blank after the first of a run: the lexer skips the
// rest of a run (`BlankRuns::Skip`).
for (const p of patternById) {
  if (takesOneBlank(p.regex))
    throw new Error(`${p.id} takes one blank: the lexer needs BlankRuns::ToLast`);
}

// Words.

/** The lowercase words of one `(?i:(…))` alternation, sorted. Each word is
 * made of `\w`, so the alternation between `\b` and `\b` matches a whole word
 * that is one of them. */
function wordList(id: string, alternation: string): string[] {
  const all = expand(alternation).map((word) => word.toLowerCase());
  if (all.some((word) => !/^\w+$/.test(word))) throw new Error(`${id}: a word that is not all \\w`);
  return [...new Set(all)].sort();
}

const words = new Map<string, string[]>();
grammar.patterns.forEach((node, index) => {
  const id = names[index]!;
  const regex = node.match ?? "";
  const whole = /^\\b\(\?i:\((.*)\)\)\\b$/s.exec(regex)?.[1];
  if (whole !== undefined) {
    words.set(id, wordList(id, whole));
    return;
  }
  const head = /^\\b\(\?i:\(([a-z|]*)\)\)\\b/.exec(regex)?.[1];
  if (head !== undefined) words.set(id, wordList(id, head));
});
const prototype = grammar.patterns[names.indexOf("Prototype")]!.match!;
const ahead = /\(\?=\(\?i:([a-z|]*)\)\)$/.exec(prototype)?.[1];
if (ahead === undefined) throw new Error("the prototype's lookahead changed");
words.set("PrototypeAhead", wordList("PrototypeAhead", ahead));
for (const id of [
  "Keywords",
  "Prototype",
  "Function",
  "Variable",
  "Operator",
  "Control",
  "Types",
  "Constant",
  "Assert",
]) {
  if (!words.has(id)) throw new Error(`${id}: its word list changed shape`);
}

// Output.

const list = (scopeNames: string[]) =>
  scopeNames.length ? `scopes![${scopeNames.map((n) => `S::${n}`).join(", ")}]` : "S::NO_SCOPES";

let g = `//! The grammar's rules as tables. Generated by \`oracle/pascal-grammar.ts\` from
//! the \`pascal\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::Pascal;
use super::scope::S;
use crate::rules;
use crate::scope::scopes;

pub(crate) type Act = rules::Act<Pascal>;
pub(crate) type PatternDefinition = rules::PatternDefinition<Pascal>;
pub(crate) type RuleDefinition = rules::RuleDefinition<Pascal>;
pub(crate) use crate::rules::Start;

/// The patterns a capture group is tokenized with: none in this grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Nested {}

`;

g += `/// A rule of the grammar that can be open at the end of a line.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub(crate) enum Rule {\n`;
for (const r of rules) g += `    ${r.id},\n`;
g += `}\n\nimpl Rule {\n    /// Every rule, in code order.\n    pub(crate) const ALL: [Rule; ${rules.length}] = [\n`;
for (const r of rules) g += `        Rule::${r.id},\n`;
g += `    ];\n\n    pub(crate) fn definition(self) -> RuleDefinition {\n        match self {\n`;
g += arms(
  rules.map((r): [string, string] => [
    r.id,
    `RuleDefinition {\n                name: ${list(r.name)},\n                content: S::NO_SCOPES,\n                end: ${r.end ? `Some(Pattern::${r.end})` : "None"},\n                backref: None,\n                patterns: &[${r.patterns.map((p) => `Pattern::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pattern {\n`;
const docs = (s: string) => (s.length > 56 ? "…" : s);
for (const p of patternById) {
  g += `    /// The ${p.act.role} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pattern {\n    /// Every pattern, for the tool that compares the matchers with Oniguruma.\n    #[cfg(all(test, feature = "json"))]\n    pub(crate) const ALL: [Pattern; ${patternById.length}] = [\n${patternById.map((p) => `        Pattern::${p.id},\n`).join("")}    ];\n\n${tooLong}    pub(crate) fn definition(self) -> PatternDefinition {\n        match self {\n`;
g += arms(
  patternById.map((p): [string, string] => {
    const act =
      p.act.role === "match"
        ? `Act::Match(${list(p.act.name)})`
        : p.act.role === "begin"
          ? `Act::Begin(Rule::${p.act.rule})`
          : "Act::End";
    const captures = p.captures.map(([n, scopeNames]) => `(${n}, ${list(scopeNames)})`).join(", ");
    return [
      p.id,
      `PatternDefinition {\n                act: ${act},\n                captures: &[${captures}],\n                nested: &[],\n            }`,
    ];
  }),
  "Pattern",
);
g += `        }\n    }\n}\n`;
g += startTable(patternById);
g += ruleCodes("Pascal", "PASCAL");
g += groupsCheck(
  "Pascal",
  patternById.flatMap((p) => p.captures.map(([n]) => n)),
);
await writePatterns("pascal", patternById);
await writeRust(`${root_dir}src/pascal/grammar.rs`, g);

const bytes = (word: string) => `b"${word}"`;
const constName = (name: string) => name.replace(/([a-z])([A-Z])/g, "$1_$2").toUpperCase();
let w = `//! The words of the grammar's \`\\b(?i:(…))\\b\` alternations, lowercase and
//! sorted. Generated by \`oracle/pascal-grammar.ts\` from the \`pascal\` grammar
//! Shiki 4.6.0 bundles; do not edit.
`;
for (const [name, list] of words) {
  const what =
    name === "PrototypeAhead"
      ? "the lookahead of the `Prototype` pattern"
      : `the \`${name}\` pattern`;
  w += `\n/// The words of ${what}.\npub(super) const ${constName(name)}: &[&[u8]] = &[${list.map(bytes).join(", ")}];\n`;
}
await writeRust(`${root_dir}src/pascal/words.rs`, w);

await writeRust(`${root_dir}src/pascal/scope.rs`, scopeFile("pascal-grammar.ts", "PASCAL", scopes));

// With RX_DUMP set to a file, writes each pattern for oracle/onig.ts.
if (Bun.env["RX_DUMP"]) {
  await Bun.write(
    Bun.env["RX_DUMP"],
    patternById
      .map((p) => JSON.stringify({ id: p.id, regex: p.regex, groups: p.captures.map(([n]) => n) }))
      .join("\n") + "\n",
  );
}
console.log(
  `${rules.length} rules, ${patternById.length} patterns, ${scopes.size} scopes, ${words.size} word lists`,
);
