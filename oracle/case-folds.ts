// Measures the characters Oniguruma, as Shiki bundles it, takes as one under
// `(?i)`, by asking it, as unicode-tables.ts measures the character sets.
// It records the orbits and the folds of more than one character in
// oracle/facts/case-folds.json.
// codegen/src/folds.rs writes src/textmate/regex/folds.rs from them.
//
// The candidates are the code points whose lower or upper case differs from
// themselves, which every member of a case fold has. For each candidate the
// orbit is the candidates that `(?i)` followed by that character matches, one
// character long. folds.rs lists every member of an orbit of more than one
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

/** The code points of a string, in order. */
const codePoints = (text: string) => [...text].map((c) => c.codePointAt(0)!);

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
console.log(
  [
    `${measured.candidates.length} candidates,`,
    `${new Set(measured.orbitOf.values()).size} orbits,`,
    `${measured.foldOf.size} multi-character folds`,
  ].join(" "),
);
