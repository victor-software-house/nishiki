// Measures the characters Oniguruma, as Shiki bundles it, takes as one under
// `(?i)`, by asking it, as unicode-tables.ts measures the character sets.
// It records the orbits and the folds of more than one character in
// oracle/facts/case-folds.json.
// It also writes src/textmate/regex/folds.rs from them, until a Rust writer
// takes that step over.
//
// The candidates are the code points whose lower or upper case differs from
// themselves, which every member of a case fold has. For each candidate the
// orbit is the candidates that `(?i)` followed by that character matches, one
// character long. The table lists every member of an orbit of more than one
// with the next member, so an orbit is one cycle and any member finds the
// others. The orbits are Unicode's simple case folds, the C and S lines of
// CaseFolding.txt, as Oniguruma applies them: those of Unicode 14 and 15, none
// of 16.
//
// Usage: bun oracle/case-folds.ts
// `mise run check:case-folds` runs it and fails when a file it writes changes.
import { type RegexEngine } from "shiki@4.6.0";
import { createOnigurumaEngine } from "shiki@4.6.0/engine/oniguruma";
import wasm from "shiki@4.6.0/wasm";
import { writeFileSync } from "node:fs";
import { dedent } from "./dedent.ts";
import { bundledOniguruma, type CaseFoldsFacts, codePointNotation, writeFacts } from "./facts.ts";
import { type PointText, pointText, scalarValues } from "./scalars.ts";

/** What Oniguruma answers about case: the orbits and the folds of more than one character. */
interface Measured {
  /** The code points whose lower or upper case differs from themselves. */
  candidates: number[];
  /** Each member of an orbit of more than one, with its orbit, ascending. */
  orbitOf: Map<number, number[]>;
  /** Each character whose fold is a string, with that string. */
  foldOf: Map<number, string>;
}

/** The tables folds.rs holds, and the counts the run prints. */
interface Tables {
  candidateCount: number;
  orbitCount: number;
  largest: number;
  next: [number, number][];
  multi: [number, number[]][];
  folds: (readonly [number[], number[]])[];
  wideCount: number;
  wideLead: number[];
  wideSecond: bigint[];
  wideTails: number[][];
}

/** The pattern that matches `cp` under `(?i)`. */
const caseless = (cp: number) => `(?i)\\x{${cp.toString(16)}}`;

/** The candidates that `(?i)` and `cp` match, one character long. */
function orbitAround(engine: RegexEngine, candidateText: PointText, cp: number): number[] {
  const scanner = engine.createScanner([caseless(cp)]);
  const orbit: number[] = [];
  let at = 0;
  for (;;) {
    const found = scanner.findNextMatchSync(candidateText.text, at);
    if (!found) break;
    const { start, end } = found.captureIndices[0];
    const matched = candidateText.pointAt[start];
    if (end - start === (matched > 0xffff ? 2 : 1)) orbit.push(matched);
    at = Math.max(end, start + 1);
  }
  return orbit;
}

/** Asks `engine` for every orbit and every fold of more than one character. */
function measure(engine: RegexEngine): Measured {
  const scalars = scalarValues();
  const candidates = scalars.filter((cp) => {
    const c = String.fromCodePoint(cp);
    return c.toLowerCase() !== c || c.toUpperCase() !== c;
  });
  const text = pointText(candidates);
  const orbitOf = new Map<number, number[]>();
  for (const cp of candidates) {
    if (orbitOf.has(cp)) continue;
    const orbit = orbitAround(engine, text, cp);
    if (orbit.length < 2) continue;
    orbit.sort((a, b) => a - b);
    for (const member of orbit) orbitOf.set(member, orbit);
  }

  // The characters whose fold is a string.
  // A candidate is a code point whose case mapping is longer than itself.
  // A member of such a code point's orbit is a candidate too, such as ẞ beside
  // ß.
  // Its fold is that mapping lowered.
  // The fold is kept when `(?i)` and the character match the whole of it.
  const foldsTo = (cp: number, fold: string) => {
    const found = engine.createScanner([caseless(cp)]).findNextMatchSync(`${fold}\n`, 0);
    return found?.captureIndices[0].start === 0 && found.captureIndices[0].end === fold.length;
  };
  const foldOf = new Map<number, string>();
  for (const cp of scalars) {
    const c = String.fromCodePoint(cp);
    const fold = [c.toUpperCase().toLowerCase(), c.toLowerCase()].find((f) => [...f].length > 1);
    if (fold !== undefined && foldsTo(cp, fold)) foldOf.set(cp, fold);
  }
  for (const [cp, fold] of [...foldOf]) {
    for (const mate of orbitOf.get(cp) ?? [])
      if (!foldOf.has(mate) && foldsTo(mate, fold)) foldOf.set(mate, fold);
  }
  return { candidates, orbitOf, foldOf };
}

