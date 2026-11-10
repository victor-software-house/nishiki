// Writes src/bat/scope.rs and src/bat/grammar.rs from the `bat` grammar
// that Shiki 4.6.0 bundles (mmims/language-batchfile, MIT): the scope names, and
// for every rule the scopes it adds, how it ends and the patterns it tries, in
// the grammar's order with its `include`s flattened. The matchers that decide
// whether a pattern matches are written by hand in src/bat/matchers.
//
// Run with `bun run oracle/bat-grammar.ts` and commit the result; the
// output is generated, do not edit it.
import bat from "@shikijs/langs@4.6.0/bat";
import { expand as spell } from "./alternation.ts";
import { arms, groupsCheck, ruleCodes, scopeFile, startTable, tooLong, writeRust } from "./rule-grammar.ts";

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

const grammar = (bat as unknown as { repository: Record<string, Node>; patterns: Node[] }[])[0]!;
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
  if (parts[parts.length - 1] === "batchfile") parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
    }
  });
  return out;
}

const scopes = new Map<string, string>([["Source", "source.batchfile"]]);
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

type Pat = {
  id: string;
  regex: string;
  act: { role: "match"; name: string[] } | { role: "begin"; rule: string } | { role: "end" };
  captures: [number, string[]][];
};
type Rule = {
  id: string;
  name: string[];
  content: string[];
  end: string;
  patterns: string[];
};

const pats = new Map<string, Pat>();
const rules = new Map<string, Rule>();

