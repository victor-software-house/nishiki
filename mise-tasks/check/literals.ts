#!/usr/bin/env bun
//MISE description="No multiline text as a literal of escaped line feeds"
//MISE tools={bun="1.4.4"}
// CODING_STANDARDS 3.10: multiline text is an indoc! or formatdoc! document in Rust,
// and a dedent template in the Bun scripts.
// It is never a literal with two or more escaped line feeds.
// A literal of line feeds alone, such as a "\n\n" separator, holds no text and passes.
// So does a template's interpolation: `${rows.join("\n")}\n` holds one line feed of its own.
// The pre-commit hook runs this when a commit touches a Rust file, as check:local does in CI.
// bench/corpus, oracle/inputs and oracle/fuzz hold language text,
// and tests/oracle holds the oracle's output.
// dedent's own tests write escaped line feeds on purpose, to test how it reads them.
import { $ } from "bun";
import { rustWriterFiles } from "../lib/rust-writers.ts";

/** A string literal opened by each quote, its escapes and interpolations kept as written. */
const literals: Record<string, RegExp> = {
  '"': /"(?:[^"\\]|\\.)*"/g,
  "'": /'(?:[^'\\]|\\.)*'/g,
  "`": /`(?:[^`\\]|\\.)*`/g,
};

/** Whether a literal's body spells multiline text: two escaped line feeds and some text. */
function multiline(body: string): boolean {
  const pieces = body.replace(/\$\{(?:[^{}]|\{[^{}]*\})*\}/g, "x").match(/\\.|[^\\]/g) ?? [];
  const lineFeeds = pieces.filter((piece) => piece === "\\n").length;
  return lineFeeds >= 2 && pieces.some((piece) => piece !== "\\n");
}

/** The lines of `text` with such a literal, as `file:line: text` problems. */
function multilineLiterals(file: string, text: string, quotes: string[]): string[] {
  return text.split("\n").flatMap((line, index) => {
    if (/^\s*\/\//.test(line)) return [];
    const found = quotes.some((quote) =>
      [...line.matchAll(literals[quote]!)].some(([literal]) => multiline(literal.slice(1, -1))),
    );
    return found ? [`${file}:${index + 1}: ${line.trim()}`] : [];
  });
}

const corpora = [":!bench/corpus/", ":!oracle/inputs/", ":!oracle/fuzz/", ":!tests/oracle/"];
const escapesOnPurpose = [":!oracle/dedent.test.ts"];
const files = (
  await $`git ls-files -- 'src/*.rs' 'tests/*.rs' '*.ts' ${corpora} ${escapesOnPurpose}`.text()
)
  .split("\n")
  .filter(Boolean);
const texts = new Map<string, string>();
for (const file of files) texts.set(file, await Bun.file(file).text());

const leftAlone = rustWriterFiles(texts);
const problems = [...texts].flatMap(([file, text]) => {
  if (leftAlone.has(file)) return [];
  return multilineLiterals(file, text, file.endsWith(".rs") ? ['"'] : ['"', "'", "`"]);
});
if (problems.length > 0) {
  console.error(problems.join("\n"));
  console.error(
    "these literals spell multiline text with escaped line feeds; " +
      "write it with indoc!, formatdoc! or dedent (CODING_STANDARDS 3.10)",
  );
  process.exit(1);
}
console.log(`✔ no literal in ${files.length} files spells multiline text with escaped line feeds`);
