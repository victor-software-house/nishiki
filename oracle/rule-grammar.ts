// The pieces every generator of a rule-engine grammar shares: the analysis of
// the bytes a pattern's match can start at, the tables of those bytes, the
// match arms that group equal table rows, the scope file, and the check that
// a grammar's `GROUPS` holds every capture group its tables name.
//
// Each `oracle/<language>-grammar.ts` imports these and keeps only what its
// grammar needs of its own.

import { $ } from "bun";

/** Writes a generated Rust file and formats it as `mise run format` does.
 * It uses the repository's `rustfmt.toml` and the formatting nightly that
 * `RUSTFMT_TOOLCHAIN` names in `mise.toml`.
 * A run then leaves a committed table unchanged. */
export async function writeRust(path: string, text: string): Promise<void> {
  const toolchain = Bun.env["RUSTFMT_TOOLCHAIN"];
  if (toolchain === undefined) {
    throw new Error(
      "RUSTFMT_TOOLCHAIN is unset: run the generator where mise has loaded mise.toml",
    );
  }
  await Bun.write(path, text);
  await $`rustfmt ${toolchain} ${path}`;
}

/** Writes `codegen/patterns/<dir>.json`: each pattern's role and regular
 * expression, keyed by its id, for the grammar of `src/<dir>/`.
 * codegen/ reads it and decides what the grammar's tables hold.
 * This script only writes the data. */
export async function writePatterns(
  dir: string,
  patterns: { id: string; regex: string; act: { role: string } }[],
): Promise<void> {
  const byId = Object.fromEntries(
    patterns.map((p) => [p.id, { role: p.act.role, regex: p.regex }]),
  );
  const root = new URL("..", import.meta.url).pathname;
  await Bun.write(`${root}codegen/patterns/${dir}.json`, `${JSON.stringify(byId, null, 2)}\n`);
}

// First bytes. For each pattern, the bytes a match can start at, so that the
// engine tries it only where one stands. The analysis reads the regular
// expression, over-approximating: a byte it leaves out cannot start a match.

type Set = bigint[]; // four 64-bit words, bit b of word b >> 6.
const FULL = (): Set => [
  0xffffffffffffffffn,
  0xffffffffffffffffn,
  0xffffffffffffffffn,
  0xffffffffffffffffn,
];
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

/** The analysis of a regex fragment.
 * `safe` says that a match of the fragment cannot start inside a word, or, if
 * it does, can start one byte earlier, so a scan that reaches the word's
 * start first never needs it. Left out, it follows from `set` and `wb`.
 * `moves` says that `safe` holds only because a run of a class with every
 * ASCII word character can move a match's start back to the word's start. */
type Analysis = {
  set: Set;
  empty: Set | null;
  bol: boolean;
  any: boolean;
  wb: boolean;
  behind: boolean;
  safe?: boolean;
  moves?: boolean;
};

/** Whether a match of `fragment` that starts inside a word is impossible or can
 * start earlier: the fragment consumes no ASCII word character first, or passes
 * a word boundary before, or says so itself. */
const safeInWord = (fragment: Analysis): boolean =>
  fragment.safe ?? (fragment.wb || intersect(fragment.set, ASCII_WORD()).every((w) => w === 0n));

/** Whether `fragment` can match empty only where a condition holds, beside paths
 * that consume: a lookaround, an anchor or `\b` on its empty path, as in
 * `(?:(?=b)|-)` or `-?(?=b)` (Kody, nishiki-archive#632).
 * The flags may come from a consuming path too, which only makes the answer
 * stricter. */
const conditionedEmpty = (fragment: Analysis): boolean =>
  fragment.empty !== null &&
  (!fragment.empty.every((w) => w === 0xffffffffffffffffn) ||
    fragment.bol ||
    fragment.wb ||
    fragment.behind);

/** One character: a literal, an escape, a class or `.`, perhaps in a group. */
const ONE_CHAR =
  /^(?:\((?:\?:)?)?(?:\\p\{[^}]*\}|\\.|\[(?:\\.|\[:[a-z]+:\]|[^\]\\])*\]|[^\\()[\]|*+?{])\)?$/;
const analysis = (
  set: Set,
  empty: Set | null = null,
  bol = false,
  any = false,
  wb = false,
  behind = false,
): Analysis => ({
  set,
  empty,
  bol,
  any,
  wb,
  behind,
});

const WORD = () =>
  union(union(union(range(48, 57), range(65, 90)), union(range(97, 122), single(95))), nonAscii());
