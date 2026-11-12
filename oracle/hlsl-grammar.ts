// Writes src/hlsl/scope.rs, src/hlsl/grammar.rs and src/hlsl/words.rs
// from the `hlsl` grammar that Shiki 4.6.0 bundles
// (microsoft/vscode, converted from tgjones/shaders-tmLanguage).
// The grammar has no repository: its root lists every pattern.
// grammar.rs holds the scopes each rule and pattern adds, how each rule ends
// and the patterns it tries, in the grammar's order.
// words.rs holds the words of the grammar's alternations.
// The matchers that decide whether a pattern matches are written by hand in
// src/hlsl/matchers.rs.
//
// Run with `bun run oracle/hlsl-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import hlsl from "@shikijs/langs@4.6.0/hlsl";
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

const grammar = (hlsl as unknown as { patterns: Node[] }[])[0]!;
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
};

/** The identifier of a scope name: its parts in camel case, without the
 * `hlsl` part. */
function scopeIdent(name: string): string {
  const parts = name.split(".");
  if (!parts.includes("hlsl")) throw new Error(`${name}: a scope without the hlsl part`);
  let out = "";
  parts
    .filter((part) => part !== "hlsl")
    .forEach((part, index) => {
      for (const piece of part.split(/[-_]/)) {
        const capture = piece.charAt(0).toUpperCase() + piece.slice(1);
        out += index === 0 ? (abbreviations[piece] ?? capture) : capture;
      }
    });
  return out;
}

const scopes = new Map<string, string>([["Source", "source.hlsl"]]);
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
  "BlockComment",
  "LineComment",
  "Float",
  "Fraction",
  "Integer",
  "Hex",
  "Bool",
  "Preprocessor",
  "Control",
  "Compile",
  "Typedef",
  "BasicType",
  "Function",
  "Semantic",
  "SemanticSm4",
  "SemanticSm5",
  "SemanticSm51",
  "Modifier",
  "FloatModifier",
  "PostfixModifier",
  "Interpolation",
  "GeometryModifier",
  "StringType",
  "ObjectType",
  "RasterizerOrderedType",
  "RwType",
  "StreamType",
  "LegacySampler",
  "Sampler",
  "LegacyTexture",
  "Texture",
  "Structured",
  "FxConstant",
  "FxStateType",
  "Technique",
  "BlendStateKey",
  "DepthStencilStateKey",
  "RasterizerStateKey",
  "SamplerStateKey",
  "Blend",
  "BlendOp",
  "DepthWriteMask",
  "ComparisonFunc",
  "StencilOp",
  "FillMode",
  "CullMode",
  "Filter",
  "AddressMode",
  "String",
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

/** The words of the `\b(…)\b` and `\b(?i:…)\b` alternations, by pattern.
 * Every word is made of `\w`, so the alternation between `\b` and `\b`
 * matches a whole word that is one of them, whatever their order. */
type WordPattern = { id: string; folded: boolean; words: string[] };
const wordPats: WordPattern[] = [];
/** The words of the semantics, `(?<=:\s?)(?i:…)\b`: the ones that stand alone
 * and the ones that `[0-9]*` may follow, lowercase. */
type Semantic = { id: string; words: string[]; digits: string[] };
const semantics: Semantic[] = [];

grammar.patterns.forEach((node, index) => {
  const id = names[index]!;
  const regex = node.match;
  if (regex === undefined) return;
  const word = /^\\b\((\?i:)?(.*)\)\\b$/s.exec(regex);
  if (word !== null && !["Float", "Fraction", "Integer", "Hex", "Function"].includes(id)) {
    const folded = word[1] !== undefined;
    const all = expand(word[2]!).map((w) => (folded ? w.toLowerCase() : w));
    if (all.some((w) => !/^\w+$/.test(w))) throw new Error(`${id}: a word that is not all \\w`);
    wordPats.push({ id, folded, words: [...new Set(all)].sort() });
    return;
  }
  const semantic = /^\(\?<=:\\s\?\)\(\?i:(.*)\)\\b$/s.exec(regex);
  if (semantic !== null) {
    // `#` stands for `[0-9]*`, which `expand` cannot spell out.
    const all = expand(semantic[1]!.replaceAll("[0-9]*", "#")).map((w) => w.toLowerCase());
    const words = all.filter((w) => !w.includes("#"));
    const digits = all.filter((w) => w.includes("#")).map((w) => w.slice(0, -1));
    // A word that digits may follow must not end in a digit itself: the
    // matcher strips every trailing digit before it looks the word up.
    if ([...words, ...digits].some((w) => !/^\w+$/.test(w)) || digits.some((w) => /\d$/.test(w))) {
      throw new Error(
        `${id}: a semantic that is not all \\w, or one before [0-9]* that ends in a digit`,
      );
    }
    semantics.push({ id, words: [...new Set(words)].sort(), digits: [...new Set(digits)].sort() });
  }
});
const expectedWords = [
  "Bool",
  "Control",
  "Compile",
  "Typedef",
  "BasicType",
  ...names.slice(names.indexOf("Modifier"), names.indexOf("String")),
];
if (wordPats.map((w) => w.id).join() !== expectedWords.join())
  throw new Error("the word patterns changed");
