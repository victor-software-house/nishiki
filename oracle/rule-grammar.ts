// The pieces every generator of a rule-engine grammar shares: the analysis of
// the bytes a pattern's match can start at, the tables of those bytes, the
// match arms that group equal table rows, the scope file, and the check that
// a grammar's `GROUPS` holds every capture group its tables name.
//
// Each `oracle/<language>-grammar.ts` imports these and keeps only what its
// grammar needs of its own.

import { $ } from "bun";

/** Writes a generated Rust file and formats it as `mise run format` does,
 * with the repository's `rustfmt.toml` and the formatting nightly that
 * `RUSTFMT_TOOLCHAIN` names in `mise.toml`, so a run leaves a committed table
 * unchanged. */
export async function writeRust(path: string, text: string): Promise<void> {
  const toolchain = Bun.env["RUSTFMT_TOOLCHAIN"];
  if (toolchain === undefined) {
    throw new Error("RUSTFMT_TOOLCHAIN is unset: run the generator where mise has loaded mise.toml");
  }
  await Bun.write(path, text);
  await $`rustfmt ${toolchain} ${path}`;
}

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

/** What a backreference repeats, for the bytes it can start with: a quote,
 * as in the grammars whose backreferences close strings, or anything, as in
 * Ruby's, which repeat a heredoc's name or a percent literal's delimiter. */
export type Backref = "quote" | "any";

class RegexParser {
  pos = 0;
  groups = 0;
  constructor(
    readonly src: string,
    readonly backref: Backref,
  ) {}

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

  /** An atom and its quantifiers. An interval can be optional three ways in
   * Oniguruma: `{0,m}`, `{,m}`, and `{n,m}` with n above m, which it reads as
   * `{m,n}` (C++'s doc comments open with `\s+{1,0}`). */
  quantified(): Info {
    const atom = this.atom();
    let min = 1;
    for (;;) {
      const c = this.src[this.pos];
      const interval = c === "{" ? /^\{(?:(\d+)(?:(,)(\d*))?|,(\d+))\}/.exec(this.src.slice(this.pos)) : null;
      if (c === "*" || c === "?") {
        min = 0;
        this.pos++;
      } else if (c === "+") {
        this.pos++;
      } else if (interval) {
        const [whole, low, comma, high, upTo] = interval;
        const from = upTo === undefined ? Number(low) : 0;
        const to = comma === undefined || high === "" ? from : Number(high);
        if (Math.min(from, to) === 0) min = 0;
        this.pos += whole.length;
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
      } else if (/^\?<\w+>/.test(rest)) {
        this.pos += /^\?<\w+>/.exec(rest)![0].length;
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
      // `\B` and the string anchors `\A`, `\z` and `\Z` match no character.
      case "B":
      case "A":
      case "z":
      case "Z":
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
      case "g": {
        // A call of a named group, which recurses: any byte can start it.
        const m = /^<\w+>/.exec(this.src.slice(this.pos));
        if (!m) throw new Error(`regex ${this.src}: \\g`);
        this.pos += m[0].length;
        return info(FULL());
      }
      case "p": {
        const m = /^\{(\^?)(\w+)\}/.exec(this.src.slice(this.pos));
        if (!m) throw new Error(`regex ${this.src}: \\p`);
        this.pos += m[0].length;
        return info(propertySet(m[1] === "^", m[2]!));
      }
      case "P": {
        this.pos += /^\{[^}]*\}/.exec(this.src.slice(this.pos))![0].length;
        return info(FULL());
      }
      case "x":
      case "u": {
        const m = /^(?:\{([0-9A-Fa-f]+)\}|([0-9A-Fa-f]{2,4}))/.exec(this.src.slice(this.pos));
        if (!m) return info(single(c.charCodeAt(0)));
        this.pos += m[0].length;
        const cp = parseInt((m[1] ?? m[2])!, 16);
        return info(codePoints(cp, cp));
      }
      case "k": {
        // A back reference to a named group repeats text the group matched.
        const m = /^<\w+>/.exec(this.src.slice(this.pos));
        if (!m) throw new Error(`regex ${this.src}: \\k`);
        this.pos += m[0].length;
        return info(FULL());
      }
      default:
        if (/[1-9]/.test(c)) return info(this.backref === "quote" ? union(single(39), single(34)) : FULL());
        return info(single(c.charCodeAt(0)));
    }
  }

