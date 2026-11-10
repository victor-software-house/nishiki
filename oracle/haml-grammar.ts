// Writes src/haml/scope.rs and src/haml/grammar.rs from the `haml` grammar
// that Shiki 4.6.0 bundles (karuna/haml-vscode, MIT).
//
// grammar.rs holds the scopes each rule and pattern adds, how each rule ends
// and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// A pattern or rule that repeats another, with the same scopes, is the same
// `Pattern` or `Rule`, so it has one matcher.
//
// The rules that include another grammar embed it (`CarriedState::Embed`):
// - the filters, such as `:ruby`, `:javascript` and `:markdown`;
// - the Ruby of a line that starts with `-`, `=` or `~`, of a tag's
//   attribute hash and object, and of an interpolation.
// Some of them list patterns of their own before or after the grammar.
// `engine::End` stops the language where a pattern before it starts.
// `Embeds::embeds_after_own` stops it where a pattern after it starts.
//
// A filter's end repeats the indent its begin read.
// The language it embeds keeps a state, and a frame carries one payload.
// The include therefore becomes a rule of its own inside the filter.
// That rule adds no scope and embeds the language.
// Its end reads the indent from the filter's frame.
//
// The includes of grammars that Shiki's `haml` does not load are dropped, as
// vscode-textmate drops them: `source.scss`, `text.plain` and
// `text.html.php#language`.
// `source.ruby#escaped_char` is Ruby's escape pattern, read from Ruby's
// grammar.
// `text.haml` is the grammar's own root.
//
// The matchers that decide whether a pattern matches are written by hand in
// src/haml/matchers.
//
// Run with `bun run oracle/haml-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import haml from "@shikijs/langs@4.6.0/haml";
import ruby from "@shikijs/langs@4.6.0/ruby";
import {
  arms,
  groupsCheck,
  ruleCodes,
  scopeFile,
  startOf,
  startTable,
  takesOneBlank,
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
type Grammar = {
  name: string;
  scopeName: string;
  repository: Record<string, Node>;
  patterns: Node[];
};

const grammar = (haml as unknown as Grammar[]).find((lang) => lang.name === "haml")!;
const repository = grammar.repository;
const rubyGrammar = (ruby as unknown as Grammar[]).find((lang) => lang.name === "ruby")!;

const root_dir = new URL("..", import.meta.url).pathname;

// Scope names.

const abbreviations: Record<string, string> = {
  punctuation: "P",
  keyword: "Kw",
  constant: "Const",
  entity: "Ent",
  variable: "Var",
  string: "Str",
  comment: "Cmt",
  meta: "Meta",
  storage: "Stor",
  source: "Src",
};

/** The identifier of a scope name: its parts in Pascal case. */
function scopeIdent(name: string): string {
  let out = "";
  name.split(".").forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const capture = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? capture) : capture;
    }
  });
  return out;
}

const scopes = new Map<string, string>([["Text", grammar.scopeName]]);
function scopeList(name: string | undefined): string[] {
  if (name === undefined) return [];
  return name.split(" ").map((part) => {
    if (part.includes("$")) throw new Error(`${part}: a dynamic scope`);
    const ident = scopeIdent(part);
    const known = scopes.get(ident);
    if (known !== undefined && known !== part) throw new Error(`${ident}: ${known} and ${part}`);
    scopes.set(ident, part);
    return ident;
  });
}

// Rules and patterns.

const pascal = (key: string) =>
  key
    .split(/[-_.]/)
    .map((piece) => piece.charAt(0).toUpperCase() + piece.slice(1))
    .join("");

/** The names of the root's patterns, in the grammar's order. A pattern that
 * repeats an earlier one takes the earlier one's name. */
const names = [
  "InterpolatedText",
  "RubyFilter",
  "Prolog",
  "JavascriptFilter",
  "Script",
  "RubyFilterAlone",
  "ConditionalComment",
  "Comment",
  "Tag",
  "RubyOrOpalFilter",
  "RubyFilterAloneAgain",
  "SassStyleFilter",
  "CoffeeFilter",
  "PlainFilter",
  "RubyFilterKeyword",
  "SassFilter",
  "SassStylesFilter",
  "PlainFilterAgain",
  "Escape",
  "RubyStart",
  "PhpFilter",
  "MarkdownFilter",
  "CssFilter",
  "SassFilterAlone",
  "ScssFilter",
];
if (names.length !== grammar.patterns.length) throw new Error("the grammar's root changed");

