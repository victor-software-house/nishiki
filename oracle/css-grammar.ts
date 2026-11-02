// Writes src/css/scope.rs and src/css/grammar.rs from the `css` grammar
// that Shiki 4.6.0 bundles (microsoft/vscode, MIT): the scope names, and
// for every rule the scopes it adds, how it ends and the patterns it tries, in
// the grammar's order with its `include`s flattened. The matchers that decide
// whether a pattern matches are written by hand in src/css/matchers.
//
// Run with `bun run oracle/css-grammar.ts` and commit the result; the
// output is generated, do not edit it.
import css from "@shikijs/langs@4.6.0/css";

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

const grammar = (css as unknown as { repository: Record<string, Node>; patterns: Node[] }[])[0]!;
const repository = grammar.repository;

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

const scopes = new Map<string, string>([["Source", "source.css"]]);
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
  patterns: string[];
};

const pats = new Map<string, Pat>();
const rules = new Map<string, Rule>();

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

function expand(node: Node, id: string, key: string): string[] {
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
function arms(entries: [string, string][], type: string): string {
  const groups = new Map<string, string[]>();
  for (const [id, body] of entries) groups.set(body, [...(groups.get(body) ?? []), `${type}::${id}`]);
  let out = "";
  for (const [body, ids] of groups) out += `            ${ids.join("\n            | ")} => ${body},\n`;
  return out;
}

const tooLong = `    #[expect(clippy::too_many_lines, reason = "a generated table")]\n`;

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


// First bytes. For each pattern, the bytes a match can start at, so that the
// engine tries it only where one stands. The analysis reads the regular
// expression, over-approximating: a byte it leaves out cannot start a match.

type Set = bigint[]; // four 64-bit words, bit b of word b >> 6.
const FULL = (): Set => [0xffffffffffffffffn, 0xffffffffffffffffn, 0xffffffffffffffffn, 0xffffffffffffffffn];
const EMPTY = (): Set => [0n, 0n, 0n, 0n];
const add = (s: Set, b: number) => {
  s[b >> 6] = s[b >> 6]! | (1n << BigInt(b & 63));
};
const union = (a: Set, b: Set): Set => a.map((w, i) => w | b[i]!);
const intersect = (a: Set, b: Set): Set => a.map((w, i) => w & b[i]!);
const single = (b: number): Set => {
  const s = EMPTY();
  add(s, b);
  return s;
};
const nonAscii = (): Set => {
  const s = EMPTY();
  for (let b = 0x80; b < 256; b++) add(s, b);
  return s;
};
const range = (lo: number, hi: number): Set => {
  const s = EMPTY();
  for (let b = lo; b <= hi; b++) add(s, b);
  return s;
};
const complement = (a: Set): Set => a.map((w) => ~w & 0xffffffffffffffffn);

type Info = { set: Set; empty: Set | null; bol: boolean; any: boolean; wb: boolean };
const info = (set: Set, empty: Set | null = null, bol = false, any = false, wb = false): Info => ({
  set,
  empty,
  bol,
  any,
  wb,
});

const WORD = () => union(union(union(range(48, 57), range(65, 90)), union(range(97, 122), single(95))), nonAscii());
const SPACE = () => union(union(single(32), range(9, 13)), nonAscii());
const DIGIT = () => union(range(48, 57), nonAscii());

class RegexParser {
  pos = 0;
  groups = 0;
  constructor(readonly src: string) {}

  parse(): Info {
    const result = this.alternation();
    if (this.pos !== this.src.length) throw new Error(`regex ${this.src}: stopped at ${this.pos}`);
    return result;
  }

  alternation(): Info {
    const branches: Info[] = [this.sequence()];
    while (this.src[this.pos] === "|") {
      this.pos++;
      branches.push(this.sequence());
    }
    if (branches.length === 1) return branches[0]!;
    let set = EMPTY();
    let empty: Set | null = null;
    let any = false;
    for (const b of branches) {
      set = union(set, b.set);
      if (b.empty) empty = empty ? union(empty, b.empty) : b.empty;
      any = any || b.any;
    }
    return info(
      set,
      empty,
      branches.every((b) => b.bol),
      any,
      branches.every((b) => b.wb),
    );
  }

  sequence(): Info {
    let set = EMPTY();
    let empty: Set | null = FULL(); // the empty path's constraint, while it lasts
    let bol = false;
    let wb = false;
    let any = false;
    while (this.pos < this.src.length && this.src[this.pos] !== "|" && this.src[this.pos] !== ")") {
      const item = this.quantified();
      any = any || item.any;
      if (empty !== null) {
        set = union(set, item.set);
        if (item.bol) bol = true;
        if (item.wb) wb = true;
        if (item.empty === null) empty = null;
        else empty = intersect(empty, item.empty);
      }
    }
    return info(set, empty, bol, any, wb);
  }

  quantified(): Info {
    const atom = this.atom();
    let min = 1;
    for (;;) {
      const c = this.src[this.pos];
      if (c === "*" || c === "?") {
        min = 0;
        this.pos++;
      } else if (c === "+") {
        this.pos++;
      } else if (c === "{" && /^\{\d+(,\d*)?\}/.test(this.src.slice(this.pos))) {
        const m = /^\{(\d+)(,\d*)?\}/.exec(this.src.slice(this.pos))!;
        if (Number(m[1]) === 0) min = 0;
        this.pos += m[0].length;
      } else {
        break;
      }
      // Lazy and possessive suffixes.
      if (this.src[this.pos] === "?" || this.src[this.pos] === "+") this.pos++;
    }
    if (min === 0) return info(atom.set, FULL(), false, atom.any);
    return atom;
  }

  atom(): Info {
    const c = this.src[this.pos]!;
    if (c === "(") return this.group();
    if (c === "[") return this.charClass();
    if (c === "^") {
      this.pos++;
      return info(EMPTY(), FULL(), true);
    }
    if (c === "$") {
      this.pos++;
      return info(EMPTY(), FULL());
    }
    if (c === ".") {
      this.pos++;
      return info(complement(single(10)));
    }
    if (c === "\\") return this.escape();
    this.pos++;
    const bytes = new TextEncoder().encode(c);
    return info(single(bytes[0]!));
  }

  group(): Info {
    this.pos++; // (
    let kind = "capture";
    if (this.src[this.pos] === "?") {
      const rest = this.src.slice(this.pos);
      if (rest.startsWith("?:") || rest.startsWith("?>")) {
        kind = "plain";
        this.pos += 2;
      } else if (rest.startsWith("?=")) {
        kind = "ahead";
        this.pos += 2;
      } else if (rest.startsWith("?!")) {
        kind = "not-ahead";
        this.pos += 2;
      } else if (rest.startsWith("?<=")) {
        kind = "behind";
        this.pos += 3;
      } else if (rest.startsWith("?<!")) {
        kind = "not-behind";
        this.pos += 3;
      } else {
        throw new Error(`regex ${this.src}: group ${rest.slice(0, 4)}`);
      }
    }
    const bodyStart = this.pos;
    const body = this.alternation();
    const bodySource = this.src.slice(bodyStart, this.pos);
    if (this.src[this.pos] !== ")") throw new Error(`regex ${this.src}: unclosed group`);
    this.pos++;
    switch (kind) {
      case "ahead": {
        // Zero-width; where the body cannot be empty, its first bytes are a
        // condition on the position.
        const constraint = body.empty === null ? union(body.set, EMPTY()) : FULL();
        return info(EMPTY(), constraint, false, body.any && body.empty !== null);
      }
      case "not-behind":
        // `(?<!\\w)` and its kin fail after a word character.
        return info(EMPTY(), FULL(), false, false, /^(\\w|\[[^\]]*\\w[^\]]*\])$/.test(bodySource));
      case "not-ahead":
      case "behind":
        return info(EMPTY(), FULL());
      default:
        return body;
    }
  }

  escape(): Info {
    this.pos++; // backslash
    const c = this.src[this.pos++]!;
    switch (c) {
      case "b":
        return info(EMPTY(), FULL(), false, false, true);
      case "B":
        return info(EMPTY(), FULL());
      case "G":
        return info(EMPTY(), FULL(), false, true);
      case "w":
        return info(WORD());
      case "W":
      case "S":
      case "D":
        return info(FULL());
      case "s":
        return info(SPACE());
      case "d":
        return info(DIGIT());
      case "h":
        return info(union(union(range(48, 57), range(65, 70)), range(97, 102)));
      case "N":
        return info(complement(single(10)));
      case "n":
        return info(single(10));
      case "t":
        return info(single(9));
      case "p": {
        const m = /^\{(\^?)(\w+)\}/.exec(this.src.slice(this.pos));
        if (!m) throw new Error(`regex ${this.src}: \\p`);
        this.pos += m[0].length;
        return info(FULL());
      }
      default:
        if (/[1-9]/.test(c)) return info(union(single(39), single(34))); // a quote, which every backreference repeats
        return info(single(c.charCodeAt(0)));
    }
  }

  charClass(): Info {
    this.pos++; // [
    let negated = false;
    if (this.src[this.pos] === "^") {
      negated = true;
      this.pos++;
    }
    let set = EMPTY();
    let first = true;
    while (this.pos < this.src.length && (this.src[this.pos] !== "]" || first)) {
      first = false;
      let c = this.src[this.pos]!;
      if (c === "[" && this.src.startsWith("[:", this.pos)) {
        const m = /^\[:(\^?)(\w+):\]/.exec(this.src.slice(this.pos))!;
        this.pos += m[0].length;
        const ascii =
          m[2] === "alpha" ? union(range(65, 90), range(97, 122)) : m[2] === "upper" ? range(65, 90) : FULL();
        set = union(set, m[2] === "alpha" || m[2] === "upper" ? union(ascii, nonAscii()) : FULL());
        continue;
      }
      if (c === "[") {
        set = union(set, this.charClass().set);
        continue;
      }
      if (c === "\\" && this.src[this.pos + 1] === "x") {
        const hex = (at: number) => parseInt(this.src.slice(at, at + 2), 16);
        const lo = hex(this.pos + 2);
        this.pos += 4;
        let hi = lo;
        if (this.src[this.pos] === "-" && this.src[this.pos + 1] !== "]") {
          this.pos++;
          if (this.src[this.pos] === "\\" && this.src[this.pos + 1] === "x") {
            hi = hex(this.pos + 2);
            this.pos += 4;
          } else hi = this.src.charCodeAt(this.pos++);
        }
        set = union(set, range(lo, hi));
        if (hi >= 0x80) set = union(set, nonAscii());
        continue;
      }
      if (c === "\\") {
        this.pos++;
        c = this.src[this.pos++]!;
        if (c === "w") set = union(set, WORD());
        else if (c === "s") set = union(set, SPACE());
        else if (c === "d") set = union(set, DIGIT());
        else if (c === "h") set = union(set, union(union(range(48, 57), range(65, 70)), range(97, 102)));
        else if (c === "n") add(set, 10);
        else if (c === "t") add(set, 9);
        else if (c === "S" || c === "W" || c === "D") set = FULL();
        else add(set, c.charCodeAt(0));
        continue;
      }
      this.pos++;
      const lo = new TextEncoder().encode(c)[0]!;
      if (this.src[this.pos] === "-" && this.src[this.pos + 1] !== "]") {
        this.pos++;
        let hi = this.src[this.pos++]!;
        if (hi === "\\") hi = this.src[this.pos++]!;
        set = union(set, range(lo, hi.charCodeAt(0)));
      } else {
        add(set, lo);
        if (lo >= 0x80) set = union(set, nonAscii());
      }
    }
    this.pos++; // ]
    return info(negated ? union(complement(set), nonAscii()) : set);
  }
}

