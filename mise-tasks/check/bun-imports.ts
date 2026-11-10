#!/usr/bin/env bun
//MISE description="Every npm import in a Bun script names a package and version that package.json pins"
//MISE tools={bun="1.4.4"}
// Bun's auto-install takes an imported package's version from its specifier,
// such as shiki@4.6.0, and the packages below it from bun.lock. bun.lock
// covers only the packages package.json names, so any other import resolves
// outside the lock. bench/corpus and oracle/inputs hold source text, not
// scripts.
import { $ } from "bun";

const pinned: Record<string, string> = (await Bun.file("package.json").json()).dependencies;
const transpiler = new Bun.Transpiler({ loader: "ts" });
const scripts = (await $`git ls-files -- '*.ts' ':!bench/corpus/' ':!oracle/inputs/'`.text()).split("\n").filter(Boolean);
const problems: string[] = [];
for (const script of scripts) {
  for (const { path } of transpiler.scanImports((await Bun.file(script).text()).replace(/^#!.*/, ""))) {
    if (path.startsWith(".") || path === "bun" || /^(node|bun):/.test(path)) continue;
    const versioned = /^((?:@[^/]+\/)?[^/@]+)@([^/]+)/.exec(path);
    if (!versioned) problems.push(`${script}: ${path} names no version`);
    else if (pinned[versioned[1]] !== versioned[2])
      problems.push(`${script}: ${path}, but package.json pins ${versioned[1]} at ${pinned[versioned[1]] ?? "no version"}`);
  }
}
if (problems.length > 0) {
  for (const problem of problems) console.error(`✘ ${problem}`);
  process.exit(1);
}
console.log(`✔ every npm import in ${scripts.length} scripts is pinned in package.json`);