const ASCII_WORD = () =>
  union(union(range(48, 57), range(65, 90)), union(range(97, 122), single(95)));
const SPACE = () => union(union(single(32), range(9, 13)), nonAscii());
const DIGIT = () => union(range(48, 57), nonAscii());

/** What a backreference repeats, for the bytes it can start with: a quote,
 * as in the grammars whose backreferences close strings, or anything, as in
 * Ruby's, which repeat a heredoc's name or a percent literal's delimiter. */
export type Backref = "quote" | "any";

class RegexParser {
  pos = 0;
  groups = 0;
  /** Whether the pattern repeats a group's text, calls a group, or is
   * anchored with `\G`, so a match cannot simply start earlier. */
  readonly refs: boolean;
  constructor(
    readonly src: string,
    readonly backref: Backref,
    /** Whether a back reference or a call of a group may match the empty
     * string, as it can when the group it names does. */
    readonly refsMayBeEmpty = false,
    /** Whether a match inside a word could start earlier, at the word's
     * start, which holds past the scan's first position only. */
    readonly runsStartEarlier = true,
  ) {
    this.refs = /\\(?:[1-9]|[kg]<|G)/.test(src);
  }

  /** The empty path of a back reference or a call: every byte may follow,
   * when such a reference may match empty. */
  refEmpty(): Set | null {
    return this.refsMayBeEmpty ? FULL() : null;
  }

  parse(): Analysis {
    const result = this.alternation();
    if (this.pos !== this.src.length) throw new Error(`regex ${this.src}: stopped at ${this.pos}`);
    return result;
  }

  alternation(): Analysis {
    const branches: Analysis[] = [this.sequence()];
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
    return {
      ...analysis(
        set,
        empty,
        branches.every((b) => b.bol),
        any,
        branches.every((b) => b.wb),
        branches.every((b) => b.behind),
      ),
      safe: branches.every(safeInWord),
      moves: branches.some((b) => b.moves === true),
    };
  }

  /** A run of parts.
   * A lookbehind counts toward `behind` only while no part before it can
   * consume a byte.
   * In `x?(?<=x)y` a match can start at `x`, where the lookbehind does not
   * apply.
   * A `\b` or a lookbehind counts toward `wb` only while every part before it
   * is safe inside a word (`safeInWord`).
   * Inside a word the next byte is a word character, so `\s*` in `\s*\bx`
   * must match empty there.
   * In `(?:[-\w]*\\)?\bx` a match inside a word can start at the word's
   * start instead.
   * That move fails after a zero-width part, which consumes nothing: a
   * lookaround or an anchor that holds at the match's start inside the word
   * need not hold at the word's start, as `(?=b)` in `(?=b)\w*\b-` does
   * not on "ab-".
   * A `\G` counts toward `any` only while every part before it can match
   * empty.
   * A match starts at or after the search start, so a `\G` after a consumed
   * byte never holds: Hack's class list writes `Gender\Gender` with one
   * backslash. */
  sequence(): Analysis {
    let set = EMPTY();
    let empty: Set | null = FULL(); // the empty path's constraint, while it lasts
    let bol = false;
    let wb = false;
    let behind = false;
    let safe = true;
    let guarded = false;
    let conditioned = false;
    let moves = false;
    let any = false;
    while (this.pos < this.src.length && this.src[this.pos] !== "|" && this.src[this.pos] !== ")") {
      const part = this.quantified();
      if (empty !== null) {
        if (part.any) any = true;
        const first = set.every((w) => w === 0n);
        set = union(set, part.set);
        if (part.bol) bol = true;
        if (part.wb && safe) wb = true;
        if (part.wb && safe) guarded = true;
        const safeHere = safeInWord(part) && !(part.moves === true && conditioned);
        if (!guarded && !safeHere) safe = false;
        if (!guarded && part.moves === true && safeHere) moves = true;
        if (part.set.every((w) => w === 0n) || conditionedEmpty(part)) conditioned = true;
        if (part.behind && first) behind = true;
        if (part.empty === null) empty = null;
        else empty = intersect(empty, part.empty);
      }
    }
    return { ...analysis(set, empty, bol, any, wb, behind), safe, moves };
  }

