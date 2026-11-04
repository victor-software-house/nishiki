// Measures how much of each grammar that a hand-written lexer follows the
// oracle fixtures exercise, as openspec/changes/oracle-coverage/design.md
// defines it (sections 2 and 3).
//
//   bun run oracle/coverage.ts
//
// A unit is an expression (`match`, `begin`, `end` or `while`), or a capture
// group of one that the grammar names or gives patterns. Every expression of a
// hand-lexed grammar is tagged with an Oniguruma comment, `(?#u<n>)`, which
// matches nothing, and Shiki runs on an engine that wraps its own Oniguruma
// engine and reads the tag of each match the scanner returns. The script
// tokenizes the text of every fixture, and fails when a scope stack differs
// from the fixture's, so the tags provably change nothing. Where a grammar has
// injections, it counts only the match vscode-textmate keeps. It measures a
// fixed fuzz corpus from oracle/fuzz.ts the same way, and writes, per
// hand-lexed grammar, the reachable expressions and captures the fixtures
// exercise, those only the fuzz corpus exercises, and the units no root
// reaches, to tests/oracle/COVERAGE.md. Every loaded grammar must
// be listed as hand-lexed or have engine tables under src/, so a new hand
// lexer cannot drop out of the count.
import { mkdtempSync, readdirSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createHighlighter, type RegexEngine } from "shiki@4.6.0";
import { createOnigurumaEngine } from "shiki@4.6.0/engine/oniguruma";
import { fixture } from "./fixture.ts";
import { grammars } from "./grammars.ts";
import { inputs } from "./inputs.ts";

/** The grammars a hand-written lexer follows, by Shiki's name, and the
 * lexer that follows each. */
const handLexed = new Map([
  ["c", "C"],
  ["cpp", "C++"],
  ["cpp-macro", "C++"],
  ["csharp", "C#"],
  ["css", "CSS"],
  ["diff", "diff"],
  ["docker", "Dockerfile"],
  ["erb", "ERB"],
  ["go", "Go"],
  ["graphql", "GraphQL"],
  ["html", "HTML"],
  ["html-derivative", "HTML"],
  ["ini", "INI"],
  ["java", "Java"],
  ["javascript", "TypeScript"],
  ["json", "JSON"],
  ["jsonc", "JSON"],
  ["jsonl", "JSON"],
  ["jsx", "TypeScript"],
  ["kotlin", "Kotlin"],
  ["less", "Less"],
  ["lua", "Lua"],
  ["make", "Make"],
  ["markdown", "Markdown"],
  ["python", "Python"],
  ["ruby", "Ruby"],
  ["rust", "Rust"],
  ["scss", "SCSS"],
  ["shellscript", "shell"],
  ["sql", "SQL"],
  ["swift", "Swift"],
  ["toml", "TOML"],
  ["tsx", "TypeScript"],
  ["typescript", "TypeScript"],
  ["xml", "XML"],
  ["yaml", "YAML"],
]);

/** A rule of a TextMate grammar, as far as coverage reads it. */
type Rule = {
  include?: string;
  match?: string;
  begin?: string;
  end?: string;
  while?: string;
  name?: string;
  captures?: Captures;
  beginCaptures?: Captures;
  endCaptures?: Captures;
  whileCaptures?: Captures;
  patterns?: Rule[];
  repository?: Record<string, Rule>;
  injections?: Record<string, Rule>;
};
type Captures = Record<string, Rule>;
type Grammar = Rule & { name: string; scopeName: string };

/** An expression of a hand-lexed grammar, and the groups of it that count. */
type Expression = { grammar: string; path: string; groups: number[] };
/** Where a rule stands: its grammar, and the repositories around it, the
 * innermost last, against which its `#name` includes resolve. */
type Place = { grammar: Grammar; repositories: Record<string, Rule>[] };

const kinds = ["match", "begin", "end", "while"] as const;
const captureKeys = ["captures", "beginCaptures", "endCaptures", "whileCaptures"] as const;

/** The groups of `captures` with their rules. vscode-textmate reads only
 * numbered keys; XML's grammar holds a `captures` with `begin`, `end` and
 * `name` keys, which it ignores. */
const groupsIn = (captures: Captures | undefined): [number, Rule][] =>
  Object.entries(captures ?? {}).flatMap(([key, rule]) =>
    /^\d+$/.test(key) && typeof rule === "object" ? [[Number(key), rule]] : [],
  );
