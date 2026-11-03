// Compares a language's hand matchers with Oniguruma, the regex engine Shiki
// runs, one expression at a time.
//
//   RX_DUMP=<file> bun run oracle/<lang>-grammar.ts
//   bun run oracle/onig.ts <lang> <file> <cases per expression> [ids...]
//
// The grammar script writes each expression as a JSON line: its id, its
// source and the capture groups the grammar names; it leaves out an end that
// repeats a group of its begin, whose text a case cannot supply. For each
// expression this script draws texts and positions from the language's oracle
// fixtures, or from the development inputs that `probe.corpus` in
// oracle/fuzz/<lang>.ts names, mutated with the snippets there, until it has
// as many matches as misses, or gives up. A position where the expression
// does not match also yields the match that Oniguruma finds after it. It runs
// the language's ignored test `matchers_answer_the_probe_cases` on the cases,
// which reads them from the file PROBE_IN names and writes its answers to
// PROBE_OUT, and lists every expression whose answer differs from
// Oniguruma's in where the match ends or in a group the grammar names.
// Oniguruma counts UTF-16 code units and the matchers count bytes, so every
// position is converted; a text with a character outside the BMP is skipped,
// since a position could fall inside it. With DEEP set, it adds the
// language's nested texts at depths around Oniguruma's bound on recursion.
import { mkdtempSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createOnigurumaEngine } from "@shikijs/engine-oniguruma@4.6.0";

type Expression = { id: string; regex: string; groups: number[] };
type Case = { rx: string; text: string; p: number; g: boolean; nl: boolean };
type Probe = {
  snippets?: string[];
  anchored?: string[];
  whole?: string[];
  deep?: Record<string, (k: number) => string>;
  /** The fixtures read, by name, when not all of them are development inputs. */
  corpus?: string[];
};

const [lang, file, perArg, ...only] = process.argv.slice(2);
if (!lang || !file || !perArg) {
  console.error("usage: bun run oracle/onig.ts <lang> <expressions file> <cases per expression> [ids...]");
  process.exit(2);
}
const root = new URL("..", import.meta.url).pathname;
const probe: Probe = (await import(`./fuzz/${lang}.ts`).catch(() => ({}))).probe ?? {};
const engine = await createOnigurumaEngine(import("@shikijs/engine-oniguruma@4.6.0/wasm-inlined"));
const expressions: Expression[] = readFileSync(file, "utf8")
  .trim()
  .split("\n")
  .map((line) => JSON.parse(line));

/** The short lines of the fixtures of `lang`, or of those `probe.corpus` names. */
const lines: string[] = [];
const names = probe.corpus?.map((name) => `${name}.txt`) ?? readdirSync(`${root}tests/oracle/${lang}`);
for (const name of names) {
  if (!name.endsWith(".txt")) continue;
  for (const record of readFileSync(`${root}tests/oracle/${lang}/${name}`, "utf8").split("\n")) {
    const line = record.slice(2);
    if (record.startsWith("L ") && line.length && line.length < 200) lines.push(line);
  }
}

let state = 12345;
const random = () => {
  state ^= state << 13;
  state >>>= 0;
  state ^= state >>> 17;
  state ^= state << 5;
  state >>>= 0;
  return state / 4294967296;
};
const pick = <T>(items: T[]): T => items[Math.floor(random() * items.length)] as T;
const snippets = probe.snippets ?? [];
const mutate = (text: string) => {
  let out = text;
  for (let n = snippets.length ? Math.floor(random() * 3) : 0; n > 0; n--) {
    const at = Math.floor(random() * (out.length + 1));
    out = out.slice(0, at) + pick(snippets) + (random() < 0.5 ? " " : "") + out.slice(at);
  }
  return out.length < 220 ? out : text;
};

/** An expression as Oniguruma sees it where `\G` does not hold: with `\G`
 * replaced by a character no text has. */
const scanners = new Map<string, ReturnType<typeof engine.createScanner>>();
const search = (regex: string, text: string, p: number, nl: boolean, g: boolean) => {
  const source = g ? regex : regex.replaceAll("\\G", "￿");
  let scanner = scanners.get(source);
  if (!scanner) {
    scanner = engine.createScanner([source]);
    scanners.set(source, scanner);
  }
  return scanner.findNextMatchSync(engine.createString(nl ? `${text}\n` : text), p);
};
/** The match that starts at `p`, if any. */
const match = (regex: string, text: string, p: number, nl: boolean, g: boolean) => {
  const found = search(regex, text, p, nl, g);
  return found !== null && found.captureIndices[0]?.start === p ? found : null;
};

/** The byte offset of the UTF-16 offset `unit` of `text`; past the text,
 * where the line feed and the position after it stand, a unit is a byte. */
const encoder = new TextEncoder();
const bytes = (text: string, unit: number) =>
  encoder.encode(text.slice(0, unit)).length + Math.max(0, unit - text.length);

