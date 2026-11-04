// Writes src/cmake/scope.rs, src/cmake/grammar.rs and src/cmake/words.rs from
// the `cmake` grammar that Shiki 4.6.0 bundles (twxs/vs.language.cmake, MIT).
// The grammar has no repository: its root lists every pattern.
// grammar.rs holds the scopes each rule and pattern adds, how each rule ends
// and the patterns it tries, in the grammar's order.
// words.rs holds the words of the grammar's `\b(?i:…)\b` alternations.
// The matchers that decide whether a pattern matches are written by hand in
// src/cmake/matchers.
//
// Run with `bun run oracle/cmake-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import cmake from "@shikijs/langs@4.6.0/cmake";
import { expand } from "./alternation.ts";
import { arms, groupsCheck, ruleCodes, scopeFile, startTable, writeRust } from "./rule-grammar.ts";

type Node = {
  name?: string;
  match?: string;
  begin?: string;
  end?: string;
  captures?: Record<string, { name?: string }>;
  patterns?: Node[];
};

const grammar = (cmake as unknown as { patterns: Node[] }[])[0]!;
const root_dir = new URL("..", import.meta.url).pathname;

// Scope names.

const abbreviations: Record<string, string> = {
  constant: "Const",
  invalid: "Inv",
  storage: "Stor",
  entity: "Ent",
  variable: "Var",
  string: "Str",
  comment: "Cmt",
  keyword: "Kw",
};

function scopeIdent(name: string): string {
  const parts = name.split(".");
  // A name without the language's suffix, such as `constant.character.escape`,
  // ends in `Bare`.
  const suffix = parts[parts.length - 1] === "cmake" ? "" : "Bare";
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

const scopes = new Map<string, string>([["Source", "source.cmake"]]);
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
  "Constants",
  "Keywords",
  "Commands",
  "Variables",
  "Expansion",
  "EnvExpansion",
  "TargetVariables",
  "CmakeVariables",
  "BracketComment",
  "BracketArgument",
  "LineComment",
  "CacheProperties",
  "SourceProperties",
  "TestProperties",
  "DirectoryProperties",
  "GlobalProperties",
  "TargetProperties",
  "EscapedString",
  "String",
  "BuildName",
  "CxxFlags",
];
if (names.length !== grammar.patterns.length) throw new Error("the grammar's root changed");

type Pat = {
  id: string;
  regex: string;
  act: { kind: "match"; name: string[] } | { kind: "begin"; rule: string } | { kind: "end" };
  captures: [number, string[]][];
};
type Rule = { id: string; name: string[]; end: string; backref: number | null; patterns: string[] };

const pats: Pat[] = [];
const rules: Rule[] = [{ id: "Root", name: [], end: "", backref: null, patterns: [] }];