/** The tables folds.rs holds, from what Oniguruma answered. */
function tables({ candidates, orbitOf, foldOf }: Measured): Tables {
  const next: [number, number][] = [];
  for (const [member, orbit] of orbitOf)
    next.push([member, orbit[(orbit.indexOf(member) + 1) % orbit.length]]);
  next.sort((a, b) => a[0] - b[0]);
  const orbits = new Set(orbitOf.values());
  const largest = Math.max(...[...orbits].map((orbit) => orbit.length));

  // A character's canonical form is the smallest member of its orbit.
  const canonical = (cp: number) => orbitOf.get(cp)?.[0] ?? cp;
  const multi: [number, number[]][] = [...foldOf]
    .sort(([a], [b]) => a - b)
    .map(([cp, fold]) => [cp, [...fold].map((f) => canonical(f.codePointAt(0)!))]);
  const unfold = new Map<string, number[]>();
  for (const [cp, fold] of multi)
    unfold.set(fold.join(","), [...(unfold.get(fold.join(",")) ?? []), cp]);
  const folds = [...unfold.entries()]
    .map(([key, chars]) => [key.split(",").map(Number), chars.sort((a, b) => a - b)] as const)
    .sort(([a], [b]) => {
      for (let i = 0; i < Math.min(a.length, b.length); i++) if (a[i] !== b[i]) return a[i] - b[i];
      return a.length - b.length;
    });

  // The characters beyond ASCII that the folds of more than one character
  // involve.
  // They are every member of the orbit of a character that folds to a string.
  // They are also every member of the orbit of a character of such a string.
  // An alternative that a fold adds, and that matches no ASCII-only text, needs
  // one of them.
  // A line without any of them runs the programs that leave those alternatives
  // out.
  // Each is listed by the first byte of its UTF-8 form, with the bytes after it.
  // A line is then checked without decoding.
  const wide = new Set<number>();
  for (const [cp, fold] of multi) {
    for (const c of [cp, ...fold])
      for (const member of orbitOf.get(c) ?? [c]) if (member > 0x7f) wide.add(member);
  }
  const tailsOf = new Map<number, number[]>();
  for (const cp of wide) {
    const bytes = [...new TextEncoder().encode(String.fromCodePoint(cp))];
    if (bytes.length > 3) throw new Error(`U+${cp.toString(16)} takes more than three bytes`);
    const tail = bytes.length === 2 ? bytes[1] : (bytes[1] << 8) | bytes[2];
    tailsOf.set(bytes[0], [...(tailsOf.get(bytes[0]) ?? []), tail]);
  }
  const leads = [...tailsOf.keys()].sort((a, b) => a - b);
  const wideLead = Array.from({ length: 256 }, (_, byte) => leads.indexOf(byte) + 1);
  const wideTails = leads.map((lead) => tailsOf.get(lead)!.sort((a, b) => a - b));
  // The second bytes each lead byte's characters can have, as a mask of their
  // low six bits.
  // A second byte is always 0x80 to 0xBF.
  const wideSecond = leads.map((lead) =>
    tailsOf
      .get(lead)!
      .reduce((mask, tail) => mask | (1n << BigInt((tail > 0xff ? tail >> 8 : tail) & 0x3f)), 0n),
  );
  return {
    candidateCount: candidates.length,
    orbitCount: orbits.size,
    largest,
    next,
    multi,
    folds,
    wideCount: wide.size,
    wideLead,
    wideSecond,
    wideTails,
  };
}