function startOf(regex: string): { set: Set; bol: boolean; any: boolean; wb: boolean } {
  let result: Info;
  let ci = false;
  let source = regex;
  if (source.startsWith("(?i)")) {
    ci = true;
    source = source.slice(4);
  }
  try {
    result = new RegexParser(source).parse();
  } catch {
    // Syntax the analysis does not read: a match can start anywhere.
    return { set: FULL(), bol: false, any: true, wb: false };
  }
  let set = result.set;
  if (ci || source.includes("(?i")) {
    for (let b = 65; b <= 90; b++) {
      const has = (set[b >> 6]! >> BigInt(b & 63)) & 1n || (set[(b + 32) >> 6]! >> BigInt((b + 32) & 63)) & 1n;
      if (has) {
        add(set, b);
        add(set, b + 32);
      }
    }
    // Onig folds the long s (C5), the st ligatures (EF) and the Kelvin sign (E2).
    const has = (b: number) => ((set[b >> 6]! >> BigInt(b & 63)) & 1n) !== 0n;
    if (has(115)) {
      add(set, 0xc5);
      add(set, 0xef);
    }
    if (has(107)) add(set, 0xe2);
  }
  if (result.empty !== null) set = union(set, result.empty);
  return { set, bol: result.bol, any: result.any, wb: result.wb };
}

