// The generator of the rule-engine languages whose grammars nest repositories and share
// expressions: Objective-C (`oracle/objective-c-grammar.ts`) and Objective-C++
// (`oracle/objective-cpp-grammar.ts`).
// For a grammar Shiki bundles, `generate` writes the module's `scope.rs`,
// `grammar.rs` and `words.rs`.
// It writes the scope names.
// For every rule it writes the scopes the rule adds, how it ends and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// The matchers that decide whether a pattern matches are written by hand.
//
// An entry that carries a repository of its own resolves an include inside it there first,
// as vscode-textmate's does.
// `$self` names the root of the grammar a rule belongs to, and `$base` this grammar's root.
// An include of another grammar is that grammar's rules, when the config lists it in `grammars`.
// Otherwise it is a language that another lexer handles, listed in `externals`.
// A rule that includes one embeds that language,
// and a capture group's list that includes one defers the group to it.
// A rule that vscode-textmate would drop, because its list lost every entry, is left out.
// A `Config` says what differs between the grammars.
import { expand as alternatives } from "./alternation.ts";
import {
  arms,
  type Backref,
  groupsCheck,
  oneBlankTable,
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
  repository?: Record<string, Node>;
  include?: string;
  applyEndPatternLast?: number;
};
export type GrammarJson = { scopeName: string; repository: Record<string, Node>; patterns: Node[] };

/**
 * The repository an include resolves in, the prefix of the ids of its own entries,
 * and the grammar it belongs to.
 */
type Scope = {
  repository: Record<string, Node>;
  own: Record<string, Node>;
  prefix: string;
  grammar: Owner;
};

/** A grammar whose rules the tables hold: its top scope, its root and the id of its root's list.
 * `$self` inside its rules names this root. */
type Owner = { top: Scope; root: Node; rootId: string };

/**
 * The language that takes the text of a rule or capture group whose patterns
 * include another grammar.
 */
