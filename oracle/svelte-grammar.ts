// Writes src/svelte/scope.rs and src/svelte/grammar.rs from the `svelte`
// grammar that Shiki 4.6.0 bundles (sveltejs/language-tools, MIT).
// It writes the scope names.
// For every rule it writes the scopes the rule adds, how it ends and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// The matchers that decide whether a pattern matches are written by hand in src/svelte/matchers.rs.
//
// Svelte's grammar needs more than Zig's:
//
// 1. An include of another grammar makes its rule embed that language.
//    `source.ts#object-literal` and the other TypeScript entries start
//    TypeScript at that entry, through `Grammar::embed_start`.
//    The patterns before and after the include become the rule's halts.
// 2. A scope name with `$N` repeats begin capture N.
//    The rule carries the group as its payload.
//    The name is built from a prefix, the payload and a suffix, and the static
//    scopes after it.
// 3. A capture group with patterns of its own is a nested list.
//    src/svelte/engine.rs tokenizes it by hand.
// 4. The grammar's injections add the bodies of `<script>`, `<style>` and
//    `<template>`.
//    The generator writes them as rules of their own, which the start tag's
//    rules try first.
//    An injection whose grammar nishiki does not lex, such as Stylus, gets no
//    rule, as vscode-textmate skips it when that grammar is not loaded.
//
// Run with `bun run oracle/svelte-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import svelte from "@shikijs/langs@4.6.0/svelte";
import { arms, groupsCheck, ruleCodes, scopeFile, startTable, tooLong, writeRust } from "./rule-grammar.ts";

type Captures = Record<string, { name?: string; patterns?: Node[] }>;
type Node = {
  name?: string;
  contentName?: string;
  match?: string;
  begin?: string;
  end?: string;
  while?: string;
  captures?: Captures;
  beginCaptures?: Captures;
  endCaptures?: Captures;
  whileCaptures?: Captures;
  patterns?: Node[];
  include?: string;
};
type Grammar = {
  scopeName: string;
  repository: Record<string, Node>;
  patterns: Node[];
  injections: Record<string, Node>;
};

const grammar = (svelte as unknown as Grammar[]).find((lang) => lang.scopeName === "source.svelte")!;
const repository = grammar.repository;

// `generics="…"` embeds TypeScript and ends at a repeat of its quote, group 3.
// An embedding rule's payload holds the embedded language's state, so it
// cannot also hold the quote: the rule becomes one rule per quote.
{
  const generics = repository["attributes-generics"]!;
  if (generics.begin !== '(generics)(=)(["\'])' || generics.end !== "(\\3)") {
    throw new Error("attributes-generics changed: check its split by quote");
  }
  repository["attributes-generics"] = {
    patterns: [
      { ...generics, begin: '(generics)(=)(")', end: '(")' },
      { ...generics, begin: "(generics)(=)(')", end: "(')" },
    ],
  };
}

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

/** The identifier of a scope name; Svelte's own names drop `.svelte`, and
 * the TypeScript and other names keep their language as a suffix. */
function scopeIdent(name: string): string {
  const parts = name.split(".");
  if (parts[parts.length - 1] === "svelte") parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
    }
  });
  return out;
}

const scopes = new Map<string, string>([["Source", "source.svelte"]]);
function scope(name: string): string {
  const ident = scopeIdent(name);
  const known = scopes.get(ident);
  if (known !== undefined && known !== name) throw new Error(`${ident}: ${known} and ${name}`);
  scopes.set(ident, name);
  return ident;
}
/** The scopes of a static name; an empty name adds none, as in vscode-textmate. */
function scopeList(name: string | undefined): string[] {
  if (name !== undefined && /\$\d/.test(name)) throw new Error(`${name}: a dynamic name where a static one belongs`);
  return name === undefined ? [] : name.split(" ").filter((part) => part !== "").map(scope);
}

/** A name with one scope that repeats begin capture `group`, such as
 * `meta.special.$3.svelte meta.special.start.svelte`: the static scopes before
 * it, its prefix and suffix, and the static scopes after it. */
