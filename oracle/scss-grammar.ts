// Writes src/scss/scope.rs and src/scss/grammar.rs from the `scss` grammar
// that Shiki 4.6.0 bundles (microsoft/vscode, from atom/language-sass, MIT):
// the scope names, and for every rule the scopes it adds, how it ends and the
// patterns it tries, in the grammar's order with its `include`s flattened.
// The matchers that decide whether a pattern matches are written by hand in
// src/scss/matchers.
//
// The grammar includes rules of the `css` grammar (`source.css#…`). Those
// come from oracle/css-grammar.ts, with the ids it gives them prefixed by
// `Css`, so that each maps to the CSS pattern whose matcher decides it; a
// rule CSS opens there lists CSS's patterns, as the grammar does.
//
// `source.sassdoc` is not bundled: a rule whose only patterns include it is
// dropped, as vscode-textmate drops a rule whose patterns are all missing.
//
// Run with `bun run oracle/scss-grammar.ts` and commit the result; the
// output is generated, do not edit it.
import scss from "@shikijs/langs@4.6.0/scss";
import * as css from "./css-grammar.ts";
import {
  arms,
  groupsCheck,
  ruleCodes,
  scopeFile,
  startTable,
  tooLong,
  writeRust,
} from "./rule-grammar.ts";

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

const grammar = (
  scss as unknown as { name: string; repository: Record<string, Node>; patterns: Node[] }[]
).find((lang) => lang.name === "scss")!;
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

/** The scope's identifier: its parts in Pascal case, without `.scss`; a
 * `.css` name keeps a `Css` suffix, so that the two never share one. */
function scopeIdent(name: string): string {
  const parts = name.split(".");
  let suffix = "";
  if (parts[parts.length - 1] === "scss") parts.pop();
  else if (parts[parts.length - 1] === "css") {
    parts.pop();
    suffix = "Css";
  }
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const capture = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? capture) : capture;
    }
  });
  return out + suffix;
}

const scopes = new Map<string, string>([["Source", "source.css.scss"]]);
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
/** A CSS scope identifier, as SCSS names the same scope. */
const fromCss = (ident: string) => scope(css.scopes.get(ident)!);

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
  /** Capture groups that the grammar tokenizes with patterns of their own,
   * as the \`Nested\` value that does. */
  nested: [number, string][];
  /** The CSS pattern it is, for one that `source.css#…` brings in. */
  css: string | null;
};
type Rule = {
  id: string;
  name: string[];
  content: string[];
  end: string;
  patterns: string[];
};

const patternById = new Map<string, Pattern>();
const rules = new Map<string, Rule>();

/** The rule that tokenizes the text of a capture group with the patterns `list`, one per distinct list. */
const nestedRules = new Map<string, string>();
function nestedRule(list: Node[]): string {
  const key = JSON.stringify(list);
  const known = nestedRules.get(key);
  if (known !== undefined) return known;
  const id = `Nested${nestedRules.size}`;
  nestedRules.set(key, id);
  const rule: Rule = { id, name: [], content: [], end: "", patterns: [] };
  rules.set(id, rule);
  rule.patterns = flatten(list, id);
  return id;
}