  /** The code point of a class member or escape at `pos`, advancing past it;
   * null for a class escape such as `\\w`, whose set goes to `into`. */
  classMember(into: { set: Set; exact: boolean }): number | null {
    const c = this.src.codePointAt(this.pos)!;
    this.pos += c > 0xffff ? 2 : 1;
    if (c !== 0x5c) return c;
    const e = this.src[this.pos++]!;
    switch (e) {
      case "w":
        into.set = union(into.set, WORD());
        return null;
      case "s":
        into.set = union(into.set, SPACE());
        return null;
      case "d":
        into.set = union(into.set, DIGIT());
        return null;
      case "h":
        into.set = union(into.set, union(union(range(48, 57), range(65, 70)), range(97, 102)));
        return null;
      case "S":
        into.set = union(into.set, union(complement(SPACE()), nonAscii()));
        return null;
      case "W":
        into.set = union(into.set, union(complement(WORD()), nonAscii()));
        return null;
      case "D":
        into.set = union(into.set, union(complement(DIGIT()), nonAscii()));
        return null;
      case "P":
        this.pos += /^\{[^}]*\}/.exec(this.src.slice(this.pos))![0].length;
        into.set = FULL();
        into.exact = false;
        return null;
      case "p": {
        const m = /^\{(\^?)(\w+)\}/.exec(this.src.slice(this.pos));
        if (!m) throw new Error(`regex ${this.src}: \\p`);
        this.pos += m[0].length;
        const property = propertySet(m[1] === "^", m[2]!);
        // A property the analysis does not know is every byte, inexact.
        if (property.every((w) => w === 0xffffffffffffffffn)) into.exact = false;
        into.set = union(into.set, property);
        return null;
      }
      case "n":
        return 10;
      case "t":
        return 9;
      case "r":
        return 13;
      case "x":
      case "u": {
        const m = /^(?:\{([0-9A-Fa-f]+)\}|([0-9A-Fa-f]{2,4}))/.exec(this.src.slice(this.pos));
        if (!m) return e.charCodeAt(0);
        this.pos += m[0].length;
        return parseInt((m[1] ?? m[2])!, 16);
      }
      default:
        return e.codePointAt(0)!;
    }
  }

  /** A class, with whether its set is exact on ASCII bytes, which a
   * negated class needs to take the complement. */
  charClass(): Info & { exact: boolean } {
    this.pos++; // [
    let negated = false;
    if (this.src[this.pos] === "^") {
      negated = true;
      this.pos++;
    }
    const into = { set: EMPTY(), exact: true };
    let first = true;
    while (this.pos < this.src.length && (this.src[this.pos] !== "]" || first)) {
      first = false;
      if (this.src[this.pos] === "[" && this.src.startsWith("[:", this.pos)) {
        const m = /^\[:(\^?)(\w+):\]/.exec(this.src.slice(this.pos))!;
        this.pos += m[0].length;
        const letters = union(range(65, 90), range(97, 122));
        const known = m[1] === "" && (m[2] === "alpha" || m[2] === "upper");
        if (!known) into.exact = false;
        into.set = union(
          into.set,
          m[2] === "alpha" ? union(letters, nonAscii()) : m[2] === "upper" ? union(range(65, 90), nonAscii()) : FULL(),
        );
        continue;
      }
      if (this.src[this.pos] === "[") {
        // A nested class adds its members, as Oniguruma reads `[a[^b]]`.
        const nested = this.charClass();
        into.set = union(into.set, nested.set);
        into.exact &&= nested.exact;
        continue;
      }
      const lo = this.classMember(into);
      if (lo === null) continue;
      if (this.src[this.pos] === "-" && this.src[this.pos + 1] !== "]") {
        this.pos++;
        const hi = this.classMember(into);
        if (hi === null) throw new Error(`regex ${this.src}: range to a class`);
        into.set = union(into.set, codePoints(lo, hi));
      } else {
        into.set = union(into.set, codePoints(lo, lo));
      }
    }
    this.pos++; // ]
    // A negated class matches every non-ASCII character its members leave
    // out, so its first bytes are the ASCII complement plus every lead byte.
    if (!negated) return { ...info(into.set), exact: into.exact };
    const set = into.exact ? union(complement(into.set), nonAscii()) : FULL();
    return { ...info(set), exact: into.exact };
  }
}

/** The first bytes of the UTF-8 encodings of the code points `lo` to `hi`. */
function codePoints(lo: number, hi: number): Set {
  let set = EMPTY();
  if (lo < 0x80) set = union(set, range(lo, Math.min(hi, 0x7f)));
  if (hi >= 0x80) set = union(set, nonAscii());
  return set;
}

/** The first bytes of the characters with a Unicode property, over-approximated. */
function propertySet(negated: boolean, name: string): Set {
  if (negated) return FULL();
  if (/^(L|Alphabetic|Letter|L[ulmto]|Lowercase|Uppercase|Alpha)$/.test(name)) return union(union(range(65, 90), range(97, 122)), nonAscii());
  if (/^(N|N[dlo]|Number|Digit)$/.test(name)) return union(range(48, 57), nonAscii());
  if (/^(M|Mn|Mc|Me|Mark)$/.test(name)) return nonAscii();
  return FULL();
}


/** The bytes a match of `regex` can start at, and its anchors. */
export function startOf(
  regex: string,
  backref: Backref = "quote",
): { set: Set; bol: boolean; any: boolean; wb: boolean } {
  let result: Info;
  let ci = false;
  let source = regex;
  if (source.startsWith("(?i)")) {
    ci = true;
    source = source.slice(4);
  }
  try {
    result = new RegexParser(source, backref).parse();
  } catch {
    // Syntax the analysis does not read: a match can start anywhere.
    return { set: FULL(), bol: false, any: true, wb: false };
  }
  // A match that starts with a lookaround starts at the bytes the lookaround
  // reads, which `(?i)` folds like the rest.
  let set = result.empty === null ? result.set : union(result.set, result.empty);
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
  return { set, bol: result.bol, any: result.any, wb: result.wb };
}

