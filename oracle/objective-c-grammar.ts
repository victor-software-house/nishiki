// Writes src/objective_c/scope.rs and src/objective_c/grammar.rs from the `objective-c` grammar that
// Shiki 4.6.0 bundles (microsoft/vscode `extensions/objective-c`, MIT, from jeff-hykin/better-objc-syntax).
// It writes the scope names.
// For every rule it writes the scopes the rule adds, how it ends and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// The matchers that decide whether a pattern matches are written by hand in src/objective_c.
//
// The grammar's `c_lang` entry carries a repository of its own.
// An include inside `c_lang` looks in that repository first, as vscode-textmate's does.
// `$base` and `$self` name the grammar's own root.
// Where Markdown embeds Objective-C, the engine resolves `$base` the same way.
//
// Run with `bun run oracle/objective-c-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import langs from "@shikijs/langs@4.6.0/objective-c";

import { expand as alternatives } from "./alternation.ts";
import { arms, groupsCheck, requireGroup, ruleCodes, scopeFile, startTable, tooLong, writeRust } from "./rule-grammar.ts";

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
  repository?: Record<string, Node>;
  include?: string;
  applyEndPatternLast?: number;
};

/** The repository an include resolves in, and the prefix of the ids of its own entries. */
type Scope = { repository: Record<string, Node>; own: Record<string, Node>; prefix: string };

const grammar = (langs as unknown as { scopeName: string; repository: Record<string, Node>; patterns: Node[] }[]).find(
  (lang) => lang.scopeName === "source.objc",
)!;
const top: Scope = { repository: grammar.repository, own: grammar.repository, prefix: "" };
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
  if (parts.length > 1 && parts[parts.length - 1] === "objc") parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_()$]/)) {
      const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
    }
  });
  return out;
}

const scopes = new Map<string, string>([["Source", "source.objc"]]);
function scope(name: string): string {
  let ident = scopeIdent(name);
  const known = scopes.get(ident);
  if (known !== undefined && known !== name) {
    // Names that differ only in punctuation get a numbered identifier.
    let n = 2;
    while (scopes.has(`${ident}${n}`) && scopes.get(`${ident}${n}`) !== name) n++;
    ident = `${ident}${n}`;
  }
  scopes.set(ident, name);
  return ident;
}

/** A scope of a rule or capture: a fixed one, or one built from capture text. */
type Sc = { fixed: string } | { prefix: string; group: number; suffix: string };

/** The scopes of `name`, which a match of `regex` fills in.
 * A `$N` whose group `regex` lacks fails the generator ([`requireGroup`]). */