/** The captures that apply to each kind of expression: its own, or the
 * rule's `captures`, as vscode-textmate reads them. */
const capturesOf = (rule: Rule, kind: (typeof kinds)[number]): Captures | undefined =>
  ({
    match: rule.captures,
    begin: rule.beginCaptures ?? rule.captures,
    end: rule.endCaptures ?? rule.captures,
    while: rule.whileCaptures ?? rule.captures,
  })[kind];

const expressions: Expression[] = [];
/** The expressions each rule holds, by unit number. */
const unitsOf = new Map<Rule, number[]>();
const places = new Map<Rule, Place>();

/** `part` under `path`, the root's path being empty. */
const under = (path: string, part: string) => (path === "" ? part : `${path}.${part}`);

/** Records where every rule of `grammar` stands and, in a hand-lexed grammar,
 * numbers and tags its expressions. */
function walk(rule: Rule, path: string, place: Place, tag: boolean): void {
  places.set(rule, place);
  const units: number[] = [];
  for (const kind of kinds) {
    const source = rule[kind];
    if (source === undefined || !tag) continue;
    const groups = groupsIn(capturesOf(rule, kind))
      .filter(([, capture]) => capture.name !== undefined || capture.patterns !== undefined)
      .map(([group]) => group);
    units.push(expressions.length);
    rule[kind] = `(?#u${expressions.length})${source}`;
    expressions.push({ grammar: place.grammar.name, path: under(path, kind), groups });
  }
  unitsOf.set(rule, units);
  const inner: Place = rule.repository
    ? { grammar: place.grammar, repositories: [...place.repositories, rule.repository] }
    : place;
  rule.patterns?.forEach((child, index) => walk(child, under(path, `patterns[${index}]`), inner, tag));
  for (const key of captureKeys) {
    for (const [group, capture] of groupsIn(rule[key])) {
      walk(capture, under(path, `${key}[${group}]`), inner, tag);
    }
  }
  for (const [name, child] of Object.entries(rule.repository ?? {})) {
    walk(child, path === "" ? `#${name}` : under(path, `repository.#${name}`), inner, tag);
  }
  for (const [selector, child] of Object.entries(rule.injections ?? {})) {
    walk(child, `injections[${selector}]`, inner, tag);
  }
}

const root = new URL("..", import.meta.url).pathname;

/** The languages on nishiki's TextMate engine, by Shiki's name: each has a
 * `src/<id>/grammar.rs` that `oracle/engine-tables.ts` wrote, whose header
 * names the spec it was made from, such as `julia~cpp~r`. */
const header = /^\/\/! The tables of Shiki \S+'s `([^`]+)` grammar, made by\n\/\/! `oracle\/engine-tables\.ts`/;
const engineLanguages = new Set<string>();
for (const path of new Bun.Glob("src/*/grammar.rs").scanSync(root)) {
  const spec = header.exec(readFileSync(`${root}${path}`, "utf8"))?.[1];
  if (spec !== undefined) engineLanguages.add(spec.split(/[+~]/)[0] ?? spec);
}

const loaded = structuredClone(grammars) as unknown as Grammar[];
for (const name of handLexed.keys()) {
  if (!loaded.some((grammar) => grammar.name === name)) {
    throw new Error(`${name}: listed as hand-lexed, but the oracle does not load it`);
  }
  if (engineLanguages.has(name)) {
    throw new Error(`${name}: listed as hand-lexed, but it runs on the TextMate engine`);
  }
}
const unlisted = loaded.filter(
  (grammar) => !handLexed.has(grammar.name) && !engineLanguages.has(grammar.name),
);
if (unlisted.length > 0) {
  const names = unlisted.map((grammar) => grammar.name).join(", ");
  throw new Error(`${names}: loaded, but neither listed as hand-lexed nor on the TextMate engine`);
}
for (const grammar of loaded) {
  walk(grammar, "", { grammar, repositories: [] }, handLexed.has(grammar.name));
}
const byScope = new Map(loaded.map((grammar) => [grammar.scopeName, grammar]));

/** The rule an include names, seen from where `from` stands, if loaded. */
function resolve(include: string, from: Place): Rule | undefined {
  if (include === "$self" || include === "$base") return from.grammar;
  if (include.startsWith("#")) {
    const name = include.slice(1);
    return from.repositories.findLast((repository) => name in repository)?.[name];
  }
  const [scope, name] = include.split("#");
  const grammar = byScope.get(scope ?? "");
  return name === undefined ? grammar : grammar?.repository?.[name];
}

