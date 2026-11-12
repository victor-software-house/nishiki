#!/usr/bin/env bun
//MISE description="No multiline text as escaped line feeds, and no literal broken by a backslash"
//MISE tools={bun="1.4.4"}
// CODING_STANDARDS 3.10: multiline text is an indoc! or formatdoc! document in Rust,
// and a dedent template in the Bun scripts.
// It is never a literal with two or more escaped line feeds.
// No Rust string literal continues on the next line through a backslash line break either:
// a reason or message fits on one line, and multiline text is a document
// (mise-tasks/lib/rust-literals.ts).
// A literal of line feeds alone, such as a "\n\n" separator, holds no text and passes.
// So does a template's interpolation: `${rows.join("\n")}\n` holds one line feed of its own.
// The pre-commit hook runs this when a commit touches a Rust file, as check:local does in CI.
// bench/corpus, oracle/inputs and oracle/fuzz hold language text,
// and tests/oracle holds the oracle's output.
// dedent's own tests write escaped line feeds on purpose, to test how it reads them.
// The Bun scripts that write Rust build it from templates with escaped line feeds,
// so they are left alone until rule-table-codegen moves them to codegen/.
// The Rust files they write are checked like any other.
import { $ } from "bun";
import { continuedLiterals } from "../lib/rust-literals.ts";
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
const files = (await $`git ls-files -- '*.rs' '*.ts' ${corpora} ${escapesOnPurpose}`.text())
  .split("\n")
  .filter(Boolean);
const texts = new Map<string, string>();
for (const file of files) texts.set(file, await Bun.file(file).text());

const writers = new Set([...rustWriterFiles(texts)].filter((file) => file.endsWith(".ts")));
const problems = [...texts].flatMap(([file, text]) => {
  if (writers.has(file)) return [];
  return multilineLiterals(file, text, file.endsWith(".rs") ? ['"'] : ['"', "'", "`"]);
});
const continued = [...texts].flatMap(([file, text]) =>
  file.endsWith(".rs") ? continuedLiterals(file, text) : [],
);
if (problems.length > 0) {
  console.error(problems.join("\n"));
  console.error(
    "these literals spell multiline text with escaped line feeds; " +
      "write it with indoc!, formatdoc! or dedent (CODING_STANDARDS 3.10)",
  );
}
if (continued.length > 0) {
  console.error(continued.join("\n"));
  console.error(
    "these string literals continue through a backslash line break; " +
      "fit the text on one line, or write it with indoc! or a raw string (CODING_STANDARDS 3.10)",
  );
}
if (problems.length > 0 || continued.length > 0) process.exit(1);
console.log(
  `✔ no literal in ${files.length} files spells multiline text with escaped line feeds ` +
    "or continues through a backslash line break",
);