type Dynamic = { before: string[]; prefix: string; group: number; suffix: string; after: string[] };
function dynamicName(name: string): Dynamic | null {
  const parts = name.split(" ").filter((part) => part !== "");
  const at = parts.findIndex((part) => /\$\d/.test(part));
  if (at < 0) return null;
  const m = /^(.*)\$(\d)(.*)$/.exec(parts[at]!)!;
  if (/\$\d/.test(m[1]! + m[3]!)) throw new Error(`${name}: two groups in one scope`);
  if (parts.slice(at + 1).some((part) => /\$\d/.test(part))) throw new Error(`${name}: two dynamic scopes`);
  return {
    before: parts.slice(0, at).map(scope),
    prefix: m[1]!,
    group: Number(m[2]),
    suffix: m[3]!,
    after: parts.slice(at + 1).map(scope),
  };
}

// Embedded languages.

/** A language a rule hands its text to, and the entry it starts at: null for
 * the language's root, else the name of a `start_*` function of its module. */
type Embed = { lang: string; feature: string; entry: string | null };

/** What each include of another grammar stands for: a language and entry,
 * or nothing, for a grammar nishiki does not lex. */
const externals: Record<string, Embed | null> = {
  "source.ts": { lang: "TypeScript", feature: "lang-typescript", entry: null },
  "source.ts#object-literal": { lang: "TypeScript", feature: "lang-typescript", entry: "start_object_literal" },
  "source.ts#object-binding-pattern": { lang: "TypeScript", feature: "lang-typescript", entry: "start_object_binding" },
  "source.ts#object-binding-pattern-const": {
    lang: "TypeScript",
    feature: "lang-typescript",
    entry: "start_object_binding_const",
  },
  "source.ts#array-binding-pattern": { lang: "TypeScript", feature: "lang-typescript", entry: "start_array_binding" },
  "source.ts#array-binding-pattern-const": {
    lang: "TypeScript",
    feature: "lang-typescript",
    entry: "start_array_binding_const",
  },
  "source.js": { lang: "JavaScript", feature: "lang-javascript", entry: null },
  "source.css": { lang: "Css", feature: "lang-css", entry: null },
  "source.css.postcss": { lang: "PostCss", feature: "lang-postcss", entry: null },
  "source.coffee": { lang: "CoffeeScript", feature: "lang-coffee", entry: null },
  "source.css.less": { lang: "Less", feature: "lang-less", entry: null },
  "source.css.scss": { lang: "Scss", feature: "lang-scss", entry: null },
  "source.sass": { lang: "Sass", feature: "lang-sass", entry: null },
  "source.stylus": null,
  "text.pug": null,
  "text.html.markdown": { lang: "Markdown", feature: "lang-markdown", entry: null },
};

/** The repository rules that stand for one TypeScript entry as a whole:
 * `type-parameters` copies the patterns of TypeScript's own type parameter
 * list, between its `<` and `>`. */
const entryRules: Record<string, Embed> = {
  "type-parameters": { lang: "TypeScript", feature: "lang-typescript", entry: "start_type_parameters" },
};

// Rules and patterns.

/** Readable ids for rules whose position would otherwise name them. */
const renames: Record<string, string> = {
  AttributesGenericsAt0: "GenericsDoubleQuoted",
  AttributesGenericsAt1: "GenericsSingleQuoted",
};

const pascal = (key: string) =>
  key
    .split(/[-_.]/)
    .map((piece) => piece.charAt(0).toUpperCase() + piece.slice(1))
    .join("");

type Act = { role: "match"; name: string[] } | { role: "begin"; rule: string } | { role: "end" } | { role: "while" };
type Pat = {
  id: string;
  regex: string;
  act: Act;
  captures: [number, string[]][];
  nested: [number, string][];
};
type Rule = {
  id: string;
  name: string[];
  dynamic: Dynamic | null;
  content: string[];
  end: string;
  while: string | null;
  /** The begin capture group that the end pattern repeats. */
  endBackref: number | null;
  /** The begin capture group the rule carries: the one its end or its name
   * repeats. */
  backref: number | null;
  embed: Embed | null;
  /** The patterns, and for an embedding rule, how many come before the
   * embedded grammar; the rest come after it. */
  patterns: string[];
  before: number;
  /** How many patterns at the head of `patterns` are `L:` injections.
   * vscode-textmate takes an `L:` injection over the rule's own match where
   * both start at one place, the end pattern included.
   * So the end pattern comes after them. */
  injected: number;
};