  /** An atom and its quantifiers. An interval can be optional three ways in
   * Oniguruma: `{0,m}`, `{,m}`, and `{n,m}` with n above m, which it reads as
   * `{m,n}` (C++'s doc comments open with `\s+{1,0}`). */
  quantified(): Analysis {
    const start = this.pos;
    const atom = this.atom();
    const one = ONE_CHAR.test(this.src.slice(start, this.pos));
    let min = 1;
    let unbounded = false;
    for (;;) {
      const c = this.src[this.pos];
      const interval =
        c === "{" ? /^\{(?:(\d+)(?:(,)(\d*))?|,(\d+))\}/.exec(this.src.slice(this.pos)) : null;
      if (c === "*" || c === "?") {
        min = 0;
        unbounded = unbounded || c === "*";
        this.pos++;
      } else if (c === "+") {
        unbounded = true;
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
    // A run of a class that holds every ASCII word character can take the
    // word characters before it too, so a match inside a word can start at
    // the word's start, with the same groups unless the pattern repeats one.
    const covers = intersect(ASCII_WORD(), complement(atom.set)).every((w) => w === 0n);
    const extends_ = this.runsStartEarlier && one && unbounded && covers && !this.refs;
    const alone = safeInWord(atom);
    const safe = extends_ || alone;
    const moves = alone ? atom.moves === true : extends_;
    if (min === 0) return { ...analysis(atom.set, FULL(), false, atom.any), safe, moves };
    return { ...atom, safe, moves };
  }

  atom(): Analysis {
    const c = this.src[this.pos]!;
    if (c === "(") return this.group();
    if (c === "[") return this.charClass();
    if (c === "^") {
      this.pos++;
      return analysis(EMPTY(), FULL(), true);
    }
    if (c === "$") {
      this.pos++;
      return analysis(EMPTY(), FULL());
    }
    if (c === ".") {
      this.pos++;
      return analysis(complement(single(10)));
    }
    if (c === "\\") return this.escape();
    this.pos++;
    const bytes = new TextEncoder().encode(c);
    return analysis(single(bytes[0]!));
  }

  group(): Analysis {
    this.pos++; // (
    let form = "capture";
    if (this.src[this.pos] === "?") {
      const rest = this.src.slice(this.pos);
      // A flag group such as `(?i)` in the middle of the expression matches
      // the empty string. `startOf` folds the whole set where `(?i` appears.
      const flags = /^\?[imx-]+\)/.exec(rest);
      if (flags) {
        this.pos += flags[0].length;
        return analysis(EMPTY(), FULL());
      }
      const scoped = /^\?[imx-]+:/.exec(rest);
      if (scoped) {
        form = "plain";
        this.pos += scoped[0].length;
      } else if (rest.startsWith("?:") || rest.startsWith("?>")) {
        form = "plain";
        this.pos += 2;
      } else if (rest.startsWith("?=")) {
        form = "ahead";
        this.pos += 2;
      } else if (rest.startsWith("?!")) {
        form = "not-ahead";
        this.pos += 2;
      } else if (rest.startsWith("?<=")) {
        form = "behind";
        this.pos += 3;
      } else if (rest.startsWith("?<!")) {
        form = "not-behind";
        this.pos += 3;
      } else if (/^\?<\w+>/.test(rest)) {
        this.pos += /^\?<\w+>/.exec(rest)![0].length;
      } else {
        throw new Error(`regex ${this.src}: group ${rest.slice(0, 4)}`);
      }
    }
    if (form === "capture") this.groups++;
    const bodyStart = this.pos;
    const body = this.alternation();
    const bodySource = this.src.slice(bodyStart, this.pos);
    if (this.src[this.pos] !== ")") throw new Error(`regex ${this.src}: unclosed group`);
    this.pos++;
    switch (form) {
      case "ahead": {
        // Zero-width; where the body cannot be empty, its first bytes are a
        // condition on the position.
        const constraint = body.empty === null ? union(body.set, EMPTY()) : FULL();
        return analysis(EMPTY(), constraint, false, body.any && body.empty !== null);
      }
      case "not-behind":
        // `(?<!\\w)` and its kin fail after a word character.
        return analysis(
          EMPTY(),
          FULL(),
          false,
          false,
          /^(\\w|\[[^\]]*\\w[^\]]*\])$/.test(bodySource),
        );
      case "behind": {
        // A body that cannot be empty needs a byte before the match, which
        // the start of the line lacks. A body of one character with no ASCII
        // word character also fails inside a word.
        const one = /^(?:\\.|\[(?:\\.|\[:[a-z]+:\]|[^\]\\])*\]|[^\\()[\]|*+?{.])$/.test(bodySource);
        const word = intersect(body.set, ASCII_WORD()).some((w) => w !== 0n);
        return analysis(EMPTY(), FULL(), false, false, one && !word, body.empty === null);
      }
      case "not-ahead":
        return analysis(EMPTY(), FULL());
      default:
        return body;
    }
  }

  escape(): Analysis {
    this.pos++; // backslash
    const c = this.src[this.pos++]!;
    switch (c) {
      case "b":
        return analysis(EMPTY(), FULL(), false, false, true);
      // `\B` and the string anchors `\A`, `\z` and `\Z` match no character.
      case "B":
      case "A":
      case "z":
      case "Z":
        return analysis(EMPTY(), FULL());
      case "G":
        return analysis(EMPTY(), FULL(), false, true);
      case "w":
        return analysis(WORD());
      case "W":
      case "S":
      case "D":
        return analysis(FULL());
      case "s":
        return analysis(SPACE());
      case "d":
        return analysis(DIGIT());
      case "h":
        return analysis(union(union(range(48, 57), range(65, 70)), range(97, 102)));
      case "N":
        return analysis(complement(single(10)));
      case "n":
        return analysis(single(10));
      case "t":
        return analysis(single(9));
      case "g": {
        // A call of a named group, which recurses: any byte can start it.
        const m = /^<\w+>/.exec(this.src.slice(this.pos));
        if (!m) throw new Error(`regex ${this.src}: \\g`);
        this.pos += m[0].length;
        return analysis(FULL(), this.refEmpty());
      }
      case "p": {
        const m = /^\{(\^?)(\w+)\}/.exec(this.src.slice(this.pos));
        if (!m) throw new Error(`regex ${this.src}: \\p`);
        this.pos += m[0].length;
        return analysis(propertySet(m[1] === "^", m[2]!));
      }
      case "P": {
        this.pos += /^\{[^}]*\}/.exec(this.src.slice(this.pos))![0].length;
        return analysis(FULL());
      }
      case "x":
      case "u": {
        const m = hexEscape(c, this.src.slice(this.pos));
        if (!m) return analysis(single(c.charCodeAt(0)));
        this.pos += m[0].length;
        const cp = parseInt((m[1] ?? m[2])!, 16);
        return analysis(codePoints(cp, cp));
      }
      case "k": {
        // A back reference to a named group repeats text the group matched.
        const m = /^<\w+>/.exec(this.src.slice(this.pos));
        if (!m) throw new Error(`regex ${this.src}: \\k`);
        this.pos += m[0].length;
        return analysis(FULL(), this.refEmpty());
      }
      default:
        if (/[1-9]/.test(c))
          return analysis(
            this.backref === "quote" ? union(single(39), single(34)) : FULL(),
            this.refEmpty(),
          );
        return analysis(single(c.charCodeAt(0)));
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
        const m = hexEscape(e, this.src.slice(this.pos));
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
  charClass(): Analysis & { exact: boolean } {
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
          m[2] === "alpha"
            ? union(letters, nonAscii())
            : m[2] === "upper"
              ? union(range(65, 90), nonAscii())
              : FULL(),
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
    if (!negated) return { ...analysis(into.set), exact: into.exact };
    const set = into.exact ? union(complement(into.set), nonAscii()) : FULL();
    return { ...analysis(set), exact: into.exact };
  }
}

