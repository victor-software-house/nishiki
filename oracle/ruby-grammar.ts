// Writes src/ruby/scope.rs and src/ruby/grammar.rs from the `ruby` grammar
// that Shiki 4.6.0 bundles (Shopify/ruby-lsp through microsoft/vscode, MIT): the scope names, and
// for every rule the scopes it adds, how it ends and the patterns it tries, in
// the grammar's order with its `include`s flattened. The matchers that decide
// whether a pattern matches are written by hand in src/ruby/matchers.
//
// Run with `bun run oracle/ruby-grammar.ts` and commit the result; the
// output is generated, do not edit it.
import ruby from "@shikijs/langs@4.6.0/ruby";
import { arms, groupsCheck, scopeFile, startTable, tooLong } from "./rule-grammar.ts";

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
  while?: string;
  repository?: Record<string, Node>;
};
type CaptureNode = { name?: string; patterns?: Node[] };

const grammar = (ruby as unknown as { name: string; repository: Record<string, Node>; patterns: Node[] }[]).find(
  (lang) => lang.name === "ruby",
)!;
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
  if (parts[parts.length - 1] === "ruby") parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
    }
  });
  return out + suffix;
}

const scopes = new Map<string, string>([["Source", "source.ruby"]]);
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
    .split(/[-_.]/)
    .map((piece) => piece.charAt(0).toUpperCase() + piece.slice(1))
    .join("");

type Pat = {
  id: string;
  regex: string;
  act: { kind: "match"; name: string[] } | { kind: "begin"; rule: string } | { kind: "end" };
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
  /** The begin capture group that the end pattern repeats, when it does. */
  backref: number | null;
  patterns: string[];
};

const pats = new Map<string, Pat>();
const rules = new Map<string, Rule>();

/** The rule that tokenizes the text of a capture group with the patterns `list`, one per distinct list. */
const nestedRules = new Map<string, string>();
/** The repositories an `include` resolves in, innermost first: a rule may carry its own. */
type Repos = Record<string, Node>[];

function nestedRule(list: Node[], repos: Repos): string {
  const key = JSON.stringify(list);
  const known = nestedRules.get(key);
  if (known !== undefined) return known;
  const id = `Nested${nestedRules.size}`;
  nestedRules.set(key, id);
  const rule: Rule = { id, key: id, name: [], content: [], end: "", backref: null, patterns: [] };
  rules.set(id, rule);
  rule.patterns = flatten(list, id, repos);
  return id;
}

