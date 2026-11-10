// Writes src/powershell/scope.rs and src/powershell/grammar.rs from the
// `powershell` grammar that Shiki 4.6.0 bundles (PowerShell/EditorSyntax, MIT).
// It writes the scope names.
// For every rule it writes the scopes the rule adds, how it ends and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// The matchers that decide whether a pattern matches are written by hand in src/powershell/matchers.
//
// Run with `bun run oracle/powershell-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import powershell from "@shikijs/langs@4.6.0/powershell";
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
  applyEndPatternLast?: boolean | number;
};

const grammar = (
  powershell as unknown as { repository: Record<string, Node>; patterns: Node[] }[]
)[0]!;
const repository = grammar.repository;

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
  // A name without the language's suffix, as some of the grammar's are, ends
  // in `Bare`, so that it does not take the suffixed name's identifier.
  const suffix = parts[parts.length - 1] === "powershell" ? "" : "Bare";
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

const scopes = new Map<string, string>([["Source", "source.powershell"]]);
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

/** Names for the root's own rules, which no repository key names. */
const rootNames: Record<string, string> = {
  RootAt0: "BlockComment",
  RootAt14: "SingleQuoted",
  RootAt15: "HereDouble",
  RootAt16: "HereSingle",
  RootAt18: "ArrayExpression",
  RootAt19: "ComplexSubexpression",
  RootAt23: "StopParsing",
};

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
  /** Whether the end pattern is tried after the rule's patterns
   * (`applyEndPatternLast`). */
  endLast: boolean;
  patterns: string[];
};

const patternById = new Map<string, Pattern>();
const rules = new Map<string, Rule>();

/** The grammar tokenizes no capture group with patterns of its own. */
function nestedList(captures: Captures | undefined): [number, string][] {
  for (const [group, value] of Object.entries(captures ?? {})) {
    if (value.patterns) throw new Error(`capture ${group}: nested patterns`);
  }
  return [];
}

function captureList(captures: Captures | undefined): [number, string[]][] {
  if (!captures) return [];
  return Object.entries(captures)
    .map(([group, value]): [number, string[]] => [Number(group), scopeList(value.name)])
    .filter(([, names]) => names.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** Registers a match or begin/end rule under `id`; returns the pattern id. */
function register(node: Node, id: string): string {
  const clash = patternById.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (clash) return id;
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
    patternById.set(`${id}End`, {
      id: `${id}End`,
      regex: node.end,
      act: { role: "end" },
      captures: captureList(node.endCaptures ?? node.captures),
      nested: nestedList(node.endCaptures ?? node.captures),
    });
    const rule: Rule = {
      id,
      name: scopeList(node.name),
      content: scopeList(node.contentName),
      end: `${id}End`,
      endLast: Boolean(node.applyEndPatternLast),
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
      if (node.include === "$self") {
        out.push(...flatten(grammar.patterns, "Root"));
        return;
      }
      if (!node.include.startsWith("#")) throw new Error(`external include ${node.include}`);
      const key = node.include.slice(1);
      const target = repository[key];
      if (!target) throw new Error(`unknown include ${key}`);
      out.push(...expand(target, pascal(key)));
    } else {
      const id = `${context}At${index}`;
      out.push(...expand(node, rootNames[id] ?? id));
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

const rootRule: Rule = { id: "Root", name: [], content: [], end: "", endLast: false, patterns: [] };
rules.set("Root", rootRule);
rootRule.patterns = flatten(grammar.patterns, "Root");

// Output.

const list = (names: string[]) =>
  names.length ? `scopes![${names.map((n) => `S::${n}`).join(", ")}]` : "S::NO_SCOPES";

let g = `//! The grammar's rules as tables. Generated by \`oracle/powershell-grammar.ts\`
//! from the \`powershell\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.
//!
//! A rule that tries its end pattern last lists it after its patterns and has
//! no end of its own.

use super::engine::PowerShell;
use super::scope::S;
use crate::rules;
use crate::scope::scopes;

pub(crate) type Act = rules::Act<PowerShell>;
pub(crate) type PatternDefinition = rules::PatternDefinition<PowerShell>;
pub(crate) type RuleDefinition = rules::RuleDefinition<PowerShell>;
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
    `RuleDefinition {\n                name: ${list(r.name)},\n                content: ${list(r.content)},\n                end: ${r.end && !r.endLast ? `Some(Pattern::${r.end})` : "None"},\n                backref: None,\n                patterns: &[${[...r.patterns, ...(r.end && r.endLast ? [r.end] : [])].map((p) => `Pattern::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pattern {\n`;
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 ? `…` : one;
};
for (const p of patternList) {
  const role = p.act.role === "end" ? "end" : p.act.role === "begin" ? "begin" : "match";
  g += `    /// The ${role} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pattern {\n${tooLong}    pub(crate) fn definition(self) -> PatternDefinition {\n        match self {\n`;
g += arms(
  patternList.map((p): [string, string] => {
    const act =
      p.act.role === "match"
        ? `Act::Match(${list(p.act.name)})`
        : p.act.role === "begin"
          ? `Act::Begin(Rule::${p.act.rule})`
          : "Act::End";
    const captures = p.captures.map(([n, names]) => `(${n}, ${list(names)})`).join(", ");
    const nested = p.nested.map(([n, name]) => `(${n}, Nested::${name})`).join(", ");
    return [
      p.id,
      `PatternDefinition {\n                act: ${act},\n                captures: &[${captures}],\n                nested: &[${nested}],\n            }`,
    ];
  }),
  "Pattern",
);
g += `        }\n    }\n}\n`;

g += startTable(patternList);
g += ruleCodes("PowerShell", "POWERSHELL");
g += groupsCheck(
  "PowerShell",
  patternList.flatMap((p) => [...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]),
);

await writeRust(`${root_dir}src/powershell/grammar.rs`, g);

const s = scopeFile("powershell-grammar.ts", "POWERSHELL", scopes);
await writeRust(`${root_dir}src/powershell/scope.rs`, s);
console.log(`${ruleList.length} rules, ${patternList.length} patterns, ${scopes.size} scopes`);