/** The digits of a `\x` or `\u` escape at the start of `rest`, after the letter.
 * Unbraced, `\x` takes at most two digits, as Oniguruma's `\xHH` does.
 * `\u` takes two to four. */
function hexEscape(letter: string, rest: string): RegExpExecArray | null {
  return letter === "x"
    ? /^(?:\{([0-9A-Fa-f]+)\}|([0-9A-Fa-f]{1,2}))/.exec(rest)
    : /^(?:\{([0-9A-Fa-f]+)\}|([0-9A-Fa-f]{2,4}))/.exec(rest);
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
  if (/^(L|Alphabetic|Letter|L[ulmto]|Lowercase|Uppercase|Alpha)$/.test(name))
    return union(union(range(65, 90), range(97, 122)), nonAscii());
  if (/^(N|N[dlo]|Number|Digit)$/.test(name)) return union(range(48, 57), nonAscii());
  if (/^(M|Mn|Mc|Me|Mark)$/.test(name)) return nonAscii();
  return FULL();
}

/** The number of capture groups in `regex`. */
function groupCount(regex: string): number {
  const parser = new RegexParser(regex, "any");
  parser.parse();
  return parser.groups;
}

/** Fails when the scope name `name` takes group `group` of a match of
 * `regex`, and `regex` lacks that group.
 * vscode-textmate keeps such a `$N` as written.
 * The rule engine would write nothing. */
