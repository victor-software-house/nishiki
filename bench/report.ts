#!/usr/bin/env bun
// Renders benchmark records (openspec bench-methodology, design decision 12).
//
// - `render [record.json...]` writes each record's Markdown beside it, every
//   record when none is named.
// - `compute <record.json> <seconds> <unit-minutes> <usd>` fills the record's
//   compute, which the host cannot know, then renders it.
// - `summary` writes the newest record's summary into bench/README.md,
//   between the record markers.
// - `check` fails when a record's Markdown or the README summary differs
//   from what its JSON renders: nobody edits either by hand.

import { readdirSync, readFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { type BenchRecord, SCHEMA, type Summary } from "./record.ts";

const RESULTS = "bench/results";
const README = "bench/README.md";
const BEGIN = "<!-- record summary: rendered by `mise run bench:report summary` -->";
const END = "<!-- end of record summary -->";
const NO_RECORD = "No record under this method yet.";
const PROBE = "openspec/research/2026-11-04-bench-host-probe-criterion.md";

/** Fails with `message` unless `condition` holds. */
function expect(condition: boolean, path: string, message: string): asserts condition {
  if (!condition) throw new Error(`${path}: ${message}`);
}

function isSummary(value: unknown): value is Summary {
  const summary = value as Summary;
  return (
    typeof summary === "object" &&
    summary !== null &&
    Array.isArray(summary.rounds) &&
    summary.rounds.every((n) => typeof n === "number") &&
    [summary.median, summary.min, summary.max, summary.spread_pct].every((n) => typeof n === "number")
  );
}

/** Reads and checks one record: the files are the boundary. */
export function readRecord(path: string): BenchRecord {
  const record = JSON.parse(readFileSync(path, "utf8")) as BenchRecord;
  expect(record.schema === SCHEMA, path, `schema ${record.schema} is not ${SCHEMA}`);
  for (const key of ["commit", "date", "started", "finished"] as const) {
    expect(typeof record[key] === "string", path, `${key} is not a string`);
  }
  expect(
    typeof record.host?.class === "string" && typeof record.host.cpu === "string" && typeof record.host.cpus === "number",
    path,
    "host is incomplete",
  );
  expect(typeof record.anchor?.drift_pct === "number" && typeof record.anchor.bound_pct === "number", path, "anchor is incomplete");
  expect(Array.isArray(record.criterion) && record.criterion.every((f) => typeof f.id === "string" && isSummary(f.value)), path, "a Criterion figure is malformed");
  expect(
    Array.isArray(record.latency) && record.latency.every((f) => isSummary(f.steady_p99_ns) && isSummary(f.first_contact_max_ns) && isSummary(f.resolution_ns)),
    path,
    "a latency figure is malformed",
  );
  expect(Array.isArray(record.cold) && record.cold.every((f) => typeof f.median_us === "number"), path, "a cold figure is malformed");
  return record;
}

/** Every record's JSON path, oldest first by start time. */
function records(): { path: string; record: BenchRecord }[] {
  const found = (readdirSync(RESULTS, { recursive: true }) as string[])
    .filter((path) => path.endsWith(".json"))
    .map((path) => ({ path: join(RESULTS, path), record: readRecord(join(RESULTS, path)) }));
  return found.sort((a, b) => ordinal(a.record.started, b.record.started));
}

/** Ordinal order, the same under every locale. */
const ordinal = (a: string, b: string) => (a < b ? -1 : a > b ? 1 : 0);

const fixed = (n: number, digits: number) => n.toLocaleString("en-US", { minimumFractionDigits: digits, maximumFractionDigits: digits });

/** A latency figure in µs, or `<` its resolution when below it. */
function micros(ns: number, resolutionNs: number): string {
  return ns < resolutionNs ? `<${fixed(resolutionNs / 1e3, 3)}` : fixed(ns / 1e3, 2);
}

/** What a record's wall-clock figures can carry, with the probe note at `probe`. */
function standing(record: BenchRecord, probe: string): string {
  return `indicative only: no host class keeps two runs of one build within 2%, and this one moves by up to ${fixed(record.anchor.bound_pct, 0)}% ([probe](${probe})), so speed decisions rest on Callgrind instruction counts`;
}

function hostLine(record: BenchRecord): string {
  const h = record.host;
  return `${h.class}: ${h.cpu}, ${h.cpus} CPUs (${h.threads_per_core} ${h.threads_per_core === 1 ? "thread" : "threads"} a core), ${fixed(h.memory_mib / 1024, 1)} GiB, ${h.os}, Linux ${h.kernel}`;
}

/** The record's own page. */
export function renderRecord(record: BenchRecord, stem: string): string {
  const sha8 = record.commit.slice(0, 8);
  const compute = record.compute
    ? `${record.compute.seconds} s, ${fixed(record.compute.unit_minutes, 1)} unit minutes, $${fixed(record.compute.usd, 2)}`
    : "not filled in";
  const out: string[] = [
    `# ${record.date}, commit ${sha8}`,
    "",
    `A recorded session under the \`bench-methodology\` method, rendered from its JSON by \`mise run bench:report\`. Its figures are ${standing(record, `../../../${PROBE}`)}.`,
    "",
    "## Session",
    "",
    "| | |",
    "|:--|:--|",
    `| Commit | \`${record.commit}\`, kept reachable by the tag \`bench/${record.commit}\` |`,
    `| Host | ${hostLine(record)} |`,
    `| Toolchain | ${record.toolchain.rustc}, ${record.toolchain.cargo} |`,
    `| When | ${record.started} to ${record.finished} |`,
    `| Rounds | ${record.rounds} of every Criterion bench and of the latency program; ${record.cold_processes} cold processes per file, the first 3 rounds dropped |`,
    `| Anchor | \`${record.anchor.id}\` for 2 s: ${fixed(record.anchor.start_mib_s, 1)} MiB/s at the start, ${fixed(record.anchor.end_mib_s, 1)} at the end, drift ${fixed(record.anchor.drift_pct, 2)}%, within the class's bound of ${fixed(record.anchor.bound_pct, 0)}% |`,
    `| Corpus | version ${record.corpus_version} |`,
    `| Compute | ${compute} |`,
    "",
    `The harness output, unchanged, is in [\`${stem}.txt\`](${stem}.txt).`,
    "Each figure is the median over the rounds; spread is the largest round minus the smallest, over the median.",
    "",
    "## Criterion",
    "",
    "| benchmark | median | unit | min | max | spread |",
    "|:--|--:|:--|--:|--:|--:|",
  ];
  for (const figure of record.criterion.toSorted((a, b) => ordinal(a.id, b.id))) {
    const digits = figure.unit === "ns" ? 0 : 2;
    const v = figure.value;
    out.push(`| ${figure.id} | ${fixed(v.median, digits)} | ${figure.unit} | ${fixed(v.min, digits)} | ${fixed(v.max, digits)} | ${fixed(v.spread_pct, 1)}% |`);
  }
  out.push(
    "",
    "## Latency",
    "",
    "µs per line; steady figures are each line's median over the passes after the first, then p50, p99 and the maximum over lines.",
    "",
    "| language | tier | lines | steady p50 | steady p99 | steady max | first-contact max | p99 spread |",
    "|:--|:--|--:|--:|--:|--:|--:|--:|",
  );
  for (const f of record.latency) {
    const res = f.resolution_ns.median;
    out.push(
      `| ${f.language} | ${f.tier} | ${f.lines} | ${micros(f.steady_p50_ns.median, res)} | ${micros(f.steady_p99_ns.median, res)} | ${micros(f.steady_max_ns.median, res)} | ${micros(f.first_contact_max_ns.median, res)} | ${fixed(f.steady_p99_ns.spread_pct, 1)}% |`,
    );
  }
  out.push("", "## Cold start", "", "µs in a fresh process, from just before the highlighter is created.", "", "| file | measure | median | p90 |", "|:--|:--|--:|--:|");
  for (const f of record.cold) {
    out.push(`| ${f.name} | ${f.measure.replace("_", " ")} | ${fixed(f.median_us, 1)} | ${fixed(f.p90_us, 1)} |`);
  }
  return `${out.join("\n")}\n`;
}

/** The median of `values`. */
function median(values: number[]): number {
  const sorted = values.toSorted((a, b) => a - b);
  const mid = sorted.length >> 1;
  return sorted.length % 2 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
}

/** One language's figure: its median over the rounds and its spread, when it has rounds. */
type Entry = { name: string; value: number; spread_pct: number | null };

/** The median over languages, the largest or smallest with its name, and the median spread over the rounds. */
function overLanguages(entries: Entry[], worst: "max" | "min"): string {
  if (entries.length === 0) return "| | |";
  const pick = entries.reduce((a, b) => ((worst === "max" ? b.value > a.value : b.value < a.value) ? b : a));
  const spreads = entries.flatMap((e) => (e.spread_pct === null ? [] : [e.spread_pct]));
  const spread = spreads.length === 0 ? "per process" : `${fixed(median(spreads), 1)}%`;
  return `${fixed(median(entries.map((e) => e.value)), 1)} | ${fixed(pick.value, 1)} (${pick.name}) | ${spread} |`;
}

/** The README's summary of the newest record, markers included: one short
 * table over the languages, linking the full record. */
export function renderSummary(newest: { path: string; record: BenchRecord } | undefined): string {
  if (!newest) return `${BEGIN}\n\n${NO_RECORD}\n\n${END}`;
  const { path, record } = newest;
  const page = join(basename(dirname(path)), `${basename(path, ".json")}.md`);
  const medium = record.latency.filter((f) => f.tier === "medium");
  const rows: [string, Entry[], "max" | "min"][] = [
    [
      "Medium throughput, MiB/s",
      record.criterion
        .filter((f) => /^lex\/[^/]+\/medium$/.test(f.id))
        .map((f) => ({ name: f.id.split("/")[1], value: f.value.median, spread_pct: f.value.spread_pct })),
      "min",
    ],
    [
      "Medium steady p99 per line, µs",
      medium.map((f) => ({ name: f.language, value: f.steady_p99_ns.median / 1e3, spread_pct: f.steady_p99_ns.spread_pct })),
      "max",
    ],
    [
      "Medium first-contact max per line, µs",
      medium.map((f) => ({ name: f.language, value: f.first_contact_max_ns.median / 1e3, spread_pct: f.first_contact_max_ns.spread_pct })),
      "max",
    ],
    [
      "Small first line in a fresh process, µs",
      record.cold
        .filter((f) => f.kind === "file" && f.measure === "first_line")
        .map((f) => ({ name: f.name.split("/")[0], value: f.median_us, spread_pct: null })),
      "max",
    ],
  ];
  const out = [
    BEGIN,
    "",
    `${record.host.class}, ${record.date}, commit \`${record.commit.slice(0, 8)}\`; [the full record](results/${page}).`,
    `Its figures are ${standing(record, `../${PROBE}`)}.`,
    "",
    "| Figure | Median over languages | Worst | Median spread over rounds |",
    "|:--|--:|--:|--:|",
    ...rows.map(([figure, entries, worst]) => `| ${figure} | ${overLanguages(entries, worst)}`),
    "",
    END,
  ];
  return out.join("\n");
}

/** The README with its summary block replaced by `summary`. */
function withSummary(readme: string, summary: string): string {
  const start = readme.indexOf(BEGIN);
  const end = readme.indexOf(END);
  if (start < 0 || end < start) throw new Error(`${README} lacks the record markers`);
  return readme.slice(0, start) + summary + readme.slice(end + END.length);
}

const stemOf = (path: string) => basename(path, ".json");
const markdownOf = (path: string) => join(dirname(path), `${stemOf(path)}.md`);

async function main(): Promise<void> {
  const [command, ...args] = process.argv.slice(2);
  switch (command) {
    case "render": {
      const paths = args.length > 0 ? args : records().map((r) => r.path);
      for (const path of paths) {
        await Bun.write(markdownOf(path), renderRecord(readRecord(path), stemOf(path)));
        console.log(`✔ ${markdownOf(path)}`);
      }
      return;
    }
    case "compute": {
      const [path, seconds, unitMinutes, usd] = args;
      const numbers = [seconds, unitMinutes, usd].map(Number);
      if (!path || numbers.some((n) => !Number.isFinite(n) || n < 0)) {
        throw new Error("usage: report.ts compute <record.json> <seconds> <unit-minutes> <usd>");
      }
      const record = readRecord(path);
      record.compute = { seconds: numbers[0], unit_minutes: numbers[1], usd: numbers[2] };
      await Bun.write(path, `${JSON.stringify(record, null, 2)}\n`);
      await Bun.write(markdownOf(path), renderRecord(record, stemOf(path)));
      console.log(`✔ ${path} and ${markdownOf(path)}`);
      return;
    }
    case "summary": {
      const readme = readFileSync(README, "utf8");
      await Bun.write(README, withSummary(readme, renderSummary(records().at(-1))));
      console.log(`✔ ${README}`);
      return;
    }
    case "check": {
      const all = records();
      const problems: string[] = [];
      for (const { path, record } of all) {
        let page = "";
        try {
          page = readFileSync(markdownOf(path), "utf8");
        } catch {
          problems.push(`${markdownOf(path)} is missing`);
          continue;
        }
        if (page !== renderRecord(record, stemOf(path))) problems.push(`${markdownOf(path)} differs from its JSON`);
      }
      const readme = readFileSync(README, "utf8");
      if (readme !== withSummary(readme, renderSummary(all.at(-1)))) {
        problems.push(`${README}'s record summary differs from the newest record`);
      }
      if (problems.length > 0) {
        throw new Error(`${problems.join("\n")}\nRun \`mise run bench:report render\` and \`mise run bench:report summary\`; never edit them by hand.`);
      }
      console.log(`✔ ${all.length} records and the README summary match their JSON`);
      return;
    }
    default:
      throw new Error("usage: report.ts render [record.json...] | compute <record.json> <seconds> <unit-minutes> <usd> | summary | check");
  }
}

if (import.meta.main) {
  try {
    await main();
  } catch (error) {
    console.error(`✘ ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  }
}
