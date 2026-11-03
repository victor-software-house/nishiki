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

const hex = (n: number) => `0x${n.toString(16)}`;
const source = `//! The characters \`(?i)\` takes as one, as cycles, measured on Oniguruma as
//! Shiki bundles it by \`oracle/case-folds.ts\`. Do not edit: regenerate with
//! \`bun oracle/case-folds.ts\`.

#![cfg_attr(rustfmt, rustfmt::skip)]

/// The most members an orbit has.
pub(super) const LARGEST_ORBIT: usize = ${largest};

/// Every character that \`(?i)\` takes as another, and the next member of its
/// orbit, by character: following \`next\` from a member visits its whole
/// orbit and comes back.
pub(super) static NEXT: &[(u32, u32)] = &[${next.map(([a, b]) => `(${hex(a)}, ${hex(b)})`).join(", ")}];
`;
writeFileSync(output, source);
console.log(`${candidates.length} candidates, ${orbits.size} orbits, ${next.length} members, largest ${largest}`);
