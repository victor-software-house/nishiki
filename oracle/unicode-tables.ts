// Measures the character sets that Oniguruma, as Shiki bundles it, gives
// \p{..}, [[:..:]], \w, \d and \s, by asking it about every code point.
// It records them as ranges in oracle/facts/unicode-sets.json.
// codegen/src/unicode.rs writes src/textmate/regex/unicode.rs from them.
//
// Usage: bun oracle/unicode-tables.ts
// `mise run check:unicode-tables` runs it and fails when a file it writes changes.
import { type RegexEngine } from "shiki@4.6.0";
import { createOnigurumaEngine } from "shiki@4.6.0/engine/oniguruma";
import wasm from "shiki@4.6.0/wasm";
import { bundledOniguruma, rangeNotation, type UnicodeSetsFacts, writeFacts } from "./facts.ts";
import { type PointText, pointText, scalarValues } from "./scalars.ts";

/** The first and last code point of a run of code points. */
type Range = [first: number, last: number];

/** A set, by its key in `lookup`, as ascending ranges. */
interface CharacterSet {
  key: string;
  ranges: Range[];
}

const categories = [
  "L",
  "Lu",
  "Ll",
  "Lt",
  "Lm",
  "Lo",
  "M",
  "Mn",
  "Mc",
  "Me",
  "N",
  "Nd",
  "Nl",
  "No",
  "P",
  "Pc",
  "Pd",
  "Ps",
  "Pe",
  "Pi",
  "Pf",
  "Po",
  "S",
  "Sm",
  "Sc",
  "Sk",
  "So",
  "Z",
  "Zs",
  "Zl",
  "Zp",
  "C",
  "Cc",
  "Cf",
  "Co",
  "Cn",
];
const binary: [string, string][] = [
  ["alphabetic", "\\p{Alphabetic}"],
  ["uppercase", "\\p{Uppercase}"],
  ["lowercase", "\\p{Lowercase}"],
  ["whitespace", "\\p{White_Space}"],
  ["xidstart", "\\p{XIDS}"],
  ["xidcontinue", "\\p{XIDC}"],
  ["idstart", "\\p{IDS}"],
  ["idcontinue", "\\p{IDC}"],
];
const scripts = [
  "Han",
  "Hangul",
  "Katakana",
  "Hiragana",
  "Greek",
  "Latin",
  "Cyrillic",
  "Arabic",
  "Hebrew",
  "Thai",
];
const posix = [
  "alpha",
  "alnum",
  "upper",
  "lower",
  "digit",
  "xdigit",
  "space",
  "blank",
  "cntrl",
  "punct",
  "graph",
  "print",
  "word",
];

/** The code points that `pattern` matches in `every`, as ranges. */
function ranges(engine: RegexEngine, every: PointText, pattern: string): Range[] {
  const scanner = engine.createScanner([`(?:${pattern})+`]);
  const out: Range[] = [];
  let at = 0;
  for (;;) {
    const found = scanner.findNextMatchSync(every.text, at);
    if (!found) break;
    const { start, end } = found.captureIndices[0];
    if (end <= start) break;
    const first = every.pointAt[start];
    const last = every.pointAt[end - 1];
    // Neighbouring runs in the string can join across the surrogate gap.
    const previous = out[out.length - 1];
    if (previous && previous[1] + 1 === first) previous[1] = last;
    else out.push([first, last]);
    at = end;
  }
  return out;
}

/** Every set `lookup` names, measured on `engine`, in the order the file lists them. */
function measure(engine: RegexEngine): CharacterSet[] {
  const every = pointText(scalarValues());
  const measured = (key: string, pattern: string): CharacterSet => ({
    key,
    ranges: ranges(engine, every, pattern),
  });
  return [
    ...categories.map((c) => measured(c.toLowerCase(), `\\p{${c}}`)),
    { key: "cs", ranges: [[0xd800, 0xdfff]] },
    ...binary.map(([key, pattern]) => measured(key, pattern)),
    ...scripts.map((s) => measured(s.toLowerCase(), `\\p{${s}}`)),
    // A name the POSIX brackets and \p{..} share is one set, measured as a bracket.
    ...posix.map((p) => measured(`posix_${p}`, `[[:${p}:]]`)),
    { key: "ascii", ranges: [[0, 0x7f]] },
  ];
}

/** The facts unicode-sets.json records: every set, as ranges in Unicode's notation. */
function facts(sets: readonly CharacterSet[]): UnicodeSetsFacts {
  return {
    measured: bundledOniguruma,
    script: "oracle/unicode-tables.ts",
    sets: sets.map(({ key, ranges }) => ({ key, ranges: ranges.map(rangeNotation) })),
  };
}

const sets = measure(await createOnigurumaEngine(wasm as any));
writeFacts("unicode-sets", facts(sets));
console.log(sets.map(({ key, ranges }) => `${key}:${ranges.length}`).join(" "));