export type Embed = { lang: string; feature: string };

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
  /** The includes the grammar names but never defines.
   * vscode-textmate skips such an include and keeps the rest of its list. */
  unresolved?: string[];
  /**
   * The other grammars whose rules the tables hold, by scope name,
   * each with the prefix of its rule ids.
   * LaTeX includes `text.tex` and `text.tex#braces`.
   * Inside such a grammar, `$self` names its own root and `$base` this grammar's.
   */
  grammars?: Record<string, { grammar: GrammarJson; prefix: string }>;
  /**
   * What each include of a grammar that another lexer handles stands for:
   * the language that takes the text, or null.
   * Null is a grammar the oracle does not load, so the include resolves to nothing,
   * as in vscode-textmate.
   */
  externals?: Record<string, Embed | null>;
  /**
   * The halt of each embedding rule's end, by the end's expression:
   * a Rust path to a `rules::FnHalt`.
   * `otherwise` is the halt of a rule that embeds nothing, which the engine never asks for.
   * With it the generator writes `Rule::end_halt`.
   */
  endHalts?: { by: Record<string, string>; otherwise: string };
  /** Whether the grammar scans blank runs with `BlankRuns::ToLast`.
   * The generated `Pattern::takes_one_blank` then lists the patterns that can match only
   * at a run's last blank.
   */
  toLast?: boolean;
  /** The expressions another module's matchers answer, with that module's `Rx` id.
   * The generated `Pattern::expr` sends them there; the rest are this module's own `Rx`. */
  shared?: { path: string; ids: Map<string, string> };
  /** The begin capture group that a raw string's end repeats, set by the matcher. */
  rawDelimiter?: { end: string; group: number };
  /** What a back-reference can start with, for the start table: a quote by default.
   * TeX's math ends at a repeat of `$` or `$$`, so it takes `"any"`. */
  backrefStart?: Backref;
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
  /** The owner of a grammar's rules, with its top scope. */
  const owner = (json: GrammarJson, prefix: string): Owner => {
    const own = { root: { patterns: json.patterns } as Node, rootId: `${prefix}Root` } as Owner;
    own.top = { repository: json.repository, own: json.repository, prefix, grammar: own };
    return own;
  };
  const base = owner(grammar, "");
  const top = base.top;
  const root = base.root;
  /** Every grammar whose rules the tables hold, by scope name. */
  const owners = new Map<string, Owner>([[grammar.scopeName, base]]);
  for (const [name, other] of Object.entries(cfg.grammars ?? {}))
    owners.set(name, owner(other.grammar, other.prefix));

  function scopeIdent(name: string): string {
    const parts = name.split(".");
    if (parts.length > 1 && parts[parts.length - 1] === cfg.suffix) parts.pop();
    let out = "";
    parts.forEach((part, index) => {
      for (const piece of part.split(/[-_()$]/)) {
        const capture = piece.charAt(0).toUpperCase() + piece.slice(1);
        out += index === 0 ? (abbreviations[piece] ?? capture) : capture;
      }
    });
    return out;
  }

  const scopes = new Map<string, string>([["Source", grammar.scopeName]]);
  /** Whether a scope name repeats the text of a capture group. */
  let groupScopes = false;
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
        groupScopes = true;
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
    backref: number | null;
    patterns: string[];
  };

  const patternById = new Map<string, Pattern>();
  const rules = new Map<string, Rule>();
  /** The captures that nest a pattern list, to expand their lists a second time. */
  const captureSources: { capture: Capture; scope: Scope; patterns: Node[]; name: string }[] = [];
  /** Nested contexts by the patterns they try, so equal ones are one rule. */
  const contexts = new Map<string, string>();
  const emptyRule = (id: string): Rule => ({
    id,
    name: [],
    content: [],
    end: "",
    endLast: false,
    backref: null,
    patterns: [],
  });

  function captureList(
    sc: Scope,
    captures: Captures | undefined,
    owner: string,
    regex: string,
  ): Capture[] {
    if (!captures) return [];
    const out: Capture[] = [];
    for (const [group, value] of Object.entries(captures)) {
      const names = scopeList(value.name, regex);
      let nested: string | null = null;
      if (value.patterns) {
        const list = flatten(sc, value.patterns, `${owner}Capture${group}`);
        const key = list.join(",");
        let id = contexts.get(key);
        if (id === undefined) {
          id = `${owner}Capture${group}`;
          contexts.set(key, id);
          rules.set(id, { ...emptyRule(id), patterns: list });
        }
        nested = id;
      }
      if (names.length || nested) {
        const capture: Capture = { group: Number(group), scopes: names, nested };
        if (value.patterns)
          captureSources.push({
            capture,
            scope: sc,
            patterns: value.patterns,
            name: `${owner}Capture${group}`,
          });
        out.push(capture);
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
    const clash = patternById.get(id);
    if (clash && clash.regex !== (node.match ?? node.begin))
      throw new Error(`${id} names two rules`);
    if (clash) return id;
    if (node.match !== undefined) {
      patternById.set(id, {
        id,
        regex: node.match,
        act: { role: "match", name: scopeList(node.name, node.match) },
        captures: [],
      });
      patternById.get(id)!.captures = captureList(sc, node.captures, id, node.match);
      return id;
    }
    if (node.begin !== undefined && node.end !== undefined) {
      let backref: number | null = null;
      if (/\\[1-9]/.test(node.end)) {
        if (node.end === cfg.rawDelimiter?.end) {
          backref = cfg.rawDelimiter.group;
        } else {
          const groups = new Set([...node.end.matchAll(/\\([1-9])/g)].map((m) => Number(m[1])));
          if (groups.size > 1)
            throw new Error(`${id}: the end repeats more than one group of the begin`);
          backref = [...groups][0]!;
          requireGroup(node.end, backref, node.begin);
        }
      }
      patternById.set(id, {
        id,
        regex: node.begin,
        act: { role: "begin", rule: id },
        captures: [],
      });
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
        backref,
        patterns: [],
      };
      rules.set(id, rule);
      patternById.get(id)!.captures = captureList(
        sc,
        node.beginCaptures ?? node.captures,
        id,
        node.begin,
      );
      patternById.get(`${id}End`)!.captures = captureList(
        sc,
        node.endCaptures ?? node.captures,
        `${id}End`,
        node.end,
      );
      sources.push({ scope: sc, node, id });
      rule.patterns = flatten(sc, node.patterns ?? [], id);
      return id;
    }
    throw new Error(`${id}: neither a match nor a begin/end rule`);
  }

  /**
   * The scope a node's own repository opens inside `sc`, its entries' ids starting with `prefix`.
   */
  function inner(sc: Scope, node: Node, prefix: string): Scope {
    if (!node.repository) return sc;
    return {
      repository: { ...sc.repository, ...node.repository },
      own: node.repository,
      prefix,
      grammar: sc.grammar,
    };
  }

  /** The prefix of the ids of the entries of `key`'s own repository. */
  function prefixOf(key: string): string {
    const prefix = cfg.prefixes[key];
    if (prefix === undefined) throw new Error(`${key}: a repository with no prefix`);
    return prefix;
  }

  /**
   * What an include in `sc` names: a node with the scope it resolves in and its id,
   * a language that takes the text, or nothing.
   * An unknown `#` include fails the generator when `strict`,
   * unless the config lists it as unresolved.
   * The drop check is not strict: inside a rule that vscode-textmate drops,
   * such an include does no harm.
   */
  type Target = { node: Node; sc: Scope; id: string } | { embed: Embed } | null;
  function resolve(sc: Scope, include: string, strict = true): Target {
    if (include === "$self")
      return { node: sc.grammar.root, sc: sc.grammar.top, id: sc.grammar.rootId };
    if (include === "$base") return { node: root, sc: top, id: "Root" };
    if (include.startsWith("#")) {
      const key = include.slice(1);
      const target = sc.repository[key];
      if (!target) {
        if (!strict || cfg.unresolved?.includes(key)) return null;
        throw new Error(`unknown include ${key}`);
      }
      const prefix = key in sc.own ? sc.prefix : "";
      return {
        node: target,
        sc: target.repository ? inner(sc, target, prefixOf(key)) : sc,
        id: prefix + pascal(key),
      };
    }
    const [name, key] = include.split("#") as [string, string | undefined];
    const other = owners.get(name);
    if (other)
      return key === undefined
        ? { node: other.root, sc: other.top, id: other.rootId }
        : resolve(other.top, `#${key}`, strict);
    if (key === undefined && cfg.externals && name in cfg.externals) {
      const embed = cfg.externals[name];
      return embed ? { embed } : null;
    }
    throw new Error(`external include ${include}`);
  }

  /** Whether vscode-textmate drops `node` where a list includes it.
   * It drops a begin/end rule, or a bare list,
   * whose list had an entry it could not keep and kept none.
   * An entry is lost when its include resolves to nothing or to such a rule,
   * so a drop goes up through the rules that hold it.
   * A list that includes itself is kept,
   * as a rule whose patterns vscode-textmate has not compiled yet is.
   */
  function dropped(sc: Scope, node: Node, seen = new Set<Node>()): boolean {
    const list = node.patterns ?? [];
    if (node.match !== undefined || list.length === 0 || seen.has(node)) return false;
    const known = drops.get(node);
    if (known !== undefined) return known;
    seen.add(node);
    const kept = list.some((entry) => !lost(sc, entry, seen));
    seen.delete(node);
    drops.set(node, !kept);
    return !kept;
  }
  /** What [`dropped`] found for each node. */
  const drops = new Map<Node, boolean>();

  /** Whether the list entry `entry` resolves to nothing vscode-textmate keeps ([`dropped`]). */
  function lost(sc: Scope, entry: Node, seen: Set<Node>): boolean {
    if (entry.include === undefined) return dropped(sc, entry, seen);
    const target = resolve(sc, entry.include, false);
    return target === null || ("node" in target && dropped(target.sc, target.node, seen));
  }

  /** The mark a list holds where it includes a language that takes the text. */
  const embedMark = (embed: Embed) => `@${embed.lang}@${embed.feature}`;
  /** The language a mark stands for. */
  const markEmbed = (mark: string): Embed | null => {
    const [, lang, feature] = mark.split("@");
    return mark.startsWith("@") ? { lang: lang!, feature: feature! } : null;
  };

  /** The pattern ids a pattern list expands to, `include`s flattened.
   * An include of a language that takes the text leaves its mark ([`embedMark`]) in place. */
  function flatten(sc: Scope, list: Node[], context: string): string[] {
    const out: string[] = [];
    list.forEach((node, index) => {
      if (node.include !== undefined) {
        const target = resolve(sc, node.include);
        if (target === null) return;
        if ("embed" in target) {
          out.push(embedMark(target.embed));
          return;
        }
        if (!dropped(target.sc, target.node))
          out.push(...expand(target.sc, target.node, target.id, true));
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
  for (let i = 0, j = 0; i < sources.length || j < captureSources.length;) {
    if (i < sources.length) {
      const { scope: sc, node, id } = sources[i++]!;
      expanding.clear();
      rules.get(id)!.patterns = flatten(sc, node.patterns ?? [], id);
      continue;
    }
    const { capture, scope: sc, patterns, name } = captureSources[j++]!;
    expanding.clear();
    const list = flatten(sc, patterns, name);
    const key = list.join(",");
    let rule = fullContexts.get(key);
    if (rule === undefined) {
      let id = capture.nested!;
      if (claimed.has(id)) id = `${name}X${claimed.size}`;
      claimed.set(id, key);
      rule = rules.get(id) ?? emptyRule(id);
      rule.patterns = list;
      rules.set(id, rule);
      fullContexts.set(key, rule);
    }
    capture.nested = rule.id;
  }

  // Output.

  let g = `//! The grammar's rules as tables. Generated by \`oracle/${cfg.generator}\`
//! from the \`${cfg.id}\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::${cfg.engine};
use super::scope::S;
use crate::scope::scopes;
use crate::rules;

pub(crate) type Act = rules::Act<${cfg.engine}>;
pub(crate) type PatternDefinition = rules::PatternDefinition<${cfg.engine}>;
pub(crate) type RuleDefinition = rules::RuleDefinition<${cfg.engine}>;
pub(crate) use crate::rules::Start;

/// A scope of a rule or a capture group.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Sc {
    /// A fixed scope.
    S(S),${
      groupScopes
        ? `
    /// A scope made of a prefix, the text of a capture group of the match and
    /// a suffix.
    Group(&'static str, u8, &'static str),`
        : ""
    }
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
            Sc::S(scope) => scope as u16,${
              groupScopes
                ? `
            Sc::Group(..) => u16::MAX,`
                : ""
            }
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
  if (ruleList.length > 256) throw new Error(`${ruleList.length} rules do not fit one byte`);

  // A rule that includes another language hands its text to that language.
  // A capture group's list that includes one defers the group to it.
  // A list can reach one language through several includes.
  // The first mark counts: the later ones repeat patterns it already tries.
  /** What each rule embeds, and whether its own patterns come after the language. */
  const embeds = new Map<string, { embed: Embed; afterOwn: boolean }>();
  /** The language each capture group's list defers its group to. */
  const deferred = new Map<string, Embed>();
  for (const r of ruleList) {
    const marks = r.patterns.flatMap((p, index) => {
      const embed = markEmbed(p);
      return embed ? [{ embed, index }] : [];
    });
    if (marks.length === 0) continue;
    if (new Set(marks.map((mark) => embedMark(mark.embed))).size > 1)
      throw new Error(`${r.id}: embeds two languages`);
    const { embed, index } = marks[0]!;
    if (r.id === "Root") throw new Error("the root embeds a language");
    if (r.end === "") {
      if (r.patterns.length > marks.length)
        throw new Error(`${r.id}: a capture group embeds a language beside other patterns`);
      deferred.set(r.id, embed);
    } else {
      if (r.backref !== null) throw new Error(`${r.id}: embeds a language and repeats a group`);
      embeds.set(r.id, {
        embed,
        afterOwn: r.patterns.slice(index + 1).some((p) => markEmbed(p) === null),
      });
    }
    r.patterns = r.patterns.filter((p) => markEmbed(p) === null);
  }

  g += `/// A rule of the grammar.
/// It is one that can be open at the end of a line, or a list of patterns
/// that tokenizes a capture group.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub(crate) enum Rule {\n`;
  for (const r of ruleList) g += `    ${r.id},\n`;
  g += `}\n\nimpl Rule {\n    /// Every rule, in code order.\n    pub(crate) const ALL: [Rule; ${ruleList.length}] = [\n`;
  for (const r of ruleList) g += `        Rule::${r.id},\n`;
  g += `    ];\n\n${tooLong}    pub(crate) fn definition(self) -> RuleDefinition {\n        match self {\n`;
  g += arms(
    ruleList.map((r): [string, string] => [
      r.id,
      `RuleDefinition {\n                name: ${scText(r.name)},\n                content: ${scText(r.content)},\n                end: ${r.end && !r.endLast ? `Some(Pattern::${r.end})` : "None"},\n                backref: ${r.backref === null ? "None" : `Some(${r.backref})`},\n                patterns: &[${[...r.patterns, ...(r.end && r.endLast ? [r.end] : [])].map((p) => `Pattern::${p}`).join(", ")}],\n            }`,
    ]),
    "Rule",
  );
  g += `        }\n    }\n}\n\n`;
  g += embedTables(ruleList, embeds, deferred);
  if (cfg.endHalts)
    g += endHaltTable(ruleList, embeds, cfg.endHalts, (r) => patternById.get(r.end)!.regex);

  g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pattern {\n`;
  const docs = (s: string) => {
    const one = s.replace(/\n/g, "\\n");
    return one.length > 56 ? `…` : one;
  };
  /** An expression as a doc comment shows it; the empty one has no code span. */
  const shown = (s: string) =>
    s === "" ? "the empty expression" : `\`${docs(s).replaceAll("`", "'")}\``;
  for (const p of patternList) g += `    /// The ${p.act.role}: ${shown(p.regex)}.\n    ${p.id},\n`;
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
  // An expression another module's matchers answer gets no `Rx` here.
  const shared = cfg.shared;
  const rxOf = new Map<string, string>();
  for (const p of patternList)
    if (!rxOf.has(p.regex) && !shared?.ids.has(p.regex)) rxOf.set(p.regex, p.id);
  g += `\n/// A regular expression of the grammar, which several patterns can share.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Rx {\n`;
  for (const [regex, id] of rxOf) g += `    /// ${shown(regex).replace(/^t/, "T")}.\n    ${id},\n`;
  g += `}\n\nimpl Rx {\n    /// Every expression, in code order.\n    #[cfg(test)]\n    pub(crate) const ALL: [Rx; ${rxOf.size}] = [\n`;
  for (const id of rxOf.values()) g += `        Rx::${id},\n`;
  // A table of one line per arm is long only past clippy's hundred lines.
  const exprArms = shared
    ? arms(
        patternList.map((p): [string, string] => {
          const other = shared.ids.get(p.regex);
          return [
            p.id,
            other === undefined
              ? `Expr::Own(Rx::${rxOf.get(p.regex)})`
              : `Expr::Shared(${shared.path}::${other})`,
          ];
        }),
        "Pattern",
      )
    : "";
  const rxArms = shared
    ? ""
    : arms(
        patternList.map((p): [string, string] => [p.id, `Rx::${rxOf.get(p.regex)}`]),
        "Pattern",
      );
  if (shared) {
    g += `    ];\n}\n\n/// The matchers that answer an expression.\n#[derive(Clone, Copy, Debug)]\npub(crate) enum Expr {\n    /// An expression of \`${shared.path}\`, whose matchers answer it.\n    Shared(${shared.path}),\n    /// An expression of this grammar alone.\n    Own(Rx),\n}\n\nimpl Pattern {\n${longTable(exprArms)}    pub(crate) fn expr(self) -> Expr {\n        match self {\n`;
    g += exprArms;
  } else {
    g += `    ];\n}\n\nimpl Pattern {\n${longTable(rxArms)}    pub(crate) fn rx(self) -> Rx {\n        match self {\n`;
    g += rxArms;
  }
  g += `        }\n    }\n}\n`;

  g += startTable(patternList, cfg.backrefStart);
  if (cfg.toLast) g += oneBlankTable(patternList, cfg.backrefStart);
  g += ruleCodes(cfg.engine, cfg.table);
  g += groupsCheck(
    cfg.engine,
    patternList.flatMap((p) => p.captures.map((c) => c.group)),
  );

  const read: Reader = {
    between(id, open, close) {
      const regex = patternById.get(id)?.regex;
      if (regex === undefined) throw new Error(`no expression ${id}`);
      const from = regex.indexOf(open);
      const to = regex.indexOf(close, from + open.length);
      if (from < 0 || to < 0) throw new Error(`${id}: no ${open}…${close}`);
      return regex.slice(from + open.length, to);
    },
    inside(id, head, tail) {
      const regex = patternById.get(id)?.regex;
      if (regex === undefined) throw new Error(`no expression ${id}`);
      if (!regex.startsWith(head) || !regex.endsWith(tail))
        throw new Error(`${id}: not ${head}…${tail}`);
      return regex.slice(head.length, regex.length - tail.length);
    },
  };
  const wordLists = cfg.words(read);

  if (write) {
    await writeRust(`${root_dir}src/${cfg.dir}/grammar.rs`, g);
    await writeRust(
      `${root_dir}src/${cfg.dir}/scope.rs`,
      scopeFile(cfg.generator, cfg.table, scopes, "", false),
    );
    if (wordLists.length)
      await writeRust(`${root_dir}src/${cfg.dir}/words.rs`, wordFile(cfg.generator, wordLists));
  }

  // With RX_DUMP set to a file, writes each expression for oracle/onig.ts.
  if (write && Bun.env["RX_DUMP"]) {
    await Bun.write(
      Bun.env["RX_DUMP"],
      [...rxOf]
        // An end that repeats a group of its begin needs the begin's text,
        // which a case cannot supply.
        .filter(([regex]) => !/\\[1-9]/.test(regex))
        .map(([regex, id]) => {
          // The groups the captures name, and the groups a `$N` in a scope name reads.
          const dollar = (list: Sc[]) => list.flatMap((s) => ("group" in s ? [s.group] : []));
          const groups = patternList
            .filter((p) => p.regex === regex)
            .flatMap((p) => [
              ...p.captures.flatMap((c) => [c.group, ...dollar(c.scopes)]),
              ...(p.act.role === "match" ? dollar(p.act.name) : []),
              ...(p.act.role === "begin"
                ? [
                    ...dollar(rules.get(p.act.rule)!.name),
                    ...dollar(rules.get(p.act.rule)!.content),
                  ]
                : []),
            ]);
          return JSON.stringify({ id, regex, groups: [...new Set(groups)].sort((a, b) => a - b) });
        })
        .join("\n") + "\n",
    );
  }
  if (write) {
    console.log(
      `${ruleList.length} rules (${stateful} open across lines), ${patternList.length} patterns, ${rxOf.size} expressions, ${scopes.size} scopes`,
    );
  }
  return rxOf;
}

// Long tables.

/** The `too_many_lines` expectation, for a table whose arms run past clippy's hundred lines. */
const longTable = (armText: string) => (armText.split("\n").length > 100 ? tooLong : "");

// Embedded languages.

/**
 * `Rule::carried`, `Rule::deferred` and `Rule::embeds_after_own`,
 * for a grammar that embeds languages.
 * Each is written only when some rule needs it.
 */
function embedTables(
  ruleList: { id: string; backref: number | null }[],
  embeds: Map<string, { embed: Embed; afterOwn: boolean }>,
  deferred: Map<string, Embed>,
): string {
  if (embeds.size === 0 && deferred.size === 0) return "";
  const lang = (embed: Embed) =>
    `rules::embedded!(${embed.lang}, ${JSON.stringify(embed.feature)})`;
  let g = "impl Rule {\n";
  if (embeds.size) {
    const carriedStateArms = arms(
      ruleList.map((r): [string, string] => {
        const embed = embeds.get(r.id);
        if (embed)
          return [
            r.id,
            `${lang(embed.embed)}.map_or(rules::CarriedState::None, rules::CarriedState::Embed)`,
          ];
        return [
          r.id,
          r.backref === null
            ? "rules::CarriedState::None"
            : `rules::CarriedState::Backref(${r.backref})`,
        ];
      }),
      "Rule",
    );
    g += `    /// What the rule carries from line to line.\n    /// A rule that embeds a language carries that language's state, when its\n    /// feature is on.\n    /// A rule whose end repeats a begin capture group carries that group.\n    #[inline]\n${longTable(carriedStateArms)}    pub(crate) fn carried_state(self) -> rules::CarriedState {\n        match self {\n${carriedStateArms}`;
    g += "        }\n    }\n";
  }
  if (deferred.size) {
    const deferredArms = arms(
      ruleList.map((r): [string, string] => [
        r.id,
        deferred.has(r.id) ? lang(deferred.get(r.id)!) : "None",
      ]),
      "Rule",
    );
    g += `\n    /// The language that lexes again a capture group whose patterns are this\n    /// list, when that language's feature is on.\n${longTable(deferredArms)}    pub(crate) fn deferred(self) -> Option<crate::Language> {\n        match self {\n${deferredArms}`;
    g += "        }\n    }\n";
  }
  const afterOwn = [...embeds].filter(([, e]) => e.afterOwn).map(([id]) => `Rule::${id}`);
  if (afterOwn.length) {
    g += `\n    /// Whether the rule lists patterns of its own after the language it embeds.\n    pub(crate) fn embeds_after_own(self) -> bool {\n        matches!(self, ${afterOwn.join(" | ")})\n    }\n`;
  }
  return `${g}}\n\n`;
}

/** `Rule::end_halt`: the halt of each embedding rule's end, from the config's `endHalts`. */
function endHaltTable(
  ruleList: { id: string; end: string }[],
  embeds: Map<string, { embed: Embed; afterOwn: boolean }>,
  halts: { by: Record<string, string>; otherwise: string },
  endOf: (rule: { id: string; end: string }) => string,
): string {
  const haltArms = arms(
    ruleList.map((r): [string, string] => {
      if (!embeds.has(r.id)) return [r.id, halts.otherwise];
      const halt = halts.by[endOf(r)];
      if (halt === undefined) throw new Error(`${r.id}: no halt for its end ${endOf(r)}`);
      return [r.id, halt];
    }),
    "Rule",
  );
  return `impl Rule {\n    /// The halt where the embedding rule's end matches, for the language it\n    /// embeds.\n    /// A rule that embeds nothing gets one the engine never asks for.\n${longTable(haltArms)}    pub(crate) fn end_halt(self) -> rules::FnHalt {\n        match self {\n${haltArms}        }\n    }\n}\n\n`;
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
