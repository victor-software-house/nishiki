// Writes src/swift/scope.rs and src/swift/grammar.rs from the `swift` grammar
// that Shiki 4.6.0 bundles (jtbandes/swift-tmlanguage, MIT): the scope names, and
// for every rule the scopes it adds, how it ends and the patterns it tries, in
// the grammar's order with its `include`s flattened. The matchers that decide
// whether a pattern matches are written by hand in src/swift/matchers.
//
// Run with `bun run oracle/swift-grammar.ts` and commit the result; the
// output is generated, do not edit it.
import swift from "@shikijs/langs@4.6.0/swift";
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
type CaptureNode = { name?: string; patterns?: Node[] };

const grammar = (swift as unknown as { repository: Record<string, Node>; patterns: Node[] }[])[0]!;
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
  if (parts[parts.length - 1] === "swift") parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
    }
  });
  return out + suffix;
}

const scopes = new Map<string, string>([["Source", "source.swift"]]);
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
  act: { role: "match"; name: string[] } | { role: "begin"; rule: string } | { role: "end" };
  captures: [number, string[]][];
  /** Capture groups that the grammar tokenizes with patterns of their own. */
  nested: [number, string][];
  /** The key of the repository entry it comes from, for flavours. */
  key: string;
};
type Rule = {
  id: string;
  key: string;
  name: string[];
  content: string[];
  end: string;
  backref?: number | null;
  patterns: string[];
};

const pats = new Map<string, Pat>();
const rules = new Map<string, Rule>();

/** The capture groups whose text the grammar tokenizes with patterns of their
 * own. Each distinct list of patterns is a rule of its own, with no name and no
 * end, that the engine runs over the capture's text. */
