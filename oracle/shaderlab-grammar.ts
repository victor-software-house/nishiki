// Writes src/shaderlab/scope.rs, src/shaderlab/grammar.rs and
// src/shaderlab/words.rs from the `shaderlab` grammar that Shiki 4.6.0
// bundles (microsoft/vscode, converted from tgjones/shaders-tmLanguage).
//
// grammar.rs holds the scopes each rule and pattern adds, how each rule ends
// and the patterns it tries, in the grammar's order.
// The grammar's two `include`s of `#numbers` become one `Pattern`.
//
// A `CGPROGRAM` or `HLSLPROGRAM` block includes `#hlsl-embedded`: Shiki's
// `source.hlsl`, then Unity's own patterns.
// The block embeds HLSL (`CarriedState::Embed`), and lists Unity's patterns as its
// own after it (`Embeds::embeds_after_own`).
// HLSL then halts where a Unity pattern starts and none of its own matches,
// as vscode-textmate tries the patterns after an include.
//
// words.rs holds the words of the grammar's `\b(…)\b` and `\b(?i:…)\b`
// alternations.
// The matchers that decide whether a pattern matches are written by hand in
// src/shaderlab/matchers.rs.
//
// Run with `bun run oracle/shaderlab-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import shaderlabLangs from "@shikijs/langs@4.6.0/shaderlab";
import { expand } from "./alternation.ts";
import {
  arms,
  groupsCheck,
  ruleCodes,
  scopeFile,
  startTable,
  takesOneBlank,
  tooLong,
  writeRust,
} from "./rule-grammar.ts";

type Node = {
  name?: string;
  match?: string;
  begin?: string;
  end?: string;
  include?: string;
  captures?: Record<string, { name?: string }>;
  beginCaptures?: Record<string, { name?: string }>;
  endCaptures?: Record<string, { name?: string }>;
  patterns?: Node[];
};
type Grammar = { name: string; patterns: Node[]; repository: Record<string, Node> };

const grammar = (shaderlabLangs as unknown as Grammar[]).find((g) => g.name === "shaderlab")!;
const root_dir = new URL("..", import.meta.url).pathname;

// Scope names.

const abbreviations: Record<string, string> = {
  constant: "Const",
  storage: "Stor",
  support: "Sup",
  string: "Str",
  comment: "Cmt",
  keyword: "Kw",
  meta: "Meta",
  variable: "Var",
};

/** The identifier of a scope name: its parts in camel case, without the
 * `shaderlab` part, and ending in `Bare` when the name has none. */
function scopeIdent(name: string): string {
  const parts = name.split(".");
  const bare = !parts.includes("shaderlab");
  let out = "";
  parts
    .filter((part) => part !== "shaderlab")
    .forEach((part, index) => {
      for (const piece of part.split(/[-_]/)) {
        const capture = piece.charAt(0).toUpperCase() + piece.slice(1);
        out += index === 0 ? (abbreviations[piece] ?? capture) : capture;
      }
    });
  return out + (bare ? "Bare" : "");
}

const scopes = new Map<string, string>([["Source", "source.shaderlab"]]);
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
  "LineComment",
  "BasicType",
  "Numbers",
  "Structure",
  "PropertyName",
  "PropertyValue",
  "ComparisonFunction",
  "StencilOperation",
  "TextureCombiners",
  "Fog",
  "BindChannels",
  "BlendOperations",
  "BlendFactors",
  "Reference",
  "Attribute",
  "Declaration",
  "CgBlock",
  "HlslBlock",
  "String",
];
if (names.length !== grammar.patterns.length) throw new Error("the grammar's root changed");

/** The names of `#hlsl-embedded`'s own patterns, after its include of
 * `source.hlsl`. */
const unityNames = [
  "Fixed",
  "Transformations",
  "Camera",
  "Time",
  "Lighting",
  "FogVariable",
  "Various",
  "TargetPlatform",
  "TargetModel",
  "UnityVersion",
  "PlatformDifference",
  "Texture2D",
  "Structures",
  "Surface",
];

type Pattern = {
  id: string;
  regex: string;
  act: { role: "match"; name: string[] } | { role: "begin"; rule: string } | { role: "end" };
  captures: [number, string[]][];
};
type Rule = { id: string; name: string[]; end: string; patterns: string[]; embeds: boolean };

const patternById: Pattern[] = [];
const rules: Rule[] = [{ id: "Root", name: [], end: "", patterns: [], embeds: false }];

