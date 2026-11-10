// Writes src/r/scope.rs and src/r/grammar.rs from the `r` grammar that Shiki
// 4.6.0 bundles (posit-dev/positron, `extensions/positron-r/syntaxes`, MIT).
//
// grammar.rs holds the scopes each rule and pattern adds, how each rule ends
// and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// A pattern or rule that repeats another, with the same scopes, is the same
// `Pattern` or `Rule`, so it has one matcher.
//
// A raw string's end and a roxygen fence's end repeat a group of their begin,
// which the rule carries as `CarriedState::Backref`.
// `@examplesIf` lexes its condition again as an expression, through a rule of
// its own.
//
// The matchers that decide whether a pattern matches are written by hand in
// src/r/matchers.
//
// Run with `bun run oracle/r-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import r from "@shikijs/langs@4.6.0/r";
import {
  arms,
  groupsCheck,
  ruleCodes,
  scopeFile,
  startOf,
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
  applyEndPatternLast?: boolean | number;
  while?: string;
};

const grammar = (
  r as unknown as {
    name: string;
    scopeName: string;
    repository: Record<string, Node>;
    patterns: Node[];
  }[]
).find((lang) => lang.name === "r")!;
const repository = grammar.repository;

const root_dir = new URL("..", import.meta.url).pathname;

// Scope names.

const abbreviations: Record<string, string> = {
  punctuation: "P",
  keyword: "Kw",
  constant: "Const",
  invalid: "Inv",
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
  // A name without the language's suffix, such as `keyword`, ends in `Bare`.
  const suffix = parts[parts.length - 1] === "r" && parts.length > 1 ? "" : "Bare";
  if (!suffix) parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const capture = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? capture) : capture;
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
/** The scopes of a rule or a capture, by identifier. */
function scopeList(name: string | undefined): string[] {
  if (name === undefined) return [];
  return name.split(" ").map((part) => {
    if (part.includes("$")) throw new Error(`${part}: a dynamic scope`);
    return scope(part);
  });
}

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
  /** Capture groups that the grammar tokenizes with patterns of their own. */
  nested: [number, string][];
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
/** The id each node registered under, by the node's text. */
const seen = new Map<string, string>();
/** The rule that tokenizes the text of a capture group with the patterns
 * `list`, one per distinct list. */
const nestedRules = new Map<string, string>();

function nestedRule(list: Node[]): string {
  const key = JSON.stringify(list);
  const known = nestedRules.get(key);
  if (known !== undefined) return known;
  const id = `Nested${nestedRules.size}`;
  nestedRules.set(key, id);
  const rule: Rule = { id, name: [], content: [], end: "", backref: null, patterns: [] };
  rules.set(id, rule);
  rule.patterns = flatten(list, id);
  return id;
}

/** The capture groups whose text the grammar tokenizes with patterns, with
 * the rule that does. */
function nestedList(captures: Captures | undefined): [number, string][] {
  if (!captures) return [];
  const out: [number, string][] = [];
  for (const [group, value] of Object.entries(captures)) {
    if (value.patterns === undefined) continue;
    out.push([Number(group), nestedRule(value.patterns)]);
  }
  return out.sort((a, b) => a[0] - b[0]);
}

