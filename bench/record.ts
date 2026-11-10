#!/usr/bin/env bun
// A recorded benchmark session (openspec bench-methodology, design
// decisions 11 and 12): `mise run bench:record`.
//
// It runs only on the reference host class, Namespace 8x16, unpinned
// (design decision 11; openspec/research/2026-11-04-bench-host-probe-criterion.md).
// On any other host it refuses and names that class.
// On a clean tree at one commit, it builds the bench workspace, then runs:
//
// - the anchor, lex/python/medium for 2 s, at the start and at the end;
// - `--rounds` rounds (5 at least) of every Criterion bench and of the
//   latency program;
// - the cold-start runner once, whose samples are its own rounds.
//
// It writes the record, bench/results/<class>/<date>-<sha8>.json.
// The harness output goes beside it as .txt.
// A session whose anchor moves more than the class's measured drift bound
// writes no record and fails. Rerun it.
// The host cannot know its own bill.
// `compute` starts null, and `bench/report.ts compute` fills it.

import { $ } from "bun";
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync } from "node:fs";
import { availableParallelism, release } from "node:os";
import { join } from "node:path";
import { parseArgs } from "node:util";
import { dedent } from "../oracle/dedent.ts";

export const SCHEMA = 1;

/** One figure over the rounds: each round's value, then their summary. */
export type Summary = {
  rounds: number[];
  median: number;
  min: number;
  max: number;
  spread_pct: number;
};

/** One Criterion benchmark: the median estimate of each round. */
export type CriterionFigure = {
  id: string;
  unit: "MiB/s" | "Melem/s" | "ns";
  value: Summary;
};

/** One language and tier of the latency program, over the rounds. */
export type LatencyFigure = {
  language: string;
  tier: string;
  lines: number;
  below_resolution: Summary;
  steady_p50_ns: Summary;
  steady_p99_ns: Summary;
  steady_max_ns: Summary;
  first_contact_max_ns: Summary;
  resolution_ns: Summary;
};

/** One file's, the theme's or the grammar's cold-start measure, over the processes. */
export type ColdFigure = {
  kind: "file" | "theme" | "grammar";
  name: string;
  measure: "new" | "first_line" | "document" | "from_json" | "from_bytes" | "link";
  median_us: number;
  p90_us: number;
};

export type BenchRecord = {
  schema: typeof SCHEMA;
  commit: string;
  date: string;
  started: string;
  finished: string;
  corpus_version: number;
  host: {
    class: string;
    cpu: string;
    cpus: number;
    threads_per_core: number;
    memory_mib: number;
    os: string;
    kernel: string;
  };
  toolchain: { rustc: string; cargo: string };
  rounds: number;
  cold_processes: number;
  anchor: { id: string; start_mib_s: number; end_mib_s: number; drift_pct: number; bound_pct: number };
  compute: { seconds: number; unit_minutes: number; usd: number } | null;
  criterion: CriterionFigure[];
  latency: LatencyFigure[];
  cold: ColdFigure[];
};

const ANCHOR = "lex/python/medium";

/**
 * The reference host class: the shape and marker that identify it, and the
 * anchor bound, its measured first-to-last drift (90th percentile over
 * languages, rounded up) in the probe of
 * openspec/research/2026-11-04-bench-host-probe-criterion.md.
 */
export const REFERENCE = {
  class: "namespace-8x16",
  cpus: 8,
  threads_per_core: 2,
  memory_mib: { min: 15_000, max: 17_000 },
  marker: "/nsc",
  bound_pct: 11,
} as const;

const MIB = 1 << 20;

export function summary(rounds: number[]): Summary {
  const sorted = [...rounds].sort((a, b) => a - b);
  const mid = sorted.length >> 1;
  const median = sorted.length % 2 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
  const min = sorted[0];
  const max = sorted[sorted.length - 1];
  return { rounds, median, min, max, spread_pct: ((max - min) / median) * 100 };
}

/** The executables a cargo build with `--message-format=json` produced. */
async function executables(args: string[]): Promise<Map<string, string>> {
  const found = new Map<string, string>();
  for await (const line of $`cargo ${args} --locked --manifest-path bench/Cargo.toml --message-format=json`.lines()) {
    if (!line.startsWith("{")) continue;
    const message = JSON.parse(line);
    if (message.reason === "compiler-artifact" && message.executable) {
      found.set(message.target.name, message.executable);
    }
  }
  return found;
}