let starts = `

impl Pat {
${tooLong}    pub(crate) fn start(self) -> Start {
        match self {
`;
const hex = (w: bigint) => {
  const digits = w.toString(16).padStart(16, "0");
  return `0x${digits.match(/.{4}/g)!.join("_")}`;
};
starts += arms(
  patList.map((p): [string, string] => {
    const { set, bol, any, wb } = startOf(p.regex);
    const words = (any ? FULL() : set).map(hex).join(", ");
    return [p.id, `Start {\n                bytes: [${words}],\n                bol: ${bol},\n                wb: ${wb},\n            }`];
  }),
  "Pat",
);
starts += `        }\n    }\n}\n`;
g += starts;

await Bun.write(`${root_dir}src/css/grammar.rs`, g);

// Scope table.
const idents = [...scopes.entries()].sort((a, b) => a[1].localeCompare(b[1]));
let s = `//! The scope names the lexer emits, in Shiki's naming. Generated by
//! \`oracle/css-grammar.ts\`; do not edit.

macro_rules! scopes {
    ($($id:ident $name:literal)*) => {
        /// A scope name; the discriminant indexes [\`NAMES\`].
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        #[expect(
            clippy::enum_variant_names,
            reason = "the names are the scope names, and the unit s ends in S"
        )]
        pub(crate) enum S { $($id),* }

        impl S {
            /// The scope in the crate's shared numbering.
            #[expect(
                clippy::as_conversions,
                reason = "a fieldless \`repr(u8)\` enum has no \`From\` impl for its discriminant"
            )]
            pub(crate) const fn scope(self) -> crate::scope::Scope {
                crate::scope::Scope::new(crate::scope::CSS, self as u16)
            }
        }

        /// The name of every scope, indexed by discriminant.
        pub(crate) const NAMES: &[&str] = &[$($name),*];
    };
}

scopes! {
`;
for (const [ident, name] of idents) s += `    ${ident} "${name}"\n`;
s += "}\n";
await Bun.write(`${root_dir}src/css/scope.rs`, s);
if (Bun.env.CSS_DUMP) await Bun.write(Bun.env.CSS_DUMP, JSON.stringify(patList.map((p) => ({ id: p.id, kind: p.act.kind, regex: p.regex, captures: p.captures, nested: p.nested })), null, 0));
console.log(`${ruleList.length} rules, ${patList.length} patterns, ${scopes.size} scopes`);
