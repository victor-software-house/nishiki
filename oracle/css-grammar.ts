// Writes src/css/scope.rs and src/css/grammar.rs from the `css` grammar
// that Shiki 4.6.0 bundles (microsoft/vscode, MIT): the scope names, and
// for every rule the scopes it adds, how it ends and the patterns it tries, in
// the grammar's order with its `include`s flattened. The matchers that decide
// whether a pattern matches are written by hand in src/css/matchers.
//
// Run with `bun run oracle/css-grammar.ts` and commit the result; the
// output is generated, do not edit it.
import css from "@shikijs/langs@4.6.0/css";
import { arms, groupsCheck, ruleCodes, scopeFile, startTable, tooLong, writeRust } from "./rule-grammar.ts";

type Captures = Record<string, { name?: string; patterns?: unknown[] }>;
export type Node = {
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

const grammar = (css as unknown as { repository: Record<string, Node>; patterns: Node[] }[])[0]!;
export const repository = grammar.repository;

/** Replaces `${n:/downcase}` in a scope name. */
const downcase = (name: string | undefined, group: number, word: string) =>
  name?.replace(`\${${group}:/downcase}`, word.toLowerCase());

/**
 * The grammar names a scope after the text a group matched, as in
 * \`keyword.other.unit.${2:/downcase}.css\`; a scope is a fixed name here, so a
 * pattern that does so becomes one pattern for each alternative of the group,
 * each with its own name. The expansion keeps the grammar's order, and so what
 * wins where more than one alternative matches.
 */
function expandDynamic(): void {
  const fonts = repository["font-features"]!;
  const begin = fonts.begin!;
  const alternatives = /\(\(@\)\(([^)]*)\)\)/.exec(begin)![1]!.split("|");
  const template = { ...fonts };
  // One entry for each at-rule keyword, listed as the grammar's single entry.
  repository["font-features"] = {
    patterns: alternatives.map((word) => ({
      ...template,
      begin: begin.replace(`(${alternatives.join("|")})`, `(${word})`),
      beginCaptures: {
        "1": { name: downcase(template.beginCaptures!["1"]!.name, 3, word)! },
        "2": template.beginCaptures!["2"]!,
      },
      name: downcase(template.name, 3, word),
      patterns: (JSON.parse(JSON.stringify(template)) as Node).patterns,
    })),
  };
  const numeric = repository["numeric-values"]!.patterns![1]!;
  const regex = numeric.match!;
  const marker = "(?:(%)|(";
  const at = regex.indexOf(marker);
  const prefix = regex.slice(0, at);
  const units = regex.slice(at + marker.length, regex.lastIndexOf(")\\b)?")).split("|");
  const unitScope = (word: string) => `keyword.other.unit.${word.toLowerCase()}.css`;
  repository["numeric-values"]!.patterns = [
    repository["numeric-values"]!.patterns![0]!,
    { match: `${prefix}(?:(%))`, name: numeric.name, captures: { "1": { name: "keyword.other.unit.percentage.css" } } },
    ...units.map((word) => ({
      match: `${prefix}(?:(${word})\\b)`,
      name: numeric.name,
      captures: { "1": { name: unitScope(word) } },
    })),
    { match: prefix, name: numeric.name },
  ];
}
expandDynamic();

/** The first capturing group of `regex`, outside character classes: where it starts and ends, and what is inside. */
function firstGroup(regex: string): { start: number; end: number; body: string } {
  let inClass = false;
  for (let i = 0; i < regex.length; i++) {
    const c = regex[i]!;
    if (c === "\\") i++;
    else if (inClass) inClass = c !== "]";
    else if (c === "[") inClass = true;
    else if (c === "(" && regex[i + 1] !== "?") {
      const end = regex.indexOf(")", i);
      return { start: i, end: end + 1, body: regex.slice(i + 1, end) };
    }
  }
  throw new Error(`no group in ${regex}`);
}

/** A pattern named after its first group (`$1`) becomes one pattern per alternative. */
function expandBackrefs(list: Node[]): Node[] {
  return list.flatMap((node) => {
    if (node.patterns) node.patterns = expandBackrefs(node.patterns);
    if (node.match === undefined || !node.name?.includes("$1")) return [node];
    const group = firstGroup(node.match);
    return group.body.split("|").map((word) => ({
      ...node,
      match: `${node.match!.slice(0, group.start)}(${word})${node.match!.slice(group.end)}`,
      name: node.name!.replace("$1", word),
    }));
  });
}
for (const entry of Object.values(repository)) if (entry.patterns) entry.patterns = expandBackrefs(entry.patterns);
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
  if (parts[parts.length - 1] === "css") parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
    }
  });
  return out + suffix;
}