type Pattern = {
  id: string;
  regex: string;
  act: { role: "match"; name: string[] } | { role: "begin"; rule: string } | { role: "end" };
  captures: [number, string[]][];
  /** Whether the generator made it: a filter body's begin or end. */
  made: boolean;
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
  /** Whether the rule lists its own patterns after the grammar it embeds. */
  ownAfter: boolean;
  patterns: string[];
};

/** The grammars HAML includes, by scope name: the `Language` variant and its
 * feature. nishiki lexes them all, and each can halt. */
const embeddable: Record<string, [string, string]> = {
  "source.ruby": ["Ruby", "lang-ruby"],
  "source.js": ["JavaScript", "lang-javascript"],
  "source.css": ["Css", "lang-css"],
  "source.sass": ["Sass", "lang-sass"],
  "source.coffee": ["CoffeeScript", "lang-coffee"],
  "text.html.markdown": ["Markdown", "lang-markdown"],
};

/** The includes of grammars that Shiki's `haml` does not load. */
const dropped = new Set(["source.scss", "text.plain", "text.html.php#language"]);

const patternById = new Map<string, Pattern>();
const rules = new Map<string, Rule>();
/** The id each node registered under, by the node's text. */
const seen = new Map<string, string>();
/** The rules some of whose patterns include a grammar that Shiki's `haml`
 * does not load. */
const missing = new Set<string>();
/** The rules that vscode-textmate drops: every pattern they list includes a
 * grammar that Shiki's `haml` does not load. */
const droppedRules = new Set<string>();