/** The code points of a string, in order. */
const codePoints = (text: string) => [...text].map((c) => c.codePointAt(0)!);

const hex = (n: number) => `0x${n.toString(16)}`;
const list = (xs: readonly number[]) => `&[${xs.map(hex).join(", ")}]`;

/** The Rust source of folds.rs. */
function rust({ largest, next, multi, folds, wideLead, wideSecond, wideTails }: Tables): string {
  return dedent`
    //! The characters \`(?i)\` takes as one, and the folds of more than one
    //! character.
    //! \`oracle/case-folds.ts\` measures them on Oniguruma as Shiki bundles it.
    //! Do not edit: regenerate with \`bun oracle/case-folds.ts\`.

    #![cfg_attr(rustfmt, rustfmt::skip)]

    /// The most members an orbit has.
    pub(super) const LARGEST_ORBIT: usize = ${largest};

    /// Every character that \`(?i)\` takes as another, and the next member of its
    /// orbit, by character.
    /// Following \`next\` from a member visits its whole orbit and comes back.
    pub(super) static NEXT: &[(u32, u32)] = &[${next.map(([a, b]) => `(${hex(a)}, ${hex(b)})`).join(", ")}];

    /// Every character whose fold is more than one character, with that fold, by
    /// character.
    /// A fold's characters are canonical.
    /// Each is the smallest member of its orbit.
    pub(super) static MULTI: &[(u32, &[u32])] = &[${multi.map(([cp, fold]) => `(${hex(cp)}, ${list(fold)})`).join(", ")}];

    /// Every fold of more than one character, with the characters that fold to
    /// it, by fold.
    pub(super) static UNFOLD: &[(&[u32], &[u32])] = &[${folds.map(([fold, chars]) => `(${list(fold)}, ${list(chars)})`).join(", ")}];

    /// A value per byte.
    /// Some bytes start the UTF-8 form of a character beyond ASCII that a fold of
    /// more than one character involves.
    /// Such a byte holds one more than the index of its group in [\`WIDE_TAILS\`].
    /// Every other byte holds 0.
    pub(super) static WIDE_LEAD: [u8; 256] = [${wideLead.join(", ")}];

    /// Per first byte of [\`WIDE_LEAD\`], the second bytes its characters can
    /// have, as a bit per value of the second byte's low six bits.
    pub(super) static WIDE_SECOND: &[u64] = &[${wideSecond.map((mask) => `0x${mask.toString(16).replace(/\B(?=(?:[0-9a-f]{4})+$)/g, "_")}`).join(", ")}];

    /// Per first byte of [\`WIDE_LEAD\`], the bytes after it of each such
    /// character, ascending: the second byte of a two-byte form, or the second
    /// and third of a three-byte form as one number, the second high.
    pub(super) static WIDE_TAILS: &[&[u16]] = &[${wideTails.map((tails) => list(tails)).join(", ")}];
    `;
}

const output = new URL("../src/textmate/regex/folds.rs", import.meta.url).pathname;
/** The facts case-folds.json records: the orbits, and the folds of more than one character. */
function facts({ orbitOf, foldOf }: Measured): CaseFoldsFacts {
  const orbits = [...new Set(orbitOf.values())].sort((a, b) => a[0]! - b[0]!);
  return {
    measured: bundledOniguruma,
    script: "oracle/case-folds.ts",
    orbits: orbits.map((orbit) => orbit.map(codePointNotation)),
    folds: [...foldOf]
      .sort(([a], [b]) => a - b)
      .map(([cp, fold]) => ({
        char: codePointNotation(cp),
        fold: codePoints(fold).map(codePointNotation),
      })),
  };
}

const measured = measure(await createOnigurumaEngine(wasm as any));
writeFacts("case-folds", facts(measured));
const folded = tables(measured);
writeFileSync(output, rust(folded));
console.log(
  [
    `${folded.candidateCount} candidates, ${folded.orbitCount} orbits,`,
    `${folded.next.length} members, largest ${folded.largest};`,
    `${folded.multi.length} multi-character folds, ${folded.folds.length} distinct;`,
    `${folded.wideCount} wide characters under ${folded.wideTails.length} first bytes`,
  ].join(" "),
);