function captureList(captures: Captures | undefined): [number, string[]][] {
  if (!captures) return [];
  return Object.entries(captures)
    .map(([group, value]): [number, string[]] => [Number(group), scopeList(value.name)])
    .filter(([, names]) => names.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** Registers a match or begin/end rule under `id`, unless the same node is
 * registered already; returns the pattern id. */
function register(node: Node, id: string): string {
  const key = JSON.stringify(node);
  const known = seen.get(key);
  if (known !== undefined) return known;
  if (patternById.has(id)) throw new Error(`${id} names two rules`);
  seen.set(key, id);
  if (node.while !== undefined) throw new Error(`${id}: while`);
  if (node.applyEndPatternLast !== undefined) throw new Error(`${id}: applyEndPatternLast`);
  if (node.match !== undefined) {
    patternById.set(id, {
      id,
      regex: node.match,
      act: { role: "match", name: scopeList(node.name) },
      captures: captureList(node.captures),
      nested: nestedList(node.captures),
    });
    return id;
  }
  if (node.begin !== undefined && node.end !== undefined) {
    patternById.set(id, {
      id,
      regex: node.begin,
      act: { role: "begin", rule: id },
      captures: captureList(node.beginCaptures ?? node.captures),
      nested: nestedList(node.beginCaptures ?? node.captures),
    });
    const end = `${id}End`;
    const endCaptures = node.endCaptures ?? node.captures;
    patternById.set(end, {
      id: end,
      regex: node.end,
      act: { role: "end" },
      captures: captureList(endCaptures),
      nested: nestedList(endCaptures),
    });
    const backrefs = [...new Set([...node.end.matchAll(/\\([1-9])/g)].map((m) => Number(m[1])))];
    if (backrefs.length > 1) throw new Error(`${id}: the end repeats more than one group`);
    const rule: Rule = {
      id,
      name: scopeList(node.name),
      content: scopeList(node.contentName),
      end,
      backref: backrefs[0] ?? null,
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
    if (node.include === "$self" || node.include === "source.r") {
      out.push(...flatten(grammar.patterns, "Root"));
    } else if (node.include !== undefined && !node.include.startsWith("#")) {
      throw new Error(`${context}: external include ${node.include}`);
    } else if (node.include !== undefined) {
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

/** A repository entry that lists one pattern takes the entry's name.
 * An entry with a name and patterns but no begin, as `raw-strings` is, adds
 * no scope: vscode-textmate reads a name only on a match or begin/end rule. */
function expand(node: Node, id: string): string[] {
  if (node.match !== undefined || node.begin !== undefined) return [register(node, id)];
  if (node.patterns?.length === 1 && node.patterns[0]!.include === undefined) {
    return [register(node.patterns[0]!, id)];
  }
  if (node.patterns) return flatten(node.patterns, id);
  throw new Error(`${id}: empty rule`);
}

const rootRule: Rule = { id: "Root", name: [], content: [], end: "", backref: null, patterns: [] };
rules.set("Root", rootRule);
rootRule.patterns = flatten(grammar.patterns, "Root");
const ruleList = [...rules.values()];
const patternList = [...patternById.values()];

// Output: the tables.

const list = (ids: string[]) =>
  ids.length ? `scopes![${ids.map((id) => `S::${id}`).join(", ")}]` : "S::NO_SCOPES";

let g = `//! The grammar's rules as tables. Generated by \`oracle/r-grammar.ts\`
//! from the \`r\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::R;
use super::scope::S;
use crate::rules;
use crate::scope::scopes;

pub(crate) type Act = rules::Act<R>;
pub(crate) type PatternDefinition = rules::PatternDefinition<R>;
pub(crate) type RuleDefinition = rules::RuleDefinition<R>;
pub(crate) use crate::rules::Start;

`;

if (ruleList.length > 256) throw new Error(`${ruleList.length} rules do not fit one byte`);

g += `/// A rule of the grammar that can be open at the end of a line.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub(crate) enum Rule {\n`;
for (const rule of ruleList) g += `    ${rule.id},\n`;
g += `}\n\nimpl Rule {\n    /// Every rule, in code order.\n    pub(crate) const ALL: [Rule; ${ruleList.length}] = [\n`;
for (const rule of ruleList) g += `        Rule::${rule.id},\n`;
g += `    ];\n\n${tooLong}    pub(crate) fn definition(self) -> RuleDefinition {\n        match self {\n`;
g += arms(
  ruleList.map((rule): [string, string] => [
    rule.id,
    `RuleDefinition {\n                name: ${list(rule.name)},\n                content: ${list(rule.content)},\n                end: ${rule.end ? `Some(Pattern::${rule.end})` : "None"},\n                backref: ${rule.backref === null ? "None" : `Some(${rule.backref})`},\n                patterns: &[${rule.patterns.map((p) => `Pattern::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n\n    /// What a rule carries from line to line: the begin capture group its end\n    /// repeats, when it does.\n    #[inline]\n    pub(crate) fn carried_state(self) -> rules::CarriedState {\n        match self {\n`;
g += arms(
  ruleList.map((rule): [string, string] => [
    rule.id,
    rule.backref === null
      ? "rules::CarriedState::None"
      : `rules::CarriedState::Backref(${rule.backref})`,
  ]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pattern {\n`;
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 || /[^\x20-\x7e]/.test(one) ? `…` : one;
};
for (const p of patternList) {
  g += `    /// The ${p.act.role} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pattern {\n    /// Every pattern, for the tool that compares the matchers with Oniguruma.\n    #[cfg(test)]\n    pub(crate) const ALL: [Pattern; ${patternList.length}] = [\n${patternList.map((p) => `        Pattern::${p.id},\n`).join("")}    ];\n\n${tooLong}    pub(crate) fn definition(self) -> PatternDefinition {\n        match self {\n`;
g += arms(
  patternList.map((p): [string, string] => {
    const act =
      p.act.role === "match"
        ? `Act::Match(${list(p.act.name)})`
        : p.act.role === "begin"
          ? `Act::Begin(Rule::${p.act.rule})`
          : "Act::End";
    const captures = p.captures.map(([n, ids]) => `(${n}, ${list(ids)})`).join(", ");
    const nested = p.nested.map(([n, rule]) => `(${n}, Rule::${rule})`).join(", ");
    return [
      p.id,
      `PatternDefinition {\n                act: ${act},\n                captures: &[${captures}],\n                nested: &[${nested}],\n            }`,
    ];
  }),
  "Pattern",
);
g += `        }\n    }\n}\n`;

for (const p of patternList) {
  if (startOf(p.regex).any && !p.regex.includes("\\G")) {
    throw new Error(`${p.id}: the first-byte analysis does not read ${p.regex}`);
  }
}
g += startTable(
  patternList.map((p) => ({ id: p.id, regex: p.regex })),
  "any",
);
g += ruleCodes("R", "R");
g += groupsCheck(
  "R",
  patternList.flatMap((p) => [...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]),
);

await writeRust(`${root_dir}src/r/grammar.rs`, g);
await writeRust(`${root_dir}src/r/scope.rs`, scopeFile("r-grammar.ts", "R", scopes));

// With RX_DUMP set to a file, writes each pattern for oracle/onig.ts, as the
// matchers answer for it.
// An end that repeats a group of its begin is left out, since a case cannot
// supply that group's text.
if (Bun.env["RX_DUMP"]) {
  const groupsOf = (p: Pattern) => [
    ...new Set([...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]),
  ];
  const probed = patternList.filter((p) => !(p.act.role === "end" && /\\[1-9]/.test(p.regex)));
  await Bun.write(
    Bun.env["RX_DUMP"],
    probed
      .map((p) => JSON.stringify({ id: p.id, regex: p.regex, groups: groupsOf(p) }))
      .join("\n") + "\n",
  );
}

console.log(`${ruleList.length} rules, ${patternList.length} patterns, ${scopes.size} scopes`);
