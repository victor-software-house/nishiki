#!/usr/bin/env bun
//MISE description="No line past 100 characters, outside the kinds that cannot break"
//MISE tools={bun="1.4.4"}
// mise-tasks/lib/line-width.ts holds the bound and the kinds exempt from it.
// bench/corpus, oracle/inputs and tests/oracle hold third-party text and the oracle's output.
import { $ } from "bun";
import { longLines, WIDTH } from "../lib/line-width.ts";
import { rustWriterFiles } from "../lib/rust-writers.ts";

const covered = ["*.rs", "*.ts", "*.sh", "*.toml", "*.yml", "*.yaml", "*.json"];
const corpora = [":!bench/corpus/", ":!oracle/inputs/", ":!tests/oracle/"];
const files = (await $`git ls-files -- ${covered} ${corpora}`.text()).split("\n").filter(Boolean);
const texts = new Map<string, string>();
for (const file of files) texts.set(file, await Bun.file(file).text());

const leftAlone = rustWriterFiles(texts);
const problems: string[] = [];
for (const [file, text] of texts) {
  if (!leftAlone.has(file)) problems.push(...longLines(file, text));
}
if (problems.length > 0) {
  console.error(problems.join("\n"));
  console.error(`these lines pass ${WIDTH} characters; break them (CODING_STANDARDS 3.10)`);
  process.exit(1);
}
console.log(`✔ every line of ${files.length} files fits in ${WIDTH} characters, or cannot break`);