/** Groups equal bodies into one arm of a `match` on `type`. */
export function arms(entries: [string, string][], type: string): string {
  const groups = new Map<string, string[]>();
  for (const [id, body] of entries) groups.set(body, [...(groups.get(body) ?? []), `${type}::${id}`]);
  let out = "";
  for (const [body, ids] of groups) out += `            ${ids.join("\n            | ")} => ${body},\n`;
  return out;
}

/** The attribute a generated table function carries. */
export const tooLong = `    #[expect(clippy::too_many_lines, reason = "a generated table")]\n`;

const hex = (w: bigint) => {
  const digits = w.toString(16).padStart(16, "0");
  return `0x${digits.match(/.{4}/g)!.join("_")}`;
};

/** `Pat::start`: the bytes each pattern can start at, and its anchors. */
export function startTable(pats: { id: string; regex: string }[], backref: Backref = "quote"): string {
  let starts = `

impl Pat {
${tooLong}    pub(crate) fn start(self) -> Start {
        match self {
`;
  starts += arms(
    pats.map((p): [string, string] => {
      const { set, bol, any, wb } = startOf(p.regex, backref);
      const words = (any ? FULL() : set).map(hex).join(", ");
      return [p.id, `Start {\n                bytes: [${words}],\n                bol: ${bol},\n                wb: ${wb},\n            }`];
    }),
    "Pat",
  );
  starts += `        }\n    }\n}\n`;
  return starts;
}

/** How a `Rule` of the grammar `engine` (such as `Css`) is stored as a frame
 * of the interned state: its language's tag, the constant `tag` of
 * `src/tag.rs`, in the high bits, and its position in `Rule::ALL` in the low
 * `bits`, 8 unless the grammar has more rules than a byte numbers (C++, whose
 * tag claims a wider range). Then its `Tabled` implementation, which serves
 * the pattern tables above to `rules::table_scan`. */
export function ruleCodes(engine: string, tag: string, bits = 8): string {
  const low = (1 << bits) - 1;
  const hex = (n: number) => `0x${n.toString(16)}`;
  const repr = bits > 8 ? "u16" : "u8";
  const [high, rest] = bits === 8 ? ["byte", "one"] : ["bits", `${bits} bits`];
  return `
impl Rule {
    /// The rule as a frame stores it: the language's tag in the high ${high}, the
    /// position in [\`Rule::ALL\`] in the low ${rest}.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless \`repr(${repr})\` enum has no \`From\` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        crate::tag::${tag} | self as u16
    }

    /// The rule a frame's code names; [\`Rule::Root\`] for a code that is
    /// not the language's, such as the empty state's.
    pub(crate) fn from_code(code: u16) -> Rule {
        if code & ${hex(0xffff & ~low)} != crate::tag::${tag} {
            return Rule::Root;
        }
        Rule::ALL
            .get(usize::from(code & ${hex(low)}))
            .copied()
            .unwrap_or(Rule::Root)
    }
}

impl rules::Tabled for ${engine} {
    fn pat_start(pat: Pat) -> rules::Start {
        pat.start()
    }

    fn tables(rule: Rule) -> &'static rules::Tables<Self> {
        static TABLES: [std::sync::OnceLock<rules::Tables<${engine}>>; Rule::ALL.len()] =
            [const { std::sync::OnceLock::new() }; Rule::ALL.len()];
        #[expect(
            clippy::as_conversions,
            reason = "a fieldless \`repr(${repr})\` enum has no \`From\` impl"
        )]
        let index = rule as usize;
        TABLES[index].get_or_init(move || rules::Tables::build(rule.data()))
    }
}
`;
}

/** A compile-time check that `GROUPS` of the grammar `engine` (such as
 * `Css`) holds the highest capture group its tables name: an index past it
 * would panic when the engine records or reads the group. */
export function groupsCheck(engine: string, groups: number[]): string {
  const needed = Math.max(0, ...groups) + 1;
  return `
const _: () = assert!(
    ${needed} <= <super::engine::${engine} as crate::rules::Grammar>::GROUPS,
    "a pattern's table names a capture group past the grammar's GROUPS"
);
`;
}

/** The scope file: every scope name the grammar's tables use, sorted, in
 * the crate's `scope_table!` for `table` (such as `CSS`). `attributes` go on
 * the generated enum. */
export function scopeFile(generator: string, table: string, scopes: Map<string, string>, attributes = ""): string {
  const idents = [...scopes.entries()].sort((a, b) => a[1].localeCompare(b[1]));
  let s = `//! The scope names the lexer emits, in Shiki's naming. Generated by
//! \`oracle/${generator}\`; do not edit.

crate::scope::scope_table! {
${attributes}    ${table};
`;
  for (const [ident, name] of idents) s += `    ${ident} "${name}"\n`;
  s += "}\n";
  return s;
}
