// Writes src/haskell/scope.rs and src/haskell/grammar.rs from the `haskell`
// grammar that Shiki 4.6.0 bundles (octref/language-haskell, BSD-3-Clause):
// the scope names, and for every rule the scopes it adds, how it ends and the
// patterns it tries, in the grammar's order with its `include`s flattened.
// The matchers that decide whether a pattern matches are written by hand in
// src/haskell/matchers.
//
// A scope name such as `keyword.other.$2.haskell` reads a capture group's
// text at run time.
// A rule whose own name reads a group, such as
// `meta.declaration.$2.algebraic.haskell`, gets a copy for each word the group
// can match, since its frame must keep the name on later lines; each copy's
// begin pattern matches only its word.
//
// Run with `bun run oracle/haskell-grammar.ts` and commit the result; the
// output is generated, do not edit it.
import haskell from "@shikijs/langs@4.6.0/haskell";
import { expand as spell } from "./alternation.ts";
import { arms, groupsCheck, ruleCodes, scopeFile, startTable, tooLong, writeRust } from "./rule-grammar.ts";

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

const grammar = (haskell as unknown as { repository: Record<string, Node>; patterns: Node[] }[])[0]!;
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
  if (parts[parts.length - 1] === "haskell") parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
    }
  });
  return out || "Plain";
}

const scopes = new Map<string, string>([["Source", "source.haskell"]]);
function scope(name: string): string {
  const ident = scopeIdent(name);
  const known = scopes.get(ident);
  if (known !== undefined && known !== name) throw new Error(`${ident}: ${known} and ${name}`);
  scopes.set(ident, name);
  return ident;
}

/** A scope of a rule or capture: a fixed one, or one built from a capture
 * group's text. */
type Sc = { fixed: string } | { prefix: string; group: number; suffix: string };
function scopeList(name: string | undefined): Sc[] {
  if (name === undefined) return [];
  return name.split(" ").map((part): Sc => {
    const m = /^(.*?)\$(\d+)(.*)$/.exec(part);
    if (!m) return { fixed: scope(part) };
    return { prefix: m[1]!, group: Number(m[2]), suffix: m[3]! };
  });
}
const scText = (list: Sc[]) =>
  list.length
    ? `scopes![${list
        .map((s) =>
          "fixed" in s
            ? `Sc::S(S::${s.fixed})`
            : `Sc::Group(${JSON.stringify(s.prefix)}, ${s.group}, ${JSON.stringify(s.suffix)})`,
        )
        .join(", ")}]`
    : "Sc::NO_SCOPES";

// Rules and patterns.

const pascal = (key: string) =>
  key
    .split(/[-_.]/)
    .map((piece) => piece.charAt(0).toUpperCase() + piece.slice(1))
    .join("");

type Pat = {
  id: string;
  regex: string;
  act: { role: "match"; name: Sc[] } | { role: "begin"; rule: string } | { role: "end" };
  captures: [number, Sc[]][];
  /** Capture groups that the grammar tokenizes with patterns, with the rule that does. */
  nested: [number, string][];
  /** The group and word a rule's copy accepts, for a rule named after a group. */
  word: [number, string] | null;
};
type Rule = {
  id: string;
  name: Sc[];
  content: Sc[];
  end: string;
  /** The begin capture group that the end pattern repeats, when it does. */
  backref: number | null;
  patterns: string[];
};

const pats = new Map<string, Pat>();
const rules = new Map<string, Rule>();

/** The rule that tokenizes a capture group's text with a pattern list, one
 * per distinct list, and the lists to expand in the second pass. */
const nestedRules = new Map<string, string>();
const nestedSources: { id: string; list: Node[] }[] = [];

function nestedRule(list: Node[]): string {
  const key = JSON.stringify(list);
  const known = nestedRules.get(key);
  if (known !== undefined) return known;
  const id = `Nested${nestedRules.size}`;
  nestedRules.set(key, id);
  rules.set(id, { id, name: [], content: [], end: "", backref: null, patterns: [] });
  nestedSources.push({ id, list });
  rules.get(id)!.patterns = flatten(list, id);
  return id;
}

function nestedList(captures: Captures | undefined): [number, string][] {
  if (!captures) return [];
  const out: [number, string][] = [];
  for (const [group, value] of Object.entries(captures)) {
    if (value.patterns === undefined) continue;
    out.push([Number(group), nestedRule(value.patterns)]);
  }
  return out.sort((a, b) => a[0] - b[0]);
}