export function requireGroup(name: string, group: number, regex: string): void {
  if (group > groupCount(regex))
    throw new Error(`${name} names group ${group}, which ${regex} lacks`);
}

/** The bytes a match of `regex` can start at, and its anchors. */
export function startOf(
  regex: string,
  backref: Backref = "quote",
): {
  set: Set;
  bol: boolean;
  any: boolean;
  wb: boolean;
  behind: boolean;
  atEnd: boolean;
  inWordFirst: boolean;
} {
  let result: Analysis;
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
    return {
      set: FULL(),
      bol: false,
      any: true,
      wb: false,
      behind: false,
      atEnd: true,
      inWordFirst: false,
    };
  }
  // A match that starts with a lookaround starts at the bytes the lookaround
  // reads, which `(?i)` folds like the rest.
  let set = result.empty === null ? result.set : union(result.set, result.empty);
  if (ci || source.includes("(?i")) {
    for (let b = 65; b <= 90; b++) {
      const has =
        (set[b >> 6]! >> BigInt(b & 63)) & 1n ||
        (set[(b + 32) >> 6]! >> BigInt((b + 32) & 63)) & 1n;
      if (has) {
        add(set, b);
        add(set, b + 32);
      }
    }
    // Onig folds the long s (C5), the sharp s (C3), the st ligatures (EF), the
    // f ligatures (EF) and the Kelvin sign (E2).
    const has = (b: number) => ((set[b >> 6]! >> BigInt(b & 63)) & 1n) !== 0n;
    if (has(115)) {
      add(set, 0xc3);
      add(set, 0xc5);
      add(set, 0xef);
    }
    if (has(102)) add(set, 0xef);
    if (has(107)) add(set, 0xe2);
  }
  // A match where no byte follows takes the empty path, and none of its
  // lookaheads may need a byte, so that path's constraint is every byte.
  // A back reference or a call can match empty, so that parse lets them.
  // Both parses run the same parser and differ only in how a reference
  // counts as empty, so this one cannot fail where the first passed.
  // Syntax handled only here would lose that, and its failure would escape
  // the catch above.
  const end = new RegexParser(source, backref, true).parse();
  const atEnd =
    end.any || (end.empty !== null && end.empty.every((w) => w === 0xffffffffffffffffn));
  // Without that case a pattern that `wb` keeps out of words only because a
  // match there can start earlier must still be tried where the scan starts.
  const strict = new RegexParser(source, backref, false, false).parse();
  const inWordFirst = result.wb && !strict.wb && !result.bol;
  return {
    set,
    bol: result.bol,
    any: result.any,
    wb: result.wb,
    behind: result.behind,
    atEnd,
    inWordFirst,
  };
}

/** The top-level alternatives of `regex`, looking inside a lookahead or a
 * group that spans the whole of it. */
function alternatives(regex: string): string[] {
  const out: string[] = [];
  let depth = 0;
  let inClass = false;
  let start = 0;
  for (let i = 0; i < regex.length; i++) {
    const c = regex[i];
    if (c === "\\") {
      i++;
    } else if (inClass) {
      if (c === "]") inClass = false;
    } else if (c === "[") {
      inClass = true;
    } else if (c === "(") {
      depth++;
    } else if (c === ")") {
      depth--;
    } else if (c === "|" && depth === 0) {
      out.push(regex.slice(start, i));
      start = i + 1;
    }
  }
  out.push(regex.slice(start));
  if (out.length > 1) return out.flatMap(alternatives);
  const whole = /^\((?:\?:|\?=)?(.*)\)$/s.exec(regex);
  if (whole !== null && balanced(whole[1]!)) return alternatives(whole[1]!);
  return leadingGroup(regex) ?? out;
}

/** The alternatives of `regex` when it starts with a capturing or `(?:`
 * group of several alternatives that no quantifier follows: each alternative
 * of the group, followed by the rest of `regex`.
 * So `(;|(^|\s)#\s).*$` reads as `;.*$`, `^#\s.*$` and `\s#\s.*$`.
 * `undefined` for any other `regex`. */