if (semantics.map((s) => s.id).join() !== "Semantic,SemanticSm4,SemanticSm5,SemanticSm51") {
  throw new Error("the semantics changed shape");
}
if (wordPats.length > 64) throw new Error("more word patterns than bits in a u64");

const preprocessor = /^\^\\s\*#\\s\*\((.*)\)$/.exec(
  grammar.patterns[names.indexOf("Preprocessor")]!.match!,
)?.[1];
if (preprocessor === undefined) throw new Error("the preprocessor pattern changed shape");
const directives = expand(preprocessor);

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
const longest = Math.max(
  ...folded.map(([w]) => w.length),
  ...semantics.flatMap((s) => [...s.words, ...s.digits].map((w) => w.length)),
);

// Output.

const list = (scopeNames: string[]) =>
  scopeNames.length ? `scopes![${scopeNames.map((n) => `S::${n}`).join(", ")}]` : "S::NO_SCOPES";

let g = `//! The grammar's rules as tables. Generated by \`oracle/hlsl-grammar.ts\` from
//! the \`hlsl\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::Hlsl;
use super::scope::S;
use crate::rules;
use crate::scope::scopes;

pub(crate) type Act = rules::Act<Hlsl>;
pub(crate) type PatternDefinition = rules::PatternDefinition<Hlsl>;
pub(crate) type RuleDefinition = rules::RuleDefinition<Hlsl>;
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
g += ruleCodes("Hlsl", "HLSL");
g += groupsCheck(
  "Hlsl",
  patternById.flatMap((p) => p.captures.map(([n]) => n)),
);
await writePatterns("hlsl", patternById);
await writeRust(`${root_dir}src/hlsl/grammar.rs`, g);

const bytes = (word: string) => `b"${word}"`;
const constName = (name: string) => name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
const hex = (bits: bigint) => `0x${bits.toString(16).padStart(16, "0").match(/.{4}/g)!.join("_")}`;
let w = `//! The words of the grammar's alternations. Generated by
//! \`oracle/hlsl-grammar.ts\` from the \`hlsl\` grammar Shiki 4.6.0 bundles; do
//! not edit.

use super::grammar::Pattern;

/// The longest lowercase word of [\`FOLDED\`] and the semantics.
pub(super) const LONGEST_FOLDED: usize = ${longest};

/// The words of the case-sensitive \`\\b(…)\\b\` patterns, sorted, each with
/// the [\`bit\`]s of the patterns that list it.
pub(super) const WORDS: &[(&[u8], u64)] = &[${exact.map(([word, bits]) => `(${bytes(word)}, ${hex(bits)})`).join(", ")}];

/// The words of the \`\\b(?i:…)\\b\` patterns, lowercase and sorted, each with
/// the [\`bit\`]s of the patterns that list it.
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
w += `    }\n}\n\n/// The directives of the \`Preprocessor\` pattern, in the grammar's order.\npub(super) const DIRECTIVES: &[&[u8]] = &[${directives.map(bytes).join(", ")}];\n`;
for (const s of semantics) {
  w += `\n/// The words of the \`${s.id}\` pattern that stand alone, lowercase and sorted.\npub(super) const ${constName(s.id)}_WORDS: &[&[u8]] = &[${s.words.map(bytes).join(", ")}];\n`;
  w += `\n/// The words of the \`${s.id}\` pattern that digits may follow, lowercase and sorted.\npub(super) const ${constName(s.id)}_DIGITS: &[&[u8]] = &[${s.digits.map(bytes).join(", ")}];\n`;
}
await writeRust(`${root_dir}src/hlsl/words.rs`, w);

await writeRust(`${root_dir}src/hlsl/scope.rs`, scopeFile("hlsl-grammar.ts", "HLSL", scopes));

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