/** Each Criterion benchmark saved under `baseline` in `home`, in its unit. */
function criterionFigures(home: string, baseline: string): Map<string, { unit: CriterionFigure["unit"]; value: number }> {
  const figures = new Map<string, { unit: CriterionFigure["unit"]; value: number }>();
  const suffix = `/${baseline}/benchmark.json`;
  for (const path of readdirSync(home, { recursive: true }) as string[]) {
    if (!path.endsWith(suffix)) continue;
    const dir = join(home, path.slice(0, -"/benchmark.json".length));
    const benchmark = JSON.parse(readFileSync(join(dir, "benchmark.json"), "utf8"));
    const ns: number = JSON.parse(readFileSync(join(dir, "estimates.json"), "utf8")).median.point_estimate;
    const throughput = benchmark.throughput ?? {};
    if ("Bytes" in throughput) {
      figures.set(benchmark.full_id, { unit: "MiB/s", value: (throughput.Bytes / ns) * 1e9 / MIB });
    } else if ("Elements" in throughput) {
      figures.set(benchmark.full_id, { unit: "Melem/s", value: (throughput.Elements / ns) * 1e3 });
    } else {
      figures.set(benchmark.full_id, { unit: "ns", value: ns });
    }
  }
  return figures;
}

/** Rows of a tab-separated table, keyed by its header. */
function tsv(text: string): Record<string, string>[] {
  const [header, ...rows] = text.trim().split("\n");
  const names = header.split("\t");
  return rows.map((row) => Object.fromEntries(row.split("\t").map((cell, i) => [names[i], cell])));
}

/** This host's description, or an error naming the reference class when it is another host. */
export async function referenceHost(): Promise<BenchRecord["host"]> {
  if (process.platform !== "linux") throw new Error(`a record comes from the reference host class, ${REFERENCE.class}, and this host is not Linux`);
  const cpuinfo = readFileSync("/proc/cpuinfo", "utf8");
  const meminfo = readFileSync("/proc/meminfo", "utf8");
  const osRelease = readFileSync("/etc/os-release", "utf8");
  const lscpu = await $`lscpu`.nothrow().quiet().text();
  const found = {
    class: REFERENCE.class,
    cpu: cpuinfo.match(/^model name\s*:\s*(.+)$/m)?.[1] ?? "unknown",
    cpus: availableParallelism(),
    threads_per_core: Number(lscpu.match(/^Thread\(s\) per core:\s*(\d+)/m)?.[1] ?? 0),
    memory_mib: Math.round(Number(meminfo.match(/^MemTotal:\s*(\d+) kB/m)?.[1] ?? 0) / 1024),
    os: osRelease.match(/^PRETTY_NAME="?([^"\n]+)"?$/m)?.[1] ?? "unknown",
    kernel: release(),
  };
  const matches =
    existsSync(REFERENCE.marker) &&
    found.cpus === REFERENCE.cpus &&
    found.threads_per_core === REFERENCE.threads_per_core &&
    found.memory_mib >= REFERENCE.memory_mib.min &&
    found.memory_mib <= REFERENCE.memory_mib.max;
  if (!matches) {
    throw new Error(
      `a record comes only from the reference host class, ${REFERENCE.class}: ${REFERENCE.cpus} CPUs, ${REFERENCE.threads_per_core} threads a core, about 16 GB and Namespace's ${REFERENCE.marker}; this host has ${found.cpus} CPUs, ${found.threads_per_core} threads a core, ${found.memory_mib} MiB${existsSync(REFERENCE.marker) ? "" : ` and no ${REFERENCE.marker}`}`,
    );
  }
  return found;
}