/** The units a root reaches: each input language's grammar, and every loaded
 * grammar's injections, through patterns, captures and includes. */
const reachable = new Set<number>();
const seen = new Set<Rule>();
const todo: Rule[] = loaded.filter((grammar) =>
  inputs.some((input) => input.lang === grammar.name),
);
for (const grammar of loaded) todo.push(...Object.values(grammar.injections ?? {}));
while (todo.length > 0) {
  const rule = todo.pop() as Rule;
  if (seen.has(rule)) continue;
  seen.add(rule);
  for (const unit of unitsOf.get(rule) ?? []) reachable.add(unit);
  const place = places.get(rule) as Place;
  if (rule.include !== undefined) {
    const target = resolve(rule.include, place);
    if (target !== undefined) todo.push(target);
  }
  todo.push(...(rule.patterns ?? []));
  for (const key of captureKeys) {
    todo.push(...groupsIn(rule[key]).map(([, capture]) => capture));
  }
}

/** Each injection's priority, by its number: -1 for an `L:` selector, which
 * wins a tie with the rule list, 1 for `R:` and 0 otherwise, as
 * vscode-textmate reads them. Every injection gets a first pattern that never
 * matches, `(?#i<n>)(?!)`, so its scanner can be told from a rule list's. */
const priorities: number[] = [];
for (const grammar of loaded) {
  for (const [selector, injection] of Object.entries(grammar.injections ?? {})) {
    const sentinel: Rule = { match: `(?#i${priorities.length})(?!)` };
    injection.patterns = [sentinel, ...(injection.patterns ?? [])];
    priorities.push(selector.startsWith("L:") ? -1 : selector.startsWith("R:") ? 1 : 0);
  }
}

/** What the run counts: on the fixtures, then on the fuzz corpus. */
type Tally = { expressions: Set<number>; captures: Set<string> };
const tally = (): Tally => ({ expressions: new Set(), captures: new Set() });
const fixtures = tally();
const fuzzed = tally();
let counting = fixtures;

type IOnigMatch = NonNullable<
  ReturnType<ReturnType<RegexEngine["createScanner"]>["findNextMatchSync"]>
>;
/** A match a scanner returned, and the text it matched in. */
type Hit = { unit: number | undefined; found: IOnigMatch; content: string };
/** One scan of the rule list, and the scans of the injections that
 * vscode-textmate makes after it at the same position. It keeps one of them:
 * the injection with the earliest match, against the rule list's match, where
 * the earlier start wins, and on a tie the rule list, unless the injection's
 * priority is -1. */
type Scan = {
  text: unknown;
  start: number;
  rule: Hit | null;
  injection: (Hit & { priority: number }) | null;
};
let scan: Scan | undefined;

const startOf = (hit: Hit) => hit.found.captureIndices[0]?.start ?? 0;
/** Counts the match vscode-textmate keeps from the open scan. */
function settle(): void {
  if (scan === undefined) return;
  const { rule, injection } = scan;
  scan = undefined;
  const kept =
    injection === null || rule === null
      ? (injection ?? rule)
      : startOf(injection) < startOf(rule) ||
          (injection.priority === -1 && startOf(injection) === startOf(rule))
        ? injection
        : rule;
  if (kept === null || kept.unit === undefined) return;
  counting.expressions.add(kept.unit);
  for (const group of expressions[kept.unit]?.groups ?? []) {
    const span = kept.found.captureIndices[group];
    if (span !== undefined && /\S/.test(kept.content.slice(span.start, span.end))) {
      counting.captures.add(`${kept.unit}:${group}`);
    }
  }
}

const tagged = (source: string | RegExp, kind: "u" | "i") =>
  typeof source === "string" ? new RegExp(`^\\(\\?#${kind}(\\d+)\\)`).exec(source)?.[1] : undefined;
