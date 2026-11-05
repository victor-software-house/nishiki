// The generator of the rule-engine languages whose grammars nest repositories and share expressions:
// Objective-C (`oracle/objective-c-grammar.ts`) and Objective-C++ (`oracle/objective-cpp-grammar.ts`).
// For a grammar Shiki bundles, `generate` writes the module's `scope.rs`, `grammar.rs` and `words.rs`.
// It writes the scope names.
// For every rule it writes the scopes the rule adds, how it ends and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// The matchers that decide whether a pattern matches are written by hand.
//
// An entry that carries a repository of its own resolves an include inside it there first, as vscode-textmate's does.
// `$base` and `$self` name the grammar's own root.
// A `Config` says what differs between the grammars.
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
export type GrammarJson = { scopeName: string; repository: Record<string, Node>; patterns: Node[] };

/** The repository an include resolves in, and the prefix of the ids of its own entries. */
type Scope = { repository: Record<string, Node>; own: Record<string, Node>; prefix: string };

/** One list of words of a long alternation. */
export type WordList = { name: string; doc: string; body: string };

/** Reads the text of a pattern's expression, for the word lists. */
export type Reader = {
  /** The text between `open` and the first `close` after it in the expression of `id`. */
  between(id: string, open: string, close: string): string;
  /** The whole expression of `id` between `head` and `tail`, which it must start and end with. */
  inside(id: string, head: string, tail: string): string;
};

/** What differs between the grammars this generator reads. */
export type Config = {
  /** The grammar Shiki bundles. */
  grammar: GrammarJson;
  /** The generator's file, as the output names it. */
  generator: string;
  /** The language id, as the module docs name the grammar. */
  id: string;
  /** The last part of every scope name, such as `objc`. */
  suffix: string;
  /** The module directory under src/. */
  dir: string;
  /** The grammar's type in the module's engine. */
  engine: string;
  /** The grammar's table name in `scope_table!` and its frame tag constant. */
  table: string;
  /** The prefix of the ids of the entries of a nested repository, by its key. */
  prefixes: Record<string, string>;
  /** The scope names whose `$N` names a group the expression lacks.
   * vscode-textmate leaves such a `$N` as written, so these become fixed scopes. */
  literal: string[];
  /** The expressions another module's matchers answer, with that module's `Rx` id.
   * The generated `Pat::expr` sends them there; the rest are this module's own `Rx`. */
  shared?: { path: string; ids: Map<string, string> };
  /** The begin capture group that a raw string's end repeats, set by the matcher. */
  rawDelimiter?: { end: string; group: number };
  /** The word lists, read from the expressions. */
  words(read: Reader): WordList[];
};

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

const pascal = (key: string) =>
  key
    .split(/[-_.]/)
    .map((piece) => piece.charAt(0).toUpperCase() + piece.slice(1))
    .join("");

/** Writes the module's grammar, scope and word files for `cfg`.
 * Returns each expression with the id of its `Rx`. */