async function main(): Promise<void> {
  const { values } = parseArgs({
    options: {
      rounds: { type: "string", default: "5" },
      "cold-processes": { type: "string", default: "31" },
    },
  });
  const rounds = Number(values.rounds);
  const coldProcesses = Number(values["cold-processes"]);
  if (!Number.isInteger(rounds) || rounds < 5) throw new Error("--rounds needs a whole number of at least 5");
  if (!Number.isInteger(coldProcesses) || coldProcesses < 4) throw new Error("--cold-processes needs a whole number of at least 4");
  const host = await referenceHost();
  if ((await $`git status --porcelain`.text()).trim() !== "") {
    throw new Error("the tree has changes: a record names one commit, so commit or stash them first");
  }

  const started = new Date();
  const commit = (await $`git rev-parse HEAD`.text()).trim();
  const date = started.toISOString().slice(0, 10);
  const stem = `${date}-${commit.slice(0, 8)}`;
  const corpusVersion = Number(readFileSync("bench/corpus.toml", "utf8").match(/^version = (\d+)$/m)?.[1]);
  const home = join(process.env.CARGO_TARGET_DIR ?? "target", "record", stem);
  rmSync(home, { recursive: true, force: true });
  mkdirSync(home, { recursive: true });
  const env = { ...process.env, CRITERION_HOME: home };

  const benches = await executables(["bench", "--no-run"]);
  const bins = await executables(["build", "--release", "--bins"]);
  const throughput = benches.get("throughput");
  const styling = benches.get("styling");
  const latency = bins.get("latency");
  const cold = bins.get("cold");
  if (!throughput || !styling || !latency || !cold) throw new Error("the bench workspace built without one of its programs");

  let raw = dedent`
    commit ${commit}
    started ${started.toISOString()}
    `;
  const run = async (title: string, command: string[]): Promise<string> => {
    const output = await $`${command}`.env(env).text();
    raw += ["", `## ${title}`, "", output].join("\n");
    return output;
  };
  const anchor = async (baseline: string): Promise<number> => {
    await run(`anchor, ${baseline}`, [throughput, "--bench", `^${ANCHOR}$`, "--warm-up-time", "1", "--measurement-time", "2", "--save-baseline", baseline]);
    const figure = criterionFigures(home, baseline).get(ANCHOR);
    if (!figure) throw new Error(`the anchor ${ANCHOR} printed no estimate`);
    return figure.value;
  };

  const anchorStart = await anchor("anchor-start");
  const criterion = new Map<string, { unit: CriterionFigure["unit"]; rounds: number[] }>();
  const latencyRows = new Map<string, Record<string, string>[]>();
  for (let round = 1; round <= rounds; round++) {
    const baseline = `round-${round}`;
    await run(`throughput, round ${round}`, [throughput, "--bench", "--save-baseline", baseline]);
    await run(`styling, round ${round}`, [styling, "--bench", "--save-baseline", baseline]);
    for (const [id, figure] of criterionFigures(home, baseline)) {
      const entry = criterion.get(id) ?? { unit: figure.unit, rounds: [] };
      entry.rounds.push(figure.value);
      criterion.set(id, entry);
    }
    for (const row of tsv(await run(`latency, round ${round}`, [latency, "--tsv"]))) {
      const key = `${row.language}\t${row.tier}`;
      latencyRows.set(key, [...(latencyRows.get(key) ?? []), row]);
    }
  }
  const coldRows = tsv(await run("cold", [cold, "--processes", String(coldProcesses), "--tsv"]));
  const anchorEnd = await anchor("anchor-end");
  const driftPct = Math.abs(anchorEnd / anchorStart - 1) * 100;

  if (driftPct > REFERENCE.bound_pct) {
    await Bun.write(join(home, `${stem}.txt`), raw);
    throw new Error(
      `the anchor moved ${driftPct.toFixed(2)}% (${anchorStart.toFixed(1)} to ${anchorEnd.toFixed(1)} MiB/s), over the reference class's bound of ${REFERENCE.bound_pct}%: no record written; rerun. The output is in ${home}`,
    );
  }
  const numbers = (rows: Record<string, string>[], column: string) => summary(rows.map((row) => Number(row[column])));
  const record: BenchRecord = {
    schema: SCHEMA,
    commit,
    date,
    started: started.toISOString(),
    finished: new Date().toISOString(),
    corpus_version: corpusVersion,
    host,
    toolchain: {
      rustc: (await $`rustc -V`.text()).trim(),
      cargo: (await $`cargo -V`.text()).trim(),
    },
    rounds,
    cold_processes: coldProcesses,
    anchor: { id: ANCHOR, start_mib_s: anchorStart, end_mib_s: anchorEnd, drift_pct: driftPct, bound_pct: REFERENCE.bound_pct },
    compute: null,
    criterion: [...criterion].map(([id, { unit, rounds }]) => ({ id, unit, value: summary(rounds) })),
    latency: [...latencyRows.values()].map((rows) => ({
      language: rows[0].language,
      tier: rows[0].tier,
      lines: Number(rows[0].lines),
      below_resolution: numbers(rows, "below_resolution"),
      steady_p50_ns: numbers(rows, "steady_p50_ns"),
      steady_p99_ns: numbers(rows, "steady_p99_ns"),
      steady_max_ns: numbers(rows, "steady_max_ns"),
      first_contact_max_ns: numbers(rows, "first_contact_max_ns"),
      resolution_ns: numbers(rows, "resolution_ns"),
    })),
    cold: coldRows.map((row) => ({
      kind: row.kind as ColdFigure["kind"],
      name: row.name,
      measure: row.measure as ColdFigure["measure"],
      median_us: Number(row.median_us),
      p90_us: Number(row.p90_us),
    })),
  };
  const dir = join("bench/results", host.class);
  mkdirSync(dir, { recursive: true });
  const json = join(dir, `${stem}.json`);
  if (existsSync(json)) throw new Error(`${json} exists: one record per commit and day`);
  await Bun.write(json, `${JSON.stringify(record, null, 2)}\n`);
  await Bun.write(join(dir, `${stem}.txt`), raw);
  console.log(`✔ ${json}, anchor drift ${driftPct.toFixed(2)}%`);
}

if (import.meta.main) {
  try {
    await main();
  } catch (error) {
    console.error(`✘ ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  }
}