function leadingGroup(regex: string): string[] | undefined {
  if (regex[0] !== "(" || (regex[1] === "?" && regex[2] !== ":")) return undefined;
  const close = closingParen(regex);
  if (close < 0 || /^[*+?{]/.test(regex.slice(close + 1))) return undefined;
  const inner = alternatives(regex.slice(regex[1] === "?" ? 3 : 1, close));
  if (inner.length < 2) return undefined;
  const rest = regex.slice(close + 1);
  return inner.flatMap((alt) => alternatives(alt + rest));
}

/** Whether the parentheses of `regex` balance, so that a leading `(` and a
 * trailing `)` enclose all of it. */
function balanced(regex: string): boolean {
  let depth = 0;
  let inClass = false;
  for (let i = 0; i < regex.length; i++) {
    const c = regex[i];
    if (c === "\\") i++;
    else if (inClass) inClass = c !== "]";
    else if (c === "[") inClass = true;
    else if (c === "(") depth++;
    else if (c === ")" && --depth < 0) return false;
  }
  return depth === 0;
}

/** The length of the single atom at the start of `regex`: an escape that
 * stands for one character, `.`, a class, or a literal character.
 * It is 0 for a group, an anchor, a back-reference or a quantifier. */
function atomLength(regex: string): number {
  const c = regex[0];
  if (c === undefined || "()|^$*+?{".includes(c)) return 0;
  if (c === ".") return 1;
  if (c === "[") {
    // A `]` right after `[` or `[^` is a literal.
    let depth = 0;
    let opened = -1;
    for (let i = 0; i < regex.length; i++) {
      const d = regex[i];
      if (d === "\\") {
        i++;
      } else if (d === "[") {
        depth++;
        opened = regex[i + 1] === "^" ? i + 1 : i;
      } else if (d === "]" && i > opened + 1 && --depth === 0) {
        return i + 1;
      }
    }
    return 0;
  }
  if (c === "\\") {
    const n = regex[1];
    if (n === undefined || /[1-9AbBGzZkgK]/.test(n)) return 0;
    if (/[pPxu]/.test(n) && regex[2] === "{") return regex.indexOf("}") + 1;
    if (n === "x") return /^\\x[0-9A-Fa-f]{2}/.test(regex) ? 4 : 2;
    if (n === "u") return /^\\u[0-9A-Fa-f]{4}/.test(regex) ? 6 : 2;
    if (n === "c") return 3;
    return 2;
  }
  return String.fromCodePoint(regex.codePointAt(0)!).length;
}

/** Whether a match of `regex` can start at a space or a tab. */
function blankStart(regex: string, backref: Backref): boolean {
  const { set, any } = startOf(regex, backref);
  const has = (b: number) => ((set[b >> 6]! >> BigInt(b & 63)) & 1n) !== 0n;
  return any || has(0x20) || has(0x09);
}

/** The length of the unit at the start of `regex` that takes one blank, and
 * only one character, when it starts at a blank; 0 when there is none.
 * The unit is a single atom that can match a blank, such as `\s`, `.` or
 * `[ \t]`.
 * It can also be a group, such as `(?:.|\\.)`, whose alternatives that can
 * start at a blank are each a single atom. */
function oneBlankUnit(regex: string, backref: Backref): number {
  const atom = atomLength(regex);
  if (atom > 0) return blankStart(regex.slice(0, atom), backref) ? atom : 0;
  if (regex[0] !== "(" || (regex[1] === "?" && regex[2] !== ":")) return 0;
  const close = closingParen(regex);
  if (close < 0) return 0;
  const inner = regex.slice(regex[1] === "?" ? 3 : 1, close);
  let found = false;
  for (const alt of alternatives(inner)) {
    if (!blankStart(alt, backref)) continue;
    if (atomLength(alt) !== alt.length) return 0;
    found = true;
  }
  return found ? close + 1 : 0;
}

/** Whether `regex` can match the empty text, so that a `match` with it may
 * not advance. Syntax the analysis does not read counts as able to. */
export function canMatchEmpty(regex: string): boolean {
  const source = regex.startsWith("(?i)") ? regex.slice(4) : regex;
  try {
    return new RegexParser(source, "any").parse().empty !== null;
  } catch {
    return true;
  }
}

/** Whether some alternative of `regex` takes exactly one blank and then
 * requires a character that is not a blank:
 * one unquantified unit that takes one blank (`oneBlankUnit`),
 * then a rest whose first byte cannot be a space or a tab.
 * A negative lookbehind before the unit is stepped over when its body has no atom that can match a
 * blank, as in MATLAB's `(?<!\.\.\.).\n`.
 * At a blank that follows another blank, such a lookbehind always passes.
 * So it cannot let the pattern fail at the last blank of a run and match at an earlier one.
 * A body that can match a blank, as in `(?<! )` or `(?<!\s\s)`, could do that,
 * so the alternative does not count.
 * Such a pattern can match at the last blank of a run and nowhere before it.
 * A `BlankRuns::ToLast` grammar tries only these patterns there.
 */
export function takesOneBlank(regex: string, backref: Backref = "quote"): boolean {
  return alternatives(regex.startsWith("(?i)") ? regex.slice(4) : regex).some((whole) => {
    const alt = afterLookbehinds(whole, backref);
    const unit = oneBlankUnit(alt, backref);
    if (unit === 0 || /^[*+?{]/.test(alt.slice(unit))) return false;
    const rest = alt.slice(unit);
    if (rest === "") return false;
    const { set, bol, any } = startOf(rest, backref);
    const blank = (b: number) => ((set[b >> 6]! >> BigInt(b & 63)) & 1n) !== 0n;
    return !any && !bol && !blank(0x20) && !blank(0x09);
  });
}

/**
 * `regex` after the negative lookbehinds it starts with whose bodies cannot match a
 * blank (`blankFree`).
 */
function afterLookbehinds(regex: string, backref: Backref): string {
  let rest = regex;
  while (rest.startsWith("(?<!")) {
    const close = closingParen(rest);
    if (close < 0 || !blankFree(rest.slice(4, close), backref)) return rest;
    rest = rest.slice(close + 1);
  }
  return rest;
}

/** Whether no atom of `regex` can match a space or a tab.
 * A back-reference counts as one that can. */
function blankFree(regex: string, backref: Backref): boolean {
  for (let i = 0; i < regex.length;) {
    const rest = regex.slice(i);
    if (/^\\([1-9]|k)/.test(rest)) return false;
    const atom = atomLength(rest);
    if (atom === 0) {
      i++;
      continue;
    }
    if (blankStart(rest.slice(0, atom), backref)) return false;
    i += atom;
  }
  return true;
}

/** The index of the `)` that closes the group `regex` starts with; -1 when there is none. */
function closingParen(regex: string): number {
  let depth = 0;
  let inClass = false;
  for (let i = 0; i < regex.length; i++) {
    const c = regex[i];
    if (c === "\\") i++;
    else if (inClass) inClass = c !== "]";
    else if (c === "[") inClass = true;
    else if (c === "(") depth++;
    else if (c === ")" && --depth === 0) return i;
  }
  return -1;
}

/** `Pattern::takes_one_blank`, from [`takesOneBlank`], for a grammar on
 * `BlankRuns::ToLast`. */
export function oneBlankTable(
  patternById: { id: string; regex: string }[],
  backref: Backref = "quote",
): string {
  const ids = patternById
    .filter((p) => takesOneBlank(p.regex, backref))
    .map((p) => `Pattern::${p.id}`);
  if (ids.length === 0)
    throw new Error("a BlankRuns::ToLast grammar with no pattern that takes one blank");
  return `

impl Pattern {
    /// Whether the pattern takes exactly one blank and then requires a
    /// character that is not a blank, as \`oracle/rule-grammar.ts\`'s
    /// \`takesOneBlank\` reads its expression.
    /// Such a pattern can match at the last blank of a run and nowhere before
    /// it.
    pub(crate) const fn takes_one_blank(self) -> bool {
        matches!(self, ${ids.join(" | ")})
    }
}
`;
}

/** Groups equal bodies into one arm of a `match` on `type`. */
export function arms(entries: [string, string][], type: string): string {
  const groups = new Map<string, string[]>();
  for (const [id, body] of entries)
    groups.set(body, [...(groups.get(body) ?? []), `${type}::${id}`]);
  let out = "";
  for (const [body, ids] of groups)
    out += `            ${ids.join("\n            | ")} => ${body},\n`;
  return out;
}

/** The attribute a generated table function carries. */
export const tooLong = `    #[expect(clippy::too_many_lines, reason = "a generated table")]\n`;

const hex = (w: bigint) => {
  const digits = w.toString(16).padStart(16, "0");
  return `0x${digits.match(/.{4}/g)!.join("_")}`;
};

/** `Pattern::start`: the bytes each pattern can start at, and its anchors. */
export function startTable(
  patternById: { id: string; regex: string }[],
  backref: Backref = "quote",
): string {
  let starts = `

impl Pattern {
${tooLong}    pub(crate) fn start(self) -> Start {
        use crate::rules::{Place, PlaceSet};
        match self {
`;
  let wordsAtStart = false;
  starts += arms(
    patternById.map((p): [string, string] => {
      const { set, bol, any, wb, behind, atEnd, inWordFirst } = startOf(p.regex, backref);
      wordsAtStart ||= inWordFirst;
      const words = (any ? FULL() : set).map(hex).join(", ");
      // `^` keeps a match at the start of the line, a lookbehind that needs a
      // byte keeps it off there, and `\\b` or a lookbehind that fails after a
      // word character keeps it out of words.
      const places = [
        ...(bol ? [] : ["Place::Away"]),
        ...(behind ? [] : ["Place::LineStart"]),
        ...(bol || wb ? [] : ["Place::InWord"]),
      ];
      // The scan counts its first position inside a word as away from words,
      // which is sound only while Away holds every pattern InWord holds.
      if (places.includes("Place::InWord") && !places.includes("Place::Away")) {
        throw new Error(`${p.id}: a pattern tried inside words must be tried away from them`);
      }
      return [
        p.id,
        `Start {\n                bytes: [${words}],\n                places: PlaceSet::of(&[${places.join(", ")}]),\n                matches_at_end: ${atEnd},\n            }`,
      ];
    }),
    "Pattern",
  );
  starts += `        }\n    }\n\n`;
  starts += `    /// Whether a pattern is tried inside a word at a scan's first position\n`;
  starts += `    /// though not past it ([\`rules::Tabled::WORDS_AT_START\`]).\n`;
  starts += `    pub(crate) const WORDS_AT_START: bool = ${wordsAtStart};\n}\n`;
  return starts;
}

/** How a `Rule` of the grammar `engine` (such as `Css`) is stored as a frame
 * of the interned state: its language's tag, the constant `tag` of
 * `src/tag.rs`, in the high bits, and its position in `Rule::ALL` in the low
 * `bits`, 8 unless the grammar has more rules than a byte numbers (C++, whose
 * tag claims a wider range). Then its `Tabled` implementation, which serves
 * the pattern tables above to `rules::table_scan`.
 * With `baseSlots`, it also serves the tables of `Rule::base_definition`. */
export function ruleCodes(engine: string, tag: string, bits = 8, baseSlots = false): string {
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
    const WORDS_AT_START: bool = Pattern::WORDS_AT_START;
    const MIXED_BLANK_RUNS: bool = rules::blank_runs::${tag};

    fn pattern_start(pattern: Pattern) -> rules::Start {
        pattern.start()
    }

    fn tables(rule: Rule) -> &'static rules::Tables<Self> {
        static TABLES: [std::sync::OnceLock<rules::Tables<${engine}>>; Rule::ALL.len()] =
            [const { std::sync::OnceLock::new() }; Rule::ALL.len()];
        #[expect(
            clippy::as_conversions,
            reason = "a fieldless \`repr(${repr})\` enum has no \`From\` impl"
        )]
        let index = rule as usize;
        TABLES[index].get_or_init(move || rules::Tables::build(rule.definition()))
    }