function captureList(captures: Node["captures"]): [number, string[]][] {
  return Object.entries(captures ?? {})
    .map(([group, value]): [number, string[]] => [Number(group), scopeList(value.name)])
    .filter(([, list]) => list.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** Registers `node` under `id` and returns the id of the pattern it adds. */
function register(node: Node, id: string): string {
  if (node.match !== undefined) {
    pats.push({ id, regex: node.match, act: { kind: "match", name: scopeList(node.name) }, captures: captureList(node.captures) });
    return id;
  }
  if (node.begin === undefined || node.end === undefined) throw new Error(`${id}: neither a match nor a begin/end rule`);
  pats.push({ id, regex: node.begin, act: { kind: "begin", rule: id }, captures: captureList(node.captures) });
  pats.push({ id: `${id}End`, regex: node.end, act: { kind: "end" }, captures: captureList(node.captures) });
  const backrefs = [...new Set([...node.end.matchAll(/\\([1-9])/g)].map((m) => Number(m[1])))];
  if (backrefs.length > 1) throw new Error(`${id}: the end repeats more than one group`);
  const rule: Rule = { id, name: scopeList(node.name), end: `${id}End`, backref: backrefs[0] ?? null, patterns: [] };
  rules.push(rule);
  rule.patterns = (node.patterns ?? []).map((inner, index) => register(inner, `${id}At${index}`));
  return id;
}

rules[0]!.patterns = grammar.patterns.map((node, index) => register(node, names[index]!));

// Words.

/** The words of one `\b(?i:…)\b` alternation, lowercase:
 * - `exact`, the plain words, sorted;
 * - `globs`, the words with `\w+` in them, as the literal parts between which
 *   each `\w+` stands;
 * - `literals`, the words with characters that are not `\w`, which the
 *   alternation reaches past the end of a word. */
type Words = { exact: string[]; globs: string[][]; literals: string[] };

const words = new Map<string, Words>();
grammar.patterns.forEach((node, index) => {
  const alternation = node.match?.match(/^(?:\^\\s\*)?\\b\(\?i:(.*)\)\\b$/s)?.[1];
  if (alternation === undefined) return;
  const all = expand(alternation.replaceAll("\\w+", "*")).map((word) => word.toLowerCase());
  const exact = [...new Set(all.filter((word) => /^\w+$/.test(word)))].sort();
  const globs = [...new Set(all.filter((word) => word.includes("*")))].map((word) => word.split("*"));
  const literals = all.filter((word) => !/^[\w*]+$/.test(word));
  // The lexer compares a word with `\w+` on its lowered letters, where `ß` is
  // `ss` and `ﬁ` is `fi`. A part whose edge next to a `\w+` is one of those
  // letters could take half of a character's letters.
  for (const parts of globs) {
    parts.forEach((part, i) => {
      const edges = [i > 0 ? part[0] : undefined, i < parts.length - 1 ? part[part.length - 1] : undefined];
      if (edges.some((edge) => edge !== undefined && "filst".includes(edge))) {
        throw new Error(`${names[index]}: ${parts.join("*")} has a fold letter next to a \\w+`);
      }
    });
  }
  words.set(names[index]!, { exact, globs, literals });
});

// Output.

const list = (scopeNames: string[]) =>
  scopeNames.length ? `scopes![${scopeNames.map((n) => `S::${n}`).join(", ")}]` : "S::NO_SCOPES";

let g = `//! The grammar's rules as tables. Generated by \`oracle/cmake-grammar.ts\` from
//! the \`cmake\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pat\`] is one regular expression of the grammar; its doc comment says
//! which. Each rule lists the patterns it tries, in the grammar's order, so
//! the first that matches at the earliest position wins as in a \`TextMate\`
//! engine.

use super::engine::CMake;
use super::scope::S;
use crate::rules;
use crate::scope::scopes;

pub(crate) type Act = rules::Act<CMake>;
pub(crate) type PatData = rules::PatData<CMake>;
pub(crate) type RuleData = rules::RuleData<CMake>;
pub(crate) use crate::rules::Start;

/// The patterns a capture group is tokenized with: none in this grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Nested {}

`;

g += `/// A rule of the grammar that can be open at the end of a line.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub(crate) enum Rule {\n`;
for (const r of rules) g += `    ${r.id},\n`;
g += `}\n\nimpl Rule {\n    /// Every rule, in code order.\n    pub(crate) const ALL: [Rule; ${rules.length}] = [\n`;
for (const r of rules) g += `        Rule::${r.id},\n`;
g += `    ];\n\n    pub(crate) fn data(self) -> RuleData {\n        match self {\n`;
g += arms(
  rules.map((r): [string, string] => [
    r.id,
    `RuleData {\n                name: ${list(r.name)},\n                content: S::NO_SCOPES,\n                end: ${r.end ? `Some(Pat::${r.end})` : "None"},\n                backref: ${r.backref === null ? "None" : `Some(${r.backref})`},\n                patterns: &[${r.patterns.map((p) => `Pat::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pat {\n`;
const docs = (s: string) => (s.length > 56 ? "…" : s);
for (const p of pats) {
  g += `    /// The ${p.act.kind} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pat {\n    pub(crate) fn data(self) -> PatData {\n        match self {\n`;
g += arms(
  pats.map((p): [string, string] => {
    const act =
      p.act.kind === "match" ? `Act::Match(${list(p.act.name)})` : p.act.kind === "begin" ? `Act::Begin(Rule::${p.act.rule})` : "Act::End";
    const caps = p.captures.map(([n, scopeNames]) => `(${n}, ${list(scopeNames)})`).join(", ");
    return [p.id, `PatData {\n                act: ${act},\n                captures: &[${caps}],\n                nested: &[],\n            }`];
  }),
  "Pat",
);
g += `        }\n    }\n}\n`;
g += startTable(pats);
g += ruleCodes("CMake", "CMAKE");
g += groupsCheck("CMake", pats.flatMap((p) => p.captures.map(([n]) => n)));
await writeRust(`${root_dir}src/cmake/grammar.rs`, g);

const bytes = (word: string) => `b"${word}"`;
const constName = (name: string) => name.replace(/([a-z])([A-Z])/g, "$1_$2").toUpperCase();
let w = `//! The words of the grammar's \`\\b(?i:…)\\b\` alternations, lowercase.
//! Generated by \`oracle/cmake-grammar.ts\` from the \`cmake\` grammar Shiki 4.6.0
//! bundles; do not edit.

/// The words of one alternation.
#[derive(Debug)]
pub(super) struct Words {
    /// The plain words, sorted.
    pub(super) exact: &'static [&'static [u8]],
    /// The words with a \`\\w+\` in them: the literal parts between which each
    /// \`\\w+\` stands. An empty part stands before a leading \`\\w+\` and after
    /// a trailing one.
    pub(super) globs: &'static [&'static [&'static [u8]]],
    /// The words with a character that is not \`\\w\`.
    pub(super) literals: &'static [&'static [u8]],
}
`;
for (const [name, set] of words) {
  w += `\n/// The words of the \`${name}\` pattern.\npub(super) const ${constName(name)}: Words = Words {\n`;
  w += `    exact: &[${set.exact.map(bytes).join(", ")}],\n`;
  w += `    globs: &[${set.globs.map((parts) => `&[${parts.map(bytes).join(", ")}]`).join(", ")}],\n`;
  w += `    literals: &[${set.literals.map(bytes).join(", ")}],\n`;
  w += `};\n`;
}
await writeRust(`${root_dir}src/cmake/words.rs`, w);

await writeRust(`${root_dir}src/cmake/scope.rs`, scopeFile("cmake-grammar.ts", "CMAKE", scopes));
console.log(`${rules.length} rules, ${pats.length} patterns, ${scopes.size} scopes, ${words.size} word lists`);