const base = await createOnigurumaEngine(import("shiki@4.6.0/wasm"));
const engine: RegexEngine = {
  createString: (text) => base.createString(text),
  createScanner(sources) {
    const scanner = base.createScanner(sources);
    const units = sources.map((source) => {
      const unit = tagged(source, "u");
      return unit === undefined ? undefined : Number(unit);
    });
    const injection = sources.map((source) => tagged(source, "i")).find((n) => n !== undefined);
    return {
      findNextMatchSync(text, start, options) {
        const found = scanner.findNextMatchSync(text, start, options);
        const content = typeof text === "string" ? text : text.content;
        const hit = found === null ? null : { unit: units[found.index], found, content };
        if (injection === undefined) {
          settle();
          scan = { text, start, rule: hit, injection: null };
        } else {
          if (scan === undefined || scan.text !== text || scan.start !== start) {
            throw new Error("an injection was scanned without the rule list before it");
          }
          if (hit !== null && (scan.injection === null || startOf(hit) < startOf(scan.injection))) {
            scan.injection = { ...hit, priority: priorities[Number(injection)] ?? 0 };
          }
        }
        return found;
      },
      dispose: () => scanner.dispose?.(),
    };
  },
};
const highlighter = await createHighlighter({
  langs: loaded as unknown as typeof grammars,
  themes: ["github-dark"],
  engine,
});

/** The text of a file in the fixture format, from its `L` records. */
const textOf = (records: string) =>
  records
    .split("\n")
    .filter((record) => record.startsWith("L "))
    .map((record) => record.slice(2))
    .join("\n");
/** Tokenizes `text` as `lang` on the tagged grammars, counting what it
 * exercises, and tells whether the scopes equal `committed`. */
function agrees(text: string, lang: string, committed: string): boolean {
  const scopes = fixture(highlighter, text, lang);
  settle();
  return scopes === committed;
}

const differing: string[] = [];
const inputLangs = [...new Set(inputs.map((input) => input.lang))].sort();
for (const lang of inputLangs) {
  const dir = `${root}tests/oracle/${lang}`;
  for (const name of readdirSync(dir).filter((file) => file.endsWith(".txt")).sort()) {
    const committed = readFileSync(`${dir}/${name}`, "utf8");
    if (!agrees(textOf(committed), lang, committed)) differing.push(`${lang}/${name}`);
  }
}

/** The fuzz corpus: per hand-lexed input language, `fuzzCount` mutated
 * windows of its fixtures, and as many generated documents where
 * oracle/fuzz/<lang>.ts has pieces, at seed 1. oracle/fuzz.ts writes the
 * same documents for the same arguments. Only their text is read: fuzz.ts
 * loads one grammar, so its scopes leave other languages' fences plain. */
const fuzzCount = 100;
const corpus = mkdtempSync(join(tmpdir(), "nishiki-coverage-"));
counting = fuzzed;
for (const lang of inputLangs.filter((lang) => handLexed.has(lang))) {
  const pieces = `${root}oracle/fuzz/${lang}.ts`;
  const extras = (await Bun.file(pieces).exists()) ? await import(pieces) : {};
  for (const mode of extras.pieces === undefined ? ["win"] : ["win", "gen"]) {
    const dir = join(corpus, `${lang}-${mode}`);
    const run = Bun.spawnSync(
      [process.execPath, "run", `${root}oracle/fuzz.ts`, lang, mode, "1", String(fuzzCount), dir],
      { stdout: "ignore", stderr: "pipe" },
    );
    if (run.exitCode !== 0) throw new Error(`fuzz.ts ${lang} ${mode}: ${run.stderr.toString()}`);
    for (const name of readdirSync(dir).sort()) {
      fixture(highlighter, textOf(readFileSync(join(dir, name), "utf8")), lang);
      settle();
    }
  }
}
rmSync(corpus, { recursive: true });

if (differing.length > 0) {
  console.error(`✘ the tagged grammars changed Shiki's scopes in ${differing.join(", ")}`);
  process.exit(1);
}
const strays = [...fixtures.expressions, ...fuzzed.expressions].filter(
  (unit) => !reachable.has(unit),
);
if (strays.length > 0) {
  const paths = strays.map((unit) => `${expressions[unit]?.grammar} ${expressions[unit]?.path}`);
  console.error(`✘ exercised but counted unreachable: ${paths.join(", ")}`);
  process.exit(1);
}

/** One counted unit: an expression, or a group of one. */
type Unit = { unit: number; group?: number };
const keyOf = ({ unit, group }: Unit) => (group === undefined ? String(unit) : `${unit}:${group}`);
const hitIn = (counted: Tally, u: Unit) =>
  u.group === undefined ? counted.expressions.has(u.unit) : counted.captures.has(keyOf(u));
const percent = (part: number, whole: number) =>
  whole === 0 ? "-" : `${((100 * part) / whole).toFixed(1)}%`;