function captureList(captures: Node["captures"]): [number, string[]][] {
  return Object.entries(captures ?? {})
    .map(([group, value]): [number, string[]] => [Number(group), scopeList(value.name)])
    .filter(([, list]) => list.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

const numbers = grammar.repository["numbers"]!.patterns;
if (numbers?.length !== 1 || numbers[0]!.match === undefined)
  throw new Error("#numbers changed shape");
const embedded = grammar.repository["hlsl-embedded"]!.patterns!;
if (embedded[0]?.include !== "source.hlsl" || embedded.length !== unityNames.length + 1) {
  throw new Error("#hlsl-embedded changed shape");
}

/** Registers `node` under `id` and returns the ids of the patterns it adds. */
function register(node: Node, id: string): string[] {
  if (node.include === "#numbers") {
    if (!patternById.some((p) => p.id === "Number")) {
      patternById.push({
        id: "Number",
        regex: numbers![0]!.match!,
        act: { role: "match", name: scopeList(numbers![0]!.name) },
        captures: [],
      });
    }
    return ["Number"];
  }
  if (node.match !== undefined) {
    patternById.push({
      id,
      regex: node.match,
      act: { role: "match", name: scopeList(node.name) },
      captures: captureList(node.captures),
    });
    return [id];
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
  const rule: Rule = {
    id,
    name: scopeList(node.name),
    end: `${id}End`,
    patterns: [],
    embeds: false,
  };
  rules.push(rule);
  const inner = node.patterns ?? [];
  if (inner.length === 1 && inner[0]!.include === "#hlsl-embedded") {
    rule.embeds = true;
    rule.patterns = unityNames.map((unity, index) => {
      if (!patternById.some((p) => p.id === unity)) register(embedded[index + 1]!, unity);
      return unity;
    });
    return [id];
  }
  rule.patterns = inner.flatMap((one, index) => register(one, `${id}At${index}`));
  return [id];
}

rules[0]!.patterns = grammar.patterns.flatMap((node, index) => register(node, names[index]!));
if (
  rules
    .filter((r) => r.embeds)
    .map((r) => r.id)
    .join() !== "CgBlock,HlslBlock"
)
  throw new Error("the embedding rules changed");

for (const p of patternById) {
  if (takesOneBlank(p.regex))
    throw new Error(`${p.id} takes one blank: the lexer needs BlankRuns::ToLast`);
}

// Words.

/** The words of the `\b(…)\b` and `\b(?i:…)\b` alternations, by pattern.
 * Every word is made of `\w`, so the alternation between `\b` and `\b`
 * matches a whole word that is one of them, whatever their order. */
type WordPattern = { id: string; folded: boolean; words: string[] };
const wordPats: WordPattern[] = [];
for (const p of patternById) {
  if (p.act.role !== "match" || ["Number", "Reference", "Declaration"].includes(p.id)) continue;
  const word = /^\\b\((\?i:)?(.*)\)\\b$/s.exec(p.regex);
  if (word === null) continue;
  const folded = word[1] !== undefined;
  const all = expand(word[2]!).map((w) => (folded ? w.toLowerCase() : w));
  if (all.some((w) => !/^\w+$/.test(w))) throw new Error(`${p.id}: a word that is not all \\w`);
  wordPats.push({ id: p.id, folded, words: [...new Set(all)].sort() });
}
const expectedWords = [...names.slice(1, 13).filter((n) => n !== "Numbers"), ...unityNames];
if (wordPats.map((w) => w.id).join() !== expectedWords.join())
  throw new Error(`the word patterns changed: ${wordPats.map((w) => w.id)}`);
if (wordPats.length > 64) throw new Error("more word patterns than bits in a u64");

/** Every word, with the bits of the patterns that list it. */
function table(folded: boolean): [string, bigint][] {
  const bits = new Map<string, bigint>();
  wordPats.forEach((w, index) => {
    if (w.folded !== folded) return;
    for (const word of w.words) bits.set(word, (bits.get(word) ?? 0n) | (1n << BigInt(index)));
  });
  return [...bits.entries()].sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0));
}
const exact = table(false);
const folded = table(true);
const longest = Math.max(...folded.map(([w]) => w.length));

// Output.

const list = (scopeNames: string[]) =>
  scopeNames.length ? `scopes![${scopeNames.map((n) => `S::${n}`).join(", ")}]` : "S::NO_SCOPES";

let g = `//! The grammar's rules as tables. Generated by \`oracle/shaderlab-grammar.ts\`
//! from the \`shaderlab\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.
//! A \`CGPROGRAM\` or \`HLSLPROGRAM\` block embeds HLSL, and lists Unity's
//! patterns after it.

use super::engine::ShaderLab;
use super::scope::S;
use crate::rules;
use crate::scope::scopes;

pub(crate) type Act = rules::Act<ShaderLab>;
pub(crate) type PatternDefinition = rules::PatternDefinition<ShaderLab>;
pub(crate) type RuleDefinition = rules::RuleDefinition<ShaderLab>;
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
g += `        }\n    }\n\n    /// What a rule carries from line to line: the state of HLSL, when the\n    /// rule embeds it and its feature is on.\n    pub(crate) fn payload(self) -> rules::CarriedState {\n        match self {\n`;
g += arms(
  rules.map((r): [string, string] => [
    r.id,
    r.embeds
      ? `rules::embedded!(Hlsl, "lang-hlsl").map_or(rules::CarriedState::None, rules::CarriedState::Embed)`
      : "rules::CarriedState::None",
  ]),
  "Rule",
);
g += `        }\n    }\n\n    /// Whether the rule lists its own patterns after the grammar it embeds.\n    pub(crate) fn embeds_after_own(self) -> bool {\n        matches!(self, ${rules
  .filter((r) => r.embeds)
  .map((r) => `Rule::${r.id}`)
  .join(" | ")})\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pattern {\n`;