function captureList(captures: Captures | undefined): [number, Sc[]][] {
  if (!captures) return [];
  return Object.entries(captures)
    .map(([group, value]): [number, Sc[]] => [Number(group), scopeList(value.name)])
    .filter(([, names]) => names.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** The source of capture group `n` of `regex`, counting capturing groups by
 * their opening parenthesis. */
function groupSource(regex: string, n: number): string {
  let count = 0;
  for (let i = 0; i < regex.length; i++) {
    const c = regex[i];
    if (c === "\\") {
      i++;
      continue;
    }
    if (c === "[") {
      let depth = 1;
      for (i++; i < regex.length && depth > 0; i++) {
        if (regex[i] === "\\") i++;
        else if (regex[i] === "[") depth++;
        else if (regex[i] === "]") depth--;
      }
      i--;
      continue;
    }
    if (c !== "(" || regex[i + 1] === "?") continue;
    count++;
    if (count !== n) continue;
    let depth = 1;
    let j = i + 1;
    for (; j < regex.length && depth > 0; j++) {
      if (regex[j] === "\\") j++;
      else if (regex[j] === "(") depth++;
      else if (regex[j] === ")") depth--;
    }
    return regex.slice(i + 1, j - 1);
  }
  throw new Error(`no group ${n} in ${regex}`);
}

/** The group a rule's own name reads, and the words that group can match. */
function namedGroup(node: Node): [number, string[]] | null {
  const m = /\$(\d+)/.exec(node.name ?? "");
  if (!m) return null;
  const group = Number(m[1]);
  return [group, spell(groupSource(node.begin!, group))];
}

/** Registers a match or begin/end rule under `id`; returns its pattern ids. */
function register(node: Node, id: string): string[] {
  const clash = pats.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (node.match !== undefined) {
    if (!pats.has(id)) {
      pats.set(id, {
        id,
        regex: node.match,
        act: { role: "match", name: scopeList(node.name) },
        captures: captureList(node.captures),
        nested: nestedList(node.captures),
        word: null,
      });
    }
    return [id];
  }
  if (node.begin !== undefined && node.end !== undefined) {
    const named = namedGroup(node);
    const copies: [string, Node, [number, string] | null][] = named
      ? named[1].map((word) => [`${id}${pascal(word)}`, { ...node, name: node.name!.replaceAll(`$${named[0]}`, word) }, [named[0], word]])
      : [[id, node, null]];
    for (const [cid, cnode, word] of copies) {
      if (pats.has(cid)) continue;
      pats.set(cid, {
        id: cid,
        regex: cnode.begin!,
        act: { role: "begin", rule: cid },
        captures: captureList(cnode.beginCaptures ?? cnode.captures),
        nested: nestedList(cnode.beginCaptures ?? cnode.captures),
        word,
      });
      if (!pats.has(`${id}End`)) {
        pats.set(`${id}End`, {
          id: `${id}End`,
          regex: cnode.end!,
          act: { role: "end" },
          captures: captureList(cnode.endCaptures ?? cnode.captures),
          nested: nestedList(cnode.endCaptures ?? cnode.captures),
          word: null,
        });
      }
      const backref = /\\([1-9])/.exec(cnode.end!);
      const rule: Rule = {
        id: cid,
        name: scopeList(cnode.name),
        content: scopeList(cnode.contentName),
        end: `${id}End`,
        backref: backref ? Number(backref[1]) : null,
        patterns: [],
      };
      rules.set(cid, rule);
      ruleSources.push({ id: cid, node: cnode, context: id });
      rule.patterns = flatten(cnode.patterns ?? [], id);
    }
    return copies.map(([cid]) => cid);
  }
  throw new Error(`${id}: neither a match nor a begin/end rule`);
}

/** The begin/end rules registered, to expand their patterns a second time. */
const ruleSources: { id: string; node: Node; context: string }[] = [];
/** The bare lists being expanded, to stop a list that includes itself. */
const expanding = new Set<string>();

/** The pattern ids a pattern list expands to, `include`s flattened. */
function flatten(list: Node[], context: string): string[] {
  const out: string[] = [];
  list.forEach((node, index) => {
    if (node.include === "$self") {
      out.push(...expandNode(root, "Root", true));
    } else if (node.include !== undefined) {
      if (!node.include.startsWith("#")) throw new Error(`external include ${node.include}`);
      const key = node.include.slice(1);
      const target = repository[key];
      if (!target) throw new Error(`unknown include ${key}`);
      out.push(...expandNode(target, pascal(key), true));
    } else {
      out.push(...expandNode(node, `${context}At${index}`, false));
    }
  });
  return out;
}

function expandNode(node: Node, id: string, shared: boolean): string[] {
  if (node.match !== undefined || node.begin !== undefined) return register(node, id);
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

const rootRule: Rule = { id: "Root", name: [], content: [], end: "", backref: null, patterns: [] };
rules.set("Root", rootRule);
rootRule.patterns = flatten(root.patterns!, "Root");

// The first pass stopped a bare list where it reached itself, which left a
// rule short of patterns it includes. Every pattern has its id by now, so a
// second pass gives each list in full.
for (const { id, node, context } of ruleSources) {
  expanding.clear();
  rules.get(id)!.patterns = flatten(node.patterns ?? [], context);
}
for (const { id, list } of nestedSources) {
  expanding.clear();
  rules.get(id)!.patterns = flatten(list, id);
}
expanding.clear();
rootRule.patterns = flatten(root.patterns!, "Root");

// Output.

let g = `//! The grammar's rules as tables. Generated by \`oracle/haskell-grammar.ts\` from
//! the \`haskell\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pat\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::Haskell;
use super::scope::S;
use crate::scope::scopes;
use crate::rules;

pub(crate) type Act = rules::Act<Haskell>;
pub(crate) type PatData = rules::PatData<Haskell>;
pub(crate) type RuleData = rules::RuleData<Haskell>;
pub(crate) use crate::rules::Start;

/// The rule that tokenizes a capture group.
pub(crate) type Nested = Rule;

/// A scope of a rule or a capture group.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Sc {
    /// A fixed scope.
    S(S),
    /// A scope made of a prefix, the text of a capture group of the match and
    /// a suffix.
    Group(&'static str, u8, &'static str),
}

impl Sc {
    /// The code that keys a list of scopes made with scopes!.
    /// A fixed scope's list has one.
    /// A scope a capture group spells has none.
    /// Its lists are never cached.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless \`repr(u16)\` enum has no \`From\` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        match self {
            Sc::S(scope) => scope as u16,
            Sc::Group(..) => u16::MAX,
        }
    }

    /// The empty scope list.
    pub(crate) const NO_SCOPES: crate::scope::Scopes<Sc> = &crate::scope::Keyed::new([], []);
}

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
    `RuleData {\n                name: ${scText(r.name)},\n                content: ${scText(r.content)},\n                end: ${r.end ? `Some(Pat::${r.end})` : "None"},\n                backref: ${r.backref === null ? "None" : `Some(${r.backref})`},\n                patterns: &[${r.patterns.map((p) => `Pat::${p}`).join(", ")}],\n            }`,
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
  const word = p.word ? `, whose group ${p.word[0]} is \`${p.word[1]}\`` : "";
  g += `    /// The ${p.act.role} \`${docs(p.regex).replaceAll("`", "'")}\`${word}.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pat {\n${tooLong}    pub(crate) fn data(self) -> PatData {\n        match self {\n`;
g += arms(
  patList.map((p): [string, string] => {
    const act =
      p.act.role === "match"
        ? `Act::Match(${scText(p.act.name)})`
        : p.act.role === "begin"
          ? `Act::Begin(Rule::${p.act.rule})`
          : "Act::End";
    const caps = p.captures.map(([n, names]) => `(${n}, ${scText(names)})`).join(", ");
    const nested = p.nested.map(([n, rule]) => `(${n}, Rule::${rule})`).join(", ");
    return [p.id, `PatData {\n                act: ${act},\n                captures: &[${caps}],\n                nested: &[${nested}],\n            }`];
  }),
  "Pat",
);
g += `        }\n    }\n}\n`;

// The grammar's long word alternations, as lists the matchers try in order.
// No `\b` follows the extensions, so a word that starts a longer one, such as
// `Strict` before `StrictData`, still wins: the order matters.
/** The words of the alternation that `inner` captures from pattern `id`. */
function words(id: string, inner: RegExp): string[] {
  const regex = pats.get(id)!.regex;
  const found = inner.exec(regex)?.[1];
  if (found === undefined) throw new Error(`${id} changed shape`);
  return spell(found);
}
const bytes = (w: string) => `b"${w}"`;
const wordList = (name: string, doc: string, list: string[]) =>
  `\n/// ${doc}\npub(crate) const ${name}: &[&[u8]] = &[${list.map(bytes).join(", ")}];\n`;
g += wordList(
  "DEPRECATED_EXTENSIONS",
  "The language extensions of `PragmaAt0At0`, after an optional `No`, in the grammar's order.",
  words("PragmaAt0At0", /^\(\?:No\)\?\(\?:(.*)\)$/),
);
g += wordList(
  "EXTENSIONS",
  "The language extensions of `PragmaAt0At1`, after an optional `No`, in the grammar's order.",
  words("PragmaAt0At1", /^\(\(\?:No\)\?\(\?:(.*)\)\)$/),
);
g += wordList(
  "PRAGMAS",
  "The pragma names of `PragmaAt3`, uppercase, in the grammar's order.",
  words("PragmaAt3", /^\(\?i\)\\b\(\?<!'\)\((.*)\)\\b\(\?!'\)$/),
);
g += wordList(
  "ESCAPES",
  "The escape names after a backslash in `StringLiteralAt0`, in the grammar's order.",
  words("StringLiteralAt0", /^\\\\\(([A-Z0-9|]*)\|\["\\&'\\\\abfnrtv\]\)$/),
);

g += startTable(patList, "any");
g += ruleCodes("Haskell", "HASKELL");
g += groupsCheck("Haskell", patList.flatMap((p) => [...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]));

await writeRust(`${root_dir}src/haskell/grammar.rs`, g);

const s = scopeFile("haskell-grammar.ts", "HASKELL", scopes, "", false);
await writeRust(`${root_dir}src/haskell/scope.rs`, s);
console.log(`${ruleList.length} rules, ${patList.length} patterns, ${scopes.size} scopes`);