function captureList(captures: Captures | undefined): [number, string[]][] {
  if (!captures) return [];
  return Object.entries(captures)
    .map(([group, value]): [number, string[]] => {
      if (value.patterns) throw new Error(`capture ${group}: nested patterns`);
      return [Number(group), scopeList(value.name)];
    })
    .filter(([, names]) => names.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** Registers a match or begin/end rule under `id`; returns the pattern id. */
function register(node: Node, id: string): string {
  const clash = pats.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (node.match !== undefined) {
    if (!pats.has(id)) {
      pats.set(id, {
        id,
        regex: node.match,
        act: { role: "match", name: scopeList(node.name) },
        captures: captureList(node.captures),
      });
    }
    return id;
  }
  if (node.begin !== undefined && node.end !== undefined) {
    if (!pats.has(id)) {
      pats.set(id, {
        id,
        regex: node.begin,
        act: { role: "begin", rule: id },
        captures: captureList(node.beginCaptures ?? node.captures),
      });
      pats.set(`${id}End`, {
        id: `${id}End`,
        regex: node.end,
        act: { role: "end" },
        captures: captureList(node.endCaptures ?? node.captures),
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
    return id;
  }
  throw new Error(`${id}: neither a match nor a begin/end rule`);
}

/** The pattern ids a pattern list expands to, `include`s flattened. */
function flatten(list: Node[], context: string): string[] {
  const out: string[] = [];
  list.forEach((node, index) => {
    if (node.include === "$self") {
      out.push(...flatten(root.patterns!, "Root"));
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
  if (node.match !== undefined || node.begin !== undefined) return [register(node, id)];
  if (node.patterns) return flatten(node.patterns, id);
  throw new Error(`${id}: empty rule`);
}

const rootPatterns = flatten(root.patterns!, "Root");
rules.set("Root", { id: "Root", name: [], content: [], end: "", patterns: rootPatterns });

// The grammar's one injection, `L:meta.block.repeat.batchfile`, tries
// `#repeatParameter` in every rule open inside a `for` line, before the rule's
// own patterns and its end.
// The rule engine has no injections, so the `for` rule and every rule it can
// open get the pattern first in their lists.
// Those rules opened outside a `for` line keep their plain lists, so each one
// the `for` rule reaches gets a copy, named with `InFor`, which the copies'
// begin patterns open.
// Each copy's end is a pattern of its own, `…EndInFor`, whose matcher fails
// where `#repeatParameter` matches, since the injection wins a tie with the end.
const injection = (grammar as unknown as { injections: Record<string, Node> }).injections;
if (Object.keys(injection).join() !== "L:meta.block.repeat.batchfile") throw new Error("the injections changed");
const repeat = flatten(injection["L:meta.block.repeat.batchfile"]!.patterns!, "Injection");
const forRule = [...rules.values()].find((r) => r.name.join() === "MetaBlockRepeat")!.id;
const reach = new Set<string>();
const stack = [forRule];
while (stack.length > 0) {
  const id = stack.pop()!;
  if (reach.has(id)) continue;
  reach.add(id);
  for (const p of rules.get(id)!.patterns) {
    const act = pats.get(p)!.act;
    if (act.role === "begin") stack.push(act.rule);
  }
}
const inFor = (id: string) => (id === forRule ? id : `${id}InFor`);
const opensCopy = (p: string) => {
  const act = pats.get(p)!.act;
  return act.role === "begin" && act.rule !== forRule && reach.has(act.rule);
};
for (const p of [...pats.values()]) {
  if (opensCopy(p.id) && p.act.role === "begin") {
    pats.set(`${p.id}InFor`, { ...p, id: `${p.id}InFor`, act: { role: "begin", rule: inFor(p.act.rule) } });
  }
}
for (const id of reach) {
  const rule = rules.get(id)!;
  const end = pats.get(rule.end)!;
  pats.set(`${rule.end}InFor`, { ...end, id: `${rule.end}InFor` });
  if (id === forRule) pats.delete(rule.end);
  rules.set(inFor(id), {
    ...rule,
    id: inFor(id),
    end: `${rule.end}InFor`,
    patterns: [...repeat, ...rule.patterns.map((p) => (opensCopy(p) ? `${p}InFor` : p))],
  });
}
// A rule that only the `for` rule opens, such as its `in` clause, now opens
// only as its copy; the plain rule and the patterns only it tried are dropped.
const live = new Set<string>();
const open = ["Root"];
while (open.length > 0) {
  const id = open.pop()!;
  if (live.has(id)) continue;
  live.add(id);
  for (const p of rules.get(id)!.patterns) {
    const act = pats.get(p)!.act;
    if (act.role === "begin") open.push(act.rule);
  }
}
for (const id of [...rules.keys()]) if (!live.has(id)) rules.delete(id);
const used = new Set([...rules.values()].flatMap((r) => [...r.patterns, ...(r.end ? [r.end] : [])]));
for (const id of [...pats.keys()]) if (!used.has(id)) pats.delete(id);

// Output.

const list = (names: string[]) => (names.length ? `scopes![${names.map((n) => `S::${n}`).join(", ")}]` : "S::NO_SCOPES");

let g = `//! The grammar's rules as tables. Generated by \`oracle/bat-grammar.ts\` from
//! the \`bat\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pat\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::Bat;
use super::scope::S;
use crate::scope::scopes;
use crate::rules;

pub(crate) type Act = rules::Act<Bat>;
pub(crate) type PatData = rules::PatData<Bat>;
pub(crate) type RuleData = rules::RuleData<Bat>;
pub(crate) use crate::rules::Start;

`;

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
    `RuleData {\n                name: ${list(r.name)},\n                content: ${list(r.content)},\n                end: ${r.end ? `Some(Pat::${r.end})` : "None"},\n                backref: None,\n                patterns: &[${r.patterns.map((p) => `Pat::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pat {\n`;
// A regex that holds a word cut in two by an alternation is elided too: the
// spelling check reads the second half as a typo.
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 ? `…` : one;
};
for (const p of patList) {
  const role = p.act.role === "end" ? "end" : p.act.role === "begin" ? "begin" : "match";
  g += `    /// The ${role} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pat {\n${tooLong}    pub(crate) fn data(self) -> PatData {\n        match self {\n`;
g += arms(
  patList.map((p): [string, string] => {
    const act =
      p.act.role === "match"
        ? `Act::Match(${list(p.act.name)})`
        : p.act.role === "begin"
          ? `Act::Begin(Rule::${p.act.rule})`
          : "Act::End";
    const caps = p.captures.map(([n, names]) => `(${n}, ${list(names)})`).join(", ");
    return [p.id, `PatData {\n                act: ${act},\n                captures: &[${caps}],\n                nested: &[],\n            }`];
  }),
  "Pat",
);
g += `        }\n    }\n}\n`;

// The command names of `CommandsAt0`, `(?<=^|[@\s])(?i:adprep|…|xcopy)(?=$|\s)`.
// The lookahead makes an alternative match only a whole name or a whole name
// pair, so the matcher looks names up instead of trying the alternatives in turn.
// A pair such as `net user` comes before `net` in the grammar, so it wins.
const commandsRegex = pats.get("CommandsAt0")!.regex;
const commandsBody = /^\(\?<=\^\|\[@\\s\]\)\(\?i:(.*)\)\(\?=\$\|\\s\)$/.exec(commandsRegex)?.[1];
if (commandsBody === undefined) throw new Error("CommandsAt0 changed shape");
const commandList = spell(commandsBody);
const singles = [...new Set(commandList.filter((w) => !w.includes(" ")))].sort();
const pairs = commandList.filter((w) => w.includes(" "));
for (const word of commandList) if (!/^[a-z0-9_-]+( [a-z0-9_-]+)?$/.test(word)) throw new Error(`command ${word}`);
for (const pair of pairs) {
  const first = pair.split(" ")[0]!;
  const single = commandList.indexOf(first);
  if (single !== -1 && single < commandList.indexOf(pair)) throw new Error(`${first} hides ${pair}`);
}
const bytes = (w: string) => `b"${w}"`;
g += `
/// The command names of \`CommandsAt0\` that hold no space, lowercase and
/// sorted.
pub(crate) const COMMANDS: &[&[u8]] = &[${singles.map(bytes).join(", ")}];

/// The command names of \`CommandsAt0\` that hold a space, in the grammar's
/// order.
/// Each comes before its first word in the grammar.
pub(crate) const COMMAND_PAIRS: &[&[u8]] = &[${pairs.map(bytes).join(", ")}];

/// The longest command name, in bytes.
pub(crate) const LONGEST_COMMAND: usize = ${Math.max(...commandList.map((w) => w.length))};
`;

g += startTable(patList);
g += ruleCodes("Bat", "BAT");
g += groupsCheck("Bat", patList.flatMap((p) => p.captures.map(([n]) => n)));

await writeRust(`${root_dir}src/bat/grammar.rs`, g);

// Scope table.
const s = scopeFile("bat-grammar.ts", "BAT", scopes);
await writeRust(`${root_dir}src/bat/scope.rs`, s);
console.log(`${ruleList.length} rules, ${patList.length} patterns, ${scopes.size} scopes`);