function scopeList(name: string | undefined, regex: string): Sc[] {
  if (name === undefined) return [];
  return name
    .split(" ")
    .filter((part) => part !== "")
    .map((part): Sc => {
      const m = /^(.*?)\$(\d+)(.*)$/.exec(part);
      if (!m) return { fixed: scope(part) };
      const group = Number(m[2]);
      requireGroup(name, group, regex);
      return { prefix: m[1]!, group, suffix: m[3]! };
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

type Cap = { group: number; scopes: Sc[]; nested: string | null };
type Pat = {
  id: string;
  regex: string;
  act: { kind: "match"; name: Sc[] } | { kind: "begin"; rule: string } | { kind: "end" };
  captures: Cap[];
};
type Rule = {
  id: string;
  name: Sc[];
  content: Sc[];
  end: string;
  endLast: boolean;
  patterns: string[];
};

const pats = new Map<string, Pat>();
const rules = new Map<string, Rule>();
/** The captures that nest a pattern list, to expand their lists a second time. */
const captureSources: { cap: Cap; scope: Scope; patterns: Node[]; name: string }[] = [];
/** Nested contexts by the patterns they try, so equal ones are one rule. */
const contexts = new Map<string, string>();

function captureList(sc: Scope, captures: Captures | undefined, owner: string, regex: string): Cap[] {
  if (!captures) return [];
  const out: Cap[] = [];
  for (const [group, value] of Object.entries(captures)) {
    const names = scopeList(value.name, regex);
    let nested: string | null = null;
    if (value.patterns) {
      const list = flatten(sc, value.patterns, `${owner}Cap${group}`);
      const key = list.join(",");
      let id = contexts.get(key);
      if (id === undefined) {
        id = `${owner}Cap${group}`;
        contexts.set(key, id);
        rules.set(id, { id, name: [], content: [], end: "", endLast: false, patterns: list });
      }
      nested = id;
    }
    if (names.length || nested) {
      const cap: Cap = { group: Number(group), scopes: names, nested };
      if (value.patterns) captureSources.push({ cap, scope: sc, patterns: value.patterns, name: `${owner}Cap${group}` });
      out.push(cap);
    }
  }
  return out.sort((a, b) => a.group - b.group);
}

/** The shared lists being expanded, to stop a list that includes itself. */
const expanding = new Set<string>();

/** The begin/end rules registered, to expand their patterns a second time. */
const sources: { scope: Scope; node: Node; id: string }[] = [];

/** Registers a match or begin/end rule under `id`; returns the pattern id. */
function register(sc: Scope, node: Node, id: string): string {
  const clash = pats.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (clash) return id;
  if (node.match !== undefined) {
    pats.set(id, { id, regex: node.match, act: { kind: "match", name: scopeList(node.name, node.match) }, captures: [] });
    pats.get(id)!.captures = captureList(sc, node.captures, id, node.match);
    return id;
  }
  if (node.begin !== undefined && node.end !== undefined) {
    if (/\\[1-9]/.test(node.end)) throw new Error(`${id}: the end repeats a group of the begin`);
    pats.set(id, { id, regex: node.begin, act: { kind: "begin", rule: id }, captures: [] });
    pats.set(`${id}End`, { id: `${id}End`, regex: node.end, act: { kind: "end" }, captures: [] });
    const rule: Rule = {
      id,
      name: scopeList(node.name, node.begin),
      content: scopeList(node.contentName, node.begin),
      end: `${id}End`,
      endLast: node.applyEndPatternLast === 1,
      patterns: [],
    };
    rules.set(id, rule);
    pats.get(id)!.captures = captureList(sc, node.beginCaptures ?? node.captures, id, node.begin);
    pats.get(`${id}End`)!.captures = captureList(sc, node.endCaptures ?? node.captures, `${id}End`, node.end);
    sources.push({ scope: sc, node, id });
    rule.patterns = flatten(sc, node.patterns ?? [], id);
    return id;
  }
  throw new Error(`${id}: neither a match nor a begin/end rule`);
}

/** The scope a node's own repository opens inside `sc`. */
function inner(sc: Scope, node: Node, prefix: string): Scope {
  if (!node.repository) return sc;
  return { repository: { ...sc.repository, ...node.repository }, own: node.repository, prefix };
}

/** The pattern ids a pattern list expands to, `include`s flattened. */
function flatten(sc: Scope, list: Node[], context: string): string[] {
  const out: string[] = [];
  list.forEach((node, index) => {
    if (node.include === "$self" || node.include === "$base") {
      out.push(...expand(top, root, "Root", true));
    } else if (node.include !== undefined) {
      if (!node.include.startsWith("#")) throw new Error(`external include ${node.include}`);
      const key = node.include.slice(1);
      const target = sc.repository[key];
      if (!target) throw new Error(`unknown include ${key}`);
      const prefix = key in sc.own ? sc.prefix : "";
      const id = prefix + pascal(key);
      out.push(...expand(inner(sc, target, "C"), target, id, true));
    } else {
      out.push(...expand(inner(sc, node, "C"), node, `${context}At${index}`, false));
    }
  });
  return out;
}

function expand(sc: Scope, node: Node, id: string, shared: boolean): string[] {
  if (node.match !== undefined || node.begin !== undefined) return [register(sc, node, id)];
  if (node.patterns) {
    // An include of a bare list: guard against a list that includes itself.
    if (shared) {
      if (expanding.has(id)) return [];
      expanding.add(id);
    }
    const out = flatten(sc, node.patterns, id);
    if (shared) expanding.delete(id);
    return out;
  }
  return []; // `{}`, which tries nothing
}

// The root comes before the rules that nest captures, so that its number
// stays small.
const rootRule: Rule = { id: "Root", name: [], content: [], end: "", endLast: false, patterns: [] };
rules.set("Root", rootRule);
rootRule.patterns = flatten(top, root.patterns!, "Root");

// The first pass stopped a list at a bare list that was being expanded above
// it, which left a rule or a capture's context short of patterns it includes.
// A second pass gives each list in full without renaming a pattern, as the
// C++ generator does.
const claimed = new Map<string, string>();
const fullContexts = new Map<string, Rule>();
expanding.clear();
expanding.add("Root");
rootRule.patterns = flatten(top, root.patterns!, "Root");
for (let i = 0, j = 0; i < sources.length || j < captureSources.length; ) {
  if (i < sources.length) {
    const { scope: sc, node, id } = sources[i++]!;
    expanding.clear();
    rules.get(id)!.patterns = flatten(sc, node.patterns ?? [], id);
    continue;
  }
  const { cap, scope: sc, patterns, name } = captureSources[j++]!;
  expanding.clear();
  const list = flatten(sc, patterns, name);
  const key = list.join(",");
  let rule = fullContexts.get(key);
  if (rule === undefined) {
    let id = cap.nested!;
    if (claimed.has(id)) id = `${name}X${claimed.size}`;
    claimed.set(id, key);
    rule = rules.get(id) ?? { id, name: [], content: [], end: "", endLast: false, patterns: [] };
    rule.patterns = list;
    rules.set(id, rule);
    fullContexts.set(key, rule);
  }
  cap.nested = rule.id;
}

// Output.

let g = `//! The grammar's rules as tables. Generated by \`oracle/objective-c-grammar.ts\`
//! from the \`objective-c\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pat\`] is one regular expression of the grammar; its doc comment says
//! which. Each rule lists the patterns it tries, in the grammar's order with
//! its \`include\`s flattened, so the first that matches at the earliest
//! position wins as in a \`TextMate\` engine.

use super::engine::ObjectiveC;
use super::scope::S;
use crate::scope::scopes;
use crate::rules;

pub(crate) type Act = rules::Act<ObjectiveC>;
pub(crate) type PatData = rules::PatData<ObjectiveC>;
pub(crate) type RuleData = rules::RuleData<ObjectiveC>;
pub(crate) use crate::rules::Start;

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
    /// The code that keys a list of scopes made with scopes!: a fixed scope's, and
    /// none for a scope a capture group spells, whose lists are never cached.
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

/// A capture group whose text the patterns of a rule tokenize again, under
/// the match's stack and the group's own scopes.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Nested {
    /// The rule whose patterns tokenize the text, which has no end.
    pub(crate) rule: Rule,
    /// The scopes of the group.
    pub(crate) scopes: crate::scope::Scopes<Sc>,
}

`;

// Rules that a state can hold come first: their numbers are what a frame stores.
const ruleList = [...rules.values()].sort((a, b) => Number(b.end !== "") - Number(a.end !== ""));
const stateful = ruleList.filter((r) => r.end !== "").length;
const patList = [...pats.values()];
if (ruleList.length > 256) throw new Error(`${ruleList.length} rules do not fit one byte`);

g += `/// A rule of the grammar: one that can be open at the end of a line, or a
/// list of patterns that tokenizes a capture group.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub(crate) enum Rule {\n`;
for (const r of ruleList) g += `    ${r.id},\n`;
g += `}\n\nimpl Rule {\n    /// Every rule, in code order.\n    pub(crate) const ALL: [Rule; ${ruleList.length}] = [\n`;
for (const r of ruleList) g += `        Rule::${r.id},\n`;
g += `    ];\n\n${tooLong}    pub(crate) fn data(self) -> RuleData {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    `RuleData {\n                name: ${scText(r.name)},\n                content: ${scText(r.content)},\n                end: ${r.end && !r.endLast ? `Some(Pat::${r.end})` : "None"},\n                backref: None,\n                patterns: &[${[...r.patterns, ...(r.end && r.endLast ? [r.end] : [])].map((p) => `Pat::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pat {\n`;
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 ? `…` : one;
};
/** An expression as a doc comment shows it; the empty one has no code span. */
const shown = (s: string) => (s === "" ? "the empty expression" : `\`${docs(s).replaceAll("`", "'")}\``);
for (const p of patList) g += `    /// The ${p.act.kind}: ${shown(p.regex)}.\n    ${p.id},\n`;
g += `}\n\nimpl Pat {\n${tooLong}    pub(crate) fn data(self) -> PatData {\n        match self {\n`;
g += arms(
  patList.map((p): [string, string] => {
    const act =
      p.act.kind === "match"
        ? `Act::Match(${scText(p.act.name)})`
        : p.act.kind === "begin"
          ? `Act::Begin(Rule::${p.act.rule})`
          : "Act::End";
    const caps = p.captures
      .filter((c) => !c.nested)
      .map((c) => `(${c.group}, ${scText(c.scopes)})`)
      .join(", ");
    const nested = p.captures
      .filter((c) => c.nested)
      .map((c) => `(${c.group}, Nested { rule: Rule::${c.nested}, scopes: ${scText(c.scopes)} })`)
      .join(", ");
    return [p.id, `PatData {\n                act: ${act},\n                captures: &[${caps}],\n                nested: &[${nested}],\n            }`];
  }),
  "Pat",
);
g += `        }\n    }\n}\n`;

// Patterns with the same regular expression share one matcher.
const rxOf = new Map<string, string>();
for (const p of patList) if (!rxOf.has(p.regex)) rxOf.set(p.regex, p.id);
g += `\n/// A regular expression of the grammar, which several patterns can share.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Rx {\n`;
for (const [regex, id] of rxOf) g += `    /// ${shown(regex).replace(/^t/, "T")}.\n    ${id},\n`;
g += `}\n\nimpl Rx {\n    /// Every expression, in code order.\n    #[cfg(test)]\n    pub(crate) const ALL: [Rx; ${rxOf.size}] = [\n`;
for (const id of rxOf.values()) g += `        Rx::${id},\n`;
g += `    ];\n}\n\nimpl Pat {\n${tooLong}    pub(crate) fn rx(self) -> Rx {\n        match self {\n`;
g += arms(patList.map((p): [string, string] => [p.id, `Rx::${rxOf.get(p.regex)}`]), "Pat");
g += `        }\n    }\n}\n`;

g += startTable(patList);
g += ruleCodes("ObjectiveC", "OBJECTIVE_C");
g += groupsCheck(
  "ObjectiveC",
  patList.flatMap((p) => p.captures.map((c) => c.group)),
);

await writeRust(`${root_dir}src/objective_c/grammar.rs`, g);

const s = scopeFile("objective-c-grammar.ts", "OBJECTIVE_C", scopes, "", false);
await writeRust(`${root_dir}src/objective_c/scope.rs`, s);

// Word lists.
//
// Many expressions test one word against a long alternation, such as
// `\bNS(Rect|Range|…)\b`.
// Every alternative is made of word characters, so the match is the whole
// word or nothing.
// The lists go into one sorted table of words, each with a bit per list it
// belongs to, which a matcher searches once.

/** The text between `open` and the first `close` after it in the expression of `id`. */
function between(id: string, open: string, close: string): string {
  const regex = pats.get(id)?.regex;
  if (regex === undefined) throw new Error(`no expression ${id}`);
  const from = regex.indexOf(open);
  const to = regex.indexOf(close, from + open.length);
  if (from < 0 || to < 0) throw new Error(`${id}: no ${open}…${close}`);
  return regex.slice(from + open.length, to);
}

/** The whole expression of `id` between `head` and `tail`, which it must start and end with. */
function inside(id: string, head: string, tail: string): string {
  const regex = pats.get(id)!.regex;
  if (!regex.startsWith(head) || !regex.endsWith(tail)) throw new Error(`${id}: not ${head}…${tail}`);
  return regex.slice(head.length, regex.length - tail.length);
}

const wordLists: { name: string; doc: string; body: string }[] = [
  { name: "LEOPARD_FUNCTION", doc: "The Cocoa functions Leopard added, of `AnonymousPattern18`.", body: inside("AnonymousPattern18", "(\\s*)\\b(", ")\\b") },
  { name: "FUNCTION", doc: "The Cocoa functions of `AnonymousPattern19`.", body: inside("AnonymousPattern19", "(\\s*)\\b(", ")\\b") },
  { name: "LEOPARD_CLASS", doc: "The Cocoa classes Leopard added, of `AnonymousPattern20`.", body: inside("AnonymousPattern20", "\\b", "\\b") },
  { name: "CLASS", doc: "The Cocoa classes of `AnonymousPattern21`.", body: inside("AnonymousPattern21", "\\b", "\\b") },
  { name: "LEOPARD_TYPE", doc: "The Cocoa types Leopard added, of `AnonymousPattern22`.", body: inside("AnonymousPattern22", "\\b", "\\b") },
  { name: "QUARTZ_CLASS", doc: "The Quartz classes of `AnonymousPattern23`.", body: inside("AnonymousPattern23", "\\b", "\\b") },
  { name: "QUARTZ_TYPE", doc: "The Quartz types of `AnonymousPattern24`.", body: inside("AnonymousPattern24", "\\b", "\\b") },
  { name: "TYPE", doc: "The Cocoa types of `AnonymousPattern25`.", body: inside("AnonymousPattern25", "\\b", "\\b") },
  { name: "CONSTANT", doc: "The Cocoa constants of `AnonymousPattern26`.", body: inside("AnonymousPattern26", "\\b", "\\b") },
  { name: "LEOPARD_NOTIFICATION", doc: "The notifications Leopard added, of `AnonymousPattern27`.", body: inside("AnonymousPattern27", "\\b", "\\b") },
  { name: "NOTIFICATION", doc: "The notifications of `AnonymousPattern28`.", body: inside("AnonymousPattern28", "\\b", "\\b") },
  { name: "LEOPARD_CONSTANT", doc: "The Cocoa constants Leopard added, of `AnonymousPattern29`.", body: inside("AnonymousPattern29", "\\b", "\\b") },
  { name: "COCOA_CONSTANT", doc: "The Cocoa constants of `AnonymousPattern30`.", body: inside("AnonymousPattern30", "\\b", "\\b") },
  { name: "PROTOCOL", doc: "The Cocoa protocols of `ProtocolListAt0`.", body: inside("ProtocolListAt0", "\\b", "\\b") },
  { name: "C99_FUNCTION", doc: "The C library functions of `CFunctionsAt0`.", body: inside("CFunctionsAt0", "(\\s*)\\b(", ")\\b") },
  { name: "MAC_TYPE", doc: "The Mac Classic types of `CLangAt29`.", body: inside("CLangAt29", "\\b(", ")\\b") },
  { name: "STORAGE", doc: "The storage types of `CStorageTypesAt1`.", body: inside("CStorageTypesAt1", "(?-im:(?<!\\w)(?:", ")(?!\\w))") },
  { name: "MEMBER_EXCLUDED", doc: "The words a member name of `CMemberAccess` may not start with.", body: between("CMemberAccess", "\\b((?!", ")[A-Z_a-z]") },
  { name: "CALL_EXCLUDED", doc: "The words before `(` that are no function call to `CLangAt33`.", body: between("CLangAt33", "(?!\\s*(?:", ")\\s*\\()") },
  { name: "DEFINE_EXCLUDED", doc: "The words before `(` that are no call to `CPreprocessorRuleDefineLineContentsAt4`.", body: between("CPreprocessorRuleDefineLineContentsAt4", "(?!(?:", ")\\s*\\()") },
  { name: "FUNCTION_EXCLUDED", doc: "The words before `(` that are no call to `CCFunctionCall`, `CFunctionCallInnardsAt5` and `CBlockInnardsAt6`.", body: between("CCFunctionCall", "(?!(?:", ")\\s*\\()") },
];
for (const id of ["CFunctionCallInnardsAt5"]) {
  if (between(id, "(?!(?:", ")\\s*\\()") !== wordLists.at(-1)!.body) throw new Error(`${id}: another excluded list`);
}
if (wordLists.length > 32) throw new Error("the lists do not fit a u32");
const masks = new Map<string, number>();
wordLists.forEach((list, index) => {
  for (const word of alternatives(list.body)) {
    if (!/^\w+$/.test(word)) throw new Error(`${list.name}: ${word} is not one word`);
    masks.set(word, (masks.get(word) ?? 0) | (1 << index));
  }
});
let w = `//! The words of the grammar's long alternations, sorted, with the lists
//! each belongs to. Generated by \`oracle/objective-c-grammar.ts\`; do not edit.

`;
wordLists.forEach((list, index) => {
  w += `/// ${list.doc}\npub(super) const ${list.name}: u32 = 1 << ${index};\n`;
});
const sorted = [...masks].sort((a, b) => Buffer.compare(Buffer.from(a[0]), Buffer.from(b[0])));
/** A mask as clippy reads it: eight hex digits in two groups of four. */
const hex = (mask: number) => {
  const digits = mask.toString(16).padStart(8, "0");
  return `0x${digits.slice(0, 4)}_${digits.slice(4)}`;
};
// The words go end to end in one byte string, with where each ends, so the
// table holds no pointer per word.
const blob = sorted.map(([word]) => word).join("");
if (blob.length > 0xffff) throw new Error(`${blob.length} bytes of words do not fit u16 ends`);
const ends: number[] = [];
let at = 0;
for (const [word] of sorted) ends.push((at += word.length));
const lines: string[] = [""];
for (const [word] of sorted) {
  if (lines.at(-1)!.length + word.length > 90) lines.push("");
  lines[lines.length - 1] += word;
}
w += `\n/// Every word of the lists, sorted by its bytes, end to end.\npub(super) static WORD_BYTES: &[u8] = b"\\\n${lines.join("\\\n")}";\n`;
w += `\n/// Where each word of [\`WORD_BYTES\`] ends: word \`k\` runs from the end of word \`k - 1\` to \`WORD_ENDS[k]\`.\npub(super) static WORD_ENDS: [u16; ${ends.length}] = [${ends.join(", ")}];\n`;
w += `\n/// The lists each word of [\`WORD_BYTES\`] belongs to, in the same order.\npub(super) static WORD_LISTS: [u32; ${ends.length}] = [${sorted.map(([, mask]) => hex(mask)).join(", ")}];\n`;
await writeRust(`${root_dir}src/objective_c/words.rs`, w);

// With RX_DUMP set to a file, writes each expression for oracle/onig.ts.
if (Bun.env["RX_DUMP"]) {
  await Bun.write(
    Bun.env["RX_DUMP"],
    [...rxOf]
      .map(([regex, id]) => {
        // The groups the captures name, and the groups a `$N` in a scope name reads.
        const dollar = (list: Sc[]) => list.flatMap((s) => ("group" in s ? [s.group] : []));
        const groups = patList
          .filter((p) => p.regex === regex)
          .flatMap((p) => [
            ...p.captures.flatMap((c) => [c.group, ...dollar(c.scopes)]),
            ...(p.act.kind === "match" ? dollar(p.act.name) : []),
            ...(p.act.kind === "begin" ? [...dollar(rules.get(p.act.rule)!.name), ...dollar(rules.get(p.act.rule)!.content)] : []),
          ]);
        return JSON.stringify({ id, regex, groups: [...new Set(groups)].sort((a, b) => a - b) });
      })
      .join("\n") + "\n",
  );
}
console.log(`${ruleList.length} rules (${stateful} open across lines), ${patList.length} patterns, ${rxOf.size} expressions, ${scopes.size} scopes`);
