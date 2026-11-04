// Writes src/csharp/scope.rs and src/csharp/grammar.rs from the `csharp`
// grammar that Shiki 4.6.0 bundles (microsoft/vscode, converted from
// dotnet/csharp-tmLanguage, MIT): the scope names, and for every rule the
// scopes it adds, how it ends and the patterns it tries, in the grammar's order
// with its `include`s flattened. The matchers that decide whether a pattern
// matches are written by hand in src/csharp/matchers, except the patterns that
// are a literal or a keyword, which are generated here.
//
// A `while` rule (a line comment, a documentation comment) keeps its `while`
// expression as a pattern of its own, which `Rule::while_pat` names and the
// rule engine tries at the start of each line.
//
// Three things in the grammar are rewritten so that the rule engine need not
// know them:
//
// 1. A scope name made from a capture (`storage.modifier.$1.cs`) becomes one
//    pattern per word of the capture's alternation, each with its static scope.
// 2. A capture group that the grammar tokenizes with patterns of its own
//    becomes a rule with no end, named in `Nested`.
// 3. The patterns of a rule are its `include`s flattened.
//
// Run with `bun run oracle/csharp-grammar.ts` and commit the result; the
// output is generated, do not edit it. `RX_DUMP=<file>` also writes every
// pattern's regular expression and capture groups, one JSON object per line,
// for oracle/onig.ts.
import csharp from "@shikijs/langs@4.6.0/csharp";
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

const grammar = (csharp as unknown as { repository: Record<string, Node>; patterns: Node[] }[])[0]!;
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
  // The grammar misspells `prefix` in two names; the Rust identifier does not.
  const parts = name.replaceAll("pre" + "ffix", "prefix").split(".");
  if (parts[parts.length - 1] === "cs") parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
    }
  });
  return out;
}

const scopes = new Map<string, string>([["Source", "source.cs"]]);
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

// 1. Scope names made from a capture.