const pats = new Map<string, Pat>();
const rules = new Map<string, Rule>();

/** The nested lists, by the include or the inline patterns they stand for.
 * Each is a rule with patterns only, which `retokenise` runs over the group. */
const nestedNames = new Map<string, string>();

/** The nested lists of a group that another language lexes again, by the grammar
 * its one pattern includes. The engine defers such a group to that
 * language (`Embeds::lex_nested`), so its rule has no patterns. */
const deferredNames: Record<string, string> = {
  "source.ts": "NestedTypeScript",
  "text.html.markdown": "NestedMarkdown",
};

function nestedName(group: string, patterns: Node[], context: string): string | null {
  const only = patterns.length === 1 ? patterns[0]! : null;
  const key =
    only?.include !== undefined
      ? only.include
      : patterns.map((p) => p.match ?? p.include ?? "?").join(" | ");
  const known = nestedNames.get(key);
  if (known !== undefined) return known;
  const deferred = only?.include === undefined ? undefined : deferredNames[only.include];
  const name =
    only?.include?.startsWith("#") === true
      ? `Nested${pascal(only.include.slice(1))}`
      : (deferred ?? `Nested${context}Group${group}`);
  nestedNames.set(key, name);
  const rule: Rule = {
    id: name,
    name: [],
    dynamic: null,
    content: [],
    end: "",
    while: null,
    endBackref: null,
    backref: null,
    embed: null,
    patterns: [],
    before: 0,
    injected: 0,
  };
  rules.set(name, rule);
  if (deferred !== undefined) return name;
  const members = flatten(patterns, name);
  if (members.some((member) => "embed" in member)) throw new Error(`${name}: a nested list embeds a grammar`);
  rule.patterns = members.map((member) => (member as { pat: string }).pat);
  return name;
}