${baseSlots ? baseTables(engine, repr) : ""}}
`;
}

/** The `Tabled` items of a grammar whose `$base` includes are slots. */
const baseTables = (engine: string, repr: string) => `
    const BASE_SLOTS: bool = true;

    fn is_base_slot(pattern: Pattern) -> bool {
        matches!(pattern, Pattern::Base)
    }

    fn base_tables(rule: Rule) -> Option<&'static rules::Tables<Self>> {
        static TABLES: [std::sync::OnceLock<Option<rules::Tables<${engine}>>>; Rule::ALL.len()] =
            [const { std::sync::OnceLock::new() }; Rule::ALL.len()];
        #[expect(
            clippy::as_conversions,
            reason = "a fieldless \`repr(${repr})\` enum has no \`From\` impl"
        )]
        let index = rule as usize;
        TABLES[index]
            .get_or_init(move || rule.base_definition().map(rules::Tables::build))
            .as_ref()
    }
`;

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
 * the generated enum.
 * A `keyed` table also gets the scope codes and the empty list.
 * The grammar's `scopes!` lists need them.
 * C++ wraps its scopes in a type of its own, which carries both. */
export function scopeFile(
  generator: string,
  table: string,
  scopes: Map<string, string>,
  attributes = "",
  keyed = true,
): string {
  const idents = [...scopes.entries()].sort((a, b) => a[1].localeCompare(b[1]));
  let s = `//! The scope names the lexer emits, in Shiki's naming. Generated by
//! \`oracle/${generator}\`; do not edit.

crate::scope::scope_table! {
${attributes}    ${keyed ? "keyed " : ""}${table};
`;
  for (const [ident, name] of idents) s += `    ${ident} "${name}"\n`;
  s += "}\n";
  return s;
}
