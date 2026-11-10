#!/usr/bin/env bun
//MISE description="No line past 100 characters, outside the kinds that cannot break"
//MISE tools={bun="1.4.4"}
// mise-tasks/lib/line-width.ts holds the bound and the kinds exempt from it.
// bench/corpus, oracle/inputs and tests/oracle hold third-party text and the oracle's output.
import { $ } from "bun";
import { longLines, WIDTH } from "../lib/line-width.ts";
import { rustWriterFiles } from "../lib/rust-writers.ts";

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