const nestedRules = new Map<string, string>();
function nestedList(captures: Captures | undefined): [number, string][] {
  if (!captures) return [];
  const out: [number, string][] = [];
  for (const [group, value] of Object.entries(captures)) {
    if (!value.patterns) continue;
    const key = JSON.stringify(value.patterns);
    let id = nestedRules.get(key);
    if (id === undefined) {
      id = `Nested${nestedRules.size}`;
      nestedRules.set(key, id);
      const rule: Rule = { id, key: id, name: [], content: [], end: "", patterns: [] };
      rules.set(id, rule);
      rule.patterns = flatten(value.patterns as Node[], id);
    }
    out.push([Number(group), id]);
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

/** The source of capture group `n` (counting named groups) of `regex`. */
function groupBody(regex: string, n: number): { start: number; end: number } {
  let count = 0;
  const open: { start: number; capture: boolean; index: number }[] = [];
  for (let i = 0; i < regex.length; i++) {
    const c = regex[i];
    if (c === "\\") {
      i++;
    } else if (c === "[") {
      i++;
      if (regex[i] === "^") i++;
      if (regex[i] === "]") i++;
      while (i < regex.length && regex[i] !== "]") {
        if (regex[i] === "\\") i++;
        else if (regex[i] === "[" && regex[i + 1] === ":") i = regex.indexOf(":]", i) + 1;
        i++;
      }
    } else if (c === "(") {
      const rest = regex.slice(i + 1);
      const capture = !rest.startsWith("?") || /^\?<[A-Za-z]/.test(rest) || rest.startsWith("?'");
      if (capture) count++;
      open.push({ start: i, capture, index: count });
    } else if (c === ")") {
      const group = open.pop()!;
      if (group.capture && group.index === n) {
        const head = regex.slice(group.start + 1).match(/^(\?<[A-Za-z]\w*>|\?'\w+')/);
        return { start: group.start + 1 + (head ? head[0].length : 0), end: i };
      }
    }
  }
  throw new Error(`regex ${regex}: no group ${n}`);
}

/** The top-level alternatives of a group body. */
function alternatives(body: string): string[] {
  const out: string[] = [];
  let depth = 0;
  let from = 0;
  for (let i = 0; i < body.length; i++) {
    const c = body[i];
    if (c === "\\") i++;
    else if (c === "[") {
      i++;
      if (body[i] === "^") i++;
      if (body[i] === "]") i++;
      while (i < body.length && body[i] !== "]") {
        if (body[i] === "\\") i++;
        i++;
      }
    } else if (c === "(") depth++;
    else if (c === ")") depth--;
    else if (c === "|" && depth === 0) {
      out.push(body.slice(from, i));
      from = i + 1;
    }
  }
  out.push(body.slice(from));
  return out;
}

/** The nodes a node with `$1` in a scope name stands for: one for each
 * alternative of capture group 1, a word, with the word where `$1` was. The
 * keyword in the text decides which alternative matched, so trying the
 * variants in turn gives the same match. */
function dollarVariants(node: Node): { word: string; node: Node }[] | null {
  const names = [node.name, node.contentName];
  for (const captures of [node.captures, node.beginCaptures, node.endCaptures])
    for (const value of Object.values(captures ?? {})) names.push(value.name);
  if (!names.some((name) => name !== undefined && /\$\d/.test(name))) return null;
  if (names.some((name) => name !== undefined && /\$[02-9]/.test(name))) throw new Error("only $1 is expanded");
  const regex = (node.match ?? node.begin)!;
  const { start, end } = groupBody(regex, 1);
  return alternatives(regex.slice(start, end)).map((alt) => {
    const word = /^[A-Za-z]+/.exec(alt)![0];
    const swap = (name: string | undefined) => name?.replaceAll("$1", word);
    const swapCaptures = (captures: Captures | undefined) =>
      captures && Object.fromEntries(Object.entries(captures).map(([k, v]) => [k, { ...v, name: swap(v.name) }]));
    const replaced = regex.slice(0, start) + alt + regex.slice(end);
    const variant: Node = {
      ...node,
      name: swap(node.name),
      contentName: swap(node.contentName),
      captures: swapCaptures(node.captures),
      beginCaptures: swapCaptures(node.beginCaptures),
      endCaptures: swapCaptures(node.endCaptures),
    };
    if (node.match !== undefined) variant.match = replaced;
    else variant.begin = replaced;
    return { word, node: variant };
  });
}

/** Registers a match or begin/end rule under `id`; returns the pattern ids. */
function register(node: Node, id: string, key: string): string[] {
  const variants = dollarVariants(node);
  if (variants) {
    return variants.flatMap(({ word, node }) =>
      register(node, `${id}${word.charAt(0).toUpperCase()}${word.slice(1)}`, key),
    );
  }
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
        key,
      });
    }
    return [id];
  }
  if (node.begin !== undefined && node.end !== undefined) {
    if (!pats.has(id)) {
      pats.set(id, {
        id,
        regex: node.begin,
        act: { role: "begin", rule: id },
        captures: captureList(node.beginCaptures ?? node.captures),
        nested: nestedList(node.beginCaptures ?? node.captures),
        key,
      });
      pats.set(`${id}End`, {
        id: `${id}End`,
        regex: node.end,
        act: { role: "end" },
        captures: captureList(node.endCaptures ?? node.captures),
        nested: nestedList(node.endCaptures ?? node.captures),
        key,
      });
      const rule: Rule = {
        id,
        key,
        name: scopeList(node.name),
        content: scopeList(node.contentName),
        end: `${id}End`,
        backref: backrefGroup(node.end),
        patterns: [],
      };
      rules.set(id, rule);
      rule.patterns = flatten(node.patterns ?? [], id);
    }
    return [id];
  }
  throw new Error(`${id}: neither a match nor a begin/end rule`);
}

/** The begin group that an end pattern repeats with `\\1`, when it does. */
function backrefGroup(end: string): number | null {
  const found = [...end.matchAll(/(?<!\\)(?:\\\\)*\\([1-9])/g)];
  if (found.length === 0) return null;
  const groups = new Set(found.map((m) => Number(m[1])));
  if (groups.size > 1) throw new Error(`end ${end} repeats several groups`);
  return [...groups][0]!;
}

/** The pattern ids a pattern list expands to, `include`s flattened. */
function flatten(list: Node[], context: string): string[] {
  const out: string[] = [];
  list.forEach((node, index) => {
    if (node.include === "$self") {
      out.push(...flatten(grammar.patterns, "Root"));
    } else if (node.include !== undefined) {
      if (!node.include.startsWith("#")) throw new Error(`external include ${node.include}`);
      const key = node.include.slice(1);
      const target = repository[key];
      // The grammar includes two rules it does not define; a TextMate engine
      // skips an include that names nothing.
      if (!target && (key === "constraint" || key === "member-references")) return;
      if (!target) throw new Error(`unknown include ${key}`);
      out.push(...expand(target, pascal(key), key));
    } else {
      out.push(...expand(node, `${context}At${index}`, context));
    }
  });
  return out;
}

