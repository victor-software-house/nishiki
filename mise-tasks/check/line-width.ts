#!/usr/bin/env bun
//MISE description="No line past 100 characters, outside the kinds that cannot break"
//MISE tools={bun="1.4.4"}
// CODING_STANDARDS 3.10 sets the bound and its kinds.
// A line counts its characters, not its bytes.
// bench/corpus, oracle/inputs and tests/oracle hold third-party text and the oracle's output.
import { $ } from "bun";
import { commentText, rustWriterFiles } from "../lib/rust-writers.ts";

const WIDTH = 100;

/**
 * The task files, workflows and settings the Copier template renders.
 * Their long lines change in the template, so this list goes
 * once `.copier-answers.yml` records the release that wraps them.
 */
const templateRendered = {
  removedAt: "v0.0.91",
  paths: [
    ".copier-answers.yml",
    ".github/actionlint.yaml",
    ".github/workflows/ci.yml",
    ".github/workflows/secrets.yml",
    ".github/workflows/template.yml",
    ".rumdl.toml",
    "mise-tasks/check/features.sh",
    "mise-tasks/check/local.sh",
    "mise-tasks/check/repo.sh",
    "mise-tasks/hooks/lockfile.sh",
    "mise-tasks/hooks/secrets/history.sh",
    "mise-tasks/lint/workflow-policy.sh",
    "mise-tasks/lint/workflows.sh",
  ],
};

/** What a test of an unbreakable line sees. */
type LongLine = {
  file: string;
  text: string;
  /** Whether the line is a Rust doc line that starts in code: a fenced block or a code span. */
  startsInCode: boolean;
};

/** A sort of line that cannot break, by what it holds. */
type Unbreakable = { name: string; holds: (line: LongLine) => boolean };

const unbreakable: Unbreakable[] = [
  { name: "a URL", holds: ({ text }) => /[a-z]+:\/\/\S/.test(text) || /^\s*url: `/.test(text) },
  {
    name: "an action pinned to a commit",
    holds: ({ text }) => /^\s*(- )?uses: \S+@[0-9a-f]{40}( #.*)?$/.test(text),
  },
  {
    name: "an import",
    holds: ({ file, text }) => file.endsWith(".ts") && /^import .* from "[^"]+"/.test(text),
  },
  {
    name: "a regex literal",
    holds: ({ file, text }) =>
      file.endsWith(".ts") && /^\/.*\/[dgimsuyv]*[;,)]?$/.test(text.trim()),
  },
  {
    name: "a Markdown table row",
    holds: ({ text }) => /^(echo ")?\|.*\|"?$/.test(commentText(text)),
  },
  {
    name: "a regex in a matcher's docs",
    holds: ({ startsInCode }) => startsInCode,
  },
  {
    name: "a JSON string",
    holds: ({ file, text }) =>
      file.endsWith(".json") &&
      /^\s*("(?:[^"\\]|\\.)*":\s*)?("(?:[^"\\]|\\.)*",?|[{[])$/.test(text),
  },
  {
    name: "a Swift matcher name",
    // It stays until the Swift generator moves to codegen/ (rust-codegen design 8.4).
    holds: ({ file, text }) => file.startsWith("src/swift/") && /\brx_\w{40,}/.test(text),
  },
];

/** The lines of `text` past `WIDTH` that can break, as `file:line: width` problems. */
function longLines(file: string, text: string): string[] {
  const rust = file.endsWith(".rs");
  const found: string[] = [];
  let inDocBlock = false;
  let inSpan = false;
  text.split("\n").forEach((line, index) => {
    const doc = rust && /^\s*\/\/[/!]/.test(line);
    const startsInCode = doc && (inDocBlock || inSpan || commentText(line).startsWith("`"));
    if (!doc) inSpan = false;
    else if (/^\s*\/\/[/!]\s*```/.test(line)) inDocBlock = !inDocBlock;
    else if (!inDocBlock) inSpan = inSpan !== (line.split("`").length % 2 === 0);
    const width = [...line].length;
    if (width <= WIDTH) return;
    if (unbreakable.some((sort) => sort.holds({ file, text: line, startsInCode }))) return;
    found.push(`${file}:${index + 1}: ${width} characters`);
  });
  return found;
}

const covered = ["*.rs", "*.ts", "*.sh", "*.toml", "*.yml", "*.yaml", "*.json"];
const corpora = [":!bench/corpus/", ":!oracle/inputs/", ":!tests/oracle/"];
const files = (await $`git ls-files -- ${covered} ${corpora}`.text()).split("\n").filter(Boolean);
const texts = new Map<string, string>();
for (const file of files) texts.set(file, await Bun.file(file).text());

const leftAlone = rustWriterFiles(texts);
for (const path of templateRendered.paths) leftAlone.add(path);
const recorded = /^_commit: v([\d.]+)$/m.exec(texts.get(".copier-answers.yml") ?? "")?.[1] ?? "";
const problems: string[] = [];
if (Bun.semver.order(recorded, templateRendered.removedAt.slice(1)) >= 0) {
  problems.push(
    `.copier-answers.yml records v${recorded}: ` +
      "remove templateRendered from mise-tasks/check/line-width.ts",
  );
}
for (const [file, text] of texts) {
  if (!leftAlone.has(file)) problems.push(...longLines(file, text));
}
if (problems.length > 0) {
  console.error(problems.join("\n"));
  console.error(`these lines pass ${WIDTH} characters; break them (CODING_STANDARDS 3.10)`);
  process.exit(1);
}
console.log(`✔ every line of ${files.length} files fits in ${WIDTH} characters, or cannot break`);
