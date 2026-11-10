// The generator that oracle/php-grammar.ts and oracle/hack-grammar.ts share.
// Hack's grammar descends from PHP's, so both grammars take the same shape.
//
// It writes src/<dir>/scope.rs, src/<dir>/grammar.rs and src/<dir>/words.rs.
//
// grammar.rs holds the scopes each rule and pattern adds, how each rule ends
// and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// A pattern or rule that repeats another, with the same scopes, is the same
// `Pattern` or `Rule`, so it has one matcher.
//
// The grammar's word lists, such as `(?i)\bcurl_(close|…)\b`, are words.rs:
// each list's words, written out, lowercased where the list ignores case.
// One lookup answers every list of a table.
//
// The rules that include another grammar embed it (`Payload::Embed`).
// The matchers that decide whether a pattern matches are written by hand.
import { arms, groupsCheck, ruleCodes, scopeFile, startOf, startTable, tooLong, writeRust } from "./rule-grammar.ts";

/** A grammar node, as Shiki bundles it. */
export type Node = {
  name?: string;
  contentName?: string;
  match?: string;
  begin?: string;
  end?: string;
  captures?: Record<string, { name?: string; patterns?: Node[] }>;
  beginCaptures?: Record<string, { name?: string; patterns?: Node[] }>;
  endCaptures?: Record<string, { name?: string; patterns?: Node[] }>;
  patterns?: Node[];
  include?: string;
  applyEndPatternLast?: boolean | number;
  while?: string;
};

/** What one grammar of the family needs. */
export type Family = {
  /** The grammar, as Shiki bundles it. */
  grammar: { name: string; scopeName: string; repository: Record<string, Node>; patterns: Node[] };
  /** The grammar's name in Shiki, as `php`. */
  name: string;
  /** The generator's file under oracle/. */
  script: string;
  /** The module under src/. */
  dir: string;
  /** The `Grammar` type in the module's engine. */
  engine: string;
  /** The language's name, as the scope table and the rule codes spell it. */
  display: string;
  /** The grammars it includes, by scope name: the `Language` variant and its feature. */
  embeddable: Record<string, [string, string]>;
  /** The word tables; a deep one takes lists from rules nested in its entry. */
  tables: { name: string; key: string; fold: boolean; slash: boolean; deep?: boolean }[];
  /** Whether every pattern of the first table's entry is one of its word lists. */
  supportAllLists: boolean;
  /**
   * The rule opened above the root that lists HTML first and the grammar's
   * patterns after it, or null for none.
   * PHP's, `Fence`, serves a Markdown fence.
   * Hack's, `Document`, starts every document, since its root lists HTML first.
   */
  htmlFirst: string | null;
};

/** Where `Sc` gets its variant for a scope a capture group spells. */
const GROUP_VARIANT = "\u0000group-variant\u0000";
/** Where `Sc::code` gets its arm for that variant. */
const GROUP_ARM = "\u0000group-arm\u0000";
/** The variant, which only a grammar with such a scope has. */
const groupVariant = `
      /// A scope made of a prefix, the lowered text of a capture group of the
      /// match and a suffix.
      Group(&'static str, u8, &'static str),`;
/** The variant's arm in `Sc::code`. */
const groupArm = `
              Sc::Group(..) => u16::MAX,`;

/** The most lines clippy's `too_many_lines` allows in a function body. */
const MAX_LINES = 100;

/**
 * Drops the `too_many_lines` expectation of each generated function in `path`
 * whose body is short enough, since an unfulfilled expectation fails clippy.
 * A grammar's rule count sets those lengths.
 * Clippy counts the lines from the body's `{` to its `}`, past blank lines and comments.
 */
async function dropShortExpects(path: string): Promise<void> {
  const lines = (await Bun.file(path).text()).split("\n");
  const drop = new Set<number>();
  lines.forEach((line, k) => {
    if (line.trim() !== tooLong.trim()) return;
    const head = lines.findIndex((l, j) => j > k && /^\s*pub\(crate\) (const )?fn /.test(l));
    const indent = /^\s*/.exec(lines[head]!)![0];
    const close = lines.findIndex((l, j) => j > head && l === `${indent}}`);
    const body = lines.slice(head, close + 1).filter((l) => l.trim() !== "" && !l.trim().startsWith("//"));
    if (body.length <= MAX_LINES) drop.add(k);
  });
  await Bun.write(path, lines.filter((_, k) => !drop.has(k)).join("\n"));
}

