// Writes src/python/scope.rs and src/python/grammar.rs from the `python`
// grammar that Shiki 4.6.0 bundles (MagicPython, MIT): the scope names, and
// for every rule the scopes it adds, how it ends and the patterns it tries, in
// the grammar's order with its `include`s flattened. The matchers that decide
// whether a pattern matches are written by hand in src/python/matchers.
//
// Run with `bun run oracle/python-grammar.ts` and commit the result; the
// output is generated, do not edit it.
import python from "@shikijs/langs@4.6.0/python";
import { arms, groupsCheck, ruleCodes, scopeFile, startTable, tooLong } from "./rule-grammar.ts";

type Captures = Record<string, { name?: string }>;
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

const grammar = (python as unknown as { repository: Record<string, Node>; patterns: Node[] }[])[0]!;
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
  let suffix = "";
  if (parts[parts.length - 1] === "python") parts.pop();
  else if (parts[parts.length - 1] === "regexp") {
    parts.pop();
    suffix = "Re";
  }
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
    }
  });
  return out + suffix;
}

const scopes = new Map<string, string>([["Source", "source.python"]]);
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

// The grammar spells `character` wrong in one key (split here so that the
// spelling check passes); the Rust names do not.
const pascal = (key: string) =>
  key
    .replace("charec" + "ter", "character")
    .split(/[-_.]/)
    .map((piece) => piece.charAt(0).toUpperCase() + piece.slice(1))
    .join("");

type Pat = {
  id: string;
  regex: string;
  act: { kind: "match"; name: string[] } | { kind: "begin"; rule: string } | { kind: "end" };
  captures: [number, string[]][];
  /** The key of the repository entry it comes from, for flavours. */
  key: string;
};
type Rule = {
  id: string;
  key: string;
  name: string[];
  content: string[];
  end: string;
  backref: number | null;
  patterns: string[];
};

const pats = new Map<string, Pat>();
const rules = new Map<string, Rule>();