export async function generate(cfg: Config, write = true): Promise<Map<string, string>> {
  const grammar = cfg.grammar;
  const top: Scope = { repository: grammar.repository, own: grammar.repository, prefix: "" };
  const root = { patterns: grammar.patterns } as Node;

  function scopeIdent(name: string): string {
    const parts = name.split(".");
    if (parts.length > 1 && parts[parts.length - 1] === cfg.suffix) parts.pop();
    let out = "";
    parts.forEach((part, index) => {
      for (const piece of part.split(/[-_()$]/)) {
        const cap = piece.charAt(0).toUpperCase() + piece.slice(1);
        out += index === 0 ? (abbreviations[piece] ?? cap) : cap;
      }
    });
    return out;
  }

  const scopes = new Map<string, string>([["Source", grammar.scopeName]]);
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
   * A `$N` whose group `regex` lacks fails the generator ([`requireGroup`]),
   * unless the config lists the name as one vscode-textmate leaves as written. */
  function scopeList(name: string | undefined, regex: string): Sc[] {
    if (name === undefined) return [];
    return name
      .split(" ")
      .filter((part) => part !== "")
      .map((part): Sc => {
        const m = /^(.*?)\$(\d+)(.*)$/.exec(part);
        if (!m || cfg.literal.includes(part)) return { fixed: scope(part) };
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
    backref: number | null;
    patterns: string[];
  };

  const pats = new Map<string, Pat>();
  const rules = new Map<string, Rule>();
  /** The captures that nest a pattern list, to expand their lists a second time. */
  const captureSources: { cap: Cap; scope: Scope; patterns: Node[]; name: string }[] = [];
  /** Nested contexts by the patterns they try, so equal ones are one rule. */
  const contexts = new Map<string, string>();
  const emptyRule = (id: string): Rule => ({ id, name: [], content: [], end: "", endLast: false, backref: null, patterns: [] });

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
          rules.set(id, { ...emptyRule(id), patterns: list });
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
      let backref: number | null = null;
      if (/\\[1-9]/.test(node.end)) {
        if (node.end !== cfg.rawDelimiter?.end) throw new Error(`${id}: the end repeats a group of the begin`);
        backref = cfg.rawDelimiter.group;
      }
      pats.set(id, { id, regex: node.begin, act: { kind: "begin", rule: id }, captures: [] });
      pats.set(`${id}End`, { id: `${id}End`, regex: node.end, act: { kind: "end" }, captures: [] });
      const rule: Rule = {
        id,
        name: scopeList(node.name, node.begin),
        content: scopeList(node.contentName, node.begin),
        end: `${id}End`,
        endLast: node.applyEndPatternLast === 1,
        backref,
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

  /** The scope a node's own repository opens inside `sc`, its entries' ids starting with `prefix`. */
  function inner(sc: Scope, node: Node, prefix: string): Scope {
    if (!node.repository) return sc;
    return { repository: { ...sc.repository, ...node.repository }, own: node.repository, prefix };
  }

  /** The prefix of the ids of the entries of `key`'s own repository. */
  function prefixOf(key: string): string {
    const prefix = cfg.prefixes[key];
    if (prefix === undefined) throw new Error(`${key}: a repository with no prefix`);
    return prefix;
  }

  /** Whether vscode-textmate drops the begin/end rule `node` in `sc`.
   * It drops a rule whose list had an include it could not resolve and kept no
   * pattern, so a rule whose every pattern is such an include never matches. */
  function dropped(sc: Scope, node: Node): boolean {
    const list = node.patterns ?? [];
    if (node.begin === undefined || list.length === 0) return false;
    return list.every((entry) => entry.include?.startsWith("#") && !sc.repository[entry.include.slice(1)]);
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
        const inside = target.repository ? inner(sc, target, prefixOf(key)) : sc;
        if (!dropped(inside, target)) out.push(...expand(inside, target, id, true));
      } else {
        if (node.repository) throw new Error(`${context}At${index}: an inline repository`);
        if (!dropped(sc, node)) out.push(...expand(sc, node, `${context}At${index}`, false));
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
  const rootRule = emptyRule("Root");
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
      rule = rules.get(id) ?? emptyRule(id);
      rule.patterns = list;
      rules.set(id, rule);
      fullContexts.set(key, rule);
    }
    cap.nested = rule.id;
  }

  // Output.

  let g = `//! The grammar's rules as tables. Generated by \`oracle/${cfg.generator}\`
//! from the \`${cfg.id}\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pat\`] is one regular expression of the grammar; its doc comment says
//! which. Each rule lists the patterns it tries, in the grammar's order with
//! its \`include\`s flattened, so the first that matches at the earliest
//! position wins as in a \`TextMate\` engine.

use super::engine::${cfg.engine};
use super::scope::S;
use crate::scope::scopes;
use crate::rules;

pub(crate) type Act = rules::Act<${cfg.engine}>;
pub(crate) type PatData = rules::PatData<${cfg.engine}>;
pub(crate) type RuleData = rules::RuleData<${cfg.engine}>;
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
      `RuleData {\n                name: ${scText(r.name)},\n                content: ${scText(r.content)},\n                end: ${r.end && !r.endLast ? `Some(Pat::${r.end})` : "None"},\n                backref: ${r.backref === null ? "None" : `Some(${r.backref})`},\n                patterns: &[${[...r.patterns, ...(r.end && r.endLast ? [r.end] : [])].map((p) => `Pat::${p}`).join(", ")}],\n            }`,
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
  // An expression another module's matchers answer gets no `Rx` here.
  const shared = cfg.shared;
  const rxOf = new Map<string, string>();
  for (const p of patList) if (!rxOf.has(p.regex) && !shared?.ids.has(p.regex)) rxOf.set(p.regex, p.id);
  g += `\n/// A regular expression of the grammar, which several patterns can share.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Rx {\n`;
  for (const [regex, id] of rxOf) g += `    /// ${shown(regex).replace(/^t/, "T")}.\n    ${id},\n`;
  g += `}\n\nimpl Rx {\n    /// Every expression, in code order.\n    #[cfg(test)]\n    pub(crate) const ALL: [Rx; ${rxOf.size}] = [\n`;
  for (const id of rxOf.values()) g += `        Rx::${id},\n`;
  if (shared) {
    g += `    ];\n}\n\n/// The matchers that answer an expression.\n#[derive(Clone, Copy, Debug)]\npub(crate) enum Expr {\n    /// An expression of \`${shared.path}\`, whose matchers answer it.\n    Shared(${shared.path}),\n    /// An expression of this grammar alone.\n    Own(Rx),\n}\n\nimpl Pat {\n${tooLong}    pub(crate) fn expr(self) -> Expr {\n        match self {\n`;
    g += arms(
      patList.map((p): [string, string] => {
        const other = shared.ids.get(p.regex);
        return [p.id, other === undefined ? `Expr::Own(Rx::${rxOf.get(p.regex)})` : `Expr::Shared(${shared.path}::${other})`];
      }),
      "Pat",
    );
  } else {
    g += `    ];\n}\n\nimpl Pat {\n${tooLong}    pub(crate) fn rx(self) -> Rx {\n        match self {\n`;
    g += arms(patList.map((p): [string, string] => [p.id, `Rx::${rxOf.get(p.regex)}`]), "Pat");
  }
  g += `        }\n    }\n}\n`;

  g += startTable(patList);
  g += ruleCodes(cfg.engine, cfg.table);
  g += groupsCheck(
    cfg.engine,
    patList.flatMap((p) => p.captures.map((c) => c.group)),
  );

  const read: Reader = {
    between(id, open, close) {
      const regex = pats.get(id)?.regex;
      if (regex === undefined) throw new Error(`no expression ${id}`);
      const from = regex.indexOf(open);
      const to = regex.indexOf(close, from + open.length);
      if (from < 0 || to < 0) throw new Error(`${id}: no ${open}…${close}`);
      return regex.slice(from + open.length, to);
    },
    inside(id, head, tail) {
      const regex = pats.get(id)?.regex;
      if (regex === undefined) throw new Error(`no expression ${id}`);
      if (!regex.startsWith(head) || !regex.endsWith(tail)) throw new Error(`${id}: not ${head}…${tail}`);
      return regex.slice(head.length, regex.length - tail.length);
    },
  };
  const wordLists = cfg.words(read);

  if (write) {
    await writeRust(`${root_dir}src/${cfg.dir}/grammar.rs`, g);
    await writeRust(`${root_dir}src/${cfg.dir}/scope.rs`, scopeFile(cfg.generator, cfg.table, scopes, "", false));
    if (wordLists.length) await writeRust(`${root_dir}src/${cfg.dir}/words.rs`, wordFile(cfg.generator, wordLists));
  }

  // With RX_DUMP set to a file, writes each expression for oracle/onig.ts.
  if (write && Bun.env["RX_DUMP"]) {
    await Bun.write(
      Bun.env["RX_DUMP"],
      [...rxOf]
        // An end that repeats a group of its begin needs the begin's text, which a case cannot supply.
        .filter(([regex]) => !/\\[1-9]/.test(regex))
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
  if (write) {
    console.log(`${ruleList.length} rules (${stateful} open across lines), ${patList.length} patterns, ${rxOf.size} expressions, ${scopes.size} scopes`);
  }
  return rxOf;
}

// Word lists.
//
// Many expressions test one word against a long alternation, such as
// `\bNS(Rect|Range|…)\b`.
// Every alternative is made of word characters, so the match is the whole
// word or nothing.
// The lists go into one sorted table of words, each with a bit per list it
// belongs to, which a matcher searches once.

/** The word file: the lists' bits and the sorted table of their words. */
function wordFile(generator: string, wordLists: WordList[]): string {
  if (wordLists.length > 32) throw new Error("the lists do not fit a u32");
  const masks = new Map<string, number>();
  wordLists.forEach((list, index) => {
    for (const word of alternatives(list.body)) {
      if (!/^\w+$/.test(word)) throw new Error(`${list.name}: ${word} is not one word`);
      masks.set(word, (masks.get(word) ?? 0) | (1 << index));
    }
  });
  let w = `//! The words of the grammar's long alternations, sorted, with the lists
//! each belongs to. Generated by \`oracle/${generator}\`; do not edit.

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
  return w;
}