/** A regex as a cell in a fenced block: one line, at most 60 characters. */
const excerpt = (source: string) => {
  const line = source.replace(/^\(\?#u\d+\)/, "").replaceAll("\n", "\\n");
  return line.length > 60 ? `${line.slice(0, 59)}…` : line;
};
const sourceOf = new Map<number, string>();
for (const rule of places.keys()) {
  for (const kind of kinds) {
    const unit = /^\(\?#u(\d+)\)/.exec(rule[kind] ?? "")?.[1];
    if (unit !== undefined) sourceOf.set(Number(unit), rule[kind] ?? "");
  }
}

const rows: string[] = [];
const sections: string[] = [];
const unreachableSections: string[] = [];
for (const name of [...handLexed.keys()].sort()) {
  const units = expressions.flatMap((expression, unit) =>
    expression.grammar === name ? [unit] : [],
  );
  const live = units.filter((unit) => reachable.has(unit));
  const groups: Unit[] = live.flatMap((unit) =>
    (expressions[unit]?.groups ?? []).map((group) => ({ unit, group })),
  );
  const all: Unit[] = [...live.map((unit) => ({ unit })), ...groups];
  const hit = live.filter((unit) => fixtures.expressions.has(unit)).length;
  const groupsHit = groups.filter((u) => hitIn(fixtures, u)).length;
  const missing = all.filter((u) => !hitIn(fixtures, u));
  const fuzzOnly = missing.filter((u) => hitIn(fuzzed, u)).length;
  rows.push(
    `| ${handLexed.get(name)} | \`${name}\` | ${hit} of ${live.length} | ${percent(hit, live.length)} | ${groupsHit} of ${groups.length} | ${percent(groupsHit, groups.length)} | ${fuzzOnly} |`,
  );
  const lines = live.flatMap((unit) => {
    const path = expressions[unit]?.path ?? "";
    if (!fixtures.expressions.has(unit)) {
      const fuzz = fuzzed.expressions.has(unit) ? "  fuzz" : "";
      return [`${path}  ${excerpt(sourceOf.get(unit) ?? "")}${fuzz}`];
    }
    const open = groups.filter((u) => u.unit === unit && !hitIn(fixtures, u));
    if (open.length === 0) return [];
    const list = open.map((u) => `${u.group}${hitIn(fuzzed, u) ? " (fuzz)" : ""}`);
    return [`${path}  groups ${list.join(", ")}`];
  });
  if (lines.length > 0) {
    sections.push(`### \`${name}\`\n\n\`\`\`text\n${lines.join("\n")}\n\`\`\``);
  }
  const dead = units.filter((unit) => !reachable.has(unit));
  if (dead.length > 0) {
    const lines = dead.map((unit) => expressions[unit]?.path ?? "");
    unreachableSections.push(`### \`${name}\`\n\n\`\`\`text\n${lines.join("\n")}\n\`\`\``);
  }
}

const report = `# Oracle coverage

How much of each grammar that a hand-written lexer follows the oracle fixtures exercise.
\`mise run oracle:coverage\` writes this file from [\`oracle/coverage.ts\`](../../oracle/coverage.ts); do not edit it.
The [oracle-coverage design](../../openspec/changes/oracle-coverage/design.md) defines the units.

- An expression is exercised when Shiki takes its match on a fixture.
- A capture is exercised when its group takes part in such a match and holds a non-whitespace byte.
- Fuzz only counts the units that only the fixed fuzz corpus exercises: ${fuzzCount} mutated windows per language, and ${fuzzCount} generated documents where \`oracle/fuzz/<lang>.ts\` has pieces, at seed 1.
  No CI job compares that corpus with nishiki.

| Lexer | Grammar | Expressions | Share | Captures | Share | Fuzz only |
|:--|:--|--:|--:|--:|--:|--:|
${rows.join("\n")}

## Units no fixture exercises

Each line is an expression's path in its grammar, then the start of its regex and \`fuzz\` when the fuzz corpus exercises it.
An expression the fixtures do exercise is listed with the groups they do not, each marked \`(fuzz)\` when the fuzz corpus exercises it.
The groups of an unexercised expression count as unexercised and are not listed.

${sections.join("\n\n")}

## Units no root reaches

Neither an input language's grammar nor an injection reaches these, so they are left out of the shares.

${unreachableSections.join("\n\n")}
`;
await Bun.write(`${root}tests/oracle/COVERAGE.md`, report);
console.log(`✔ tests/oracle/COVERAGE.md: ${handLexed.size} grammars`);