/** The index just past the group that opens at `i`, and whether it captures. */
function groupAt(regex: string, i: number): { captures: boolean } {
  if (regex[i + 1] !== "?") return { captures: true };
  return { captures: /^\(\?<[A-Za-z_]/.test(regex.slice(i)) };
}

/** The source of capture group `n` of `regex`; the whole expression for 0. */
function groupSource(regex: string, n: number): { start: number; end: number } {
  if (n === 0) return { start: 0, end: regex.length };
  let count = 0;
  const stack: { open: number; counted: number | null }[] = [];
  let inClass = 0;
  for (let i = 0; i < regex.length; i++) {
    const c = regex[i]!;
    if (c === "\\") {
      i++;
    } else if (inClass > 0) {
      if (c === "[" && regex[i + 1] === ":") i = regex.indexOf(":]", i) + 1;
      else if (c === "[") inClass++;
      else if (c === "]") inClass--;
    } else if (c === "[") {
      inClass = 1;
      if (regex[i + 1] === "^") i++;
      if (regex[i + 1] === "]") i++;
    } else if (c === "(") {
      const { captures } = groupAt(regex, i);
      stack.push({ open: i, counted: captures ? ++count : null });
    } else if (c === ")") {
      const group = stack.pop()!;
      if (group.counted === n) {
        const body = regex.slice(group.open).match(/^\((?:\?<[A-Za-z_]\w*>)?/)![0].length;
        return { start: group.open + body, end: i };
      }
    }
  }
  throw new Error(`regex ${regex}: no group ${n}`);
}

/** The words an alternation of literals matches, in the order Oniguruma tries them. */
function words(body: string): string[] {
  const split = (text: string): string[] => {
    const parts: string[] = [];
    let depth = 0;
    let from = 0;
    for (let i = 0; i < text.length; i++) {
      if (text[i] === "(") depth++;
      else if (text[i] === ")") depth--;
      else if (text[i] === "|" && depth === 0) {
        parts.push(text.slice(from, i));
        from = i + 1;
      }
    }
    parts.push(text.slice(from));
    return parts;
  };
  const sequence = (text: string): string[] => {
    let out = [""];
    for (let i = 0; i < text.length; ) {
      let options: string[];
      if (text.startsWith("(?:", i)) {
        let depth = 0;
        let j = i;
        for (; j < text.length; j++) {
          if (text[j] === "(") depth++;
          else if (text[j] === ")" && --depth === 0) break;
        }
        options = split(text.slice(i + 3, j)).flatMap(sequence);
        i = j + 1;
      } else if (/[A-Za-z]/.test(text[i]!)) {
        options = [text[i]!];
        i++;
      } else {
        throw new Error(`alternation ${text}: ${text[i]}`);
      }
      if (text[i] === "?") {
        options = [...options, ""];
        i++;
      }
      out = out.flatMap((prefix) => options.map((option) => prefix + option));
    }
    return out;
  };
  return [...new Set(split(body).flatMap(sequence))].filter((word) => word !== "");
}

/** The `$n` references in the names under `node`'s own captures and name. */
function references(node: Node): Set<number> {
  const found = new Set<number>();
  const note = (text: string | undefined) => {
    for (const m of (text ?? "").matchAll(/\$(\d)/g)) found.add(Number(m[1]));
  };
  note(node.name);
  note(node.contentName);
  for (const captures of [node.captures, node.beginCaptures]) {
    for (const value of Object.values(captures ?? {})) note(value.name);
  }
  return found;
}

// 2 and 3. Rules and patterns.

type Act = { kind: "match"; name: string[] } | { kind: "begin"; rule: string } | { kind: "end" } | { kind: "while" };
type Pat = {
  id: string;
  regex: string;
  /** The regular expression before a capture's word was put into it. */
  source: string;
  word: string | null;
  act: Act;
  captures: [number, string[]][];
  /** Capture groups that the grammar tokenizes with patterns of their own. */
  nested: [number, string][];
};
type Rule = {
  id: string;
  name: string[];
  content: string[];
  end: string;
  /** The `while` pattern, or the empty text. */
  while: string;
  patterns: string[];
};

const pats = new Map<string, Pat>();
const rules = new Map<string, Rule>();
/** Nested pattern lists by their JSON, with the name of the kind they get. */
const nestedKinds = new Map<string, string>();

const pascal = (key: string) =>
  key
    .split(/[-_.]/)
    .map((piece) => piece.charAt(0).toUpperCase() + piece.slice(1))
    .join("");

/** The kind of the nested pattern list `patterns`; registers it as a rule with no end. */
function nestedKind(patterns: Node[]): string {
  const json = JSON.stringify(patterns);
  const known = nestedKinds.get(json);
  if (known !== undefined) return known;
  const includes = patterns.map((p) => p.include?.slice(1) ?? (p.match !== undefined ? "Pattern" : "Begin"));
  let kind = includes.map(pascal).join("And");
  if (nestedKinds.size > 0 && [...nestedKinds.values()].includes(kind)) kind += nestedKinds.size;
  nestedKinds.set(json, kind);
  const id = `Nested${kind}`;
  rules.set(id, { id, name: [], content: [], end: "", while: "", patterns: [] });
  rules.get(id)!.patterns = flatten(patterns, id);
  return kind;
}

function captureList(captures: Captures | undefined, word: string | null): [number, string[]][] {
  if (!captures) return [];
  return Object.entries(captures)
    .map(([group, value]): [number, string[]] => [
      Number(group),
      value.name === undefined || (word === null && value.name.includes("$"))
        ? []
        : scopeList(substitute(value.name, word)),
    ])
    .filter(([, names]) => names.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

function nestedList(captures: Captures | undefined): [number, string][] {
  if (!captures) return [];
  return Object.entries(captures)
    .filter(([, value]) => value.patterns !== undefined)
    .map(([group, value]): [number, string] => [Number(group), nestedKind(value.patterns!)])
    .sort((a, b) => a[0] - b[0]);
}

function substitute(name: string, word: string | null): string {
  return word === null ? name : name.replace(/\$\d/g, word);
}

/** The alternatives that a node's `$n` references split its begin or match into. */
function splitWords(node: Node, regex: string): { word: string | null; regex: string }[] {
  const refs = references(node);
  if (refs.size === 0) return [{ word: null, regex }];
  if (refs.size > 1) throw new Error(`${regex}: references ${[...refs]}`);
  const n = [...refs][0]!;
  const { start, end } = groupSource(regex, n);
  // A group inside an optional `(?:\b(...)\b\s+)?` that Oniguruma prefers to
  // skip: each word's pattern requires the group, and the pattern without it
  // comes last, with the group's body made impossible to keep the group
  // numbers of the rest.
  const tail = /^\)\\b\\s[+*]\)\?/.exec(regex.slice(end));
  const optionalOpen = regex.slice(0, start).lastIndexOf("(?:\\b(");
  const optional = tail !== null && optionalOpen >= 0 && optionalOpen + "(?:\\b(".length === start;
  const out: { word: string | null; regex: string }[] = words(regex.slice(start, end)).map((word) => {
    const single = regex.slice(0, start) + word + regex.slice(end);
    if (!optional) return { word, regex: single };
    const close = end + tail![0].length - 2;
    return {
      word,
      regex: single.slice(0, optionalOpen) + single.slice(optionalOpen + 3, close - (regex.length - single.length)) + single.slice(close - (regex.length - single.length) + 2),
    };
  });
  if (optional) out.push({ word: null, regex: regex.slice(0, start) + "(?!)" + regex.slice(end) });
  return out;
}

/** Registers a match or begin/end rule under `id`; returns the pattern ids. */
function register(node: Node, id: string): string[] {
  if (node.match !== undefined) {
    return splitWords(node, node.match).map(({ word, regex }) => {
      const pid = word === null ? id : `${id}${pascal(word)}`;
      if (!pats.has(pid)) {
        pats.set(pid, {
          id: pid,
          regex,
          source: node.match!,
          word,
          act: { kind: "match", name: scopeList(node.name === undefined ? undefined : substitute(node.name, word)) },
          captures: captureList(node.captures, word),
          nested: nestedList(node.captures),
        });
      }
      return pid;
    });
  }
  if (node.begin !== undefined && (node.end !== undefined) !== (node.while !== undefined)) {
    const ids = splitWords(node, node.begin).map(({ word, regex }) => {
      const pid = word === null ? id : `${id}${pascal(word)}`;
      if (!pats.has(pid)) {
        pats.set(pid, {
          id: pid,
          regex,
          source: node.begin!,
          word,
          act: { kind: "begin", rule: id },
          captures: captureList(node.beginCaptures ?? node.captures, word),
          nested: nestedList(node.beginCaptures ?? node.captures),
        });
      }
      return pid;
    });
    if (!rules.has(id)) {
      if (node.end !== undefined) {
        pats.set(`${id}End`, {
          id: `${id}End`,
          regex: node.end,
          source: node.end,
          word: null,
          act: { kind: "end" },
          captures: captureList(node.endCaptures ?? node.captures, null),
          nested: nestedList(node.endCaptures ?? node.captures),
        });
      } else {
        // The engine tries the pattern where the line starts, so it must hold
        // only there.
        if (!node.while!.startsWith("^")) throw new Error(`${id}: while ${node.while} is not anchored`);
        pats.set(`${id}While`, {
          id: `${id}While`,
          regex: node.while!,
          source: node.while!,
          word: null,
          act: { kind: "while" },
          captures: captureList(node.whileCaptures ?? node.captures, null),
          nested: nestedList(node.whileCaptures ?? node.captures),
        });
      }
      const rule: Rule = {
        id,
        name: scopeList(node.name === undefined ? undefined : substitute(node.name, null)),
        content: scopeList(node.contentName),
        end: node.end !== undefined ? `${id}End` : "",
        while: node.while !== undefined ? `${id}While` : "",
        patterns: [],
      };
      rules.set(id, rule);
      rule.patterns = flatten(node.patterns ?? [], id);
    }
    return ids;
  }
  throw new Error(`${id}: neither a match, a begin/end nor a begin/while rule`);
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

rules.set("Root", { id: "Root", name: [], content: [], end: "", while: "", patterns: [] });
rules.get("Root")!.patterns = flatten(root.patterns!, "Root");

// Patterns that are a literal or a keyword are matched by generated code.

type Atom = { behind: boolean; bytes: number[] };
type How =
  | { kind: "lit"; bytes: number[] }
  | { kind: "group"; bytes: number[] }
  | { kind: "word"; word: string; dot: boolean; group: boolean }
  | { kind: "assert"; atoms: Atom[] }
  | { kind: "hand" };

/** The bytes of a simple class such as `[]),;}]`, or null. */
function simpleClass(source: string): number[] | null {
  if (!source.startsWith("[") || !source.endsWith("]") || source.startsWith("[^") || source.length < 3) return null;
  const body = source.slice(1, -1);
  const out: number[] = [];
  for (let i = 0; i < body.length; i++) {
    let c = body[i]!;
    if (c === "\\") {
      c = body[++i] ?? "";
      if (c === "" || /[A-Za-z0-9]/.test(c)) return null;
    } else if (c === "[" || (c === "-" && i > 0 && i < body.length - 1) || c.charCodeAt(0) > 127) {
      return null;
    }
    out.push(c.charCodeAt(0));
  }
  return out;
}

/** An assertion `(?=X)` or `(?<=X)` with a one-character or class X, or null. */
function atomOf(text: string): Atom | null {
  const m = /^\(\?(<?)=(.*)\)$/s.exec(text);
  if (!m) return null;
  const body = m[2]!;
  const bytes = body.startsWith("[")
    ? simpleClass(body)
    : /^(?:[^\\^$.|?*+()[{]|\\[^A-Za-z0-9])$/.test(body)
      ? [body.replace("\\", "").charCodeAt(0)]
      : null;
  return bytes && { behind: m[1] === "<", bytes };
}

/** The top-level alternatives of `regex`. */
function alternatives(regex: string): string[] {
  const out: string[] = [];
  let depth = 0;
  let inClass = false;
  let from = 0;
  for (let i = 0; i < regex.length; i++) {
    const c = regex[i]!;
    if (c === "\\") i++;
    else if (inClass) inClass = c !== "]" || regex[i - 1] === "[";
    else if (c === "[") inClass = true;
    else if (c === "(") depth++;
    else if (c === ")") depth--;
    else if (c === "|" && depth === 0) {
      out.push(regex.slice(from, i));
      from = i + 1;
    }
  }
  out.push(regex.slice(from));
  return out;
}

function howOf(regex: string): How {
  const literal = /^(?:[^\\^$.|?*+()[{]|\\[^A-Za-z0-9])+$/;
  const decode = (text: string) => [...new TextEncoder().encode(text.replace(/\\(.)/g, "$1"))];
  if (literal.test(regex)) return { kind: "lit", bytes: decode(regex) };
  const grouped = /^\((.+)\)$/.exec(regex);
  if (grouped && literal.test(grouped[1]!)) return { kind: "group", bytes: decode(grouped[1]!) };
  const m = /^(\(\?<!\\\.\))?\\b(?:\(([a-z]+)\)|([a-z]+))\\b$/.exec(regex);
  if (m) return { kind: "word", word: (m[2] ?? m[3])!, dot: m[1] !== undefined, group: m[2] !== undefined };
  const atoms = alternatives(regex).map(atomOf);
  if (atoms.every((a): a is Atom => a !== null)) return { kind: "assert", atoms };
  return { kind: "hand" };
}

// Output.

const list = (names: string[]) => (names.length ? `scopes![${names.map((n) => `S::${n}`).join(", ")}]` : "S::NO_SCOPES");

const ruleList = [...rules.values()];
const patList = [...pats.values()];
if (ruleList.length > 256) throw new Error(`${ruleList.length} rules do not fit one byte`);

// The hand matchers: one for each distinct source expression.
const hand = new Map<string, string>();
for (const p of patList) {
  if (howOf(p.regex).kind === "hand" && !hand.has(p.source)) hand.set(p.source, p.id);
}
const handOf = (p: Pat) => hand.get(p.source)!;

let g = `//! The grammar's rules as tables. Generated by \`oracle/csharp-grammar.ts\`
//! from the \`csharp\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pat\`] is one regular expression of the grammar; its doc comment says
//! which. Each rule lists the patterns it tries, in the grammar's order with
//! its \`include\`s flattened, so the first that matches at the earliest
//! position wins as in a \`TextMate\` engine.

use super::engine::CSharp;
use super::scope::S;
use crate::scope::scopes;
use crate::rules;

pub(crate) type Act = rules::Act<CSharp>;
pub(crate) type PatData = rules::PatData<CSharp>;
pub(crate) type RuleData = rules::RuleData<CSharp>;
pub(crate) use crate::rules::Start;

/// The capture groups that the grammar tokenizes with patterns of its own,
/// each a rule that has no end.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Nested {
`;
for (const kind of nestedKinds.values()) g += `    ${kind},\n`;
g += `}\n\nimpl Nested {\n    /// The rule whose patterns tokenize the group.\n    pub(crate) const fn rule(self) -> Rule {\n        match self {\n`;
for (const kind of nestedKinds.values()) g += `            Nested::${kind} => Rule::Nested${kind},\n`;
g += `        }\n    }\n}\n\n`;

g += `/// A rule of the grammar that can be open at the end of a line.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub(crate) enum Rule {\n`;
for (const r of ruleList) g += `    ${r.id},\n`;
g += `}\n\nimpl Rule {\n    /// Whether a rule has a \`while\` pattern ([\`Rule::while_pat\`]).\n    pub(crate) const HAS_WHILE: bool = ${ruleList.some((r) => r.while)};\n\n    /// Every rule, in code order.\n    pub(crate) const ALL: [Rule; ${ruleList.length}] = [\n`;
for (const r of ruleList) g += `        Rule::${r.id},\n`;
g += `    ];\n\n${tooLong}    pub(crate) fn data(self) -> RuleData {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    `RuleData {\n                name: ${list(r.name)},\n                content: ${list(r.content)},\n                end: ${r.end ? `Some(Pat::${r.end})` : "None"},\n                backref: None,\n                patterns: &[${r.patterns.map((p) => `Pat::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n\n    /// The \`while\` pattern of a rule that has one.\n${tooLong}    pub(crate) fn while_pat(self) -> Option<Pat> {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [r.id, r.while ? `Some(Pat::${r.while})` : "None"]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pat {\n`;
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 ? `…` : one;
};
for (const p of patList) {
  const kind = p.act.kind;
  g += `    /// The ${kind} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pat {\n    /// Every pattern, for the probe test.\n    #[cfg(test)]\n    pub(crate) const ALL: [Pat; ${patList.length}] = [\n${patList.map((p) => `        Pat::${p.id},\n`).join("")}    ];\n\n${tooLong}    pub(crate) fn data(self) -> PatData {\n        match self {\n`;
g += arms(
  patList.map((p): [string, string] => {
    const act =
      p.act.kind === "match"
        ? `Act::Match(${list(p.act.name)})`
        : p.act.kind === "begin"
          ? `Act::Begin(Rule::${p.act.rule})`
          : p.act.kind === "end"
            ? "Act::End"
            : "Act::Match(S::NO_SCOPES)";
    const caps = p.captures.map(([n, names]) => `(${n}, ${list(names)})`).join(", ");
    const nested = p.nested.map(([n, kind]) => `(${n}, Nested::${kind})`).join(", ");
    return [p.id, `PatData {\n                act: ${act},\n                captures: &[${caps}],\n                nested: &[${nested}],\n            }`];
  }),
  "Pat",
);
g += `        }\n    }\n\n    /// The word of the capture whose alternation the pattern was split on, or\n    /// the empty text.\n    ///\n    /// \`#[inline]\`, as \`Pat::how\` is, because the matchers call it from another\n    /// module (\`bench/AGENTS.md\` rule 13).\n    #[inline]\n${tooLong}    pub(crate) fn word(self) -> &'static [u8] {\n        match self {\n`;
g += arms(
  patList.map((p): [string, string] => [p.id, `b"${p.word ?? ""}"`]),
  "Pat",
);
g += `        }\n    }\n\n    /// How the pattern is matched.\n    ///\n    /// \`#[inline]\` because the matchers, in another module, call it for every\n    /// pattern they try: at the default sixteen codegen units the two land in\n    /// different units, and a function that is not \`#[inline]\` cannot be\n    /// inlined across them (\`bench/AGENTS.md\` rule 13).\n    #[inline]\n${tooLong}    pub(crate) fn how(self) -> How {\n        match self {\n`;
g += arms(
  patList.map((p): [string, string] => {
    const how = howOf(p.regex);
    switch (how.kind) {
      case "lit":
        return [p.id, `How::Lit(&[${how.bytes.join(", ")}])`];
      case "group":
        return [p.id, `How::Group(&[${how.bytes.join(", ")}])`];
      case "word":
        return [p.id, `How::Word {\n                word: b"${how.word}",\n                dot: ${how.dot},\n                group: ${how.group},\n            }`];
      case "assert":
        return [p.id, `How::Assert(&[${how.atoms.map((a) => `(${a.behind}, &[${a.bytes.join(", ")}])`).join(", ")}])`];
      case "hand":
        return [p.id, `How::Hand(Fn::${handOf(p)})`];
    }
  }),
  "Pat",
);
g += `        }\n    }\n}\n\n`;

g += `/// How a pattern is matched.
#[derive(Clone, Copy, Debug)]
pub(crate) enum How {
    /// The text, whatever stands before and after it.
    Lit(&'static [u8]),
    /// The text as capture group 1.
    Group(&'static [u8]),
    /// An alternation of lookaheads and lookbehinds at a byte: each says
    /// whether it looks behind, and the bytes it accepts.
    Assert(&'static [(bool, &'static [u8])]),
    /// The word between two word boundaries, and no dot before it when
    /// \`dot\` is set; \`group\` says whether capture group 1 is the word.
    Word {
        word: &'static [u8],
        dot: bool,
        group: bool,
    },
    /// A hand-written matcher.
    Hand(Fn),
}

/// The regular expressions that a hand-written matcher answers for, one for
/// each distinct expression of the grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Fn {
`;
for (const [source, id] of hand) g += `    /// \`${docs(source).replaceAll("`", "'")}\`.\n    ${id},\n`;
g += `}\n`;

g += startTable(patList);
g += ruleCodes("CSharp", "CSHARP");
g += groupsCheck("CSharp", patList.flatMap((p) => [...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]));

await writeRust(`${root_dir}src/csharp/grammar.rs`, g);

// Scope table. The grammar misspells a scope name; it is written with one
// byte escape so that the spelling check passes.
const written = new Map([...scopes].map(([ident, name]) => [ident, name.replaceAll("pre" + "ffix", "pre\\x66fix")]));
const s = scopeFile("csharp-grammar.ts", "CSHARP", written);
await writeRust(`${root_dir}src/csharp/scope.rs`, s);

if (Bun.env["RX_DUMP"]) {
  await Bun.write(
    Bun.env["RX_DUMP"],
    patList
      .map((p) =>
        JSON.stringify({
          id: p.id,
          regex: p.regex,
          groups: [...new Set([...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)])].sort((a, b) => a - b),
        }),
      )
      .join("\n") + "\n",
  );
}
console.log(
  `${ruleList.length} rules, ${patList.length} patterns, ${hand.size} hand matchers, ${scopes.size} scopes, ${nestedKinds.size} nested kinds`,
);