/** The capture groups whose text the grammar tokenizes with patterns, with the rule that does. */
function nestedList(captures: Captures | undefined, repos: Repos): [number, string][] {
  if (!captures) return [];
  const out: [number, string][] = [];
  for (const [group, value] of Object.entries(captures)) {
    if (value.patterns === undefined) continue;
    out.push([Number(group), nestedRule(value.patterns as Node[], repos)]);
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
function register(node: Node, id: string, key: string, repos: Repos): string {
  const clash = pats.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (node.match !== undefined) {
    if (!pats.has(id)) {
      pats.set(id, {
        id,
        regex: node.match,
        act: { kind: "match", name: scopeList(node.name) },
        captures: captureList(node.captures),
        nested: nestedList(node.captures, repos),
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
        nested: nestedList(node.beginCaptures ?? node.captures, repos),
        key,
      });
      pats.set(`${id}End`, {
        id: `${id}End`,
        regex: node.end,
        act: { kind: "end" },
        captures: captureList(node.endCaptures ?? node.captures),
        nested: nestedList(node.endCaptures ?? node.captures, repos),
        key,
      });
      const backref = /\\([1-9])/.exec(node.end);
      const rule: Rule = {
        id,
        key,
        name: scopeList(node.name),
        content: scopeList(node.contentName),
        end: `${id}End`,
        backref: backref ? Number(backref[1]) : null,
        patterns: [],
      };
      rules.set(id, rule);
      rule.patterns = flatten(node.patterns ?? [], id, node.repository ? [node.repository, ...repos] : repos);
    }
    return id;
  }
  throw new Error(`${id}: neither a match nor a begin/end rule`);
}

/** Whether `node` is one of the grammar's heredocs that embed another
 * grammar through a `while` rule. They wait for the engine's shared embedding
 * hook; until then such a heredoc is read by the plain heredoc rules. */
function embedsGrammar(node: Node): boolean {
  return (node.patterns ?? []).some((inner) => inner.while !== undefined);
}

/** The pattern ids a pattern list expands to, `include`s flattened. */
function flatten(list: Node[], context: string, repos: Repos): string[] {
  const out: string[] = [];
  list.forEach((node, index) => {
    if (node.include === "$self" || node.include === "source.ruby") {
      out.push(...flatten(root.patterns!, "Root", [repository]));
    } else if (node.include !== undefined && !node.include.startsWith("#")) {
      // Another grammar (HTML after `__END__`): waits for the embedding hook.
    } else if (node.include !== undefined) {
      const key = node.include.slice(1);
      const target = repos.map((repo) => repo[key]).find((found) => found !== undefined);
      if (!target) throw new Error(`unknown include ${key}`);
      out.push(...expand(target, pascal(key), key, repos));
    } else if (!embedsGrammar(node)) {
      out.push(...expand(node, `${context}At${index}`, context, repos));
    }
  });
  return out;
}

function expand(node: Node, id: string, key: string, repos: Repos): string[] {
  const inner = node.repository ? [node.repository, ...repos] : repos;
  if (node.match !== undefined || node.begin !== undefined) return [register(node, id, key, inner)];
  if (node.patterns) return flatten(node.patterns, id, inner);
  throw new Error(`${id}: empty rule`);
}

const rootPatterns = flatten(root.patterns!, "Root", [repository]);
rules.set("Root", {
  id: "Root",
  key: "",
  name: [],
  content: [],
  end: "",
  backref: null,
  patterns: rootPatterns,
});

// Output.

const list = (names: string[]) => (names.length ? `&[${names.map((n) => `S::${n}`).join(", ")}]` : "&[]");

let g = `//! The grammar's rules as tables. Generated by \`oracle/ruby-grammar.ts\` from
//! the \`ruby\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pat\`] is one regular expression of the grammar; its doc comment says
//! which. Each rule lists the patterns it tries, in the grammar's order with
//! its \`include\`s flattened, so the first that matches at the earliest
//! position wins as in a \`TextMate\` engine.

use super::engine::Ruby;
use super::scope::S;
use crate::rules;

pub(crate) type Act = rules::Act<Ruby>;
pub(crate) type PatData = rules::PatData<Ruby>;
pub(crate) type RuleData = rules::RuleData<Ruby>;
pub(crate) use crate::rules::Start;

/// The rule that tokenizes a capture group.
pub(crate) type Nested = Rule;

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
    `RuleData {\n                name: ${list(r.name)},\n                content: ${list(r.content)},\n                end: ${r.end ? `Some(Pat::${r.end})` : "None"},\n                backref: ${r.backref === null ? "None" : `Some(${r.backref})`},\n                patterns: &[${r.patterns.map((p) => `Pat::${p}`).join(", ")}],\n            }`,
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
    const nested = p.nested.map(([n, kind]) => `(${n}, Rule::${kind})`).join(", ");
    return [p.id, `PatData {\n                act: ${act},\n                captures: &[${caps}],\n                nested: &[${nested}],\n            }`];
  }),
  "Pat",
);
g += `        }\n    }\n}\n`;

g += startTable(patList, "any");
g += groupsCheck("Ruby", patList.flatMap((p) => [...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]));

await Bun.write(`${root_dir}src/ruby/grammar.rs`, g);

const s = scopeFile("ruby-grammar.ts", "RUBY", scopes);
await Bun.write(`${root_dir}src/ruby/scope.rs`, s);
console.log(`${ruleList.length} rules, ${patList.length} patterns, ${scopes.size} scopes`);
