// Writes src/asm/scope.rs, src/asm/grammar.rs and src/asm/words.rs from the
// `asm` grammar that Shiki 4.6.0 bundles (13xforever/x86_64-assembly-vscode,
// MIT).
// grammar.rs holds the scopes each rule and pattern adds, how each rule ends
// and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// words.rs holds the words of the grammar's word alternations.
// The matchers that decide whether a pattern matches are written by hand in
// src/asm/matchers.
//
// The root starts with 222 patterns, the registers and then the mnemonics.
// Each is `(?i)\b(…)\b` over a finite list of words made of ASCII `\w`.
// Such a pattern matches at a place only where `\b` holds before it and the
// whole run of `\w` from there is one of its words: a shorter word would end
// before a `\w`, where no `\b` stands.
// Nothing comes before them, so at a run's start the first of them that holds
// the run is the pattern that wins, as in a `TextMate` engine.
// The generator therefore merges them into one pattern, `Word`.
// One sorted table maps each word to the first pattern that lists it, its
// owner, and the matcher scopes the match with the owner's name.
// The generator checks each of these conditions and fails when a grammar
// update breaks one.
// The 222 expressions stay in `Rx`, for the probe, which checks each alone.
//
// Under `(?i)`, Oniguruma folds `ß` to `ss` and each Latin ligature to its
// letters only inside one literal string of the expression.
// `\b(?:fild|fist)\b` takes `ﬁld`, but `\bf(i?(ld|stp?))\b` does not, since
// the `f` and the `i` stand in different parts of it.
// So whether a spelling with `ß` or a ligature matches depends on how each
// expression is written, and no fold of the word alone can tell.
// The generator asks Oniguruma instead: it tries every spelling of each
// listed word that puts `ß` or a ligature in place of its letters, against
// each expression that folds case, and keeps the spellings it accepts.
//
// Run with `bun run oracle/asm-grammar.ts` and commit the result.
// The output is generated; do not edit it.
// `RX_DUMP=<file>` also writes every expression and its capture groups, one
// JSON object per line, for oracle/onig.ts.
import asm from "@shikijs/langs@4.6.0/asm";
import { createOnigurumaEngine } from "@shikijs/engine-oniguruma@4.6.0";
import { expand } from "./alternation.ts";
import { arms, groupsCheck, oneBlankTable, ruleCodes, scopeFile, startTable, takesOneBlank, tooLong, writeRust } from "./rule-grammar.ts";

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

const grammar = (asm as unknown as Grammar[]).find((lang) => lang.scopeName === "source.asm.x86_64")!;
const repository = grammar.repository;
const root_dir = new URL("..", import.meta.url).pathname;

/** The includes the grammar names but never defines.
 * vscode-textmate skips such an include and keeps the rest of its list.
 * A rule whose every pattern is such an include is dropped whole, as
 * vscode-textmate drops it: so the three string rules are, and a quote starts
 * no string. */
const unresolved = new Set(["string_escaped_char", "string_placeholder"]);

/** Whether vscode-textmate drops `node`: its patterns are all includes the
 * grammar never defines. */
const dropped = (node: Node) =>
  (node.patterns?.length ?? 0) > 0 &&
  node.patterns!.every((inner) => inner.include !== undefined && unresolved.has(inner.include.slice(1)));

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
  invalid: "Inv",
  meta: "Meta",
};

/** The identifier of a scope name: its parts in camel case, without the
 * `x86_64` part.
 * A last part of one `s`, as in `keyword.operator.word.mnemonic.other.s`,
 * reads `SPart`: an identifier that ends with the word `S`, the scope enum's
 * name, fails Clippy's `enum_variant_names`. */
function scopeIdent(name: string): string {
  let out = "";
  name
    .split(".")
    .filter((part) => part !== "x86_64")
    .forEach((part, index) => {
      for (const piece of part.split(/[-_]/)) {
        const capture = piece.charAt(0).toUpperCase() + piece.slice(1);
        out += index === 0 ? (abbreviations[piece] ?? capture) : capture;
      }
    });
  return /[a-z]S$/.test(out) ? `${out}Part` : out;
}

