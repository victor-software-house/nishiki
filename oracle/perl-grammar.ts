// Writes src/perl/scope.rs, src/perl/grammar.rs and src/perl/words.rs from
// the `perl` grammar that Shiki 4.6.0 bundles (textmate/perl.tmbundle
// through microsoft/vscode, MIT).
//
// grammar.rs holds the scopes each rule and pattern adds, how each rule ends
// and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// A pattern or rule that repeats another, with the same scopes, is the same
// `Pattern` or `Rule`, so it has one matcher.
//
// The grammar's long word list, the built-in functions and handles, is
// words.rs: its words written out and sorted, so one lookup answers it.
//
// A heredoc's end repeats two groups of its begin: the `~` of `<<~` and the
// label.
// Each heredoc pattern becomes two, one with the `~` and one without, so the
// end repeats the label alone.
// A heredoc tagged HTML, XML, CSS, JAVASCRIPT or SQL holds a rule that opens
// at each line's start, ends at its line feed and embeds that language
// (`Payload::Embed`).
// A heredoc tagged POSTSCRIPT includes a grammar that Shiki lacks, so its
// lines keep the heredoc's scopes.
//
// The matchers that decide whether a pattern matches are written by hand in
// src/perl/matchers.
//
// Run with `bun run oracle/perl-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import perl from "@shikijs/langs@4.6.0/perl";
import { arms, groupsCheck, ruleCodes, scopeFile, startOf, startTable, tooLong, writeRust } from "./rule-grammar.ts";

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

const grammar = (perl as unknown as { name: string; scopeName: string; repository: Record<string, Node>; patterns: Node[] }[]).find(
  (lang) => lang.name === "perl",
)!;
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
  // A name without the language's suffix, such as `string`, ends in `Bare`.
  const suffix = parts[parts.length - 1] === "perl" && parts.length > 1 ? "" : "Bare";
  if (!suffix) parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
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

// Word lists.

/** Expands a regex made of literals, groups, alternations, `?`, `??` and
 * simple classes into the words it matches, or `null` where it holds anything
 * else. Order is dropped: between `\b` and `\b` a lazy `?` matches the same
 * words. */
