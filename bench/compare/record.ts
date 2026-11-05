#!/usr/bin/env bun
// A recorded comparison with other highlighters (openspec bench-methodology,
// task 7.4): `mise run bench:compare-record`.
//
// It runs only on the reference host class, as bench/record.ts does, and on a
// clean tree at one commit.
// It runs every engine of the comparison on the medium and large tiers:
// the Rust harness's engines through `compare --tsv`, and Shiki's two engines
// and @pierre/highlights through `shiki.ts time --tsv`.
// An anchor, nishiki's warm throughput on python's medium file in the Rust
// harness, runs at the start and the end.
//
// It writes bench/compare/results/<class>/<date>-<sha8>.json, the record,
// and the harnesses' output beside it as .txt.
// A session whose anchor moves more than the class's bound writes no record.
// `bench/report.ts compute` fills in the compute, which the host cannot know.

import { $ } from "bun";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { parseArgs } from "node:util";
import { type BenchRecord, REFERENCE, referenceHost, type Summary, summary } from "../record.ts";

export const COMPARE_SCHEMA = 1;

/** One engine of the comparison, as the record names it. */
export type CompareEngine = {
  name: string;
  version: string;
  features: string;
  backend: string;
  runtime: "rust" | "bun";
};

/** One engine's figures on one file. */
export type CompareRow = {
  file: string;
  engine: string;
  setup_ms: Summary;
  first_ms: Summary;
  mib_per_s: Summary;
  tokens: number;
  fidelity: number | null;
};

export type CompareRecord = {
  schema: typeof COMPARE_SCHEMA;
  commit: string;
  date: string;
  started: string;
  finished: string;
  corpus_version: number;
  host: BenchRecord["host"];
  toolchain: { rustc: string; cargo: string; bun: string };
  engines: CompareEngine[];
  left_out: Record<string, string[]>;
  tiers: string[];
  rounds: number;
  cold_processes: number;
  anchor: BenchRecord["anchor"];
  compute: BenchRecord["compute"];
  rows: CompareRow[];
};

const TIERS = ["medium", "large"];
const MANIFEST = "bench/compare/Cargo.toml";
const FEATURES = "giallo";

/** A crate's version in bench/compare/Cargo.lock. */
function locked(crate: string): string {
  const lock = readFileSync("bench/compare/Cargo.lock", "utf8");
  const version = new RegExp(`name = "${crate}"\\nversion = "([^"]+)"`).exec(lock)?.[1];
  if (!version) throw new Error(`${crate} is not in bench/compare/Cargo.lock`);
  return version;
}

/** A package's pinned version in bench/compare/shiki.ts's imports. */
function pinned(spec: string): string {
  const version = new RegExp(`"${spec.replace("/", "\\/")}@(\\d+\\.\\d+\\.\\d+)`).exec(readFileSync("bench/compare/shiki.ts", "utf8"))?.[1];
  if (!version) throw new Error(`${spec} has no pinned version in bench/compare/shiki.ts`);
  return version;
}

/** The engines of the comparison, with the versions their builds pin. */
function engines(commit: string): CompareEngine[] {
  return [
    { name: "nishiki", version: commit.slice(0, 8), features: "every language, textmate tier", backend: "hand lexers and its TextMate engine", runtime: "rust" },
    { name: "syntect", version: `${locked("syntect")}, two-face ${locked("two-face")}`, features: "default", backend: "Oniguruma", runtime: "rust" },
    { name: "tree-sitter", version: locked("tree-sitter-highlight"), features: "one grammar crate per language", backend: "tree-sitter", runtime: "rust" },
    { name: "giallo", version: locked("giallo"), features: "giallo", backend: "its own TextMate engine", runtime: "rust" },
    { name: "shiki-oniguruma", version: pinned("shiki"), features: "limits off", backend: "Oniguruma in WebAssembly", runtime: "bun" },
    { name: "shiki-javascript", version: pinned("shiki"), features: "limits off", backend: "JavaScript regex engine", runtime: "bun" },
    { name: "pierre", version: pinned("@pierre/highlights"), features: "line limit off", backend: "its own WebAssembly lexers", runtime: "bun" },
  ];
}

/** The rows of a `--tsv` output, keyed by its header. */
function tsvRows(text: string): Record<string, string>[] {
  const [header, ...rows] = text.trim().split("\n");
  const names = (header ?? "").split("\t");
  return rows.map((row) => Object.fromEntries(row.split("\t").map((cell, i) => [names[i], cell])));
}

/** One TSV row as a record row. */
function compareRow(row: Record<string, string>): CompareRow {
  const list = (cell: string | undefined) => (cell ?? "").split(",").map(Number);
  return {
    file: row.file ?? "",
    engine: row.engine ?? "",
    setup_ms: summary(list(row.setup_ms)),
    first_ms: summary(list(row.first_ms)),
    mib_per_s: summary(list(row.mib_per_s)),
    tokens: Number(row.tokens),
    fidelity: row.fidelity === "n/a" ? null : Number(row.fidelity),
  };
}

