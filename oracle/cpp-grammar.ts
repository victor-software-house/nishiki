// Writes src/cpp/scope.rs and src/cpp/grammar.rs from the `cpp` grammar that
// Shiki 4.6.0 bundles (microsoft/vscode `extensions/cpp`, MIT) and the grammars
// it embeds: `cpp-macro` for the bodies of `#define`, and the regular
// expression and GLSL grammars of raw strings. The output holds the scope
// names and, for every rule, the scopes it adds, how it ends and the patterns
// it tries, in the grammar's order with its `include`s flattened. The matchers
// that decide whether a pattern matches are written by hand in src/cpp.
//
// Run with `bun run oracle/cpp-grammar.ts` and commit the result; the output
// is generated, do not edit it. With RX_DUMP set to a file, it also writes the
// expressions there for oracle/onig.ts.
import langs from "@shikijs/langs@4.6.0/cpp";

import {
  arms,
  groupsCheck,
  requireGroup,
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
  applyEndPatternLast?: number;
};
type Grammar = {
  scopeName: string;
  repository: Record<string, Node>;
  patterns: Node[];
  /** Prefix of the ids of everything the grammar defines. */
  prefix: string;
};

const all = langs as unknown as Omit<Grammar, "prefix">[];
const prefixes: Record<string, string> = {
  "source.cpp": "",
  "source.cpp.embedded.macro": "Macro",
};
// The regular expression, GLSL and C grammars that raw strings embed are not
// lexed yet: an include of them adds no patterns.
const grammars = new Map<string, Grammar>();
for (const g of all) {
  const prefix = prefixes[g.scopeName];
  if (prefix === undefined) continue;
  grammars.set(g.scopeName, { ...g, repository: g.repository ?? {}, prefix });
}
const cpp = grammars.get("source.cpp")!;

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
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_()$]/)) {
      const capture = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? capture) : capture;
    }
  });
  return out;
}