export const scopes = new Map<string, string>([["Source", "source.css"]]);
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
export const pascal = (key: string) =>
  key
    .split(/[-_.]/)
    .map((piece) => piece.charAt(0).toUpperCase() + piece.slice(1))
    .join("");

export type Pat = {
  id: string;
  regex: string;
  act: { kind: "match"; name: string[] } | { kind: "begin"; rule: string } | { kind: "end" };
  captures: [number, string[]][];
  /** Capture groups that the grammar tokenizes with patterns of their own. */
  nested: [number, string][];
  /** The key of the repository entry it comes from, for flavours. */
  key: string;
};
export type Rule = {
  id: string;
  key: string;
  name: string[];
  content: string[];
  end: string;
  patterns: string[];
};

export const pats = new Map<string, Pat>();
export const rules = new Map<string, Rule>();

/** The capture groups whose text the grammar tokenizes with patterns, by kind. */
function nestedList(captures: Captures | undefined): [number, string][] {
  if (!captures) return [];
  const out: [number, string][] = [];
  for (const [group, value] of Object.entries(captures)) {
    const first = (value.patterns?.[0] ?? null) as { match?: string; include?: string } | null;
    if (first === null) continue;
    const kind =
      first.include === "#escapes" ? "Escapes" : first.include === "#comment-block" ? "CommentBlock" : null;
    if (kind === null) throw new Error(`capture ${group}: unknown nested patterns`);
    out.push([Number(group), kind]);
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
        nested: nestedList(node.captures),
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
        nested: nestedList(node.beginCaptures ?? node.captures),
        key,
      });
      pats.set(`${id}End`, {
        id: `${id}End`,
        regex: node.end,
        act: { kind: "end" },
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
      out.push(...expand(target, pascal(key), key));
    } else {
      out.push(...expand(node, `${context}At${index}`, context));
    }
  });
  return out;
}

export function expand(node: Node, id: string, key: string): string[] {
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
  patterns: rootPatterns,
});

// Output, when run rather than imported by another generator.
if (import.meta.main) {
// Output.

const list = (names: string[]) => (names.length ? `&[${names.map((n) => `S::${n}`).join(", ")}]` : "&[]");

let g = `//! The grammar's rules as tables. Generated by \`oracle/css-grammar.ts\` from
//! the \`css\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pat\`] is one regular expression of the grammar; its doc comment says
//! which. Each rule lists the patterns it tries, in the grammar's order with
//! its \`include\`s flattened, so the first that matches at the earliest
//! position wins as in a \`TextMate\` engine.

use super::engine::Css;
use super::scope::S;
use crate::rules;

pub(crate) type Act = rules::Act<Css>;
pub(crate) type PatData = rules::PatData<Css>;
pub(crate) type RuleData = rules::RuleData<Css>;
pub(crate) use crate::rules::Start;

/// The patterns a capture group is tokenized with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Nested {
    /// The escapes of a name.
    Escapes,
    /// A block comment.
    CommentBlock,
}

`;

/** Match arms of \`type::Id\`s, with the ids that share a body joined by \`|\`. */
const ruleList = [...rules.values()];
const patList = [...pats.values()];

g += `/// A rule of the grammar that can be open at the end of a line.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u8)]\n#[expect(\n    clippy::enum_variant_names,\n    reason = "the names are the grammar's rule names"\n)]\npub(crate) enum Rule {\n`;
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
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 ? `…` : one;
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
    const nested = p.nested.map(([n, kind]) => `(${n}, Nested::${kind})`).join(", ");
    return [p.id, `PatData {\n                act: ${act},\n                captures: &[${caps}],\n                nested: &[${nested}],\n            }`];
  }),
  "Pat",
);
g += `        }\n    }\n}\n`;


g += startTable(patList);
g += ruleCodes("Css", "CSS");
g += groupsCheck("Css", patList.flatMap((p) => [...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]));

await writeRust(`${root_dir}src/css/grammar.rs`, g);

// Scope table.
const s = scopeFile("css-grammar.ts", "CSS", scopes, `    #[expect(
        clippy::enum_variant_names,
        reason = "the names are the scope names, and the unit s ends in S"
    )]
`);
await writeRust(`${root_dir}src/css/scope.rs`, s);
if (Bun.env.CSS_DUMP) await Bun.write(Bun.env.CSS_DUMP, JSON.stringify(patList.map((p) => ({ id: p.id, kind: p.act.kind, regex: p.regex, captures: p.captures, nested: p.nested })), null, 0));
console.log(`${ruleList.length} rules, ${patList.length} patterns, ${scopes.size} scopes`);
}