/** Writes the module's tables for `cfg`. */
export async function generate(cfg: Family): Promise<void> {
  type Captures = Record<string, { name?: string; patterns?: Node[] }>;

  const grammar = cfg.grammar;
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
    const suffix = parts[parts.length - 1] === "php" && parts.length > 1 ? "" : "Bare";
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

  const scopes = new Map<string, string>([["Source", grammar.scopeName]]);
  function scope(name: string): string {
    const ident = scopeIdent(name);
    const known = scopes.get(ident);
    if (known !== undefined && known !== name) throw new Error(`${ident}: ${known} and ${name}`);
    scopes.set(ident, name);
    return ident;
  }
  /** A scope of a rule or a capture: a fixed one's identifier, or
   * `prefix|group|suffix` for one that lowers a capture group's text, as
   * `keyword.control.${1:/downcase}.php` does. */
  function scopeList(name: string | undefined): string[] {
    if (name === undefined) return [];
    return name.split(" ").map((part) => {
      const m = /^(.*?)\$\{(\d+):\/downcase\}(.*)$/.exec(part);
      if (m) return `${m[1]}|${m[2]}|${m[3]}`;
      if (part.includes("$")) throw new Error(`${part}: a dynamic scope other than a lowered group`);
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
          if (c === "G") {
            // `\G` after a word's first letters never holds, so the branch holds no word.
            atom = [];
          } else if (/[A-Za-z0-9]/.test(c)) return null;
          else atom = [c];
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
    /** Whether the end is tried after the rule's patterns, as `applyEndPatternLast` asks. */
    endLast: boolean;
    /** The begin capture group that the end pattern repeats, when it does. */
    backref: number | null;
    /** The nishiki `Language` and its cargo feature, when the rule's patterns
     * include another grammar that nishiki lexes. */
    embed: [string, string] | null;
    /** Whether the rule lists its own patterns after the grammar it embeds. */
    ownAfter: boolean;
    patterns: string[];
  };

  const embeddable = cfg.embeddable;

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
    const rule: Rule = { id, name: [], content: [], end: "", endLast: false, backref: null, embed: null, ownAfter: false, patterns: [] };
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

  /** Registers a match or begin/end rule under `id`, unless the same node is
   * registered already; returns the pattern id. */
  function register(node: Node, id: string): string {
    const key = JSON.stringify(node);
    const known = seen.get(key);
    if (known !== undefined) return known;
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
        endLast: Boolean(node.applyEndPatternLast),
        backref: backrefs[0] ?? null,
        embed: null,
        ownAfter: false,
        patterns: [],
      };
      rules.set(id, rule);
      rule.patterns = flatten(node.patterns ?? [], id);
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
      if (node.include === "$self" || node.include === grammar.scopeName) {
        out.push(...flatten(grammar.patterns, "Root"));
      } else if (node.include !== undefined && !node.include.startsWith("#")) {
        const embed = embeddable[node.include];
        if (!embed) throw new Error(`${context}: external include ${node.include}`);
        if (context === "Root" && index === 0 && node.include === "text.html.basic" && cfg.htmlFirst !== null) return;
        if (index !== list.length - 1) throw new Error(`${context}: patterns after ${node.include}`);
        rules.get(context)!.embed = embed;
      } else if (node.include !== undefined) {
        const key = node.include.slice(1);
        const target = repository[key];
        if (!target) throw new Error(`unknown include ${key}`);
        out.push(...expand(target, pascal(key)));
      } else {
        for (const [variant, suffix] of labelled(node)) out.push(...expand(variant, `${context}At${index}${suffix}`));
      }
    });
    return out;
  }

  /** A heredoc or nowdoc that embeds a language, one per label it names.
   *
   * Its end repeats its label, as `^\s*(\3)(?!…)`.
   * Its payload holds the embedded language's state, so it cannot hold the
   * label too.
   * Each label, such as `SQL` and `DQL` for `[DS]QL`, becomes a rule of its
   * own, whose end names the label.
   * The labels of one pattern share no text, so the patterns match where the
   * one did, in its place.
   * Any other node stays as it is. */
  function labelled(node: Node): [Node, string][] {
    const embeds = node.patterns?.some((p) => p.include !== undefined && embeddable[p.include] !== undefined);
    const backref = node.end === undefined ? null : /\\([1-9])/.exec(node.end);
    if (!embeds || !backref || node.begin === undefined) return [[node, ""]];
    const group = Number(backref[1]);
    /** The capturing groups, in the order they open: where each opens and closes. */
    const spans: [number, number][] = [];
    /** The open groups: the index of each capturing one in `spans`, or -1. */
    const open: number[] = [];
    for (let i = 0; i < node.begin.length; i++) {
      const c = node.begin[i];
      if (c === "\\") i++;
      else if (c === "(") {
        const capturing = node.begin[i + 1] !== "?";
        open.push(capturing ? spans.push([i, -1]) - 1 : -1);
      } else if (c === ")") {
        const k = open.pop();
        if (k !== undefined && k >= 0) spans[k]![1] = i;
      }
    }
    const [from, to] = spans[group - 1]!;
    const labels = expandWords(node.begin.slice(from + 1, to));
    if (!labels) throw new Error(`${node.begin}: the label is no word list`);
    return labels.map((label) => [
      {
        ...node,
        begin: node.begin!.slice(0, from + 1) + label + node.begin!.slice(to),
        end: node.end!.replace(`\\${group}`, label),
      },
      labels.length > 1 ? label.charAt(0) + label.slice(1).toLowerCase() : "",
    ]);
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

  const rootRule: Rule = { id: "Root", name: [], content: [], end: "", endLast: false, backref: null, embed: null, ownAfter: false, patterns: [] };
  rules.set("Root", rootRule);
  rootRule.patterns = flatten(grammar.patterns, "Root");
  if (cfg.htmlFirst !== null) rules.set(cfg.htmlFirst, {
    id: cfg.htmlFirst,
    name: [],
    content: [],
    end: "",
    endLast: false,
    backref: null,
    embed: embeddable["text.html.basic"]!,
    ownAfter: true,
    patterns: rootRule.patterns,
  });

  const ruleList = [...rules.values()];
  const patternList = [...patternById.values()];

  /** A table of word lists: the long lists of one repository key that share a
   * form. Shorter lists stay hand matchers. */
  type Table = { name: string; key: string; fold: boolean; slash: boolean; lists: Pattern[] };
  const tables: Table[] = cfg.tables.map((t) => ({ ...t, lists: [] }));
  /** The table and the place in it of each pattern that is a listed word list. */
  const tableOf = new Map<string, [Table, number]>();
  for (const table of tables) {
    const own = new RegExp(`^${pascal(table.key)}(At\\d+)${table.deep ? "*" : "?"}$`);
    table.lists = patternList.filter((p) => own.test(p.id) && p.words?.fold === table.fold && p.words.slash === table.slash);
    if (table.lists.length === 0) throw new Error(`${table.name}: no lists`);
    table.lists.forEach((p, k) => tableOf.set(p.id, [table, k]));
  }
  const support = repository["support"]?.patterns?.length;
  if (cfg.supportAllLists && support !== tables[0]!.lists.length) throw new Error(`Support: ${tables[0]!.lists.length} of ${support} patterns are word lists`);

  // A table answers for its lists as one pattern, its head, the first list:
  // the first list that holds the word at a position. A rule that holds the
  // lists holds all of them, in order, in a row, and the row collapses to the
  // head. Another rule may hold one list that is not the head alone, as the
  // root holds `print` and `echo`; then that list is the first to hold each
  // of its words, so the table finds it.
  for (const table of tables) {
    const ids = table.lists.map((p) => p.id);
    const head = ids[0]!;
    const first = new Map<string, number>();
    table.lists.forEach((p, k) => {
      for (const word of p.words!.words) if (!first.has(word)) first.set(word, k);
    });
    for (const r of ruleList) {
      const from = r.patterns.indexOf(head);
      if (from >= 0) {
        if (r.patterns.slice(from, from + ids.length).join() !== ids.join()) {
          throw new Error(`${r.id}: holds the ${table.name} lists, but not all of them in a row`);
        }
        r.patterns = [...r.patterns.slice(0, from), head, ...r.patterns.slice(from + ids.length)];
      }
      for (const id of r.patterns) {
        const place = tableOf.get(id);
        if (place?.[0] !== table || id === head) continue;
        const k = place[1];
        const taken = table.lists[k]!.words!.words.find((word) => first.get(word) !== k);
        if (taken !== undefined) throw new Error(`${r.id}: holds ${id} alone, but an earlier list holds ${taken}`);
      }
    }
  }

  /** The regex the start table reads for a pattern: a table's head starts
   * where any of its lists can. */
  const startRegex = (p: Pattern): string => {
    const place = tableOf.get(p.id);
    if (!place || place[1] !== 0) return p.regex;
    const lead = p.regex.slice(0, p.regex.length - p.words!.body.length - 2);
    return `${lead}(?:${place[0].lists.map((l) => l.words!.body).join("|")})\\b`;
  };

  // A word with a separator is found by its two runs. Its first run must not be
  // a word of the table, or the order of the alternation would decide the match.
  for (const table of tables) {
    const all = new Set(table.lists.flatMap((p) => p.words!.words));
    for (const word of all) {
      const head = word.split(/[\\-]/)[0]!;
      if (head !== word && all.has(head)) throw new Error(`${table.name}: ${word} starts with the word ${head}`);
    }
  }

  // Output: the tables.

  let captured = false;
  const sc = (id: string) => {
    const [prefix, group, suffix] = id.split("|");
    if (group !== undefined) captured = true;
    return group === undefined ? `Sc::S(S::${id})` : `Sc::Group(${JSON.stringify(prefix)}, ${group}, ${JSON.stringify(suffix)})`;
  };
  const list = (ids: string[]) => (ids.length ? `scopes![${ids.map(sc).join(", ")}]` : "Sc::NO_SCOPES");

  let g = `//! The grammar's rules as tables. Generated by \`oracle/${cfg.script}\`
  //! from the \`${cfg.name}\` grammar Shiki 4.6.0 bundles; do not edit.
  //!
  //! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
  //! which.
  //! Each rule lists the patterns it tries, in the grammar's order with its
  //! \`include\`s flattened.
  //! So the first that matches at the earliest position wins, as in a
  //! \`TextMate\` engine.

  use super::engine::{End, ${cfg.engine}, embedded};
  use super::scope::S;
  use crate::rules;
  use crate::scope::scopes;

  pub(crate) type Act = rules::Act<${cfg.engine}>;
  pub(crate) type PatternDefinition = rules::PatternDefinition<${cfg.engine}>;
  pub(crate) type RuleDefinition = rules::RuleDefinition<${cfg.engine}>;
  pub(crate) use crate::rules::Start;

  `;

  g += `/// A scope of a rule or a capture group.
  #[derive(Clone, Copy, Debug)]
  pub(crate) enum Sc {
      /// A fixed scope.
      S(S),${GROUP_VARIANT}
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
              Sc::S(scope) => scope as u16,${GROUP_ARM}
          }
      }

      /// The empty scope list.
      pub(crate) const NO_SCOPES: crate::scope::Scopes<Sc> = &crate::scope::Keyed::new([], []);
  }

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
      `RuleDefinition {\n                name: ${list(r.name)},\n                content: ${list(r.content)},\n                end: ${r.end && !r.endLast ? `Some(Pattern::${r.end})` : "None"},\n                backref: ${r.backref === null ? "None" : `Some(${r.backref})`},\n                patterns: &[${[...r.patterns, ...(r.endLast ? [r.end] : [])].map((p) => `Pattern::${p}`).join(", ")}],\n            }`,
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
    if (r.id === cfg.htmlFirst || r.id === "Root") return "End::Never";
    if (r.id === "SqlStringDoubleQuoted") return "End::DoubleSql";
    if (r.id === "SqlStringSingleQuoted") return "End::SingleSql";
    const label = /^\^\\s\*\(([A-Z]+)\)(?:\(\?!|\\b)/.exec(patternById.get(r.end)?.regex ?? "")?.[1];
    if (label === undefined) throw new Error(`${r.id}: an embedding rule whose end is not a label`);
    if (r.id.startsWith("HeredocInterior")) return `End::Heredoc(b"${label}")`;
    if (r.id.startsWith("NowdocInterior")) return `End::Nowdoc(b"${label}")`;
    throw new Error(`${r.id}: an embedding rule of no known kind`);
  };
  g += `        }\n    }\n\n    /// Where the rule takes the line back from the language it embeds.\n${tooLong}    pub(crate) fn end_halt(self) -> End {\n        match self {\n`;
  g += arms(ruleList.map((r): [string, string] => [r.id, endHalt(r)]), "Rule");
  const ownAfter = ruleList.filter((r) => r.ownAfter).map((r) => `Rule::${r.id}`);
  g += "        }\n    }\n";
  if (ownAfter.length > 0) {
    g += `\n    /// Whether the rule lists its own patterns after the grammar it embeds.\n    pub(crate) fn embeds_after_own(self) -> bool {\n        matches!(self, ${ownAfter.join(" | ")})\n    }\n`;
  }
  g += "}\n\n";

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
      const captures = p.captures.map(([n, ids]) => `(${n}, ${list(ids)})`).join(", ");
      const nested = p.nested.map(([n, rule]) => `(${n}, Rule::${rule})`).join(", ");
      return [p.id, `PatternDefinition {\n                act: ${act},\n                captures: &[${captures}],\n                nested: &[${nested}],\n            }`];
    }),
    "Pattern",
  );
  g += `        }\n    }\n\n    /// The word list a pattern is, as its table and its place in the table.\n${tooLong}    pub(crate) fn words(self) -> Option<(super::words::Table, u8)> {\n        match self {\n`;
  g += arms(
    patternList.map((p): [string, string] => {
      const place = tableOf.get(p.id);
      return [p.id, place ? `Some((super::words::Table::${place[0].name}, ${place[1]}))` : "None"];
    }),
    "Pattern",
  );
  g += `        }\n    }\n}\n`;

  for (const p of patternList) {
    if (startOf(p.regex).any && !p.regex.includes("\\G")) {
      throw new Error(`${p.id}: the first-byte analysis does not read ${p.regex}`);
    }
  }
  g += `
  /// Every pattern that is a list of a word table, as a match arm.
  macro_rules! table_patterns {
      () => {
  ${tables.flatMap((x) => x.lists).map((p, k) => `        ${k === 0 ? "" : "| "}Pattern::${p.id}\n`).join("")}    };
  }
  pub(crate) use table_patterns;
  `;
  g += startTable(patternList.map((p) => ({ id: p.id, regex: startRegex(p) })), "any");
  g += `
  impl Pattern {
      /// The pattern of a table's list, by its place in the table.
      pub(crate) fn of_list(table: super::words::Table, list: u8) -> Pattern {
          let lists: &[Pattern] = match table {
  ${tables.map((x) => `            super::words::Table::${x.name} => &${x.name.toUpperCase()}_LISTS,\n`).join("")}        };
          lists[usize::from(list)]
      }
  }
  ${tables.map((x) => `
  /// The patterns of the \`${x.key}\` lists, in table order.
  static ${x.name.toUpperCase()}_LISTS: [Pattern; ${x.lists.length}] = [${x.lists.map((p) => `Pattern::${p.id}`).join(", ")}];
  `).join("")}`;
  g += ruleCodes(cfg.engine, cfg.display);
  g += groupsCheck(cfg.engine, patternList.flatMap((p) => [...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]));

  // Output: the words.

  /** A table's words, sorted, each with the first list that holds it. */
  function wordTable(lists: Pattern[]): [string, number][] {
    const first = new Map<string, number>();
    lists.forEach((p, k) => {
      for (const word of p.words!.words) if (!first.has(word)) first.set(word, k);
    });
    return [...first].sort((a, b) => (a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0));
  }

  let w = `//! The grammar's long word lists. Generated by \`oracle/${cfg.script}\`
  //! from the \`${cfg.name}\` grammar Shiki 4.6.0 bundles; do not edit.
  //!
  //! A table holds every word of its lists, sorted, in one text.
  //! Each word has the list that holds it first, in the grammar's order.

  /// Which table a word list is in.
  #[derive(Clone, Copy, Debug, PartialEq, Eq)]
  pub(crate) enum Table {
  ${tables.map((t) => `    /// The lists of the grammar's \`${t.key}\` entry.\n    ${t.name},\n`).join("")}}

  impl Table {
      /// The table's words.
      pub(crate) fn words(self) -> &'static Words {
          match self {
  ${tables.map((t) => `            Table::${t.name} => &${t.name.toUpperCase()},\n`).join("")}        }
      }
  }

  pub(crate) use crate::php_family::words::{Backslash, Case, Words};

  `;
  for (const t of tables) {
    const table = wordTable(t.lists);
    const text = table.map(([word]) => word).join("");
    if (text.length > 65535) throw new Error(`${t.name}: ${text.length} bytes do not fit a u16 end`);
    if (t.lists.length > 255) throw new Error(`${t.name}: ${t.lists.length} lists do not fit a byte`);
    const longest = Math.max(...table.map(([word]) => word.length));
    if (longest > 90) throw new Error(`${t.name}: a word of ${longest} bytes is past the lookup buffer`);
    let end = 0;
    const ends = table.map(([word]) => (end += word.length));
    w += `/// The \`${t.key}\` lists: ${table.length} words in ${t.lists.length} ${t.lists.length === 1 ? "list" : "lists"}.\n`;
    w += `pub(crate) static ${t.name.toUpperCase()}: Words = Words {\n    text: ${JSON.stringify(text)},\n    ends: &[${ends.join(", ")}],\n    lists: &[${table.map(([, k]) => k).join(", ")}],\n    case: Case::${t.fold ? "Folded" : "Exact"},\n    backslash: Backslash::${t.slash ? "Optional" : "Never"},\n};\n\n`;
  }

  g = g.split(GROUP_VARIANT).join(captured ? groupVariant : "").split(GROUP_ARM).join(captured ? groupArm : "");
  await writeRust(`${root_dir}src/${cfg.dir}/grammar.rs`, g);
  await dropShortExpects(`${root_dir}src/${cfg.dir}/grammar.rs`);
  await writeRust(`${root_dir}src/${cfg.dir}/words.rs`, w);
  await writeRust(`${root_dir}src/${cfg.dir}/scope.rs`, scopeFile(cfg.script, cfg.display, scopes, "", false));

  // With RX_DUMP set to a file, writes each pattern for oracle/onig.ts, as the
  // matchers answer for it:
  // - an end that repeats a group of its begin is left out, since a case
  //   cannot supply that group's text;
  // - a table's head answers for every list of the table, so its expression is
  //   the union of the lists;
  // - a list that no rule holds alone is never tried, and is left out;
  // - a group that a dynamic scope name reads counts as named.
  if (Bun.env["RX_DUMP"]) {
    const held = new Set(ruleList.flatMap((r) => r.patterns));
    const dynamic = (ids: string[]) => ids.flatMap((id) => (id.includes("|") ? [Number(id.split("|")[1])] : []));
    const groupsOf = (p: Pattern) => [
      ...new Set([
        ...p.captures.flatMap(([n, ids]) => [n, ...dynamic(ids)]),
        ...p.nested.map(([n]) => n),
        ...(p.act.role === "match" ? dynamic(p.act.name) : []),
      ]),
    ];
    const probed = patternList.filter((p) => {
      if (p.act.role === "end" && /\\[1-9]/.test(p.regex)) return false;
      return !tableOf.has(p.id) || held.has(p.id);
    });
    await Bun.write(
      Bun.env["RX_DUMP"],
      probed.map((p) => JSON.stringify({ id: p.id, regex: startRegex(p), groups: groupsOf(p) })).join("\n") + "\n",
    );
  }

  console.log(
    `${ruleList.length} rules, ${patternList.length} patterns, ${scopes.size} scopes, ${tables.map((x) => `${x.lists.length} ${x.name}`).join(", ")} word lists`,
  );

}
