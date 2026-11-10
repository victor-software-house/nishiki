// Writes tests/engine/absent/: Oniguruma's answers, as Shiki bundles it,
// for patterns that hold the absent operator `(?~…)`.
// `absent_operator_answers_as_oniguruma` in src/textmate/regex/tests.rs checks
// the engine against them.
//
// Each line is searched with a newline after it, from its start and from its
// third character. Positions are UTF-8 offsets.
//
// Usage: bun oracle/absent-probe.ts
import { createOnigurumaEngine } from "shiki@4.6.0/engine/oniguruma";
import wasm from "shiki@4.6.0/wasm";
import { mkdirSync, writeFileSync } from "node:fs";

const engine = await createOnigurumaEngine(wasm as any);

// The two patterns of C#'s rule for `/** */` documentation comments, which Org,
// Razor and Typst embed.
const csharpProbes: [string, string[]][] = [
  ["\\G(?=(?~\\*/)$)", [" * doc */", " * doc", "", "*/", "/** doc", " * a */ b"]],
  ["^(\\s*+)(\\*(?!/))?(?=(?~\\*/)$)", ["   * doc line", "   */", "   * end */", " * a */ b", "*", ""]],
];

// The bodies put inside the operator.
const bodies = [
  "\\*/",
  "ab",
  "ab|a",
  "a|ab",
  "b.*d",
  "b.*?d",
  "c$",
  "$",
  "^b",
  "\\Ab",
  "(?<=a)b",
  "a?",
  "x*",
  "a+",
  "(a)b\\1",
  "b(?=c)",
  "(?i)B",
  "\\bb",
  ".",
  "(?>a|ab)c",
  "[ab]{2}",
  "ab|bc|cd",
  "\\G",
  "(?!a)",
  "é",
];

// The patterns the operator is put in. `X` stands for it.
const templates = [
  "^X",
  "X",
  "^X(?~cd)",
  "aXc",
  "^(X)c?",
  "(?=X$)",
  "^XX",
  "X*?c",
  "(?:X)+c",
  "(?i)^X",
  "^X|^b",
  "^X+?d",
];

// The lines every generated pattern is asked about.
const lines = ["", "abc", "aab", "xabcd", "xbxdyd", "abababc", "a*/x", "c*/ab", "a*b/ c", "xcdab", "aaéb", "bad cab"];

const probes: [string, string[]][] = [
  ...csharpProbes,
  ...bodies.flatMap((body) => templates.map((template): [string, string[]] => [template.replaceAll("X", `(?~${body})`), lines])),
];

const patterns = probes.map(([pattern]) => pattern);
const allLines = [...new Set(probes.flatMap(([, list]) => list))];
const cases: string[] = [];
const expected: string[] = [];
const utf8 = (text: string, index: number) => Buffer.byteLength(text.slice(0, index));
// The start and end Shiki's engine gives a group that did not take part.
// On text that is not all ASCII, it gives the text's length instead.
// A group that took part with no characters at the text's end looks the same,
// so the probe leaves such cases out.
const unset = 0xffffffff;
let ambiguous = 0;
probes.forEach(([pattern, list], p) => {
  const scanner = engine.createScanner([pattern]);
  for (const line of list) {
    const text = `${line}\n`;
    for (const from of line.length > 2 ? [0, 2] : [0]) {
      const found = scanner.findNextMatchSync(text, from);
      if (
        found &&
        /[^\x00-\x7f]/.test(text) &&
        found.captureIndices.slice(1).some(({ start, end }) => start === text.length && end === text.length)
      ) {
        ambiguous++;
        continue;
      }
      cases.push(`${p}\t${allLines.indexOf(line)}\t${utf8(text, from)}`);
      expected.push(
        found
          ? found.captureIndices
              .map(({ start, end }) => (start === unset || end < start ? "x" : `${utf8(text, start)},${utf8(text, end)}`))
              .join(";")
          : "-",
      );
    }
  }
});

const hex = (text: string) => Buffer.from(text, "utf8").toString("hex");
const dir = new URL("../tests/engine/absent/", import.meta.url).pathname;
mkdirSync(dir, { recursive: true });
writeFileSync(`${dir}patterns.hex`, `${patterns.map(hex).join("\n")}\n`);
writeFileSync(`${dir}lines.hex`, `${allLines.map(hex).join("\n")}\n`);
writeFileSync(`${dir}cases.tsv`, `${cases.join("\n")}\n`);
writeFileSync(`${dir}expected.tsv`, `${expected.join("\n")}\n`);
console.log(`${patterns.length} patterns, ${allLines.length} lines, ${cases.length} cases, ${ambiguous} left out`);