const scopes = new Map<string, string>([["Source", "source.cpp"]]);
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
  return name.split(" ").map((part): Sc => {
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
    .join("")
    // The grammar's key has a misspelling that the identifiers do not keep.
    .replaceAll("Init" + "ilizer", "Initializer");

type Capture = { group: number; scopes: Sc[]; nested: string | null };
type Pattern = {
  id: string;
  regex: string;
  act: { role: "match"; name: Sc[] } | { role: "begin"; rule: string } | { role: "end" };
  captures: Capture[];
};
type Rule = {
  id: string;
  name: Sc[];
  content: Sc[];
  end: string;
  endLast: boolean;
  patterns: string[];
};

const patternById = new Map<string, Pattern>();
const rules = new Map<string, Rule>();
/** The captures that nest a pattern list, to expand their lists a second time. */
const captureSources: { capture: Capture; g: Grammar; patterns: Node[]; name: string }[] = [];
/** Nested contexts by the patterns they try, so equal ones are one rule. */
const contexts = new Map<string, string>();

function captureList(
  g: Grammar,
  captures: Captures | undefined,
  owner: string,
  regex: string,
): Capture[] {
  if (!captures) return [];
  const out: Capture[] = [];
  for (const [group, value] of Object.entries(captures)) {
    const scopes = scopeList(value.name, regex);
    let nested: string | null = null;
    if (value.patterns) {
      const list = flatten(g, value.patterns, `${owner}Capture${group}`);
      const key = list.join(",");
      let id = contexts.get(key);
      if (id === undefined) {
        id = `${owner}Capture${group}`;
        contexts.set(key, id);
        rules.set(id, { id, name: [], content: [], end: "", endLast: false, patterns: list });
      }
      nested = id;
    }
    if (scopes.length || nested) {
      const capture: Capture = { group: Number(group), scopes, nested };
      if (value.patterns) {
        captureSources.push({
          capture,
          g,
          patterns: value.patterns,
          name: `${owner}Capture${group}`,
        });
      }
      out.push(capture);
    }
  }
  return out.sort((a, b) => a.group - b.group);
}

/** The shared lists being expanded, to stop a list that includes itself. */
const expanding = new Set<string>();

/** The begin/end rules registered, to expand their patterns a second time. */
const sources: { g: Grammar; node: Node; id: string }[] = [];

/** Registers a match or begin/end rule under `id`; returns the pattern id. */
function register(g: Grammar, node: Node, id: string): string {
  const clash = patternById.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (clash) return id;
  if (node.match !== undefined) {
    patternById.set(id, {
      id,
      regex: node.match,
      act: { role: "match", name: scopeList(node.name, node.match) },
      captures: [],
    });
    patternById.get(id)!.captures = captureList(g, node.captures, id, node.match);
    return id;
  }
  if (node.begin !== undefined && node.end !== undefined) {
    patternById.set(id, { id, regex: node.begin, act: { role: "begin", rule: id }, captures: [] });
    patternById.set(`${id}End`, {
      id: `${id}End`,
      regex: node.end,
      act: { role: "end" },
      captures: [],
    });
    const rule: Rule = {
      id,
      name: scopeList(node.name, node.begin),
      content: scopeList(node.contentName, node.begin),
      end: `${id}End`,
      endLast: node.applyEndPatternLast === 1,
      patterns: [],
    };
    rules.set(id, rule);
    patternById.get(id)!.captures = captureList(
      g,
      node.beginCaptures ?? node.captures,
      id,
      node.begin,
    );
    patternById.get(`${id}End`)!.captures = captureList(
      g,
      node.endCaptures ?? node.captures,
      `${id}End`,
      node.end,
    );
    sources.push({ g, node, id });
    rule.patterns = flatten(g, node.patterns ?? [], id);
    return id;
  }
  throw new Error(`${id}: neither a match nor a begin/end rule`);
}

/** The grammar and node an `include` names, seen from grammar `g`. */
function resolve(g: Grammar, include: string): { g: Grammar; node: Node; key: string } | null {
  let target = g;
  let key: string | null = null;
  if (include === "$self") {
    key = null;
  } else if (include.startsWith("#")) {
    key = include.slice(1);
  } else {
    const [scopeName, k] = include.split("#");
    const other = grammars.get(scopeName!);
    if (!other) return null; // a grammar the oracle does not load
    target = other;
    key = k ?? null;
  }
  if (key === null) return { g: target, node: { patterns: target.patterns }, key: "Root" };
  const node = target.repository[key];
  if (!node) throw new Error(`unknown include ${include} in ${g.scopeName}`);
  return { g: target, node, key };
}

/** The pattern ids a pattern list expands to, `include`s flattened. */
function flatten(g: Grammar, list: Node[], context: string): string[] {
  const out: string[] = [];
  list.forEach((node, index) => {
    if (node.include !== undefined) {
      const found = resolve(g, node.include);
      if (found === null) return;
      out.push(...expand(found.g, found.node, found.g.prefix + pascal(found.key), true));
    } else {
      out.push(...expand(g, node, `${context}At${index}`, false));
    }
  });
  return out;
}

function expand(g: Grammar, node: Node, id: string, shared: boolean): string[] {
  if (node.match !== undefined || node.begin !== undefined) return [register(g, node, id)];
  if (node.patterns) {
    // An include of a bare list: guard against a list that includes itself.
    if (shared) {
      if (expanding.has(id)) return [];
      expanding.add(id);
    }
    const out = flatten(g, node.patterns, id);
    if (shared) expanding.delete(id);
    return out;
  }
  return []; // `{}`, which tries nothing
}

// The root comes before the rules that nest captures, so that its number
// stays small.
const root: Rule = { id: "Root", name: [], content: [], end: "", endLast: false, patterns: [] };
rules.set("Root", root);
root.patterns = flatten(cpp, cpp.patterns, "Root");

// The first pass stopped a list at a bare list that was being expanded above
// it, which left a rule or a capture's context short of patterns it includes.
// Every pattern a list can reach has its id by now, because the first pass
// visited them in grammar order, so a second pass gives each list in full
// without renaming a pattern. A rule or context that only this pass reaches
// is expanded in turn. Contexts are shared by equal lists again, and a
// context keeps the name the first pass gave it where its list is still its
// own.
const claimed = new Map<string, string>();
const fullContexts = new Map<string, Rule>();
for (let i = 0, j = 0; i < sources.length || j < captureSources.length;) {
  if (i < sources.length) {
    const { g, node, id } = sources[i++]!;
    expanding.clear();
    rules.get(id)!.patterns = flatten(g, node.patterns ?? [], id);
    continue;
  }
  const { capture, g, patterns, name } = captureSources[j++]!;
  expanding.clear();
  const list = flatten(g, patterns, name);
  const key = list.join(",");
  let rule = fullContexts.get(key);
  if (rule === undefined) {
    let id = capture.nested!;
    if (claimed.has(id)) id = `${name}X${claimed.size}`;
    claimed.set(id, key);
    rule = rules.get(id) ?? { id, name: [], content: [], end: "", endLast: false, patterns: [] };
    rule.patterns = list;
    rules.set(id, rule);
    fullContexts.set(key, rule);
  }
  capture.nested = rule.id;
}

// Output.

let g = `//! The grammar's rules as tables. Generated by \`oracle/cpp-grammar.ts\` from
//! the \`cpp\` grammar Shiki 4.6.0 bundles and the grammars it embeds; do not
//! edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::Cpp;
use super::scope::S;
use crate::scope::scopes;
use crate::rules;

pub(crate) type Act = rules::Act<Cpp>;
pub(crate) type PatternDefinition = rules::PatternDefinition<Cpp>;
pub(crate) type RuleDefinition = rules::RuleDefinition<Cpp>;
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
const patternList = [...patternById.values()];

g += `/// A rule of the grammar.
/// It is one that can be open at the end of a line, or a list of patterns
/// that tokenizes a capture group.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Rule {\n`;
for (const r of ruleList) g += `    ${r.id},\n`;
g += `}\n\nimpl Rule {\n    /// Every rule, in code order.\n    pub(crate) const ALL: [Rule; ${ruleList.length}] = [\n`;
for (const r of ruleList) g += `        Rule::${r.id},\n`;
g += `    ];\n\n${tooLong}    pub(crate) fn definition(self) -> RuleDefinition {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    `RuleDefinition {\n                name: ${scText(r.name)},\n                content: ${scText(r.content)},\n                end: ${r.end && !r.endLast ? `Some(Pattern::${r.end})` : "None"},\n                backref: None,\n                patterns: &[${[...r.patterns, ...(r.end && r.endLast ? [r.end] : [])].map((p) => `Pattern::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pattern {\n`;
const docs = (s: string) => {
  // The grammar factors the access words as `p(?:rotected|rivate|ub` + `lic)`;
  // the documentation spells the alternation out, which matches the same text.
  const one = s
    .replace(/\n/g, "\\n")
    .replaceAll("p(?:rotected|rivate|ub" + "lic)", "(?:protected|private|public)");
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
        ? `Act::Match(${scText(p.act.name)})`
        : p.act.role === "begin"
          ? `Act::Begin(Rule::${p.act.rule})`
          : "Act::End";
    const captures = p.captures
      .filter((c) => !c.nested)
      .map((c) => `(${c.group}, ${scText(c.scopes)})`)
      .join(", ");
    const nested = p.captures
      .filter((c) => c.nested)
      .map((c) => `(${c.group}, Nested { rule: Rule::${c.nested}, scopes: ${scText(c.scopes)} })`)
      .join(", ");
    return [
      p.id,
      `PatternDefinition {\n                act: ${act},\n                captures: &[${captures}],\n                nested: &[${nested}],\n            }`,
    ];
  }),
  "Pattern",
);
g += `        }\n    }\n}\n`;

// Patterns with the same regular expression share one matcher.
const rxOf = new Map<string, string>();
for (const p of patternList) if (!rxOf.has(p.regex)) rxOf.set(p.regex, p.id);
g += `\n/// A regular expression of the grammar, which several patterns can share.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Rx {\n`;
for (const [regex, id] of rxOf)
  g += `    /// \`${docs(regex).replaceAll("`", "'")}\`.\n    ${id},\n`;
g += `}\n\nimpl Rx {\n    /// Every expression, in code order.\n    #[cfg(test)]\n    pub(crate) const ALL: [Rx; ${rxOf.size}] = [\n`;
for (const id of rxOf.values()) g += `        Rx::${id},\n`;
g += `    ];

    /// The other expression of the end pattern of a rule of the macro grammar.
    /// Such an end is another expression or a line feed that no backslash
    /// precedes.
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "the 37 macro end patterns are the exception"
    )]
    pub(crate) fn macro_base(self) -> Option<Rx> {
        match self {
`;
const macroTail = "|(?=(?<!\\\\)\\n)";
const macroArms: [string, string][] = [];
for (const [regex, id] of rxOf) {
  if (!regex.endsWith(macroTail)) continue;
  const base = regex.slice(0, -macroTail.length);
  const found =
    rxOf.get(base) ??
    (base.startsWith("(?:") && base.endsWith(")") ? rxOf.get(base.slice(3, -1)) : undefined);
  if (found === undefined) throw new Error(`no base for ${id}`);
  macroArms.push([id, `Some(Rx::${found})`]);
}
g += arms(macroArms, "Rx");
g += `            _ => None,
        }
    }
}

impl Pattern {\n${tooLong}    pub(crate) fn rx(self) -> Rx {\n        match self {\n`;
g += arms(
  patternList.map((p): [string, string] => [p.id, `Rx::${rxOf.get(p.regex)}`]),
  "Pattern",
);
g += `        }\n    }\n}\n`;

g += startTable(patternList);
g += ruleCodes("Cpp", "CPP", 9);
g += groupsCheck(
  "Cpp",
  patternList.flatMap((p) => p.captures.map((c) => c.group)),
);

await writeRust(`${root_dir}src/cpp/grammar.rs`, g);

// Scope table.
const s = scopeFile("cpp-grammar.ts", "CPP", scopes, "", false);
await writeRust(`${root_dir}src/cpp/scope.rs`, s);
console.log(
  `${ruleList.length} rules (${stateful} open across lines), ${patternList.length} patterns, ${rxOf.size} expressions, ${scopes.size} scopes`,
);

// The type names that a member access may not name: the alternation in the
// negative lookahead of `MemberAccess`, spelled out.
const memberRegex = [...rxOf.keys()].find((r) => r.includes("(?!(?:uint_least32_t"));
if (memberRegex === undefined) throw new Error("no member access expression");
const typeList = /\(\?!\(\?:([^)]*)\)\\W\)/.exec(memberRegex)![1]!.split("|");
let t = `//! Words the grammar's expressions exclude, sorted. Generated by
//! \`oracle/cpp-grammar.ts\`; do not edit.

/// The type names a member access may not name.
pub(super) const TYPE_WORDS: &[&[u8]] = &[
`;
for (const w of [...new Set(typeList)].sort()) t += `    b"${w}",\n`;
t += "];\n";

// The two lists of builtin storage types, which `InlineBuiltinStorageType`
// tries in this order, sorted because a whole word is in a list or is not.
const builtinRegex = [...rxOf.keys()].find((r) => r.startsWith("\\s*+(?<!\\w)(?:(?:(?:(unsigned|"));
if (builtinRegex === undefined) throw new Error("no builtin storage type expression");
const builtin = /^\\s\*\+\(\?<!\\w\)\(\?:\(\?:\(\?:\(([^)]*)\)\|\(([^)]*)\)\)/.exec(builtinRegex)!;
for (const [name, doc, list] of [
  [
    "PRIMITIVE_TYPES",
    "The first list of builtin storage types, sorted: a word that no word character follows is in it or is not.",
    builtin[1]!,
  ],
  ["STDINT_TYPES", "The second list, of the types of the C library.", builtin[2]!],
] as const) {
  t += `\n/// ${doc}\npub(super) const ${name}: &[&[u8]] = &[\n`;
  for (const w of [...new Set(list.split("|"))].sort()) t += `    b"${w}",\n`;
  t += "];\n";
}

// The lists of words that a negative lookahead excludes, `(?!\\b(?:a|b|…)\\b)`
// and `(?!(?:a|b|…)\\b)`, which the expressions for names and types repeat.
// Each is named by a hash of its text.
const exclusions = new Map<string, string[]>();
for (const regex of rxOf.keys()) {
  for (const m of regex.matchAll(/\(\?!(?:\\b)?\(\?:([A-Za-z_0-9|]{200,})\)\\b\)/g)) {
    const list = m[1]!;
    const name = new Bun.CryptoHasher("md5").update(list).digest("hex").slice(0, 4).toUpperCase();
    exclusions.set(name, [...new Set(list.split("|"))].sort());
  }
}
// The lists in `(?<!(?:\\W|^)(?:a|b|…))`, which the function call
// expressions use.
const behind = new Map<string, string[]>();
for (const regex of rxOf.keys()) {
  for (const m of regex.matchAll(/\(\?<!\(\?:\\W\|\^\)\(\?:([A-Za-z_0-9|]{200,})\)\)/g)) {
    const list = m[1]!;
    const name = new Bun.CryptoHasher("md5").update(list).digest("hex").slice(0, 4).toUpperCase();
    behind.set(name, [...new Set(list.split("|"))].sort());
  }
}
// The predefined macros of `PredefinedMacrosAt0`, in the order it tries them.
const predefinedRegex = [...rxOf.keys()].find((r) => r.startsWith("\\b(__cplusplus|"));
if (predefinedRegex === undefined) throw new Error("no predefined macro expression");
// `__FP_FAST_FMAF??` is a lazy optional letter: with the closing `\b`, both spellings match whole words.
const predefined = /^\\b\(([A-Za-z_0-9|?]+)\)\\b$/
  .exec(predefinedRegex)![1]!
  .split("|")
  .flatMap((w) => (w.endsWith("??") ? [w.slice(0, -3), w.slice(0, -2)] : [w]));
t +=
  "\n/// The predefined macros, in the order the expression tries them.\npub(super) const PREDEFINED: &[&[u8]] = &[\n";
for (const w of predefined) t += `    b"${w}",\n`;
t += "];\n";

t += "\n// The lists below are named by a hash of the alternation they come from.\n";
for (const [name, words] of [...behind].sort()) {
  t += `\n/// A list of ${words.length} words that a call may not end.\npub(super) const BEHIND_${name}: &[&[u8]] = &[\n`;
  for (const w of words) t += `    b"${w}",\n`;
  t += "];\n";
}
for (const [name, words] of [...exclusions].sort()) {
  t += `\n/// A list of ${words.length} excluded words.\npub(super) const NOT_${name}: &[&[u8]] = &[\n`;
  for (const w of words) t += `    b"${w}",\n`;
  t += "];\n";
}
await writeRust(`${root_dir}src/cpp/types.rs`, t);

if (process.env["RX_DUMP"]) {
  const lines = [...rxOf].map(([regex, id]) => {
    const groups = new Set<number>();
    for (const p of patternList)
      if (p.regex === regex) for (const c of p.captures) groups.add(c.group);
    return JSON.stringify({ id, regex, groups: [...groups].sort((a, b) => a - b) });
  });
  await Bun.write(process.env["RX_DUMP"], lines.join("\n") + "\n");
}