async function main(): Promise<void> {
  const { values } = parseArgs({
    options: {
      rounds: { type: "string", default: "5" },
      cold: { type: "string", default: "11" },
    },
  });
  const rounds = Number(values.rounds);
  const cold = Number(values.cold);
  if (!Number.isInteger(rounds) || rounds < 5) throw new Error("--rounds needs a whole number of at least 5");
  if (!Number.isInteger(cold) || cold < 3) throw new Error("--cold needs a whole number of at least 3");
  const host = await referenceHost();
  if ((await $`git status --porcelain`.text()).trim() !== "") {
    throw new Error("the tree has changes: a record names one commit, so commit or stash them first");
  }

  const started = new Date();
  const commit = (await $`git rev-parse HEAD`.text()).trim();
  const date = started.toISOString().slice(0, 10);
  const stem = `${date}-${commit.slice(0, 8)}`;
  const corpusVersion = Number(readFileSync("bench/corpus.toml", "utf8").match(/^version = (\d+)$/m)?.[1]);
  const cargo = ["cargo", "run", "--quiet", "--locked", "--release", "--manifest-path", MANIFEST, "--features", FEATURES, "--"];

  let raw = `commit ${commit}\nstarted ${started.toISOString()}\n`;
  const run = async (title: string, command: string[]): Promise<string> => {
    const output = await $`${command}`.text();
    raw += `\n## ${title}\n\n${output}`;
    return output;
  };
  await run("Shiki's reference colours", ["bun", "bench/compare/shiki.ts", "reference"]);
  await run("Pierre's theme", [...cargo, "--pierre-theme", "target/compare/pierre-theme.json"]);
  let anchorFile = "";
  const anchor = async (title: string): Promise<number> => {
    const rows = tsvRows(await run(title, [...cargo, "--tsv", "--tier", "medium", "--rounds", "5", "--cold", "3", "python"]));
    const nishiki = rows.find((row) => row.engine === "nishiki");
    if (!nishiki) throw new Error("the anchor printed no nishiki row");
    anchorFile = nishiki.file ?? "";
    return summary((nishiki.mib_per_s ?? "").split(",").map(Number)).median;
  };

  const anchorStart = await anchor("anchor, start");
  const rows: CompareRow[] = [];
  for (const tier of TIERS) {
    const output = await run(`Rust engines, ${tier} tier`, [...cargo, "--tsv", "--tier", tier, "--rounds", String(rounds), "--cold", String(cold)]);
    rows.push(...tsvRows(output).map(compareRow));
  }
  const bun = await run("Bun engines", ["bun", "bench/compare/shiki.ts", "time", "--tsv", "--rounds", String(rounds), "--cold", String(cold), ...TIERS.flatMap((tier) => ["--tier", tier])]);
  rows.push(...tsvRows(bun).map(compareRow));
  const anchorEnd = await anchor("anchor, end");
  const driftPct = Math.abs(anchorEnd / anchorStart - 1) * 100;
  const home = join("target/compare/record", stem);
  if (driftPct > REFERENCE.bound_pct) {
    mkdirSync(home, { recursive: true });
    await Bun.write(join(home, `${stem}.txt`), raw);
    throw new Error(
      `the anchor moved ${driftPct.toFixed(2)}% (${anchorStart.toFixed(1)} to ${anchorEnd.toFixed(1)} MiB/s), over the reference class's bound of ${REFERENCE.bound_pct}%: no record written; rerun. The output is in ${home}`,
    );
  }

  const all = engines(commit);
  const languages = [...new Set(rows.map((row) => row.file.split("/")[0] ?? ""))].sort();
  const left_out = Object.fromEntries(
    all.map((engine) => [
      engine.name,
      languages.filter((language) => !rows.some((row) => row.engine === engine.name && row.file.startsWith(`${language}/`))),
    ]),
  );
  const record: CompareRecord = {
    schema: COMPARE_SCHEMA,
    commit,
    date,
    started: started.toISOString(),
    finished: new Date().toISOString(),
    corpus_version: corpusVersion,
    host,
    toolchain: {
      rustc: (await $`rustc -V`.text()).trim(),
      cargo: (await $`cargo -V`.text()).trim(),
      bun: (await $`bun --version`.text()).trim(),
    },
    engines: all,
    left_out,
    tiers: TIERS,
    rounds,
    cold_processes: cold,
    anchor: { id: `nishiki on ${anchorFile}`, start_mib_s: anchorStart, end_mib_s: anchorEnd, drift_pct: driftPct, bound_pct: REFERENCE.bound_pct },
    compute: null,
    rows,
  };
  const dir = join("bench/compare/results", host.class);
  mkdirSync(dir, { recursive: true });
  const json = join(dir, `${stem}.json`);
  if (existsSync(json)) throw new Error(`${json} exists: one record per commit and day`);
  await Bun.write(json, `${JSON.stringify(record, null, 2)}\n`);
  await Bun.write(join(dir, `${stem}.txt`), raw);
  console.log(`✔ ${json}, ${rows.length} rows, anchor drift ${driftPct.toFixed(2)}%`);
}

if (import.meta.main) {
  try {
    await main();
  } catch (error) {
    console.error(`✘ ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  }
}