function captureList(captures: Captures | undefined): [number, string[]][] {
  if (!captures) return [];
  return Object.entries(captures)
    .map(([group, value]): [number, string[]] => {
      if (value.patterns !== undefined) throw new Error(`group ${group}: patterns of its own`);
      return [Number(group), scopeList(value.name)];
    })
    .filter(([, list]) => list.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** Registers a match or begin/end rule under `id`, unless the same node is
 * registered already; returns the pattern id, or `null` for a dropped
 * rule. */
function register(node: Node, id: string): string | null {
  const key = JSON.stringify(node);
  const known = seen.get(key);
  if (known !== undefined) return droppedRules.has(known) ? null : known;
  if (patternById.has(id)) throw new Error(`${id} names two rules`);
  seen.set(key, id);
  if (node.applyEndPatternLast) throw new Error(`${id}: applyEndPatternLast`);
  if (node.while !== undefined) throw new Error(`${id}: while`);
  if (node.match !== undefined) {
    patternById.set(id, {
      id,
      regex: node.match,
      act: { role: "match", name: scopeList(node.name) },
      captures: captureList(node.captures),
      made: false,
    });
    return id;
  }
  if (node.begin === undefined || node.end === undefined)
    throw new Error(`${id}: neither a match nor a begin/end rule`);
  patternById.set(id, {
    id,
    regex: node.begin,
    act: { role: "begin", rule: id },
    captures: captureList(node.beginCaptures ?? node.captures),
    made: false,
  });
  const end = `${id}End`;
  patternById.set(end, {
    id: end,
    regex: node.end,
    act: { role: "end" },
    captures: captureList(node.endCaptures ?? node.captures),
    made: false,
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
    ownAfter: false,
    patterns: [],
  };
  rules.set(id, rule);
  rule.patterns = flatten(node.patterns ?? [], id);
  // vscode-textmate skips a rule whose patterns are all missing, as the
  // `:plain`, `:php` and `:scss` filters are.
  if (missing.has(id) && rule.patterns.length === 0 && rule.embed === null) {
    patternById.delete(id);
    patternById.delete(end);
    rules.delete(id);
    droppedRules.add(id);
    return null;
  }
  if (rule.embed !== null && rule.backref !== null) body(rule, node.end);
  return id;
}

/** Moves a filter's embedding into a rule of its own, `<filter>Body`, which
 * opens where the filter's begin ends and closes where the filter's end
 * would match. The filter keeps its backreference. */
function body(filter: Rule, end: string): void {
  if (filter.patterns.length > 0)
    throw new Error(`${filter.id}: patterns beside the grammar it embeds`);
  const id = `${filter.id}Body`;
  patternById.set(id, {
    id,
    regex: "\\G",
    act: { role: "begin", rule: id },
    captures: [],
    made: true,
  });
  patternById.set(`${id}End`, {
    id: `${id}End`,
    regex: `(?=${end})`,
    act: { role: "end" },
    captures: [],
    made: true,
  });
  rules.set(id, {
    id,
    name: [],
    content: [],
    end: `${id}End`,
    backref: null,
    embed: filter.embed,
    ownAfter: false,
    patterns: [],
  });
  filter.embed = null;
  filter.patterns = [id];
}

/** A registered pattern id as a list: empty for a dropped rule. */
function one(id: string | null): string[] {
  return id === null ? [] : [id];
}

/** The root's pattern ids. */
function rootIds(): string[] {
  return grammar.patterns.flatMap((node, index) => one(register(node, names[index]!)));
}

/** The pattern ids a pattern list expands to, `include`s flattened. */
function flatten(list: Node[], context: string): string[] {
  const out: string[] = [];
  list.forEach((node, index) => {
    const include = node.include;
    if (include === "$self" || include === grammar.scopeName) {
      out.push(...rootIds());
    } else if (include === "source.ruby#escaped_char") {
      out.push(...one(register(rubyGrammar.repository["escaped_char"]!, "RubyEscapedChar")));
    } else if (include !== undefined && embeddable[include] !== undefined) {
      const rule = rules.get(context);
      if (!rule) throw new Error(`${context}: ${include} outside a rule`);
      if (rule.embed !== null) throw new Error(`${context}: two embedded grammars`);
      rule.embed = embeddable[include]!;
      rule.ownAfter = list
        .slice(index + 1)
        .some((after) => after.include === undefined || !dropped.has(after.include));
    } else if (include !== undefined && dropped.has(include)) {
      // vscode-textmate drops an include of a grammar it has not loaded.
      missing.add(context);
    } else if (include !== undefined) {
      if (!include.startsWith("#")) throw new Error(`${context}: external include ${include}`);
      const key = include.slice(1);
      const target = repository[key];
      if (!target) throw new Error(`unknown include ${key}`);
      out.push(...expand(target, pascal(key)));
    } else {
      out.push(...expand(node, `${context}At${index}`));
    }
  });
  return out;
}

/** A repository entry that lists one pattern takes the entry's name. */
function expand(node: Node, id: string): string[] {
  if (node.match !== undefined || node.begin !== undefined) return one(register(node, id));
  if (node.patterns?.length === 1 && node.patterns[0]!.include === undefined) {
    return one(register(node.patterns[0]!, id));
  }
  if (node.patterns) return flatten(node.patterns, id);
  throw new Error(`${id}: empty rule`);
}

const rootRule: Rule = {
  id: "Root",
  name: [],
  content: [],
  end: "",
  backref: null,
  embed: null,
  ownAfter: false,
  patterns: [],
};
rules.set("Root", rootRule);
rootRule.patterns = rootIds();

// No pattern may take a blank after the first of a run: the lexer skips the
// rest of a run (`BlankRuns::Skip`).
for (const p of patternById.values()) {
  if (takesOneBlank(p.regex, "any"))
    throw new Error(`${p.id} takes one blank: the lexer needs BlankRuns::ToLast`);
}

const ruleList = [rootRule, ...[...rules.values()].filter((r) => r !== rootRule)];
const patternList = [...patternById.values()];

/** Where an embedding rule takes the line back, as `engine::End` spells it:
 * its end, and the patterns it lists before the grammar it embeds. */
function endHalt(r: Rule): string {
  if (r.embed === null) return "End::Never";
  if (r.id.endsWith("Body")) {
    const end = patternById.get(r.end)!.regex;
    if (end === "(?=^(?!\\1\\s+|$\\n*))") return "End::Filter(Filter::Unindented)";
    if (end === "(?=^(?=\\1\\s+|$\\n*))") return "End::Filter(Filter::Indented)";
    if (end === "(?=(?m:(?<=\\n)(?!\\1\\s+|$\\n*)))") return "End::Filter(Filter::AfterLineFeed)";
    throw new Error(`${r.id}: a filter end of no known kind: ${end}`);
  }
  const ends: Record<string, string> = {
    Rubyline: "End::RubyLine",
    TagAt3: "End::Attributes",
    TagAt5: "End::Object",
    InterpolatedRubyAt1: "End::Interpolation",
    NestCurlyAndSelf: "End::Curly",
  };
  const end = ends[r.id];
  if (end === undefined) throw new Error(`${r.id}: an embedding rule of no known kind`);
  return end;
}

// Output.

const list = (ids: string[]) =>
  ids.length ? `scopes![${ids.map((id) => `S::${id}`).join(", ")}]` : "S::NO_SCOPES";

let g = `//! The grammar's rules as tables. Generated by \`oracle/haml-grammar.ts\`
//! from the \`haml\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::{End, Filter, Haml};
use super::scope::S;
use crate::rules::{self, embedded};
use crate::scope::scopes;

pub(crate) type Act = rules::Act<Haml>;
pub(crate) type PatternDefinition = rules::PatternDefinition<Haml>;
pub(crate) type RuleDefinition = rules::RuleDefinition<Haml>;
pub(crate) use crate::rules::Start;

/// The patterns a capture group is tokenized with: none in this grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Nested {}

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
    `RuleDefinition {\n                name: ${list(r.name)},\n                content: ${list(r.content)},\n                end: ${r.end ? `Some(Pattern::${r.end})` : "None"},\n                backref: ${r.backref === null ? "None" : `Some(${r.backref})`},\n                patterns: &[${r.patterns.map((p) => `Pattern::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n\n    /// What a rule carries from line to line.\n    /// A rule whose patterns include another language's grammar carries that\n    /// language's state, when its feature is on.\n    /// A rule whose end repeats a begin capture group carries that group.\n    #[inline]\n    pub(crate) fn payload(self) -> rules::CarriedState {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    r.embed
      ? `embedded!(${r.embed[0]}, "${r.embed[1]}").map_or(rules::CarriedState::None, rules::CarriedState::Embed)`
      : r.backref === null
        ? "rules::CarriedState::None"
        : `rules::CarriedState::Backref(${r.backref})`,
  ]),
  "Rule",
);
g += `        }\n    }\n\n    /// Where the rule takes the line back from the language it embeds.\n    pub(crate) fn end_halt(self) -> End {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [r.id, endHalt(r)]),
  "Rule",
);
g += `        }\n    }\n\n    /// Whether the rule lists its own patterns after the grammar it embeds.\n    pub(crate) fn embeds_after_own(self) -> bool {\n        matches!(self, ${ruleList
  .filter((r) => r.ownAfter)
  .map((r) => `Rule::${r.id}`)
  .join(" | ")})\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pattern {\n`;
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 || /[^\x20-\x7e]/.test(one) ? `…` : one;
};
for (const p of patternList) {
  const what = p.made ? (p.act.role === "begin" ? "made begin" : "made end") : p.act.role;
  g += `    /// The ${what} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
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
    return [
      p.id,
      `PatternDefinition {\n                act: ${act},\n                captures: &[${captures}],\n                nested: &[],\n            }`,
    ];
  }),
  "Pattern",
);
g += `        }\n    }\n}\n`;

for (const p of patternList) {
  if (startOf(p.regex, "any").any && !p.regex.includes("\\G")) {
    throw new Error(`${p.id}: the first-byte analysis does not read ${p.regex}`);
  }
}
g += startTable(patternList, "any");
g += ruleCodes("Haml", "HAML");
g += groupsCheck(
  "Haml",
  patternList.flatMap((p) => p.captures.map(([n]) => n)),
);

await writeRust(`${root_dir}src/haml/grammar.rs`, g);
// A dropped rule's scopes are emitted by no pattern, so the table leaves them
// out.
const used = new Set<string>(["Text"]);
for (const rule of rules.values())
  for (const ident of [...rule.name, ...rule.content]) used.add(ident);
for (const pattern of patternById.values()) {
  if (pattern.act.role === "match") for (const ident of pattern.act.name) used.add(ident);
  for (const [, list] of pattern.captures) for (const ident of list) used.add(ident);
}
const emitted = new Map([...scopes].filter(([ident]) => used.has(ident)));
await writeRust(`${root_dir}src/haml/scope.rs`, scopeFile("haml-grammar.ts", "HAML", emitted));

// With RX_DUMP set to a file, writes each pattern for oracle/onig.ts, as the
// matchers answer for it. An end that repeats a group of its begin is left
// out, since a case cannot supply that group's text, and so are the patterns
// the generator made.
// The block opener's matcher stops before the line feed its `$\n*` takes:
// the line feed holds no token, and the engine tries the Ruby line's end at
// the line's end, as vscode-textmate does after the match. The probe asks
// for the expression without the line feed: `\s*` takes no line feed, and
// `\n*` is gone.
const probeRegex: Record<string, string> = { RubylineAt5: "([,<|]|do|\\{)(?:(?!\\n)\\s)*(#.*)?$" };
if (Bun.env["RX_DUMP"]) {
  const probed = patternList.filter(
    (p) => !p.made && !(p.act.role === "end" && /\\[1-9]/.test(p.regex)),
  );
  await Bun.write(
    Bun.env["RX_DUMP"],
    probed
      .map((p) =>
        JSON.stringify({
          id: p.id,
          regex: probeRegex[p.id] ?? p.regex,
          groups: p.captures.map(([n]) => n),
        }),
      )
      .join("\n") + "\n",
  );
}

console.log(`${ruleList.length} rules, ${patternList.length} patterns, ${scopes.size} scopes`);
