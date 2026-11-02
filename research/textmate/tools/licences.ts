// Lists every grammar of Shiki 4.6.0 with its source repository, the licence
// tm-grammars records, and the licence GitHub reports for the source repo.
// Usage: bun tools/licences.ts > ../../docs/research/2026-11-02-textmate-engine/licences.tsv
import { grammars } from "tm-grammars@1.32.22";
import { $ } from "bun";

type G = { name: string; source?: string; license?: string; licenseUrl?: string };
const list = grammars as G[];
const repos = new Map<string, string>();
for (const g of list) {
  const m = g.source?.match(/github\.com\/([^/]+\/[^/#]+)/);
  if (m && !repos.has(m[1])) repos.set(m[1], "");
}
for (const repo of repos.keys()) {
  const r = await $`gh api repos/${repo}/license --jq .license.spdx_id`.nothrow().quiet();
  repos.set(repo, r.exitCode === 0 ? r.stdout.toString().trim() : "none-found");
}
console.log("name\tsource\ttm-grammars licence\tgithub licence");
for (const g of list) {
  const m = g.source?.match(/github\.com\/([^/]+\/[^/#]+)/);
  console.log([g.name, g.source ?? "", g.license ?? "", m ? repos.get(m[1]) : "not-github"].join("\t"));
}
