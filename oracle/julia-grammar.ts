// Writes src/julia/scope.rs, src/julia/grammar.rs and src/julia/classes.rs
// from the `julia` grammar that Shiki 4.6.0 bundles
// (JuliaEditorSupport/atom-language-julia, MIT).
//
// grammar.rs holds the scopes each rule and pattern adds, how each rule ends
// and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
//
// A string macro named for Python, JavaScript, SQL or R includes that
// language's grammar.
// Its rule embeds the language (`CarriedState::Embed`), whose feature `lang-julia`
// turns on.
// The `cxx` macro includes `source.cpp#root_context`, a key the C++ grammar
// lacks, so it adds no patterns.
// It keeps only its Julia patterns.
//
// classes.rs holds the characters of the grammar's identifier classes and of
// `[[\p{S}\p{P}]&&[^@\s]]`.
// They are measured on Shiki's own Oniguruma engine: one scan of a text of
// every code point for each class.
//
// The matchers that decide whether a pattern matches are written by hand in
// src/julia/matchers.
//
// Run with `bun run oracle/julia-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import { createOnigurumaEngine } from "@shikijs/engine-oniguruma@4.6.0";
import julia from "@shikijs/langs@4.6.0/julia";
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
};

const grammar = (
  julia as unknown as { name: string; repository: Record<string, Node>; patterns: Node[] }[]
).find((lang) => lang.name === "julia")!;
const repository = grammar.repository;

const root_dir = new URL("..", import.meta.url).pathname;

// Scope names.

const abbreviations: Record<string, string> = {
  punctuation: "P",
  keyword: "Kw",
  constant: "Const",
  entity: "Ent",
  variable: "Var",
  support: "Supp",
  string: "Str",
  comment: "Cmt",
  meta: "Meta",
};