const per = Number(perArg);
const cases: Case[] = [];
for (const expression of only.length ? expressions.filter((e) => only.includes(e.id)) : expressions) {
  const hasG = expression.regex.includes("\\G");
  let [hits, misses, tries] = [0, 0, 0];
  while ((hits < per / 2 || misses < per / 2) && tries < 60000) {
    tries++;
    const anchored = expression.regex.includes("\\G(") && probe.anchored !== undefined && random() < 0.7;
    const whole = !anchored && probe.whole !== undefined && random() < 0.25;
    const base = anchored ? pick(probe.anchored ?? []) : whole ? pick(probe.whole ?? []) : pick(lines);
    const text = !anchored && random() < 0.6 ? mutate(base) : base;
    const choice = anchored || whole ? 0 : random();
    let p = anchored || whole ? 0 : Math.floor(random() * (text.length + 2));
    if (choice < 0.15) p = 0;
    else if (choice < 0.3) p = text.length;
    else if (choice < 0.35) p = text.length + 1;
    const nl = random() < 0.85;
    if (!nl && p > text.length) p = text.length;
    if (/[\ud800-\udfff]/.test(text)) continue;
    const g = hasG && random() < 0.5;
    const found = search(expression.regex, text, p, nl, g);
    const at = found?.captureIndices[0]?.start;
    if (at === p && hits < per / 2) {
      hits++;
      cases.push({ rx: expression.id, text, p, g, nl });
    } else if (at !== p && misses < per / 2) {
      misses++;
      cases.push({ rx: expression.id, text, p, g, nl });
    }
    // Where `\G` does not hold, a match found after the position is a
    // position where the expression matches.
    if (at !== undefined && at !== p && !g && hits < per / 2) {
      hits++;
      cases.push({ rx: expression.id, text, p: at, g, nl });
    }
  }
  if (hits < 5) console.log(`note: ${expression.id} found only ${hits} matching cases`);
}
if (process.env.DEEP) {
  for (const [id, nested] of Object.entries(probe.deep ?? {})) {
    if (only.length && !only.includes(id)) continue;
    for (const k of [16, 17, 18, 19, 20, 21, 22]) {
      for (const nl of [true, false]) cases.push({ rx: id, text: nested(k), p: 0, g: false, nl });
    }
  }
}

const work = mkdtempSync(join(tmpdir(), "nishiki-onig-"));
const input = join(work, "cases.jsonl");
const output = join(work, "answers.jsonl");
writeFileSync(input, `${cases.map((c) => JSON.stringify({ ...c, p: bytes(c.text, c.p) })).join("\n")}\n`);
const run = spawnSync(
  "cargo",
  ["test", "--locked", "--all-features", "--lib", `${lang}::tests::probe::matchers_answer_the_probe_cases`, "--", "--ignored"],
  { cwd: root, env: { ...process.env, PROBE_IN: input, PROBE_OUT: output }, encoding: "utf8" },
);
if (run.status !== 0) {
  console.log(run.stdout.slice(-3000), run.stderr.slice(-3000));
  process.exit(1);
}
const answers: ({ end: number; caps: ([number, number] | null)[] } | null)[] = readFileSync(output, "utf8")
  .trim()
  .split("\n")
  .map((line) => JSON.parse(line));

const byId = new Map(expressions.map((e) => [e.id, e]));
const stats = new Map<string, { n: number; bad: number; first?: string }>();
cases.forEach((c, i) => {
  const expression = byId.get(c.rx) as Expression;
  const found = match(expression.regex, c.text, c.p, c.nl, c.g);
  // Oniguruma reports a group that did not take part as starting at 2^32 - 1,
  // and counts UTF-16 code units where the matchers count bytes.
  const b = (unit: number) => bytes(c.text, unit);
  const want = found && {
    end: b(found.captureIndices[0]?.end ?? 0),
    caps: found.captureIndices.map((x) => (x.start === 4294967295 ? null : ([b(x.start), b(x.end)] as [number, number]))),
  };
  const got = answers[i] ?? null;
  let ok = want === null ? got === null : got !== null && got.end === want.end;
  if (ok && want && got) {
    for (const k of expression.groups) {
      if (k === 0) continue;
      const w = want.caps[k];
      const g = got.caps[k];
      if (!w || w[0] >= w[1]) {
        if (g && g[0] < g[1]) ok = false;
      } else if (!g || g[0] !== w[0] || g[1] !== w[1]) ok = false;
    }
  }
  const stat = stats.get(c.rx) ?? { n: 0, bad: 0 };
  stat.n++;
  if (!ok) {
    stat.bad++;
    stat.first ??= JSON.stringify({ text: c.text, p: b(c.p), g: c.g, nl: c.nl, want, got });
  }
  stats.set(c.rx, stat);
});
let bad = 0;
for (const [id, stat] of stats) {
  if (stat.bad) {
    bad++;
    console.log(`${id}: ${stat.bad}/${stat.n} differ; e.g. ${stat.first}`);
  }
}
console.log(`${stats.size} expressions, ${bad} differ`);
process.exit(bad ? 1 : 0);