const docs = (s: string) => (s.length > 56 ? "…" : s);
for (const p of patternById) {
  g += `    /// The ${p.act.role} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pattern {\n    /// Every pattern, for the tool that compares the matchers with Oniguruma.\n    #[cfg(test)]\n    pub(crate) const ALL: [Pattern; ${patternById.length}] = [\n${patternById.map((p) => `        Pattern::${p.id},\n`).join("")}    ];\n\n${tooLong}    pub(crate) fn definition(self) -> PatternDefinition {\n        match self {\n`;
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
g += ruleCodes("ShaderLab", "SHADERLAB");
g += groupsCheck(
  "ShaderLab",
  patternById.flatMap((p) => p.captures.map(([n]) => n)),
);
await writeRust(`${root_dir}src/shaderlab/grammar.rs`, g);

const bytes = (word: string) => `b"${word}"`;
const hex = (bits: bigint) => `0x${bits.toString(16).padStart(16, "0").match(/.{4}/g)!.join("_")}`;
let w = `//! The words of the grammar's alternations. Generated by
//! \`oracle/shaderlab-grammar.ts\` from the \`shaderlab\` grammar Shiki 4.6.0
//! bundles; do not edit.

use super::grammar::Pattern;

/// The longest lowercase word of [\`FOLDED\`].
pub(super) const LONGEST_FOLDED: usize = ${longest};

/// The words of the case-sensitive \`\\b(…)\\b\` patterns, Unity's, sorted,
/// each with the [\`bit\`]s of the patterns that list it.
pub(super) const WORDS: &[(&[u8], u64)] = &[${exact.map(([word, bits]) => `(${bytes(word)}, ${hex(bits)})`).join(", ")}];

/// The words of the \`\\b(?i:…)\\b\` patterns, ShaderLab's own, lowercase and
/// sorted, each with the [\`bit\`]s of the patterns that list it.
pub(super) const FOLDED: &[(&[u8], u64)] = &[${folded.map(([word, bits]) => `(${bytes(word)}, ${hex(bits)})`).join(", ")}];

/// The bit of a word pattern in [\`WORDS\`] and [\`FOLDED\`]; 0 for every other
/// pattern.
pub(super) fn bit(pattern: Pattern) -> u64 {
    match pattern {
`;
const wordBits = new Map(wordPats.map((wp, index) => [wp.id, index]));
w += arms(
  patternById.map((p): [string, string] => {
    const index = wordBits.get(p.id);
    return [p.id, index === undefined ? "0" : `1 << ${index}`];
  }),
  "Pattern",
).replaceAll("            ", "        ");
w += `    }\n}\n`;
await writeRust(`${root_dir}src/shaderlab/words.rs`, w);

await writeRust(
  `${root_dir}src/shaderlab/scope.rs`,
  scopeFile("shaderlab-grammar.ts", "SHADERLAB", scopes),
);

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
  `${rules.length} rules, ${patternById.length} patterns, ${scopes.size} scopes, ${wordPats.length} word patterns`,
);
