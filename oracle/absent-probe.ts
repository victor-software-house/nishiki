// Writes tests/engine/absent/: Oniguruma's answers, as Shiki bundles it,
// for patterns that hold the absent operator `(?~…)`.
// `absent_operator_answers_as_oniguruma` in src/textmate/regex/tests.rs checks
// the engine against them.
//
// Each line is searched with a newline after it, from its start and from its
// third character. Positions are UTF-8 offsets. A case whose answer is
// ambiguous, as `oniguruma-answer.ts` says, is left out.
//
// Usage: bun oracle/absent-probe.ts
import { createOnigurumaEngine } from "shiki@4.6.0/engine/oniguruma";
import wasm from "shiki@4.6.0/wasm";
import { mkdirSync, writeFileSync } from "node:fs";
import { answer, utf8 } from "./oniguruma-answer.ts";

const engine = await createOnigurumaEngine(wasm as any);

// The two patterns of C#'s rule for `/** */` documentation comments, which Org,
// Razor and Typst embed.
const csharpProbes: [string, string[]][] = [
  ["\\G(?=(?~\\*/)$)", [" * doc */", " * doc", "", "*/", "/** doc", " * a */ b"]],
  [
    "^(\\s*+)(\\*(?!/))?(?=(?~\\*/)$)",
    ["   * doc line", "   */", "   * end */", " * a */ b", "*", ""],
  ],
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
const lines = [
  "",
  "abc",
  "aab",
  "xabcd",
  "xbxdyd",
  "abababc",
  "a*/x",
  "c*/ab",
  "a*b/ c",
  "xcdab",
  "aaéb",
  "bad cab",
];

const probes: [string, string[]][] = [
  ...csharpProbes,
  ...bodies.flatMap((body) =>
    templates.map((template): [string, string[]] => [
      template.replaceAll("X", `(?~${body})`),
      lines,
    ]),
  ),
];

const patterns = probes.map(([pattern]) => pattern);
const allLines = [...new Set(probes.flatMap(([, list]) => list))];
const cases: string[] = [];
const expected: string[] = [];
let ambiguous = 0;
probes.forEach(([pattern, list], p) => {
  const scanner = engine.createScanner([pattern]);
  for (const line of list) {
    const text = `${line}\n`;
    for (const from of line.length > 2 ? [0, 2] : [0]) {
      const found = answer(scanner.findNextMatchSync(text, from)?.captureIndices, text);
      if (found === undefined) {
        ambiguous++;
        continue;
      }
      cases.push(`${p}\t${allLines.indexOf(line)}\t${utf8(text, from)}`);
      expected.push(found);
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
console.log(
  `${patterns.length} patterns, ${allLines.length} lines, ${cases.length} cases, ${ambiguous} left out`,
);