function captureList(captures: Captures | undefined): [number, string[]][] {
  if (!captures) return [];
  return Object.entries(captures)
    .map(([group, value]): [number, string[]] => [Number(group), scopeList(value.name)])
    .filter(([, names]) => names.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

function nestedList(captures: Captures | undefined, context: string): [number, string][] {
  if (!captures) return [];
  const out: [number, string][] = [];
  for (const [group, value] of Object.entries(captures)) {
    if (value.patterns === undefined) continue;
    const name = nestedName(group, value.patterns, context);
    if (name !== null) out.push([Number(group), name]);
  }
  return out.sort((a, b) => a[0] - b[0]);
}

/** One member of a flattened pattern list: a pattern id, or the embedded
 * grammar of an include of another one. */
type Member = { pat: string } | { embed: Embed };

/** Registers a match or begin rule under `id`; returns the pattern id. */
function register(node: Node, id: string): string {
  const clash = pats.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (pats.has(id)) return id;
  if (node.match !== undefined) {
    // vscode-textmate ignores the patterns of a match rule.
    pats.set(id, {
      id,
      regex: node.match,
      act: { role: "match", name: scopeList(node.name) },
      captures: captureList(node.captures),
      nested: nestedList(node.captures, id),
    });
    return id;
  }
  if (node.begin === undefined || (node.end === undefined && node.while === undefined)) {
    throw new Error(`${id}: neither a match nor a begin rule`);
  }
  pats.set(id, {
    id,
    regex: node.begin,
    act: { role: "begin", rule: id },
    captures: captureList(node.beginCaptures ?? node.captures),
    nested: nestedList(node.beginCaptures ?? node.captures, id),
  });
  const closing = node.end ?? node.while!;
  const closeId = node.end !== undefined ? `${id}End` : `${id}While`;
  const closeCaptures = node.end !== undefined ? (node.endCaptures ?? node.captures) : (node.whileCaptures ?? node.captures);
  pats.set(closeId, {
    id: closeId,
    regex: closing,
    act: node.end !== undefined ? { role: "end" } : { role: "while" },
    captures: captureList(closeCaptures),
    nested: nestedList(closeCaptures, closeId),
  });
  const dynamic = node.name !== undefined ? dynamicName(node.name) : null;
  const endGroups = [...new Set([...closing.matchAll(/\\([1-9])/g)].map((m) => Number(m[1])))];
  if (endGroups.length > 1) throw new Error(`${id}: the end repeats more than one group`);
  const groups = [...new Set([...endGroups, ...(dynamic === null ? [] : [dynamic.group])])];
  if (groups.length > 1) throw new Error(`${id}: the end and the name repeat different groups`);
  const rule: Rule = {
    id,
    name: dynamic === null ? scopeList(node.name) : dynamic.before,
    dynamic,
    content: scopeList(node.contentName),
    end: node.end !== undefined ? closeId : "",
    while: node.while !== undefined ? closeId : null,
    endBackref: endGroups[0] ?? null,
    backref: groups[0] ?? null,
    embed: null,
    patterns: [],
    before: 0,
    injected: 0,
  };
  rules.set(id, rule);
  const members = flatten(node.patterns ?? [], id);
  const embeds = members.filter((member) => "embed" in member);
  if (embeds.length > 1) throw new Error(`${id}: embeds two grammars`);
  if (embeds.length === 1) {
    if (rule.backref !== null) throw new Error(`${id}: embeds and repeats a group`);
    rule.embed = (embeds[0] as { embed: Embed }).embed;
    rule.before = members.findIndex((member) => "embed" in member);
  }
  rule.patterns = members.flatMap((member) => ("pat" in member ? [member.pat] : []));
  return id;
}

/** The members a pattern list expands to, `include`s flattened. */
function flatten(list: Node[], context: string): Member[] {
  const out: Member[] = [];
  list.forEach((node, index) => {
    if (node.include === undefined) {
      out.push(...expand(node, `${context}At${index}`));
      return;
    }
    if (!node.include.startsWith("#")) {
      if (!(node.include in externals)) throw new Error(`${context}: unknown include ${node.include}`);
      const embed = externals[node.include]!;
      if (embed !== null) out.push({ embed });
      return;
    }
    const key = node.include.slice(1);
    if (key in entryRules) {
      out.push({ embed: entryRules[key]! });
      return;
    }
    const target = repository[key];
    if (!target) throw new Error(`unknown include ${key}`);
    out.push(...expand(target, pascal(key)));
  });
  return out;
}

function expand(node: Node, id: string): Member[] {
  if (node.match !== undefined || node.begin !== undefined) return [{ pat: register(node, renames[id] ?? id) }];
  if (node.patterns) return flatten(node.patterns, id);
  throw new Error(`${id}: empty rule`);
}

// The injections: the bodies of the language tags.
// vscode-textmate tries an `L:` injection before the rule's own patterns,
// in every rule whose scopes its selector matches. Inside a language tag
// those are the tag's rule and its `meta.lang.$3` rule. Each body becomes a
// rule those two try first; its matcher checks the tag and the `lang` value
// from their payloads, which stand for the selector.

/** Each body rule and the selectors it stands for. */
const bodies = new Map<string, string[]>();
for (const [selector, injection] of Object.entries(grammar.injections)) {
  if (selector === "L:(source.ts, source.js, source.coffee)") continue; // the TypeScript embed mode
  const forms = injection.patterns!;
  for (const node of forms) {
    const include = node.patterns![0]!.include!;
    if (!include.startsWith("#") && externals[include] === null) continue; // a grammar the bundle lacks
    const lang = include.startsWith("#") ? "Template" : externals[include]!.lang;
    const form = node.while !== undefined ? "Block" : forms.length > 1 ? "Line" : "";
    const id = `Body${lang}${form}`;
    bodies.set(id, [...(bodies.get(id) ?? []), selector]);
    register(node, id);
  }
}

const rootMembers = flatten(grammar.patterns, "Root");
if (rootMembers.some((member) => "embed" in member)) throw new Error("the root embeds a grammar");
rules.set("Root", {
  id: "Root",
  name: [],
  dynamic: null,
  content: [],
  end: "",
  while: null,
  endBackref: null,
  backref: null,
  embed: null,
  patterns: rootMembers.map((member) => (member as { pat: string }).pat),
  before: 0,
  injected: 0,
});
for (const id of ["TagsLang", "TagsLangAt0"]) {
  const rule = rules.get(id);
  if (rule === undefined) throw new Error(`no rule ${id} for the bodies`);
  rule.patterns = [...bodies.keys(), ...rule.patterns];
  rule.injected += bodies.size;
}

// The template body's selector, `L:meta.template.svelte - meta.lang - (meta
// source)`, holds in every rule inside a template without `lang`.
// `meta.template.svelte` is a `meta` scope below all of them.
// So a rule hides the template from it when one of its own scopes is a
// `source` or a `meta.lang` scope.
// Every other rule tries the template body first; its matcher walks the
// open rules to the template.
// The root, the embedding rules and the deferred lists run no such patterns.

/** Whether one of the rule's own scopes hides the template body's selector. */
function hidesTemplate(rule: Rule): boolean {
  const idents = [...rule.name, ...rule.content, ...(rule.dynamic?.before ?? []), ...(rule.dynamic?.after ?? [])];
  const names = [...idents.map((ident) => scopes.get(ident)!), rule.dynamic?.prefix ?? ""];
  return names.some((name) => name.startsWith("source.") || name.startsWith("meta.lang"));
}
for (const rule of rules.values()) {
  const deferred = Object.values(deferredNames).includes(rule.id);
  if (rule.id === "Root" || rule.embed !== null || deferred || rule.patterns.includes("BodyTemplate") || hidesTemplate(rule)) continue;
  rule.patterns = ["BodyTemplate", ...rule.patterns];
  rule.injected += 1;
}

// Output.

const list = (names: string[]) => (names.length ? `scopes![${names.map((n) => `S::${n}`).join(", ")}]` : "S::NO_SCOPES");

const ruleList = [...rules.values()];
const patList = [...pats.values()];
if (ruleList.length > 256) throw new Error(`${ruleList.length} rules do not fit one byte`);

let g = `//! The grammar's rules as tables. Generated by \`oracle/svelte-grammar.ts\`
//! from the \`svelte\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pat\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.
//!
//! A rule that includes another grammar embeds that language; the patterns
//! it lists before the include are its halts.

use super::engine::Svelte;
use super::scope::S;
use crate::rules::{self, embedded};
use crate::scope::scopes;

pub(crate) type Act = rules::Act<Svelte>;
pub(crate) type PatData = rules::PatData<Svelte>;
pub(crate) type RuleData = rules::RuleData<Svelte>;
pub(crate) use crate::rules::Start;

/// The start function of a TypeScript entry, when TypeScript is built.
macro_rules! entry {
    ($start:ident) => {{
        #[cfg(feature = "lang-typescript")]
        let start: Option<fn(&mut crate::arena::Arena, u32) -> u32> = Some(crate::typescript::$start);
        #[cfg(not(feature = "lang-typescript"))]
        let start = None;
        start
    }};
}

`;

g += `/// A rule of the grammar that can be open at the end of a line.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub(crate) enum Rule {\n`;
for (const r of ruleList) g += `    ${r.id},\n`;
g += `}\n\nimpl Rule {\n    /// Every rule, in code order.\n    pub(crate) const ALL: [Rule; ${ruleList.length}] = [\n`;
for (const r of ruleList) g += `        Rule::${r.id},\n`;
g += `    ];\n\n${tooLong}    pub(crate) fn data(self) -> RuleData {\n        match self {\n`;
/** A rule's end pattern, where it goes first, and its patterns, with the end
 * after the injections where the rule has some. */
function endAndPatterns(r: Rule): [string, string[]] {
  if (r.end === "" || r.injected === 0) return [r.end, r.patterns];
  return ["", [...r.patterns.slice(0, r.injected), r.end, ...r.patterns.slice(r.injected)]];
}
g += arms(
  ruleList.map((r): [string, string] => {
    const [end, patterns] = endAndPatterns(r);
    return [
      r.id,
      `RuleData {\n                name: ${list(r.name)},\n                content: ${list(r.content)},\n                end: ${end ? `Some(Pat::${end})` : "None"},\n                backref: ${r.endBackref === null ? "None" : `Some(${r.endBackref})`},\n                patterns: &[${patterns.map((p) => `Pat::${p}`).join(", ")}],\n            }`,
    ];
  }),
  "Rule",
);
g += `        }\n    }\n\n    /// The \`while\` pattern of a rule that has one.\n    pub(crate) fn while_pat(self) -> Option<Pat> {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [r.id, r.while ? `Some(Pat::${r.while})` : "None"]),
  "Rule",
);
/** The features Cargo.toml defines: an embedded language whose feature is
 * not there yet, such as PostCSS before its lexer lands, embeds nothing. */
const features = new Set(
  [...(await Bun.file(`${root_dir}Cargo.toml`).text()).matchAll(/^([a-z_-]+) = /gm)].map((m) => m[1]!),
);
for (const r of ruleList) {
  if (r.embed !== null && !features.has(r.embed.feature)) {
    console.warn(`${r.id}: no feature ${r.embed.feature} in Cargo.toml yet; the rule embeds nothing`);
  }
}
g += `        }\n    }\n\n    /// What a rule carries from line to line.\n    /// A rule that embeds a language carries that language's state, when its\n    /// feature is on.\n    /// A rule whose end or name repeats a begin capture group carries that\n    /// group.\n    #[inline]\n    pub(crate) fn payload(self) -> rules::Payload {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    r.embed && features.has(r.embed.feature)
      ? `embedded!(${r.embed.lang}, "${r.embed.feature}").map_or(rules::Payload::None, rules::Payload::Embed)`
      : r.backref === null
        ? "rules::Payload::None"
        : `rules::Payload::Backref(${r.backref})`,
  ]),
  "Rule",
);
g += `        }\n    }\n\n    /// The TypeScript entry an embedding rule starts at; none for the root.\n    pub(crate) fn embed_start(self) -> Option<fn(&mut crate::arena::Arena, u32) -> u32> {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [r.id, r.embed?.entry ? `entry!(${r.embed.entry})` : "None"]),
  "Rule",
);
g += `        }\n    }\n\n    /// The prefix of the name scope that repeats the rule's payload.\n    pub(crate) fn name_delimiter(self) -> Option<&'static str> {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [r.id, r.dynamic ? `Some("${r.dynamic.prefix}")` : "None"]),
  "Rule",
);
g += `        }\n    }\n\n    /// The text after the payload in that name scope.\n    pub(crate) fn name_suffix(self) -> &'static str {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [r.id, `"${r.dynamic?.suffix ?? ""}"`]),
  "Rule",
);
g += `        }\n    }\n\n    /// The static name scopes after that name scope.\n    pub(crate) fn name_after(self) -> crate::scope::Scopes<S> {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [r.id, list(r.dynamic?.after ?? [])]),
  "Rule",
);
g += `        }\n    }\n\n    /// Whether one of the rule's own scopes, a \`source\` or a \`meta.lang\` scope,\n    /// keeps the template body's injection out of the rules inside it.\n    pub(crate) fn hides_template(self) -> bool {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [r.id, hidesTemplate(r) ? "true" : "false"]),
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
          : p.act.role === "end"
            ? "Act::End"
            : "Act::Match(S::NO_SCOPES)";
    const caps = p.captures.map(([n, names]) => `(${n}, ${list(names)})`).join(", ");
    const nested = p.nested.map(([n, name]) => `(${n}, Rule::${name})`).join(", ");
    return [p.id, `PatData {\n                act: ${act},\n                captures: &[${caps}],\n                nested: &[${nested}],\n            }`];
  }),
  "Pat",
);
g += `        }\n    }\n}\n`;

g += startTable(patList, "quote");
g += groupsCheck("Svelte", patList.flatMap((p) => [...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]));
g += ruleCodes("Svelte", "SVELTE");

await writeRust(`${root_dir}src/svelte/grammar.rs`, g);

const scopeText = scopeFile("svelte-grammar.ts", "SVELTE", scopes);
await writeRust(`${root_dir}src/svelte/scope.rs`, scopeText);

// With RX_DUMP set to a file, writes each pattern for oracle/onig.ts. An end
// that repeats a group of its begin is left out: its text comes from the
// line that opened the rule, which a probe case does not have.
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