function expandWords(src: string): string[] | null {
  let i = 0;
  const alt = (): string[] | null => {
    const out: string[] = [];
    const first = seq();
    if (!first) return null;
    out.push(...first);
    while (src[i] === "|") {
      i++;
      const next = seq();
      if (!next) return null;
      out.push(...next);
    }
    return out;
  };
  const seq = (): string[] | null => {
    let acc = [""];
    while (i < src.length && src[i] !== "|" && src[i] !== ")") {
      let atom: string[] | null;
      if (src[i] === "(") {
        i++;
        if (src.startsWith("?:", i)) i += 2;
        atom = alt();
        if (src[i] !== ")") return null;
        i++;
      } else if (src[i] === "[") {
        const j = src.indexOf("]", i);
        const body = src.slice(i + 1, j);
        i = j + 1;
        if (/[\\^]/.test(body)) return null;
        atom = [];
        for (let k = 0; k < body.length; k++) {
          if (body[k + 1] === "-" && k + 2 < body.length) {
            for (let c = body.charCodeAt(k); c <= body.charCodeAt(k + 2); c++) atom.push(String.fromCharCode(c));
            k += 2;
          } else atom.push(body[k]!);
        }
      } else if (src[i] === "\\") {
        const c = src[i + 1]!;
        if (/[A-Za-z0-9]/.test(c)) return null;
        atom = [c];
        i += 2;
      } else if (/[*+{.^$]/.test(src[i]!)) return null;
      else {
        atom = [src[i]!];
        i++;
      }
      if (src[i] === "?") {
        i++;
        if (src[i] === "?") i++;
        atom = [...atom, ""];
      }
      const next: string[] = [];
      for (const a of acc) for (const b of atom) next.push(a + b);
      acc = next;
      if (acc.length > 100000) return null;
    }
    return acc;
  };
  const words = alt();
  return i === src.length ? words : null;
}

/** A word list: `(?i)\b(…)\b`, or `(\\)?\b(…)\b` with a leading backslash.
 * A word is a run of word bytes, or two runs joined by `\` or `-`, as in
 * `XMLDiff\DOM` and `OCI-Lob`. */
type WordList = { fold: boolean; slash: boolean; body: string; words: string[] };

function wordList(regex: string): WordList | null {
  const m = /^(\(\?i\))?(\(\\\\\)\?)?\\b(.*)\\b$/.exec(regex);
  if (!m) return null;
  const words = expandWords(m[3]!);
  if (!words || words.some((word) => !/^[A-Za-z0-9_]+([\\-][A-Za-z0-9_]+)?$/.test(word))) return null;
  const fold = m[1] !== undefined;
  return { fold, slash: m[2] !== undefined, body: m[3]!, words: [...new Set(words.map((w) => (fold ? w.toLowerCase() : w)))] };
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
  /** The word list it is, when it is one. */
  words: WordList | null;
};
type Rule = {
  id: string;
  name: string[];
  content: string[];
  end: string;
  /** The begin capture group that the end pattern repeats, when it does. */
  backref: number | null;
  /** The nishiki `Language` and its cargo feature, when the rule's patterns
   * include another grammar that nishiki lexes. */
  embed: [string, string] | null;
  /** Whether the end is tried after the rule's patterns, as
   * `applyEndPatternLast` asks: it is then the last of them. */
  endLast: boolean;
  patterns: string[];
};

/** The grammars Perl includes, by scope name: the `Language` variant and its
 * feature, or `null` for PostScript, which Shiki lacks. */
const embeddable: Record<string, [string, string] | null> = {
  "text.html.basic": ["Html", "lang-html"],
  "text.xml": ["Xml", "lang-xml"],
  "source.sql": ["Sql", "lang-sql"],
  "source.js": ["JavaScript", "lang-javascript"],
  "source.css": ["Css", "lang-css"],
  "source.postscript": null,
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
  const rule: Rule = { id, name: [], content: [], end: "", backref: null, embed: null, endLast: false, patterns: [] };
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

/** The rules some of whose patterns include a grammar that Shiki lacks. */
const missing = new Set<string>();
/** The rules that vscode-textmate drops: every pattern they list includes a
 * grammar that Shiki lacks, or is itself dropped. */
const dropped = new Set<string>();

/** Registers a match or begin/end rule under `id`, unless the same node is
 * registered already; returns the pattern id, or `null` for a dropped
 * rule. */
function register(node: Node, id: string): string | null {
  const key = JSON.stringify(node);
  const known = seen.get(key);
  if (known !== undefined) return dropped.has(known) ? null : known;
  if (patternById.has(id)) throw new Error(`${id} names two rules`);
  seen.set(key, id);
  if (node.while !== undefined) throw new Error(`${id}: while`);
  if (node.match !== undefined) {
    patternById.set(id, {
      id,
      regex: node.match,
      act: { role: "match", name: scopeList(node.name) },
      captures: captureList(node.captures),
      nested: nestedList(node.captures),
      words: wordList(node.match),
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
      words: null,
    });
    const end = `${id}End`;
    const endCaptures = node.endCaptures ?? node.captures;
    patternById.set(end, {
      id: end,
      regex: node.end,
      act: { role: "end" },
      captures: captureList(endCaptures),
      nested: nestedList(endCaptures),
      words: null,
    });
    const backrefs = [...new Set([...node.end.matchAll(/\\([1-9])/g)].map((m) => Number(m[1])))];
    if (backrefs.length > 1) throw new Error(`${id}: the end repeats more than one group`);
    const rule: Rule = {
      id,
      name: scopeList(node.name),
      content: scopeList(node.contentName),
      end,
      backref: backrefs[0] ?? null,
      embed: null,
      endLast: node.applyEndPatternLast === 1 || node.applyEndPatternLast === true,
      patterns: [],
    };
    rules.set(id, rule);
    rule.patterns = flatten(node.patterns ?? [], id);
    // vscode-textmate skips a rule whose patterns are all missing, as a
    // heredoc of PostScript, which Shiki lacks, is.
    if (missing.has(id) && rule.patterns.length === 0 && rule.embed === null) {
      patternById.delete(id);
      patternById.delete(end);
      rules.delete(id);
      dropped.add(id);
      return null;
    }
    if (rule.embed && rule.backref !== null) {
      throw new Error(`${id}: a rule that embeds carries the language's state, not its backreference`);
    }
    return id;
  }
  throw new Error(`${id}: neither a match nor a begin/end rule`);
}

/** The pattern ids a pattern list expands to, `include`s flattened. */
function flatten(list: Node[], context: string): string[] {
  const out: string[] = [];
  list.forEach((node, index) => {
    if (node.include === "$self" || node.include === "source.perl") {
      out.push(...flatten(grammar.patterns, "Root"));
    } else if (node.include !== undefined && !node.include.startsWith("#")) {
      const embed = embeddable[node.include];
      if (embed === undefined) throw new Error(`${context}: external include ${node.include}`);
      if (index !== list.length - 1) throw new Error(`${context}: patterns after ${node.include}`);
      if (embed === null) missing.add(context);
      else rules.get(context)!.embed = embed;
    } else if (node.include !== undefined) {
      const key = node.include.slice(1);
      const target = repository[key];
      if (!target) throw new Error(`unknown include ${key}`);
      out.push(...expand(target, pascal(key)));
    } else {
      for (const [variant, suffix] of heredocForms(node)) {
        const ids = expand(variant, `${context}At${index}${suffix}`);
        if (ids.length === 0) missing.add(context);
        out.push(...ids);
      }
    }
  });
  return out;
}

/** A heredoc, once with the `~` of `<<~` and once without.
 *
 * Its end, `^((?!\5)\s+)?((\6))$`, repeats the `~` and the label.
 * vscode-textmate writes a group's text into the end, or nothing where the
 * group took no part.
 * With the `~`, the end allows the closing line's indentation.
 * Without it, `(?!)` allows none.
 * The form with the `~` comes first, as `(~)?` tries it first.
 * The label stays a back-reference, unless the begin names it, as `(HTML)`
 * does; then the end names it too.
 * Any other node stays as it is. */
function heredocForms(node: Node): [Node, string][] {
  if (node.begin === undefined || !node.begin.includes("(<<(~)?)")) return [[node, ""]];
  if (node.end !== "^((?!\\5)\\s+)?((\\6))$") throw new Error(`${node.begin}: a heredoc with another end`);
  const label = /^\(\(\(\(<<\(~\)\?\) \*[^)]*\)\(([A-Z]+)\)/.exec(node.begin)?.[1];
  const end = (tilde: string) => node.end!.replace("\\5", tilde).replace("\\6", label ?? "\\6");
  return [
    [{ ...node, begin: node.begin.replace("(<<(~)?)", "(<<(~))"), end: end("~") }, "Indented"],
    [{ ...node, begin: node.begin.replace("(<<(~)?)", "(<<())"), end: end("") }, ""],
  ];
}

/** A repository entry that lists one pattern takes the entry's name. */
function expand(node: Node, id: string): string[] {
  const one = (registered: string | null) => (registered === null ? [] : [registered]);
  if (node.match !== undefined || node.begin !== undefined) return one(register(node, id));
  if (node.patterns?.length === 1 && node.patterns[0]!.include === undefined) {
    return one(register(node.patterns[0]!, id));
  }
  if (node.patterns) {
    const ids = flatten(node.patterns, id);
    if (missing.has(id) && ids.length === 0) throw new Error(`${id}: a repository entry whose patterns are all missing`);
    return ids;
  }
  throw new Error(`${id}: empty rule`);
}

const rootRule: Rule = { id: "Root", name: [], content: [], end: "", backref: null, embed: null, endLast: false, patterns: [] };
rules.set("Root", rootRule);
rootRule.patterns = flatten(grammar.patterns, "Root");
const ruleList = [...rules.values()];
const patternList = [...patternById.values()];

/** The built-in functions and handles, the one long word list. */
const functionLists = patternList.filter((p) => p.words !== null && p.regex.length > 1000);
if (functionLists.length !== 1) throw new Error(`${functionLists.length} long word lists`);
const functions = functionLists[0]!;
if (functions.words!.fold || functions.words!.slash) throw new Error(`${functions.id}: a folded or slashed list`);

// Output: the tables.

const list = (ids: string[]) => (ids.length ? `scopes![${ids.map((id) => `S::${id}`).join(", ")}]` : "S::NO_SCOPES");

let g = `//! The grammar's rules as tables. Generated by \`oracle/perl-grammar.ts\`
//! from the \`perl\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::{End, Perl, embedded};
use super::scope::S;
use crate::rules;
use crate::scope::scopes;

pub(crate) type Act = rules::Act<Perl>;
pub(crate) type PatternDefinition = rules::PatternDefinition<Perl>;
pub(crate) type RuleDefinition = rules::RuleDefinition<Perl>;
pub(crate) use crate::rules::Start;

`;

if (ruleList.length > 256) throw new Error(`${ruleList.length} rules do not fit one byte`);

g += `/// A rule of the grammar that can be open at the end of a line.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub(crate) enum Rule {\n`;
for (const r of ruleList) g += `    ${r.id},\n`;
g += `}\n\nimpl Rule {\n    /// Every rule, in code order.\n    pub(crate) const ALL: [Rule; ${ruleList.length}] = [\n`;
for (const r of ruleList) g += `        Rule::${r.id},\n`;
g += `    ];\n\n${tooLong}    pub(crate) fn definition(self) -> RuleDefinition {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    `RuleDefinition {\n                name: ${list(r.name)},\n                content: ${list(r.content)},\n                end: ${r.end && !r.endLast ? `Some(Pattern::${r.end})` : "None"},\n                backref: ${r.backref === null ? "None" : `Some(${r.backref})`},\n                patterns: &[${[...r.patterns, ...(r.end && r.endLast ? [r.end] : [])].map((p) => `Pattern::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n\n    /// What a rule carries from line to line.\n    /// A rule whose patterns include another language's grammar carries that\n    /// language's state, when its feature is on.\n    /// A rule whose end repeats a begin capture group carries that group.\n${tooLong}    #[inline]\n    pub(crate) fn payload(self) -> rules::Payload {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    r.embed
      ? `embedded!(${r.embed[0]}, "${r.embed[1]}").map_or(rules::Payload::None, rules::Payload::Embed)`
      : r.backref === null
        ? "rules::Payload::None"
        : `rules::Payload::Backref(${r.backref})`,
  ]),
  "Rule",
);
/** Where an embedding rule takes the line back, as `engine::End` spells it. */
const endHalt = (r: Rule): string => {
  if (r.embed === null) return "End::Never";
  const begin = patternById.get(r.id)?.regex;
  const end = patternById.get(r.end)?.regex;
  if (begin === "^" && end === "\\n" && r.patterns.length === 0) return "End::LineFeed";
  const interpolation = [...Array(9).keys()].map((k) => `EscapedCharAt${k}`).concat([...Array(11).keys()].map((k) => `VariableAt${k}`));
  if (begin === "^" && end === "\\n" && r.patterns.join() === interpolation.join()) return "End::Interpolated";
  if (end === "^(?:(=end)\\s+(html)|(?==cut))") return "End::PodHtml";
  throw new Error(`${r.id}: an embedding rule of no known kind`);
};
g += `        }\n    }\n\n    /// Where the rule takes the line back from the language it embeds.\n${tooLong}    pub(crate) fn end_halt(self) -> End {\n        match self {\n`;
g += arms(ruleList.map((r): [string, string] => [r.id, endHalt(r)]), "Rule");
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
    const caps = p.captures.map(([n, ids]) => `(${n}, ${list(ids)})`).join(", ");
    const nested = p.nested.map(([n, rule]) => `(${n}, Rule::${rule})`).join(", ");
    return [p.id, `PatternDefinition {\n                act: ${act},\n                captures: &[${caps}],\n                nested: &[${nested}],\n            }`];
  }),
  "Pattern",
);
g += `        }\n    }\n}\n`;

for (const p of patternList) {
  if (startOf(p.regex).any && !p.regex.includes("\\G")) {
    throw new Error(`${p.id}: the first-byte analysis does not read ${p.regex}`);
  }
}
g += startTable(patternList.map((p) => ({ id: p.id, regex: p.regex })), "any");
g += ruleCodes("Perl", "PERL");
g += groupsCheck("Perl", patternList.flatMap((p) => [...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]));

// Output: the words.

const words = [...functions.words!.words].sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));
const text = words.join("");
if (text.length > 65535) throw new Error(`${text.length} bytes do not fit a u16 end`);
let end = 0;
const ends = words.map((word) => (end += word.length));
const w = `//! The grammar's long word list. Generated by \`oracle/perl-grammar.ts\`
//! from the \`perl\` grammar Shiki 4.6.0 bundles; do not edit.

/// The built-in functions and handles of \`${functions.id}\`: ${words.length} words,
/// sorted, one after another.
pub(crate) const FUNCTIONS: &str = ${JSON.stringify(text)};

/// Where each word of [\`FUNCTIONS\`] ends.
pub(crate) static FUNCTION_ENDS: [u16; ${ends.length}] = [${ends.join(", ")}];
`;

await writeRust(`${root_dir}src/perl/grammar.rs`, g);
await writeRust(`${root_dir}src/perl/words.rs`, w);
await writeRust(`${root_dir}src/perl/scope.rs`, scopeFile("perl-grammar.ts", "PERL", scopes));

// With RX_DUMP set to a file, writes each pattern for oracle/onig.ts, as the
// matchers answer for it.
// An end that repeats a group of its begin is left out, since a case cannot
// supply that group's text.
// So is `\z`, which vscode-textmate rewrites so that it never matches a
// line, where Oniguruma matches it at the end of the text.
if (Bun.env["RX_DUMP"]) {
  const groupsOf = (p: Pattern) => [...new Set([...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)])];
  const probed = patternList.filter((p) => !(p.act.role === "end" && /\\[1-9]/.test(p.regex)) && p.regex !== "\\z");
  await Bun.write(
    Bun.env["RX_DUMP"],
    probed.map((p) => JSON.stringify({ id: p.id, regex: p.regex, groups: groupsOf(p) })).join("\n") + "\n",
  );
}

console.log(`${ruleList.length} rules, ${patternList.length} patterns, ${scopes.size} scopes, ${words.length} functions`);