function nestedList(captures: Captures | undefined): [number, string][] {
  if (!captures) return [];
  const out: [number, string][] = [];
  for (const [group, value] of Object.entries(captures)) {
    if (value.patterns === undefined) continue;
    out.push([Number(group), `Rule(Rule::${nestedRule(value.patterns as Node[])})`]);
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

/** Whether every pattern of `list` includes a grammar that is not bundled. */
function allMissing(list: Node[]): boolean {
  return list.length > 0 && list.every((node) => node.include === "source.sassdoc");
}

/** Registers a match or begin/end rule under `id`; returns its pattern ids,
 * none for a rule dropped for missing patterns. */
function register(node: Node, id: string): string[] {
  const clash = patternById.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (node.match !== undefined) {
    if (!patternById.has(id)) {
      patternById.set(id, {
        id,
        regex: node.match,
        act: { role: "match", name: scopeList(node.name) },
        captures: captureList(node.captures),
        nested: nestedList(node.captures),
        css: null,
      });
    }
    return [id];
  }
  if (node.begin !== undefined && node.end !== undefined) {
    if (allMissing(node.patterns ?? [])) return [];
    if (!patternById.has(id)) {
      patternById.set(id, {
        id,
        regex: node.begin,
        act: { role: "begin", rule: id },
        captures: captureList(node.beginCaptures ?? node.captures),
        nested: nestedList(node.beginCaptures ?? node.captures),
        css: null,
      });
      patternById.set(`${id}End`, {
        id: `${id}End`,
        regex: node.end,
        act: { role: "end" },
        captures: captureList(node.endCaptures ?? node.captures),
        nested: nestedList(node.endCaptures ?? node.captures),
        css: null,
      });
      const rule: Rule = {
        id,
        name: scopeList(node.name),
        content: scopeList(node.contentName),
        end: `${id}End`,
        patterns: [],
      };
      rules.set(id, rule);
      rule.patterns = flatten(node.patterns ?? [], id);
    }
    return [id];
  }
  throw new Error(`${id}: neither a match nor a begin/end rule`);
}

/** The SCSS id of CSS pattern `id`, registering it, and the CSS rule it
 * opens with that rule's patterns, on first use. */
function fromCssPattern(id: string): string {
  const own = `Css${id}`;
  if (patternById.has(own)) return own;
  const pattern = css.patternById.get(id)!;
  const act: Pattern["act"] =
    pattern.act.role === "match"
      ? { role: "match", name: pattern.act.name.map(fromCss) }
      : pattern.act.role === "begin"
        ? { role: "begin", rule: `Css${pattern.act.rule}` }
        : { role: "end" };
  patternById.set(own, {
    id: own,
    regex: pattern.regex,
    act,
    captures: pattern.captures.map(([n, names]): [number, string[]] => [n, names.map(fromCss)]),
    nested: pattern.nested.map(([n, name]): [number, string] => [n, `Css(CssNested::${name})`]),
    css: id,
  });
  if (pattern.act.role === "begin") fromCssRule(pattern.act.rule);
  return own;
}

function fromCssRule(id: string): void {
  const own = `Css${id}`;
  if (rules.has(own)) return;
  const rule = css.rules.get(id)!;
  const mine: Rule = {
    id: own,
    name: rule.name.map(fromCss),
    content: rule.content.map(fromCss),
    end: `Css${rule.end}`,
    patterns: [],
  };
  rules.set(own, mine);
  fromCssPattern(rule.end);
  mine.patterns = rule.patterns.map(fromCssPattern);
}

/** The pattern ids a pattern list expands to, `include`s flattened. */
function flatten(list: Node[], context: string): string[] {
  const out: string[] = [];
  list.forEach((node, index) => {
    if (node.include === "$self") {
      out.push(...flatten(root.patterns!, "Root"));
    } else if (node.include === "source.sassdoc") {
      // Not bundled: nothing to include.
    } else if (node.include?.startsWith("source.css#")) {
      const key = node.include.slice("source.css#".length);
      const target = css.repository[key];
      if (!target) throw new Error(`unknown CSS include ${key}`);
      out.push(...css.expand(target, css.pascal(key), key).map(fromCssPattern));
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
  if (node.match !== undefined || node.begin !== undefined) return register(node, id);
  if (node.patterns) return flatten(node.patterns, id);
  throw new Error(`${id}: empty rule`);
}

const rootPatterns = flatten(root.patterns!, "Root");
rules.set("Root", { id: "Root", name: [], content: [], end: "", patterns: rootPatterns });

// Output.

const list = (names: string[]) =>
  names.length ? `scopes![${names.map((n) => `S::${n}`).join(", ")}]` : "S::NO_SCOPES";

let g = `//! The grammar's rules as tables. Generated by \`oracle/scss-grammar.ts\` from
//! the \`scss\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.
//! A \`Css\` pattern or rule is the
//! CSS grammar's, which \`source.css#…\` includes.

use super::engine::Scss;
use super::scope::S;
use crate::scope::scopes;
use crate::css::{Nested as CssNested, Pattern as CssPattern};
use crate::rules;

pub(crate) type Act = rules::Act<Scss>;
pub(crate) type PatternDefinition = rules::PatternDefinition<Scss>;
pub(crate) type RuleDefinition = rules::RuleDefinition<Scss>;
pub(crate) use crate::rules::Start;

/// What tokenizes a capture group: a rule of the grammar, or one of CSS's
/// nested patterns, for a CSS pattern that \`source.css#…\` brings in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Nested {
    Rule(Rule),
    Css(CssNested),
}

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
    `RuleDefinition {\n                name: ${list(r.name)},\n                content: ${list(r.content)},\n                end: ${r.end ? `Some(Pattern::${r.end})` : "None"},\n                backref: None,\n                patterns: &[${r.patterns.map((p) => `Pattern::${p}`).join(", ")}],\n            }`,
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
g += `        }\n    }\n\n`;
g += `    /// The CSS pattern this one is, for one that \`source.css#…\` brings in.\n${tooLong}    #[inline]\n    pub(crate) fn css(self) -> Option<CssPattern> {\n        match self {\n`;
g += arms(
  patternList.map((p): [string, string] => [
    p.id,
    p.css === null ? "None" : `Some(CssPattern::${p.css})`,
  ]),
  "Pattern",
);
g += `        }\n    }\n}\n`;

g += `
/// The scope that names each of CSS's units, in the order of CSS's \`UNITS\`.
pub(crate) const UNIT_SCOPES: [S; ${css.unitScopes.length}] = [${css.unitScopes.map((u) => `S::${fromCss(u)}`).join(", ")}];
`;
g += startTable(patternList);
g += ruleCodes("Scss", "SCSS");
g += groupsCheck(
  "Scss",
  patternList.flatMap((p) => [...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]),
);

await writeRust(`${root_dir}src/scss/grammar.rs`, g);

const s = scopeFile("scss-grammar.ts", "SCSS", scopes);
await writeRust(`${root_dir}src/scss/scope.rs`, s);
if (Bun.env.SCSS_DUMP)
  await Bun.write(
    Bun.env.SCSS_DUMP,
    JSON.stringify(
      patternList.map((p) => ({
        id: p.id,
        role: p.act.role,
        regex: p.regex,
        captures: p.captures,
        nested: p.nested,
        css: p.css,
      })),
      null,
      0,
    ),
  );
console.log(`${ruleList.length} rules, ${patternList.length} patterns, ${scopes.size} scopes`);