function scopeIdent(name: string): string {
  const parts = name.split(".");
  // A name without the language's suffix, such as
  // `meta.embedded.inline.cpp`, ends in `Bare`.
  const suffix = parts[parts.length - 1] === "julia" ? "" : "Bare";
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

const scopes = new Map<string, string>([["Source", "source.julia"]]);
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

/** Names for the patterns of a repository entry that lists several, which
 * no repository key names. */
const names: Record<string, string> = {
  OperatorAt0: "Arrow",
  OperatorAt1: "Update",
  OperatorAt2: "Shift",
  OperatorAt3: "TypeRelation",
  OperatorAt4: "Relation",
  OperatorAt5: "TernaryQuestion",
  OperatorAt6: "TernaryColon",
  OperatorAt7: "Boolean",
  OperatorAt8: "RangeColon",
  OperatorAt9: "Applies",
  OperatorAt10: "Bitwise",
  OperatorAt11: "Arithmetic",
  OperatorAt12: "Compose",
  OperatorAt13: "Isa",
  OperatorAt14: "In",
  OperatorAt15: "Dots",
  OperatorAt16: "InterpolationOperator",
  OperatorAt17: "TransposedVariable",
  OperatorAt18: "TransposedMatrix",
  OperatorAt19: "TransposedParens",
  ArrayAt0: "ArrayBegin",
  ArrayAt1: "ArrayEndWord",
  KeywordAt0: "KeywordOther",
  KeywordAt1: "KeywordControl",
  KeywordAt2: "KeywordEnd",
  KeywordAt3: "StorageModifier",
  KeywordAt4: "Export",
  KeywordAt5: "Public",
  KeywordAt6: "Import",
  KeywordAt7: "Using",
  KeywordAt8: "As",
  KeywordAt9: "MacroName",
  NumberAt0: "Number",
  NumberAt1: "GlobalConstant",
  NumberAt2: "LanguageConstant",
  CommentTagsAt0: "Todo",
  CommentTagsAt1: "Fixme",
  CommentTagsAt2: "Changed",
  CommentTagsAt3: "Xxx",
  FunctionDeclAt0: "FunctionAssignment",
  FunctionDeclAt1: "FunctionKeyword",
  StringEscapedCharRawAt0: "RawEscapedQuote",
  StringEscapedCharRawAt1: "RawEscapedBackslashes",
  PunctuationAt0: "Comma",
  PunctuationAt1: "Semicolon",
  ForBlockAt0: "Outer",
  CommentAt1: "LineComment",
  StringAt0: "DocMacro",
  StringAt1: "Cxx",
  StringAt2: "Py",
  StringAt3: "Js",
  StringAt4: "RMacro",
  StringAt5: "RawTriple",
  StringAt6: "Raw",
  StringAt7: "Sql",
  StringAt8: "VarTriple",
  StringAt9: "Var",
  StringAt10: "Docstring",
  StringAt11: "Char",
  StringAt12: "Triple",
  StringAt13: "Quoted",
  StringAt14: "RegexTriple",
  StringAt15: "Regex",
  StringAt16: "MacroTriple",
  StringAt17: "Macro",
  StringAt18: "CommandTriple",
  StringAt19: "Command",
  StringDollarSignInterpolateAt0: "InterpolatedName",
  StringDollarSignInterpolateAt1: "Interpolation",
};

/** The languages a string macro embeds, by the grammar it includes.
 * `lang-julia` turns on their features. */
const embeddable: Record<string, string> = {
  "source.python": "Python",
  "source.js": "JavaScript",
  "source.sql": "Sql",
  "source.r": "R",
};

/** The include that adds no patterns: a key the C++ grammar lacks. */
const empty = new Set(["source.cpp#root_context"]);

type Pattern = {
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
  embed: string | null;
  patterns: string[];
};

const patternById = new Map<string, Pattern>();
const rules = new Map<string, Rule>();

function captureList(captures: Captures | undefined): [number, string[]][] {
  if (!captures) return [];
  for (const [group, value] of Object.entries(captures)) {
    if (value.patterns) throw new Error(`capture ${group}: nested patterns`);
  }
  return Object.entries(captures)
    .map(([group, value]): [number, string[]] => [Number(group), scopeList(value.name)])
    .filter(([, list]) => list.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** Registers a match or begin/end rule under `id`; returns the pattern id. */
function register(node: Node, id: string): string {
  const clash = patternById.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (clash) return id;
  if (node.applyEndPatternLast) throw new Error(`${id}: applyEndPatternLast`);
  if (node.match !== undefined) {
    patternById.set(id, {
      id,
      regex: node.match,
      act: { role: "match", name: scopeList(node.name) },
      captures: captureList(node.captures),
    });
    return id;
  }
  if (node.begin !== undefined && node.end !== undefined) {
    patternById.set(id, {
      id,
      regex: node.begin,
      act: { role: "begin", rule: id },
      captures: captureList(node.beginCaptures ?? node.captures),
    });
    patternById.set(`${id}End`, {
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
      embed: null,
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
    if (node.include === "$self") {
      out.push(...flatten(grammar.patterns, "Root"));
    } else if (node.include !== undefined && embeddable[node.include]) {
      // The include comes first, so the embedded grammar's patterns win
      // over the Julia patterns after it.
      if (index !== 0) throw new Error(`${context}: ${node.include} is not first`);
      rules.get(context)!.embed = embeddable[node.include]!;
    } else if (node.include !== undefined && empty.has(node.include)) {
      return;
    } else if (node.include !== undefined) {
      if (!node.include.startsWith("#")) throw new Error(`external include ${node.include}`);
      const key = node.include.slice(1);
      const target = repository[key];
      if (!target) throw new Error(`unknown include ${key}`);
      out.push(...expand(target, pascal(key)));
    } else {
      const id = `${context}At${index}`;
      out.push(...expand(node, names[id] ?? id));
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

const rootRule: Rule = { id: "Root", name: [], content: [], end: "", embed: null, patterns: [] };
rules.set("Root", rootRule);
rootRule.patterns = flatten(grammar.patterns, "Root");

// Output: the tables.

const list = (ids: string[]) =>
  ids.length ? `scopes![${ids.map((n) => `S::${n}`).join(", ")}]` : "S::NO_SCOPES";

let g = `//! The grammar's rules as tables. Generated by \`oracle/julia-grammar.ts\`
//! from the \`julia\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::Julia;
use super::scope::S;
use crate::rules;
use crate::scope::scopes;

pub(crate) type Act = rules::Act<Julia>;
pub(crate) type PatternDefinition = rules::PatternDefinition<Julia>;
pub(crate) type RuleDefinition = rules::RuleDefinition<Julia>;
pub(crate) use crate::rules::Start;

`;

const ruleList = [...rules.values()];
const patternList = [...patternById.values()];
if (ruleList.length > 256) throw new Error(`${ruleList.length} rules do not fit one byte`);

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
g += `        }\n    }\n\n    /// What a rule carries from line to line.\n    /// A string macro that includes another language's grammar carries that language's state.\n    #[inline]\n    pub(crate) fn carried_state(self) -> rules::CarriedState {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    r.embed
      ? `rules::CarriedState::Embed(crate::Language::${r.embed})`
      : "rules::CarriedState::None",
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
g += `}\n\nimpl Pattern {\n    /// Every pattern, for the tool that compares the matchers with Oniguruma.\n    #[cfg(all(test, feature = "json"))]\n    pub(crate) const ALL: [Pattern; ${patternList.length}] = [\n${patternList.map((p) => `        Pattern::${p.id},\n`).join("")}    ];\n\n${tooLong}    pub(crate) fn definition(self) -> PatternDefinition {\n        match self {\n`;
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

// The first-byte analysis does not read the identifier classes. For it, each
// stands for the bytes its characters can start with: the ASCII ones and every
// lead byte.
const classSource = (regex: string) =>
  regex
    .replaceAll(IDENT_START, "[_$A-Za-z\\x80-\\xff]")
    .replaceAll(IDENT_REST, "[!_$A-Za-z0-9\\x80-\\xff]");

const IDENT_START =
  "[_ⁱ-⁾₁-₎℘℮⅀-⅄∂∅∆∇∎-∑∞-∢∫-∳∿⊤⊥⊾-⋃◸-◿♯⟀⟁⟘⟙⦛-⦴⨀-⨆⨉-⨖⨛⨜゛゜𝛁𝛛𝛻𝜕𝜵𝝏𝝯𝞉𝞩𝟃𝟎-𝟡[:alpha:]\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lm}\\p{Lo}\\p{Nl}\\p{Sc}[^←-⇿\\P{So}]]";
const IDENT_REST =
  "[!_′-‷⁗ⁱ-⁾₁-₎℘℮⅀-⅄∂∅∆∇∎-∑∞-∢∫-∳∿⊤⊥⊾-⋃◸-◿♯⟀⟁⟘⟙⦛-⦴⨀-⨆⨉-⨖⨛⨜゛゜𝛁𝛛𝛻𝜕𝜵𝝏𝝯𝞉𝞩𝟃𝟎-𝟡[:word:]\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lm}\\p{Lo}\\p{Nl}\\p{Sc}[^\\x01-¡\\P{Mn}][^\\x01-¡\\P{Mc}][^\\x01-¡\\D][^\\x01-¡\\P{Pc}][^\\x01-¡\\P{Sk}][^\\x01-¡\\P{Me}][^\\x01-¡\\P{No}][^←-⇿\\P{So}]]";
/** The interpolation's classes: the identifier classes without `$`. */
const INTERP_START = IDENT_START.replace("\\p{Sc}[^←-⇿\\P{So}]]", "[^←-⇿\\P{So}][^$\\P{Sc}]]");
const INTERP_REST = IDENT_REST.replace("\\p{Sc}[^\\x01-¡\\P{Mn}]", "[^\\x01-¡\\P{Mn}]").replace(
  "[^←-⇿\\P{So}]]",
  "[^←-⇿\\P{So}][^$\\P{Sc}]]",
);
const MACRO_OPERATOR = "[[\\p{S}\\p{P}]&&[^@\\s]]";

const analysed = patternList.map((p) => ({
  id: p.id,
  regex: classSource(
    p.regex
      .replaceAll(INTERP_START, IDENT_START)
      .replaceAll(INTERP_REST, IDENT_REST)
      .replaceAll(MACRO_OPERATOR, "[!-?\\[-`{-~\\x80-\\xff]"),
  ),
}));
for (const p of analysed) {
  const { any } = startOf(p.regex);
  if (any) throw new Error(`${p.id}: the first-byte analysis does not read ${p.regex}`);
}
g += startTable(analysed);
g += ruleCodes("Julia", "JULIA");
g += groupsCheck(
  "Julia",
  patternList.flatMap((p) => p.captures.map(([n]) => n)),
);

await writeRust(`${root_dir}src/julia/grammar.rs`, g);
await writeRust(`${root_dir}src/julia/scope.rs`, scopeFile("julia-grammar.ts", "JULIA", scopes));

// With RX_DUMP set to a file, writes each pattern for oracle/onig.ts.
if (Bun.env["RX_DUMP"]) {
  await Bun.write(
    Bun.env["RX_DUMP"],
    patternList
      .map((p) => JSON.stringify({ id: p.id, regex: p.regex, groups: p.captures.map(([n]) => n) }))
      .join("\n") + "\n",
  );
}

// Output: the classes.

/** Every code point that can be in a class: ASCII without the line feed,
 * U+0080 to the end of plane 3, and the variation selectors of plane 14. */
const points: number[] = [];
for (let cp = 1; cp < 0x34000; cp++) {
  if (cp !== 10 && (cp < 0xd800 || cp > 0xdfff)) points.push(cp);
}
for (let cp = 0xe0000; cp < 0xe0200; cp++) points.push(cp);

const engine = await createOnigurumaEngine(import("@shikijs/engine-oniguruma@4.6.0/wasm-inlined"));

/** Whether each code point matches `cls`, from one scan of a text of every
 * point with the engine Shiki runs grammars on. */
function measure(cls: string): boolean[] {
  const offsets: number[] = [];
  let text = "";
  for (const cp of points) {
    offsets.push(text.length);
    text += String.fromCodePoint(cp);
  }
  offsets.push(text.length);
  const index = new Map(offsets.map((offset, i) => [offset, i]));
  const scanner = engine.createScanner([`${cls}+`]);
  const string = engine.createString(text);
  const flags = points.map(() => false);
  for (let from = 0; ;) {
    const match = scanner.findNextMatchSync(string, from);
    if (!match) break;
    const { start, end } = match.captureIndices[0]!;
    for (let i = index.get(start)!; offsets[i]! < end; i++) flags[i] = true;
    from = end;
  }
  return flags;
}

/** The bracket class of pattern `id` that starts with `head`, as written. */
function classOf(id: string, head: string): string {
  const regex = patternById.get(id)!.regex;
  const start = regex.indexOf(head);
  if (start < 0) throw new Error(`${id} has no class ${head}`);
  return regex.slice(start, regex.indexOf("]", start) + 1);
}

const identStart = measure(IDENT_START);
const identRest = measure(IDENT_REST);
const tables: [string, string, boolean[]][] = [
  ["IDENT_START", "The characters an identifier starts with.", identStart],
  ["IDENT_REST", "The characters an identifier goes on with.", identRest],
  [
    "MACRO_OPERATOR",
    "The characters of `[[\\p{S}\\p{P}]&&[^@\\s]]`, a macro named by symbols.",
    measure(MACRO_OPERATOR),
  ],
  ["ARROW", "The arrows of `keyword.operator.arrow`.", measure(classOf("Arrow", "[←"))],
  ["RELATION", "The relations of `keyword.operator.relation`.", measure(classOf("Relation", "[∈"))],
  [
    "PLUS",
    "The operators of `keyword.operator.arithmetic` that a dot may lead, first class.",
    measure(classOf("Arithmetic", "[-*+|")),
  ],
  [
    "TIMES",
    "The operators of `keyword.operator.arithmetic` that a dot may lead, second class.",
    measure(classOf("Arithmetic", "[%\\&")),
  ],
];
if (points.some((_, i) => identStart[i] && !identRest[i]))
  throw new Error("an identifier's first character cannot go on");

// The interpolation's classes differ from the identifier classes in `$` alone,
// which the matchers decide in ASCII.
for (const [cls, base] of [
  [INTERP_START, identStart],
  [INTERP_REST, identRest],
] as const) {
  const flags = measure(cls);
  const differ = points.filter((_, i) => flags[i] !== base[i]);
  if (differ.length !== 1 || differ[0] !== 0x24)
    throw new Error(`interpolation class differs in ${differ}`);
}

function ranges(flags: boolean[]): [number, number][] {
  const out: [number, number][] = [];
  flags.forEach((on, i) => {
    const cp = points[i]!;
    if (!on || cp < 0x80) return;
    const last = out[out.length - 1];
    if (last && last[1] + 1 === cp) last[1] = cp;
    else out.push([cp, cp]);
  });
  return out;
}

/** The ASCII characters of `flags`, as a mask whose bit `c` is character `c`. */
function ascii(flags: boolean[]): string {
  let mask = 0n;
  flags.forEach((on, i) => {
    const cp = points[i]!;
    if (on && cp < 0x80) mask |= 1n << BigInt(cp);
  });
  const digits = mask.toString(16).padStart(32, "0");
  return `0x${digits.match(/.{4}/g)!.join("_")}`;
}

function table(name: string, doc: string, flags: boolean[]): string {
  const rows = ranges(flags)
    .map(([lo, hi]) => `    (0x${lo.toString(16)}, 0x${hi.toString(16)}),`)
    .join("\n");
  const mask = points.some((cp, i) => flags[i] && cp < 0x80)
    ? `/// ${doc}\n/// Bit \`c\` is ASCII character \`c\`.\npub(super) const ${name}_ASCII: u128 = ${ascii(flags)};\n\n`
    : "";
  return `${mask}/// ${doc}\n/// The characters beyond ASCII, as sorted inclusive ranges.\npub(super) const ${name}: &[(u32, u32)] = &[\n${rows}\n];\n`;
}

const c = `//! The characters of the grammar's classes, as Shiki 4.6.0 reads them.
//! Generated by \`oracle/julia-grammar.ts\`; do not edit.

${tables.map(([name, doc, flags]) => table(name, doc, flags)).join("\n")}`;
await writeRust(`${root_dir}src/julia/classes.rs`, c);

console.log(
  `${ruleList.length} rules, ${patternList.length} patterns, ${scopes.size} scopes; ` +
    `class ranges ${tables.map(([, , flags]) => ranges(flags).length).join(" ")}`,
);