function captureList(captures: Captures | undefined): [number, string[]][] {
  if (!captures) return [];
  return Object.entries(captures)
    .map(([group, value]): [number, string[]] => [Number(group), scopeList(value.name)])
    .filter(([, names]) => names.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** Registers a match or begin/end rule under `id`; returns the pattern id. */
function register(node: Node, id: string, key: string): string {
  const clash = pats.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (node.match !== undefined) {
    if (!pats.has(id)) {
      pats.set(id, {
        id,
        regex: node.match,
        act: { kind: "match", name: scopeList(node.name) },
        captures: captureList(node.captures),
        key,
      });
    }
    return id;
  }
  if (node.begin !== undefined && node.end !== undefined) {
    if (!pats.has(id)) {
      pats.set(id, {
        id,
        regex: node.begin,
        act: { kind: "begin", rule: id },
        captures: captureList(node.beginCaptures ?? node.captures),
        key,
      });
      pats.set(`${id}End`, {
        id: `${id}End`,
        regex: node.end,
        act: { kind: "end" },
        captures: captureList(node.endCaptures ?? node.captures),
        key,
      });
      const backref = /\\([1-9])/.exec(node.end);
      const rule: Rule = {
        id,
        key,
        name: scopeList(node.name),
        content: scopeList(node.contentName),
        end: `${id}End`,
        backref: backref ? Number(backref[1]) : null,
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
    if (node.include !== undefined) {
      if (!node.include.startsWith("#")) throw new Error(`external include ${node.include}`);
      const key = node.include.slice(1);
      const target = repository[key];
      if (!target) throw new Error(`unknown include ${key}`);
      out.push(...expand(target, pascal(key), key));
    } else {
      out.push(...expand(node, `${context}At${index}`, context));
    }
  });
  return out;
}

function expand(node: Node, id: string, key: string): string[] {
  if (node.match !== undefined || node.begin !== undefined) return [register(node, id, key)];
  if (node.patterns) return flatten(node.patterns, id);
  throw new Error(`${id}: empty rule`);
}

const rootPatterns = flatten(root.patterns!, "Root");
rules.set("Root", {
  id: "Root",
  key: "",
  name: [],
  content: [],
  end: "",
  backref: null,
  patterns: rootPatterns,
});

// Output.

const flavourOf = (key: string) => {
  const m = /^(single|double)-(one|three)-/.exec(key);
  return m ? `${pascal(m[1]!)}${pascal(m[2]!)}` : null;
};

const list = (names: string[]) => (names.length ? `&[${names.map((n) => `S::${n}`).join(", ")}]` : "&[]");

let g = `//! The grammar's rules as tables. Generated by \`oracle/python-grammar.ts\` from
//! the \`python\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pat\`] is one regular expression of the grammar; its doc comment says
//! which. Each rule lists the patterns it tries, in the grammar's order with
//! its \`include\`s flattened, so the first that matches at the earliest
//! position wins as in a \`TextMate\` engine.

use super::engine::Python;
use super::scope::S;
use crate::rules;

pub(crate) type Act = rules::Act<Python>;
pub(crate) type PatData = rules::PatData<Python>;
pub(crate) type RuleData = rules::RuleData<Python>;
pub(crate) use crate::rules::Start;

/// The quote style a regular-expression rule of the grammar is written for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Flavour {
    SingleOne,
    DoubleOne,
    SingleThree,
    DoubleThree,
}

`;

/** Match arms of \`type::Id\`s, with the ids that share a body joined by \`|\`. */
const ruleList = [...rules.values()];
const patList = [...pats.values()];

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
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pat {\n`;
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 90 ? `${one.slice(0, 90)}…` : one;
};
for (const p of patList) {
  const kind = p.act.kind === "end" ? "end" : p.act.kind === "begin" ? "begin" : "match";
  g += `    /// The ${kind} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pat {\n${tooLong}    pub(crate) fn data(self) -> PatData {\n        match self {\n`;
g += arms(
  patList.map((p): [string, string] => {
    const act =
      p.act.kind === "match"
        ? `Act::Match(${list(p.act.name)})`
        : p.act.kind === "begin"
          ? `Act::Begin(Rule::${p.act.rule})`
          : "Act::End";
    const caps = p.captures.map(([n, names]) => `(${n}, ${list(names)})`).join(", ");
    return [p.id, `PatData {\n                act: ${act},\n                captures: &[${caps}],\n                nested: &[],\n            }`];
  }),
  "Pat",
);
g += `        }\n    }\n\n    /// The quote style and the kind of a pattern of a regular-expression\n    /// string, which the grammar writes out once for each quote style.\n    #[expect(clippy::too_many_lines, clippy::wildcard_enum_match_arm, reason = "a generated table")]\n    pub(crate) fn regexp(self) -> Option<(Flavour, Re)> {\n        match self {\n`;
const reKinds = new Set<string>();
const reArms: string[] = [];
for (const p of patList) {
  const m = /^(Single|Double)(One|Three)Regexp(.+)$/.exec(p.id);
  if (!m) continue;
  reKinds.add(m[3]!);
  reArms.push(`            Pat::${p.id} => Some((Flavour::${m[1]}${m[2]}, Re::${m[3]})),\n`);
}
g += reArms.join("");
g += `            _ => None,\n        }\n    }\n\n    /// Which matchers decide the pattern, so that the dispatch needs one\n    /// jump.\n    #[expect(clippy::too_many_lines, clippy::wildcard_enum_match_arm, reason = "a generated table")]\n    pub(crate) fn family(self) -> Family {\n        match self {\n`;
const families: Record<string, string[]> = { Regexp: [], Fstring: [], String: [], Code: [] };
for (const p of patList) {
  const family = /Regexp|^CommentsString/.test(p.id)
    ? "Regexp"
    : /^Fstring|^Impossible$/.test(p.id)
      ? "Fstring"
      : /^(String|Docstring|EscapeSequence)/.test(p.id)
        ? "String"
        : "Code";
  families[family]!.push(`Pat::${p.id}`);
}
for (const f of ["Regexp", "Fstring", "String"]) g += `            ${families[f]!.join("\n            | ")} => Family::${f},\n`;
g += `            _ => Family::Code,\n        }\n    }\n}\n\n/// The group of matchers that decides a pattern.\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub(crate) enum Family {\n    Code,\n    String,\n    Fstring,\n    Regexp,\n}\n\n/// The kinds of pattern a regular-expression string has for each quote style.\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub(crate) enum Re {\n`;
for (const k of reKinds) g += `    ${k},\n`;
g += `}\n`;


g += startTable(patList);
g += ruleCodes("Python", "PYTHON");
g += groupsCheck("Python", [...patList.flatMap((p) => [...p.captures.map(([n]) => n)]), ...ruleList.flatMap((r) => (r.backref === null ? [] : [r.backref]))]);

await Bun.write(`${root_dir}src/python/grammar.rs`, g);

// Scope table.
const s = scopeFile("python-grammar.ts", "PYTHON", scopes);
await Bun.write(`${root_dir}src/python/scope.rs`, s);
console.log(`${ruleList.length} rules, ${patList.length} patterns, ${scopes.size} scopes`);