const scopes = new Map<string, string>([["Source", "source.asm.x86_64"]]);
function scopeList(name: string | undefined): string[] {
  if (name === undefined) return [];
  return name.split(" ").map((one) => {
    if (one.includes("$")) throw new Error(`${name}: a name built from a capture`);
    const ident = one === "source.asm.x86_64" ? "Source" : scopeIdent(one);
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
};
type Rule = { id: string; name: string[]; content: string[]; end: string; patterns: string[] };

const patternById = new Map<string, Pattern>();
const rules = new Map<string, Rule>();

function captureList(captures: Captures | undefined): [number, string[]][] {
  return Object.entries(captures ?? {})
    .map(([group, value]): [number, string[]] => {
      if (value.patterns !== undefined) throw new Error(`group ${group}: patterns of its own`);
      return [Number(group), scopeList(value.name)];
    })
    .filter(([, list]) => list.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** Registers a match or begin/end rule under `id`; returns its pattern id. */
function register(node: Node, id: string): string {
  const clash = patternById.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (patternById.has(id)) return id;
  if (node.match !== undefined) {
    patternById.set(id, { id, regex: node.match, act: { role: "match", name: scopeList(node.name) }, captures: captureList(node.captures) });
    return id;
  }
  if (node.begin === undefined || node.end === undefined) throw new Error(`${id}: neither a match nor a begin/end rule`);
  if (/\\[1-9]/.test(node.end)) throw new Error(`${id}: the end repeats a group`);
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
  const rule: Rule = { id, name: scopeList(node.name), content: scopeList(node.contentName), end: `${id}End`, patterns: [] };
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
    if (node.include !== undefined) {
      if (!node.include.startsWith("#")) throw new Error(`external include ${node.include}`);
      const key = node.include.slice(1);
      const target = repository[key];
      if (!target) {
        if (!unresolved.has(key)) throw new Error(`unknown include ${key}`);
        return;
      }
      out.push(...expandNode(target, pascal(key), true));
    } else {
      out.push(...expandNode(node, `${context}At${index}`, false));
    }
  });
  return out;
}

function expandNode(node: Node, id: string, shared: boolean): string[] {
  if (node.begin !== undefined && dropped(node)) return [];
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
rootRule.patterns = flatten(grammar.patterns, "Root");

// The first pass stopped a bare list where it reached itself, which left a
// rule short of patterns it includes.
// Every pattern has its id by now, so a second pass gives each list in full.
for (const { id, node } of ruleSources) {
  expanding.clear();
  rules.get(id)!.patterns = flatten(node.patterns ?? [], id);
}

// The merged word patterns.

/** The words of `regex` when it is `(?i)\b(…)\b` over a finite list of words
 * made of ASCII `\w`, and `undefined` otherwise. */
function pureWords(regex: string): string[] | undefined {
  const body = /^\(\?i\)\\b(.*)\\b$/.exec(regex)?.[1];
  if (body === undefined || !/^(?:[\w|()?:\[\]-]|\{\d+\})*$/.test(body)) return undefined;
  // `alternation.ts` reads a count as `{m,n}`, so an exact `{n}` is spelled
  // `{n,n}` for it.
  const words = expand(body.replace(/\{(\d+)\}/g, "{$1,$1}"));
  return words.every((word) => /^[0-9A-Z_a-z]+$/.test(word)) ? words : undefined;
}

/** The ids of the patterns that the includes of the root's first two
 * entries, the registers and the mnemonics, expand to. */
const [registers, mnemonics] = grammar.patterns;
if (registers?.include !== "#registers" || mnemonics?.include !== "#mnemonics") {
  throw new Error("the root no longer starts with the registers and the mnemonics");
}
const merged = flatten([registers, mnemonics], "Root");
merged.forEach((id, index) => {
  if (rootRule.patterns[index] !== id) throw new Error(`${id}: the word patterns are not contiguous from the root's start`);
  const pattern = patternById.get(id)!;
  if (pattern.act.role !== "match" || pattern.captures.length > 0) throw new Error(`${id}: not a plain match`);
  if (pureWords(pattern.regex) === undefined) throw new Error(`${id}: not (?i)\\b(…)\\b over ASCII \\w words`);
});
if (pureWords(patternById.get(rootRule.patterns[merged.length]!)!.regex) !== undefined) {
  throw new Error("a word pattern follows the mnemonics: the merge would leave it out");
}
for (const rule of rules.values()) {
  if (rule.id !== "Root" && rule.patterns.some((id) => merged.includes(id))) {
    throw new Error(`${rule.id} lists a merged word pattern`);
  }
}

/** Each lowercase word and its owner, the index in `merged` of the first
 * pattern that lists it; and the later patterns that list it too. */
const owners = new Map<string, number>();
const alsoIn: [string, number][] = [];
merged.forEach((id, index) => {
  for (const word of new Set(pureWords(patternById.get(id)!.regex)!.map((w) => w.toLowerCase()))) {
    if (owners.has(word)) alsoIn.push([word, index]);
    else owners.set(word, index);
  }
});
const sortedWords = [...owners.keys()].sort();
const mergedRegex = `(?i)\\b(?:${merged.map((id) => /^\(\?i\)\\b(.*)\\b$/.exec(patternById.get(id)!.regex)![1]).join("|")})\\b`;
const mergedPats = merged.map((id) => patternById.get(id)!);
for (const id of merged) patternById.delete(id);
patternById.set("Word", { id: "Word", regex: mergedRegex, act: { role: "match", name: [] }, captures: [] });
rootRule.patterns = ["Word", ...rootRule.patterns.slice(merged.length)];

// Blank runs.

const oneBlank = [...patternById.values()].filter((p) => takesOneBlank(p.regex));
const toLast = oneBlank.length > 0;

// Output.

let g = `//! The grammar's rules as tables. Generated by \`oracle/asm-grammar.ts\` from
//! the \`asm\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar, or [\`Pattern::Word\`], the
//! registers and mnemonics merged; its doc comment shows which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::Asm;
use super::scope::S;
use crate::rules;
use crate::scope::scopes;

pub(crate) type Act = rules::Act<Asm>;
pub(crate) type PatternDefinition = rules::PatternDefinition<Asm>;
pub(crate) type RuleDefinition = rules::RuleDefinition<Asm>;
pub(crate) use crate::rules::Start;

/// The patterns a capture group is tokenized with: none in this grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Nested {}

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

const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 || /[^\x20-\x7e]/.test(one) ? "…" : one;
};
g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pattern {\n`;
for (const p of patternList) {
  const what = p.id === "Word" ? "The registers and mnemonics, merged" : `The ${p.act.role} \`${docs(p.regex).replaceAll("`", "'")}\``;
  g += `    /// ${what}.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pattern {\n${tooLong}    pub(crate) fn definition(self) -> PatternDefinition {\n        match self {\n`;
g += arms(
  patternList.map((p): [string, string] => {
    const act =
      p.act.role === "match" ? `Act::Match(${scText(p.act.name)})` : p.act.role === "begin" ? `Act::Begin(Rule::${p.act.rule})` : "Act::End";
    const captures = p.captures.map(([n, list]) => `(${n}, ${scText(list)})`).join(", ");
    return [p.id, `PatternDefinition {\n                act: ${act},\n                captures: &[${captures}],\n                nested: &[],\n            }`];
  }),
  "Pattern",
);
g += `        }\n    }\n}\n`;

// The expressions the probe checks: every pattern but `Word`, and the merged
// patterns one by one.
const rxList = [...mergedPats.map((p) => p.id), ...patternList.filter((p) => p.id !== "Word").map((p) => p.id)];
g += `
/// One expression of the grammar, for the tool that compares the matchers
/// with Oniguruma: a pattern, or one of the patterns merged into
/// [\`Pattern::Word\`].
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Rx {
${rxList.map((id) => `    ${id},\n`).join("")}}

/// What answers an expression of [\`Rx\`].
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Answer {
    /// The pattern's own matcher.
    Pattern(Pattern),
    /// The word list of the merged pattern at this index.
    List(u8),
}

#[cfg(test)]
impl Rx {
    /// Every expression.
    pub(crate) const ALL: [Rx; ${rxList.length}] = [
${rxList.map((id) => `        Rx::${id},\n`).join("")}    ];

${tooLong}    pub(crate) fn answer(self) -> Answer {
        match self {
${mergedPats.map((p, index) => `            Rx::${p.id} => Answer::List(${index}),\n`).join("")}${patternList
  .filter((p) => p.id !== "Word")
  .map((p) => `            Rx::${p.id} => Answer::Pattern(Pattern::${p.id}),\n`)
  .join("")}        }
    }
}
`;
g += startTable(patternList);
if (toLast) g += oneBlankTable(patternList);
g += ruleCodes("Asm", "ASM");
g += groupsCheck("Asm", patternList.flatMap((p) => p.captures.map(([n]) => n)));
await writeRust(`${root_dir}src/asm/grammar.rs`, g);

// Spellings with `ß` or a ligature.

const engine = await createOnigurumaEngine(import("@shikijs/engine-oniguruma@4.6.0/wasm-inlined"));

/** Whether `regex` matches the whole of `text`. */
function accepts(regex: string, text: string): boolean {
  const string = engine.createString(text);
  const m = engine.createScanner([regex]).findNextMatchSync(string, 0);
  return m !== null && m.captureIndices[0]!.start === 0 && m.captureIndices[0]!.end === text.length;
}

/** The letters that `ß` and each Latin ligature fold to, with the forms that
 * fold the same way.
 * The first form is the one the tables keep: the matcher reads `ẞ` as `ß`
 * and `ﬅ` as `ﬆ`. */
const ligatures: [string, string[]][] = [
  ["ss", ["ß", "ẞ"]],
  ["ff", ["ﬀ"]],
  ["fi", ["ﬁ"]],
  ["fl", ["ﬂ"]],
  ["ffi", ["ﬃ"]],
  ["ffl", ["ﬄ"]],
  ["st", ["ﬆ", "ﬅ"]],
];
const kept = (spelling: string) => spelling.replaceAll("ẞ", "ß").replaceAll("ﬅ", "ﬆ");

/** Each spelling of the lowercase `word` that puts `ß` or a ligature in
 * place of some of its letters, by the form the tables keep. */
function spellings(word: string): Map<string, string[]> {
  const out = new Map<string, string[]>();
  const go = (at: number, acc: string, changed: boolean) => {
    if (at === word.length) {
      if (changed) out.set(kept(acc), [...(out.get(kept(acc)) ?? []), acc]);
      return;
    }
    go(at + 1, acc + word[at], changed);
    for (const [letters, forms] of ligatures) {
      if (word.startsWith(letters, at)) for (const form of forms) go(at + letters.length, acc + form, true);
    }
  };
  go(0, "", false);
  return out;
}

/** The kept spellings of `words` that `regex` accepts.
 * Every form of one kept spelling must agree, or the matcher's reading of
 * `ẞ` as `ß` and `ﬅ` as `ﬆ` would be wrong. */
function acceptedSpellings(regex: string, words: string[]): string[] {
  const out: string[] = [];
  for (const word of words) {
    for (const [spelling, forms] of spellings(word)) {
      const verdicts = new Set(forms.map((form) => accepts(regex, form)));
      if (verdicts.size > 1) throw new Error(`${regex}: the forms of ${spelling} disagree`);
      if (verdicts.has(true)) out.push(spelling);
    }
  }
  return out;
}

/** Each kept spelling of the merged words and the indices of the merged
 * patterns that accept it, first the owner. */
const spelled = new Map<string, number[]>();
mergedPats.forEach((p, index) => {
  for (const spelling of acceptedSpellings(p.regex, pureWords(p.regex)!.map((w) => w.toLowerCase()))) {
    spelled.set(spelling, [...new Set([...(spelled.get(spelling) ?? []), index])]);
  }
});
const sortedSpellings = [...spelled.keys()].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));

// Words.

const bytes = (word: string) => `b"${word}"`;
/** A Rust byte string of `text`'s UTF-8, with each byte above ASCII escaped. */
const utf8 = (text: string) =>
  `b"${[...Buffer.from(text)].map((b) => (b < 0x80 ? String.fromCharCode(b) : `\\x${b.toString(16).padStart(2, "0")}`)).join("")}"`;
const constName = (name: string) => name.replace(/([a-z])([A-Z])/g, "$1_$2").toUpperCase();
const blob = sortedWords.join("");
let end = 0;
const ends = sortedWords.map((word) => (end += word.length));
if (end > 0xffff) throw new Error("the word blob outgrows u16 ends");
if (merged.length > 0xff) throw new Error("the merged patterns outgrow u8 owners");

let w = `//! The words of the grammar's word alternations, lowercase and sorted.
//! Generated by \`oracle/asm-grammar.ts\` from the \`asm\` grammar Shiki 4.6.0
//! bundles; do not edit.

use super::scope::S;
use crate::scope::{Scopes, scopes};

/// The longest word of [\`WORDS\`], which sizes a lowered run.
pub(super) const LONGEST: usize = ${Math.max(...sortedWords.map((word) => word.length))};

/// The ${sortedWords.length} words of the registers and mnemonics, merged into [\`super::grammar::Pattern::Word\`], one after another.
/// [\`ENDS\`] says where each ends.
pub(super) const WORDS: &[u8; ${blob.length}] = b"${blob}";

/// Where each word of [\`WORDS\`] ends.
pub(super) const ENDS: [u16; ${ends.length}] = [${ends.join(", ")}];

/// Each word's owner: the index of the first merged pattern that lists it.
pub(super) const OWNERS: [u8; ${ends.length}] = [${sortedWords.map((word) => owners.get(word)).join(", ")}];

/// The scopes each merged pattern names, by its index.
pub(super) const NAMES: [Scopes<S>; ${mergedPats.length}] = [
${mergedPats.map((p) => `    ${p.act.role === "match" ? scText(p.act.name) : "S::NO_SCOPES"},\n`).join("")}];

/// The words that a later merged pattern lists too, with its index, for the
/// probe that checks each pattern alone.
#[cfg(test)]
pub(super) const ALSO_IN: &[(&[u8], u8)] = &[${alsoIn.map(([word, index]) => `(${bytes(word)}, ${index})`).join(", ")}];

/// The spellings of the merged words with \`ß\` or a Latin ligature that
/// Oniguruma accepts, lowercase, with \`ẞ\` read as \`ß\` and \`ﬅ\` as \`ﬆ\`, sorted
/// by their bytes.
/// Oniguruma folds such a character only inside one literal string of an
/// expression, so the generator asked it for each spelling.
pub(super) const SPELLINGS: [&[u8]; ${sortedSpellings.length}] = [${sortedSpellings.map(utf8).join(", ")}];

/// Each spelling's owner: the index of the first merged pattern that accepts
/// it.
pub(super) const SPELLING_OWNERS: [u8; ${sortedSpellings.length}] = [${sortedSpellings.map((s) => spelled.get(s)![0]).join(", ")}];

/// The spellings that a later merged pattern accepts too, with its index,
/// for the probe.
#[cfg(test)]
pub(super) const SPELLINGS_ALSO_IN: &[(&[u8], u8)] = &[${sortedSpellings.flatMap((s) => spelled.get(s)!.slice(1).map((i) => `(${utf8(s)}, ${i})`)).join(", ")}];
`;

/** The literal list of a support pattern, and how the matcher reads it:
 * between `\b` and `\b` (`word`), after `..` (`dots`) or inside `{…}`
 * (`brace`).
 * A pattern that ignores case gets lowercase words.
 * `undefined` for a pattern of another shape, such as `\b__\?USE_\w+\?__\b`.
 * Between two `\b`, no word of a list may be a prefix of another that a
 * character other than `\w` follows there: both could then match, and the
 * alternation's order, which the list does not keep, would decide. */
function supportList(regex: string): { form: string; words: string[] } | undefined {
  const forms: [string, RegExp][] = [
    ["word", /^(\(\?i\))?\\b((?:[\w|()?:\[\]-]|\\\?)*)\\b$/],
    ["dots", /^()\\\.\\\.\(\?:([\w|()?:]*)\)\\b$/],
    ["brace", /^()\\\{\(([\w|()?:\[\]-]*)\)\}$/],
  ];
  for (const [form, pattern] of forms) {
    const m = pattern.exec(regex);
    if (!m) continue;
    const words = [...new Set(expand(m[2]!).map((word) => (m[1] ? word.toLowerCase() : word)))].sort();
    if (form === "word") {
      for (const a of words) {
        for (const b of words) {
          if (b.length > a.length && b.startsWith(a) && !/\w/.test(b[a.length]!)) throw new Error(`${regex}: ${a} and ${b} both end at a boundary`);
        }
      }
    }
    return { form, words };
  }
  return undefined;
}
const supportIds = patternList.map((p) => p.id).filter((id) => id.startsWith("SupportAt"));
for (const id of supportIds) {
  const list = supportList(patternById.get(id)!.regex);
  if (list === undefined) continue;
  w += `\n/// The words of the \`${id}\` pattern, read ${list.form === "word" ? "between two `\\b`" : list.form === "dots" ? "after `..`" : "inside `{…}`"}.\npub(super) const ${constName(id)}: &[&[u8]] = &[${list.words.map(bytes).join(", ")}];\n`;
  const regex = patternById.get(id)!.regex;
  if (list.form === "word" && regex.startsWith("(?i)")) {
    const accepted = acceptedSpellings(regex, list.words).sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
    w += `\n/// The spellings of the \`${id}\` pattern's words with \`ß\` or a Latin ligature that Oniguruma accepts, as [\`SPELLINGS\`] keeps them.\npub(super) const ${constName(id)}_SPELLINGS: &[&[u8]] = &[${accepted.map(utf8).join(", ")}];\n`;
  }
}
/** The directive words of a preprocessor rule, as written: the group after
 * `^\\s*[#%]\\s*`, which `\\b` follows, or the function names after `%`,
 * which `\\s*\\(` follows.
 * Each list's words are made of `\\w`, so the word is the whole run of `\\w`
 * before the next character. */
const directives: [string, RegExp][] = [
  ["PreprocessorAt0", /^\^\\s\*\[#%\]\\s\*\((.*)\)\\b$/],
  ["PreprocessorAt1", /^\^\\s\*\[#%\]\\s\*\((.*)\)\\b\\s\+$/],
  ["PreprocessorAt2", /^\^\\s\*\[#%\]\\s\*\((.*)\)\\b$/],
  ["PreprocessorAt3", /^\^\\s\*\[#%\]\\s\*\((.*)\)\\b$/],
  ["PreprocessorFunctionsAt0", /^\(\(%\)\((.*)\)\\s\*\(\\\(\)\)$/],
];
for (const [id, form] of directives) {
  const body = form.exec(patternById.get(id)!.regex)?.[1];
  if (body === undefined) throw new Error(`${id} changed shape`);
  const words = [...new Set(expand(body))].sort();
  if (!words.every((word) => /^\w+$/.test(word))) throw new Error(`${id}: a directive that is not all \\w`);
  w += `\n/// The directive words of the \`${id}\` pattern.\npub(super) const ${constName(id)}: &[&[u8]] = &[${words.map(bytes).join(", ")}];\n`;
}
await writeRust(`${root_dir}src/asm/words.rs`, w);

await writeRust(`${root_dir}src/asm/scope.rs`, scopeFile("asm-grammar.ts", "ASM", scopes));

if (Bun.env["RX_DUMP"]) {
  const dump = [...mergedPats, ...patternList.filter((p) => p.id !== "Word")];
  await Bun.write(
    Bun.env["RX_DUMP"],
    dump.map((p) => JSON.stringify({ id: p.id, regex: p.regex, groups: p.captures.map(([n]) => n) })).join("\n") + "\n",
  );
}
console.log(
  `${ruleList.length} rules, ${patternList.length} patterns (${merged.length} merged into Word), ${scopes.size} scopes, ${sortedWords.length} words (${blob.length} bytes), ToLast: ${oneBlank.map((p) => p.id).join(", ") || "none"}`,
);
