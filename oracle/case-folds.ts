// Writes src/textmate/regex/folds.rs: the characters Oniguruma, as Shiki
// bundles it, takes as one under `(?i)`, measured by asking it, as
// unicode-tables.ts measures the character sets.
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
// Usage: bun oracle/case-folds.ts [out file]
// `mise run check:case-folds` writes to a temporary file and compares.
import { createOnigurumaEngine } from "shiki@4.6.0/engine/oniguruma";
import wasm from "shiki@4.6.0/wasm";
import { writeFileSync } from "node:fs";

const output = process.argv[2] ?? new URL("../src/textmate/regex/folds.rs", import.meta.url).pathname;
const engine = await createOnigurumaEngine(wasm as any);

const candidates: number[] = [];
for (let cp = 0; cp <= 0x10ffff; cp++) {
  if (cp >= 0xd800 && cp <= 0xdfff) continue;
  const c = String.fromCodePoint(cp);
  if (c.toLowerCase() !== c || c.toUpperCase() !== c) candidates.push(cp);
}
// The candidates as one string, with the code point each UTF-16 index starts.
let text = "";
const unitToPoint: number[] = [];
for (const cp of candidates) {
  text += String.fromCodePoint(cp);
  unitToPoint.push(cp);
  if (cp > 0xffff) unitToPoint.push(-1);
}

const orbitOf = new Map<number, number[]>();
for (const cp of candidates) {
  if (orbitOf.has(cp)) continue;
  const scanner = engine.createScanner([`(?i)\\x{${cp.toString(16)}}`]);
  const orbit: number[] = [];
  let at = 0;
  for (;;) {
    const found = scanner.findNextMatchSync(text, at);
    if (!found) break;
    const { start, end } = found.captureIndices[0];
    const point = unitToPoint[start];
    if (end - start === (point > 0xffff ? 2 : 1)) orbit.push(point);
    at = Math.max(end, start + 1);
  }
  if (orbit.length < 2) continue;
  orbit.sort((a, b) => a - b);
  for (const member of orbit) orbitOf.set(member, orbit);
}

const next: [number, number][] = [];
for (const [member, orbit] of orbitOf) next.push([member, orbit[(orbit.indexOf(member) + 1) % orbit.length]]);
next.sort((a, b) => a[0] - b[0]);
const orbits = new Set(orbitOf.values());
const largest = Math.max(...[...orbits].map((orbit) => orbit.length));

// A character's canonical form is the smallest member of its orbit.
const canonical = (cp: number) => orbitOf.get(cp)?.[0] ?? cp;

// The characters whose fold is a string: a candidate is a code point whose
// case mapping is longer than itself, or a member of such a code point's
// orbit, such as ẞ beside ß. Its fold is that mapping lowered, kept when
// `(?i)` and the character match the whole of it.
const foldsTo = (cp: number, fold: string) => {
  const found = engine.createScanner([`(?i)\\x{${cp.toString(16)}}`]).findNextMatchSync(`${fold}\n`, 0);
  return found?.captureIndices[0].start === 0 && found.captureIndices[0].end === fold.length;
};
const foldOf = new Map<number, string>();
for (let cp = 0; cp <= 0x10ffff; cp++) {
  if (cp >= 0xd800 && cp <= 0xdfff) continue;
  const c = String.fromCodePoint(cp);
  const fold = [c.toUpperCase().toLowerCase(), c.toLowerCase()].find((f) => [...f].length > 1);
  if (fold !== undefined && foldsTo(cp, fold)) foldOf.set(cp, fold);
}
for (const [cp, fold] of [...foldOf]) {
  for (const mate of orbitOf.get(cp) ?? []) if (!foldOf.has(mate) && foldsTo(mate, fold)) foldOf.set(mate, fold);
}
const multi: [number, number[]][] = [...foldOf]
  .sort(([a], [b]) => a - b)
  .map(([cp, fold]) => [cp, [...fold].map((f) => canonical(f.codePointAt(0)!))]);
const unfold = new Map<string, number[]>();
for (const [cp, fold] of multi) unfold.set(fold.join(","), [...(unfold.get(fold.join(",")) ?? []), cp]);
const folds = [...unfold.entries()]
  .map(([key, chars]) => [key.split(",").map(Number), chars.sort((a, b) => a - b)] as const)
  .sort(([a], [b]) => {
    for (let i = 0; i < Math.min(a.length, b.length); i++) if (a[i] !== b[i]) return a[i] - b[i];
    return a.length - b.length;
  });

const hex = (n: number) => `0x${n.toString(16)}`;
const list = (xs: readonly number[]) => `&[${xs.map(hex).join(", ")}]`;
const source = `//! The characters \`(?i)\` takes as one, and the folds of more than one
//! character, measured on Oniguruma as Shiki bundles it by
//! \`oracle/case-folds.ts\`. Do not edit: regenerate with
//! \`bun oracle/case-folds.ts\`.

#![cfg_attr(rustfmt, rustfmt::skip)]

/// The most members an orbit has.
pub(super) const LARGEST_ORBIT: usize = ${largest};

/// Every character that \`(?i)\` takes as another, and the next member of its
/// orbit, by character: following \`next\` from a member visits its whole
/// orbit and comes back.
pub(super) static NEXT: &[(u32, u32)] = &[${next.map(([a, b]) => `(${hex(a)}, ${hex(b)})`).join(", ")}];

/// Every character whose fold is more than one character, with that fold, by
/// character. A fold's characters are canonical: each is the smallest member
/// of its orbit.
pub(super) static MULTI: &[(u32, &[u32])] = &[${multi.map(([cp, fold]) => `(${hex(cp)}, ${list(fold)})`).join(", ")}];

/// Every fold of more than one character, with the characters that fold to
/// it, by fold.
pub(super) static UNFOLD: &[(&[u32], &[u32])] = &[${folds.map(([fold, chars]) => `(${list(fold)}, ${list(chars)})`).join(", ")}];
`;
writeFileSync(output, source);
console.log(
  `${candidates.length} candidates, ${orbits.size} orbits, ${next.length} members, largest ${largest}; ` +
    `${multi.length} multi-character folds, ${folds.length} distinct`,
);
