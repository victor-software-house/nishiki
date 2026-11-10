// Writes src/textmate/regex/unicode.rs: the character sets that Oniguruma, as
// Shiki bundles it, gives \p{..}, [[:..:]], \w, \d and \s, measured by asking
// it about every code point, as ranges.
//
// Usage: bun oracle/unicode-tables.ts
import { createOnigurumaEngine } from "shiki@4.6.0/engine/oniguruma";
import wasm from "shiki@4.6.0/wasm";
import { writeFileSync } from "node:fs";
import { dedent } from "./dedent.ts";

const engine = await createOnigurumaEngine(wasm as any);

// Every scalar value but the surrogates, as one string, with the code point
// each UTF-16 index belongs to.
const points: number[] = [];
for (let cp = 0; cp <= 0x10ffff; cp++) if (cp < 0xd800 || cp > 0xdfff) points.push(cp);
let text = "";
const unitToPoint: number[] = [];
for (const cp of points) {
  text += String.fromCodePoint(cp);
  unitToPoint.push(cp);
  if (cp > 0xffff) unitToPoint.push(cp);
}
unitToPoint.push(0x110000);

function ranges(pattern: string): [number, number][] {
  const scanner = engine.createScanner([`(?:${pattern})+`]);
  const out: [number, number][] = [];
  let at = 0;
  for (;;) {
    const found = scanner.findNextMatchSync(text, at);
    if (!found) break;
    const { start, end } = found.captureIndices[0];
    if (end <= start) break;
    const first = unitToPoint[start];
    const last = unitToPoint[end - 1];
    // Neighbouring runs in the string can join across the surrogate gap.
    const previous = out[out.length - 1];
    if (previous && previous[1] + 1 === first) previous[1] = last;
    else out.push([first, last]);
    at = end;
  }
  return out;
}

const categories = [
  "L", "Lu", "Ll", "Lt", "Lm", "Lo", "M", "Mn", "Mc", "Me", "N", "Nd", "Nl", "No", "P", "Pc", "Pd", "Ps", "Pe", "Pi",
  "Pf", "Po", "S", "Sm", "Sc", "Sk", "So", "Z", "Zs", "Zl", "Zp", "C", "Cc", "Cf", "Co", "Cn",
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
const scripts = ["Han", "Hangul", "Katakana", "Hiragana", "Greek", "Latin", "Cyrillic", "Arabic", "Hebrew", "Thai"];
const posix = [
  "alpha", "alnum", "upper", "lower", "digit", "xdigit", "space", "blank", "cntrl", "punct", "graph", "print", "word",
];

const tables: [string, [number, number][]][] = [];
for (const c of categories) tables.push([c.toLowerCase(), ranges(`\\p{${c}}`)]);
tables.push(["cs", [[0xd800, 0xdfff]]]);
for (const [key, pattern] of binary) tables.push([key, ranges(pattern)]);
for (const s of scripts) tables.push([s.toLowerCase(), ranges(`\\p{${s}}`)]);
// A name the POSIX brackets and \p{..} share is one set, measured as a bracket.
for (const p of posix) tables.push([`posix_${p}`, ranges(`[[:${p}:]]`)]);
tables.push(["ascii", [[0, 0x7f]]]);

const ident = (key: string) => `SET_${key.toUpperCase()}`;
let source = dedent`
  //! Character sets.
  //! \`oracle/unicode-tables.ts\` measures them on Oniguruma as Shiki bundles it.
  //! Do not edit: regenerate with \`bun oracle/unicode-tables.ts\`.

  #![cfg_attr(rustfmt, rustfmt::skip)]
  #![expect(clippy::unreadable_literal, reason = "generated tables of code points")]

  `;
for (const [key, list] of tables) {
  source += `static ${ident(key)}: &[(u32, u32)] = &[${list.map(([a, b]) => `(0x${a.toString(16)}, 0x${b.toString(16)})`).join(", ")}];\n`;
}
source += dedent`

  /// The set a normalized name stands for.
  /// A normalized name is lower case, without \`_\`, \`-\` or spaces.
  /// Returns \`None\` for a name this table lacks.
  pub(super) fn lookup(name: &str) -> Option<&'static [(u32, u32)]> {
      Some(match name {
  ${tables
  .filter(([key]) => !key.startsWith("posix_"))
  .map(([key]) => `        "${key}" => ${ident(key)},`)
  .join("\n")}
  ${posix.map((p) => `        "${p}" => SET_POSIX_${p.toUpperCase()},`).join("\n")}
          _ => return None,
      })
  }
  `;
writeFileSync(new URL("../src/textmate/regex/unicode.rs", import.meta.url).pathname, source);
console.log(tables.map(([k, v]) => `${k}:${v.length}`).join(" "));