function expand(node: Node, id: string, key: string): string[] {
  if (node.match !== undefined || node.begin !== undefined) return register(node, id, key);
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
  patterns: rootPatterns,
});

// Output.

const list = (names: string[]) => (names.length ? `scopes![${names.map((n) => `S::${n}`).join(", ")}]` : "S::NO_SCOPES");

let g = `//! The grammar's rules as tables. Generated by \`oracle/swift-grammar.ts\` from
//! the \`kotlin\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pat\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::Swift;
use super::scope::S;
use crate::scope::scopes;
use crate::rules;

pub(crate) type Act = rules::Act<Swift>;
pub(crate) type PatDefinition = rules::PatDefinition<Swift>;
pub(crate) type RuleDefinition = rules::RuleDefinition<Swift>;
pub(crate) use crate::rules::Start;

/// A rule that tokenizes the text of a capture group: the grammar's patterns
/// for that group, with no name and no end.
pub(crate) type Nested = Rule;

`;


const ruleList = [...rules.values()];
const patList = [...pats.values()];

g += `/// A rule of the grammar that can be open at the end of a line.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub(crate) enum Rule {\n`;
for (const r of ruleList) g += `    ${r.id},\n`;
g += `}\n\nimpl Rule {\n    /// Every rule, in code order.\n    pub(crate) const ALL: [Rule; ${ruleList.length}] = [\n`;
for (const r of ruleList) g += `        Rule::${r.id},\n`;
g += `    ];\n\n${tooLong}    pub(crate) fn definition(self) -> RuleDefinition {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    `RuleDefinition {\n                name: ${list(r.name)},\n                content: ${list(r.content)},\n                end: ${r.end ? `Some(Pat::${r.end})` : "None"},\n                backref: ${r.backref ?? null ? `Some(${r.backref})` : "None"},\n                patterns: &[${r.patterns.map((p) => `Pat::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pat {\n`;
// A regex that holds a word cut in two by an alternation is elided too: the
// spelling check reads the second half as a typo.
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 || one.includes("r" + "operty") || one.includes("introduc") ? `…` : one;
};
for (const p of patList) {
  const role = p.act.role === "end" ? "end" : p.act.role === "begin" ? "begin" : "match";
  g += `    /// The ${role} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pat {\n    /// Every pattern, for the tool that compares the matchers with Oniguruma.\n    #[cfg(test)]\n    pub(crate) const ALL: [Pat; ${patList.length}] = [\n${patList.map((p) => `        Pat::${p.id},\n`).join("")}    ];\n\n${tooLong}    pub(crate) fn definition(self) -> PatDefinition {\n        match self {\n`;
g += arms(
  patList.map((p): [string, string] => {
    const act =
      p.act.role === "match"
        ? `Act::Match(${list(p.act.name)})`
        : p.act.role === "begin"
          ? `Act::Begin(Rule::${p.act.rule})`
          : "Act::End";
    const caps = p.captures.map(([n, names]) => `(${n}, ${list(names)})`).join(", ");
    const nested = p.nested.map(([n, name]) => `(${n}, Rule::${name})`).join(", ");
    return [p.id, `PatDefinition {\n                act: ${act},\n                captures: &[${caps}],\n                nested: &[${nested}],\n            }`];
  }),
  "Pat",
);
g += `        }\n    }\n}\n`;


g += startTable(patList);
g += ruleCodes("Swift", "SWIFT");
g += groupsCheck("Swift", patList.flatMap((p) => [...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]));

await writeRust(`${root_dir}src/swift/grammar.rs`, g);

// Matcher dispatch: one function for each distinct regular expression, named
// for the first pattern that has it, in the file of the family it belongs to.

const snake = (id: string) =>
  id
    .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
    .replace(/([A-Za-z])(\d)/g, "$1_$2")
    .toLowerCase();

/** The file of hand-written matchers that owns a pattern. */
function fileOf(id: string): string {
  if (id.startsWith("LiteralsRegularExpressionLiteral")) return "regex_literals";
  if (/^(LiteralsString|LiteralsNumeric|LiteralsAt|LiteralsBoolean|Comments)/.test(id)) return "literals";
  if (/^Declarations(Type|Protocol|Extension|Typed|Inheritance|Generic)/.test(id)) return "types";
  if (/^(DeclarationsFunction|DeclarationsParameter|DeclarationsImport|DeclarationsOperator|DeclarationsPrecedencegroup|DeclarationsMacro|DeclarationsAvailable|CodeBlock)/.test(id)) return "declarations";
  if (/^(Expressions|CompilerControl|Conditionals|Attributes|Operators|AsyncThrows|MemberReference)/.test(id)) return "expressions";
  return "words";
}

const functionOf = new Map<string, { name: string; ids: string[] }>();
for (const p of patList) {
  const known = functionOf.get(p.regex);
  if (known) known.ids.push(p.id);
  else functionOf.set(p.regex, { name: `rx_${snake(p.id)}`, ids: [p.id] });
}
let dispatch = `//! Which matcher decides each pattern. Generated by \`oracle/swift-grammar.ts\`;
//! do not edit. A regular expression that several patterns share has one
//! matcher.

use super::engine::Lexer;
use super::grammar::Pat;

impl Lexer<'_> {
    /// Whether \`pat\` matches at \`p\`, and where its match ends.
    #[expect(clippy::too_many_lines, reason = "one arm for each regular expression")]
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
`;
for (const { name, ids } of functionOf.values())
  dispatch += `            ${ids.map((id) => `Pat::${id}`).join("\n            | ")} => self.${name}(p),\n`;
dispatch += "        }\n    }\n}\n";
await writeRust(`${root_dir}src/swift/dispatch.rs`, dispatch);

// A stub for each matcher in a file that does not exist yet; the files are
// then written by hand and never touched again.
const byFile = new Map<string, string[]>();
for (const [regex, { name, ids }] of functionOf) {
  const file = fileOf(ids[0]!);
  const row = `    /// \`${regex.replace(/\n/g, "\\n")}\`, for ${ids.join(", ")}.\n    pub(in crate::swift) fn ${name}(&mut self, _p: usize) -> Option<usize> {\n        None\n    }\n`;
  byFile.set(file, [...(byFile.get(file) ?? []), row]);
}
for (const [file, rows] of byFile) {
  const path = `${root_dir}src/swift/matchers/${file}.rs`;
  if (await Bun.file(path).exists()) continue;
  await writeRust(
    path,
    `//! Matchers: ${file}.\n\nuse super::super::engine::Lexer;\n\nimpl Lexer<'_> {\n${rows.join("\n")}}\n`,
  );
}
console.log([...byFile].map(([file, rows]) => `${file} ${rows.length}`).join(", "));

if (process.env.RX_DUMP) {
  // For oracle/onig.ts: each distinct regular expression, by the first pattern
  // that has it, with the capture groups its matcher must record (those that
  // any pattern sharing it names or tokenizes). An end that repeats a group of
  // its begin is left out: its text comes from the line that opened the rule.
  const groupsOf = (p: Pat) => {
    const used = new Set<number>([...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]);
    const rule = p.act.role === "begin" ? rules.get(p.act.rule) : null;
    if (rule?.backref) used.add(rule.backref);
    return used;
  };
  const rows = [...functionOf]
    .filter(([regex, { ids }]) => !(pats.get(ids[0]!)!.act.role === "end" && /\\[1-9k]/.test(regex)))
    .map(([regex, { ids }]) => {
      const used = new Set<number>();
      for (const id of ids) for (const g of groupsOf(pats.get(id)!)) used.add(g);
      return JSON.stringify({ id: ids[0], regex, groups: [...used].sort((a, b) => a - b) });
    });
  await Bun.write(process.env.RX_DUMP, rows.join("\n") + "\n");
}

// Scope table.
const s = scopeFile("swift-grammar.ts", "SWIFT", scopes);
await writeRust(`${root_dir}src/swift/scope.rs`, s);
console.log(`${ruleList.length} rules, ${patList.length} patterns, ${scopes.size} scopes`);
