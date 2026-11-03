// Writes tests/engine/case-folds/: Oniguruma's answers, as Shiki bundles it,
// for patterns under `(?i)` on the characters its simple case folds join,
// which `case_insensitive_matching_answers_as_oniguruma` in
// src/textmate/regex/tests.rs checks the engine against. Each line is searched
// from its start with a newline after it, and positions are UTF-8 offsets.
//
// Usage: bun oracle/case-fold-probe.ts
import { createOnigurumaEngine } from "shiki@4.6.0/engine/oniguruma";
import wasm from "shiki@4.6.0/wasm";
import { mkdirSync, writeFileSync } from "node:fs";

const engine = await createOnigurumaEngine(wasm as any);

// Each pattern with the lines it is asked about. A line holds one character
// unless it tests where a match starts or what a back-reference repeats.
const probes: [string, string[]][] = [
  ["(?i)s", ["ſ", "S", "x"]],
  ["(?i)ſ", ["s", "S"]],
  ["(?i)k", ["K", "K"]],
  ["(?i)K", ["k", "K"]],
  ["(?i)\\x{212A}", ["k"]],
  ["(?i)\\u017F", ["S"]],
  ["(?i)key", ["xxKey", "xxKEY", "xxkex"]],
  ["(?i)ß", ["ẞ"]],
  ["(?i)ẞ", ["ß"]],
  ["(?i)σ", ["ς", "Σ"]],
  ["(?i)ς", ["Σ"]],
  ["(?i)µ", ["μ", "Μ"]],
  ["(?i)Å", ["Å", "å"]],
  ["(?i)ǅ", ["ǆ", "Ǆ"]],
  ["(?i)θ", ["ϑ", "ϴ", "Θ"]],
  ["(?i)i", ["İ", "ı", "I"]],
  ["(?i)ı", ["I", "i"]],
  ["(?i)[a-z]", ["ſ", "K", "Q"]],
  ["(?i)[s]", ["ſ"]],
  ["(?i)[k]", ["K"]],
  ["(?i)[^s]", ["ſ", "S", "t"]],
  ["(?i)[^k]", ["K", "K"]],
  ["(?i)[^a-z]", ["A", "ſ", "1"]],
  ["(?i)[a-z&&[^k]]", ["K", "K", "j"]],
  ["(?i)[\\w&&[^s]]", ["ſ", "s"]],
  ["(?i)[[a-c]]", ["B"]],
  ["(?i)[^[a-c]]", ["B", "d"]],
  ["(?i)[\\x{41}-\\x{43}]", ["b"]],
  ["(?i)[\\x{212A}]", ["K", "k"]],
  ["(?i)\\p{Lu}", ["a", "A"]],
  ["(?i)[\\p{Lu}]", ["a"]],
  ["(?i)[[:upper:]]", ["a"]],
  ["(?i)[[:lower:]]", ["ſ", "A"]],
  ["(?i)\\p{Ll}", ["A"]],
  ["(?i)[^\\p{Ll}]", ["A"]],
  ["(?i)\\P{Ll}", ["A"]],
  ["(?i)\\w", ["K"]],
  ["(?i:k)", ["K"]],
  ["(?i)(?-i:k)", ["K", "k"]],
  ["(?i)(s)\\1", ["sſ", "sS", "ss"]],
  ["(?i)(ſ)\\1", ["ſs"]],
  ["(?i)(\\w+)-\\1", ["Kelvin-KELVIN"]],
];

const patterns = probes.map(([pattern]) => pattern);
const lines = [...new Set(probes.flatMap(([, list]) => list))];
const cases: string[] = [];
const expected: string[] = [];
const utf8 = (text: string, index: number) => Buffer.byteLength(text.slice(0, index));
probes.forEach(([pattern, list], p) => {
  const scanner = engine.createScanner([pattern]);
  for (const line of list) {
    const text = `${line}\n`;
    cases.push(`${p}\t${lines.indexOf(line)}\t0`);
    const found = scanner.findNextMatchSync(text, 0);
    expected.push(
      found
        ? found.captureIndices
            .map(({ start, end }) => (start < 0 || end < start ? "x" : `${utf8(text, start)},${utf8(text, end)}`))
            .join(";")
        : "-",
    );
  }
});

const hex = (text: string) => Buffer.from(text, "utf8").toString("hex");
const dir = new URL("../tests/engine/case-folds/", import.meta.url).pathname;
mkdirSync(dir, { recursive: true });
writeFileSync(`${dir}patterns.hex`, `${patterns.map(hex).join("\n")}\n`);
writeFileSync(`${dir}lines.hex`, `${lines.map(hex).join("\n")}\n`);
writeFileSync(`${dir}cases.tsv`, `${cases.join("\n")}\n`);
writeFileSync(`${dir}expected.tsv`, `${expected.join("\n")}\n`);
console.log(`${patterns.length} patterns, ${lines.length} lines, ${cases.length} cases`);
