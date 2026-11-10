// Writes tests/engine/case-folds/: Oniguruma's answers, as Shiki bundles it,
// for patterns under `(?i)` on the characters and strings its case folds join,
// which `case_insensitive_matching_answers_as_oniguruma` in
// src/textmate/regex/tests.rs checks the engine against. Each line is searched
// from its start with a newline after it, and positions are UTF-8 offsets.
// The script fails on an answer `oniguruma-answer.ts` calls ambiguous, so no
// chosen case is lost.
//
// Usage: bun oracle/case-fold-probe.ts
import { createOnigurumaEngine } from "shiki@4.6.0/engine/oniguruma";
import wasm from "shiki@4.6.0/wasm";
import { mkdirSync, writeFileSync } from "node:fs";
import { answer } from "./oniguruma-answer.ts";

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
  // Folds of more than one character.
  ["(?i)ß", ["ss", "SS", "sS", "ſs", "ſſ"]],
  ["(?i)ss", ["ß", "ẞ", "xß"]],
  ["(?i)SS", ["ß"]],
  ["(?i)ſs", ["ß"]],
  ["(?i)class", ["claß", "CLASS", "claſs"]],
  ["(?i)sss", ["ßs", "sß", "sss"]],
  ["(?i)ßs", ["sss"]],
  ["(?i)sßs", ["ssss"]],
  ["(?i)ssss", ["ßß"]],
  ["(?i)xss", ["xß"]],
  ["(?i)ßx", ["ssx"]],
  ["(?i)ß+", ["ssss"]],
  ["(?i)ß{2}", ["ssß"]],
  ["(?i)s{2}", ["ß"]],
  ["(?i)s+", ["ß"]],
  ["(?i)ss?", ["ß"]],
  ["(?i)[s][s]", ["ß"]],
  ["(?i)(s)(s)", ["ß"]],
  ["(?i)s\\x{73}", ["ß"]],
  ["(?i)(?:ss)", ["ß"]],
  ["(?i)(?>ss)", ["ß"]],
  ["(?i)a(?:ss)b", ["aßb"]],
  ["(?i)s(?i:s)", ["ß"]],
  ["(?i:s)s", ["ß"]],
  ["s(?i)s", ["ß"]],
  ["(?i)ss|x", ["ß"]],
  ["(?i)(?:s|t)s", ["ß"]],
  ["(?i)st", ["ﬅ", "ﬆ"]],
  ["(?i)ﬆ", ["st", "ſt", "ST"]],
  ["(?i)fi", ["ﬁ", "ﬁx"]],
  ["(?i)ffi", ["ﬃ", "ﬀi"]],
  ["(?i)ff", ["ﬃ"]],
  ["(?i)ﬃ", ["ffi", "FFI", "ﬀi"]],
  ["(?i)ŉ", ["ʼn", "ʼN"]],
  ["(?i)ʼn", ["ŉ"]],
  ["(?i)İ", ["i̇", "İ"]],
  ["(?i)i̇", ["İ"]],
  ["(?i)ΐ", ["ΐ", "ΐ"]],
  ["(?i)\\x{df}", ["ss"]],
  ["(?i)[ß]", ["ss", "ẞ"]],
  ["(?i)[ßx]", ["ss"]],
  ["(?i)[^ß]", ["ss"]],
  ["(?i)[a-ß]", ["ss"]],
  ["(?i)[ß]+", ["ssss"]],
  ["(?i)[ß]x", ["ssx"]],
  ["(?i)[\\w&&ß]", ["ss"]],
  ["(?i)[\\w]x", ["ssx", "ffix", "stx"]],
  ["(?i)\\wx", ["ssx"]],
  ["(?i)[a-z]x", ["ssx"]],
  ["(?i)[[:alpha:]]x", ["ssx"]],
  ["(?i)[\\p{Ll}]x", ["ssx", "ﬃx"]],
  ["(?i)[ßa-z]x", ["ssx"]],
  ["(?i)[\\w]+-", ["straße-", "ﬁnal-"]],
  ["(?i)[\\w]+x", ["ßssx", "ﬃx", "ssy"]],
  ["(?i)(?:(?!--)[\\w])*x", ["ssx", "ﬁx", "s-ﬁx"]],
  ["(?i)(?:(?!t)[\\w])+", ["st", "ﬆ"]],
  ["(?i)(?:(?!t)[\\w])+x", ["stx", "sﬆx"]],
  ["(?i)(?:(?!--|==)[-\\w])*-", ["ss--", "ﬁ-"]],
  ["(?i)([\\w])+x", ["ssx", "ﬁx"]],
  ["(?i)[\\w-]+", ["ß-ss", "-ﬁ", "ss-ß", "ﬁ-"]],
  ["(?i)[\\w-]+x", ["ßx", "ss-ﬁx", "-ssx"]],
  ["(?i)x[\\w-]+", ["xß", "xss", "x-ﬁ"]],
  ["(?i)[\\w-]+ß", ["ss-ss", "ßß", "-ß"]],
  ["(?i)[\\w-]+ﬁ", ["a-fi", "fifi"]],
  ["(?i)[\\w]+?s", ["ßs", "ss"]],
  ["(?i)[\\w]++s", ["ßs"]],
  ["(?i)[\\w]{2,}", ["ß", "ßs", "ss"]],
  ["(?i)[\\w]{2,}?x", ["ssx"]],
  ["(?i)[\\p{L}]+", ["\u03b9\u0308\u0301", "ΐ"]],
  ["(?i)[\\p{L}]+-", ["\u03b9\u0308\u0301-"]],
  ["(?i)(?<=ß)x", ["ssx", "ßx"]],
  ["(?i)(?<=ss)x", ["ßx", "ssx"]],
  ["(?i)(?<=[ß])x", ["ssx", "ßx"]],
  ["(?i)(?<=[\\w])x", ["ssx"]],
  ["(?i)(ß)\\1", ["ßss", "ßß"]],
  ["(?i)(ss)\\1", ["ssß", "ssSS"]],
];

const patterns = probes.map(([pattern]) => pattern);
const lines = [...new Set(probes.flatMap(([, list]) => list))];
const cases: string[] = [];
const expected: string[] = [];
probes.forEach(([pattern, list], p) => {
  const scanner = engine.createScanner([pattern]);
  for (const line of list) {
    const text = `${line}\n`;
    const found = answer(scanner.findNextMatchSync(text, 0)?.captureIndices, text);
    if (found === undefined) throw new Error(`${pattern} on ${JSON.stringify(line)}: an ambiguous answer`);
    cases.push(`${p}\t${lines.indexOf(line)}\t0`);
    expected.push(found);
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
