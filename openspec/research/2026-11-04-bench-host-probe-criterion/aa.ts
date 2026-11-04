// aa.ts: the analysis of bench-methodology task 3.2.
//
//   bun aa.ts extract <criterion-dir>   one TSV row per Criterion run of
//                                       lex/<language>/medium: language,
//                                       mode, run, median ns an iteration
//   bun aa.ts report <host.tsv>         per mode, a line per language, then
//                                       the summary the research note quotes
//   bun aa.ts check                     tests the A/A magnitude on known pairs
//
// The A/A gap is the median of the even runs against the median of the odd
// runs. The spread is the largest run minus the smallest, over the median of
// all runs. The drift is |last run / first run - 1|: the probe's analogue of
// the anchor measured at a session's start and end.
import { Glob } from "bun";

type Row = { language: string; mode: string; run: number; ns: number };

const median = (values: number[]): number => {
  const sorted = [...values].sort((a, b) => a - b);
  const mid = sorted.length >> 1;
  return sorted.length % 2 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
};

const quantile = (values: number[], q: number): number => {
  const sorted = [...values].sort((a, b) => a - b);
  const position = (sorted.length - 1) * q;
  const low = Math.floor(position);
  const high = Math.ceil(position);
  return sorted[low] + (sorted[high] - sorted[low]) * (position - low);
};

/** The A/A gap as a magnitude: the one place a sign is dropped, so no summary can rank a negative gap below a positive one. */
const aaGap = (odd: number[], even: number[]): number => Math.abs(median(even) / median(odd) - 1);

/** The geometric mean: a run's speed over every language, each weighted alike. */
const geomean = (values: number[]): number => Math.exp(values.reduce((sum, value) => sum + Math.log(value), 0) / values.length);

const pct = (value: number): string => `${(value * 100).toFixed(2)}%`;

async function extract(dir: string): Promise<void> {
  const rows: Row[] = [];
  for await (const path of new Glob("**/lex_*/medium/*-*/estimates.json").scan(dir)) {
    const match = /lex_([^/]+)\/medium\/(free|pin)-(\d+)\/estimates\.json$/.exec(path);
    if (!match) continue;
    const estimates = await Bun.file(`${dir}/${path}`).json();
    rows.push({ language: match[1], mode: match[2], run: Number(match[3]), ns: estimates.median.point_estimate });
  }
  rows.sort((a, b) => a.language.localeCompare(b.language) || a.mode.localeCompare(b.mode) || a.run - b.run);
  console.log("language\tmode\trun\tmedian_ns");
  for (const row of rows) console.log(`${row.language}\t${row.mode}\t${row.run}\t${row.ns.toFixed(1)}`);
}

async function report(tsv: string): Promise<void> {
  const rows: Row[] = (await Bun.file(tsv).text())
    .trim()
    .split("\n")
    .slice(1)
    .map((line) => {
      const [language, mode, run, ns] = line.split("\t");
      return { language, mode, run: Number(run), ns: Number(ns) };
    });
  for (const mode of ["free", "pin"]) {
    const languages = [...new Set(rows.filter((row) => row.mode === mode).map((row) => row.language))];
    const lines = languages.map((language) => {
      const runs = rows.filter((row) => row.mode === mode && row.language === language).sort((a, b) => a.run - b.run);
      const rate = runs.map((row) => 1e9 / row.ns);
      const odd = rate.filter((_, i) => runs[i].run % 2 === 1);
      const even = rate.filter((_, i) => runs[i].run % 2 === 0);
      const gap = aaGap(odd, even);
      const spread = (Math.max(...rate) - Math.min(...rate)) / median(rate);
      const drift = Math.abs(rate[rate.length - 1] / rate[0] - 1);
      return { language, runs: runs.length, gap, spread, drift };
    });
    console.log(`## ${mode}`);
    for (const l of lines) {
      console.log(`${l.language.padEnd(16)} runs ${l.runs}  aa ${pct(l.gap).padStart(7)}  spread ${pct(l.spread).padStart(7)}  drift ${pct(l.drift).padStart(7)}`);
    }
    const runNumbers = [...new Set(rows.filter((row) => row.mode === mode).map((row) => row.run))].sort((a, b) => a - b);
    const hostSpeed = runNumbers.map((run) => geomean(rows.filter((row) => row.mode === mode && row.run === run).map((row) => 1e9 / row.ns)));
    const residual = languages.map((language) => {
      const rate = runNumbers.map((run, i) => 1e9 / (rows.find((row) => row.mode === mode && row.run === run && row.language === language)?.ns ?? Number.NaN) / hostSpeed[i]);
      return (Math.max(...rate) - Math.min(...rate)) / median(rate);
    });
    console.log(`host speed by run, against run ${runNumbers[0]}: ${hostSpeed.map((speed) => pct(speed / hostSpeed[0] - 1)).join(" ")}`);
    const worst = lines.reduce((a, b) => (b.gap > a.gap ? b : a));
    const widest = lines.reduce((a, b) => (b.spread > a.spread ? b : a));
    const spreads = lines.map((l) => l.spread);
    const drifts = lines.map((l) => l.drift);
    console.log(
      [
        `summary ${mode}:`,
        `worst aa ${pct(worst.gap)} (${worst.language});`,
        `over 2%: ${lines.filter((l) => l.gap > 0.02).length} of ${lines.length};`,
        `spread median ${pct(median(spreads))}, worst ${pct(widest.spread)} (${widest.language});`,
        `drift median ${pct(median(drifts))}, p90 ${pct(quantile(drifts, 0.9))}, max ${pct(Math.max(...drifts))};`,
        `spread without the host's speed: median ${pct(median(residual))};`,
        `python drift ${pct(lines.find((l) => l.language === "python")?.drift ?? Number.NaN)}`,
      ].join(" "),
    );
  }
}

/** Known pairs: a gap of -10% and one of +5% must rank the -10% first and both count as over 2%. */
function check(): void {
  const down = aaGap([100, 100], [90, 90]);
  const up = aaGap([100, 100], [105, 105]);
  const ok = Math.abs(down - 0.1) < 1e-9 && Math.abs(up - 0.05) < 1e-9 && down > up && down > 0.02 && up > 0.02;
  console.log(ok ? "✔ aaGap ranks and counts by magnitude" : `✘ aaGap: down ${down}, up ${up}`);
  if (!ok) process.exit(1);
}

const [command, path] = Bun.argv.slice(2);
if (command === "check") check();
else if (command === "extract" && path) await extract(path);
else if (command === "report" && path) await report(path);
else {
  console.error("usage: bun aa.ts check | extract <criterion-dir> | report <host.tsv>");
  process.exit(2);
}
