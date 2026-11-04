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
// from the fixture's, so the tags provably change nothing. It prints, per
// hand-lexed grammar, the reachable expressions and captures and how many the
// fixtures exercise, and the units no root reaches. Every loaded grammar must
// be listed as hand-lexed or have engine tables under src/, so a new hand
// lexer cannot drop out of the count.
import { readdirSync, readFileSync } from "node:fs";
import { createHighlighter, type RegexEngine } from "shiki@4.6.0";
import { createOnigurumaEngine } from "shiki@4.6.0/engine/oniguruma";
import { fixture } from "./fixture.ts";
import { grammars } from "./grammars.ts";
import { inputs } from "./inputs.ts";

/** The grammars a hand-written lexer follows, by Shiki's name. */
const handLexed = new Set([
  "c",
  "cpp",
  "cpp-macro",
  "csharp",
  "css",
  "diff",
  "docker",
  "erb",
  "go",
  "graphql",
  "html",
  "html-derivative",
  "ini",
  "java",
  "javascript",
  "json",
  "jsonc",
  "jsonl",
  "jsx",
  "kotlin",
  "less",
  "lua",
  "make",
  "markdown",
  "python",
  "ruby",
  "rust",
  "scss",
  "shellscript",
  "sql",
  "swift",
  "toml",
  "tsx",
  "typescript",
  "xml",
  "yaml",
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
    expressions.push({ grammar: place.grammar.name, path: `${path}.${kind}`, groups });
  }
  unitsOf.set(rule, units);
  const inner: Place = rule.repository
    ? { grammar: place.grammar, repositories: [...place.repositories, rule.repository] }
    : place;
  rule.patterns?.forEach((child, index) => walk(child, `${path}.patterns[${index}]`, inner, tag));
  for (const key of captureKeys) {
    for (const [group, capture] of groupsIn(rule[key])) {
      walk(capture, `${path}.${key}[${group}]`, inner, tag);
    }
  }
  for (const [name, child] of Object.entries(rule.repository ?? {})) {
    walk(child, `${path === "" ? "" : `${path}.repository.`}#${name}`, inner, tag);
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
for (const name of handLexed) {
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

const exercised = new Set<number>();
/** Captures exercised, as `<unit>:<group>`. */
const capturesExercised = new Set<string>();
const base = await createOnigurumaEngine(import("shiki@4.6.0/wasm"));
const engine: RegexEngine = {
  createString: (text) => base.createString(text),
  createScanner(sources) {
    const scanner = base.createScanner(sources);
    const tags = sources.map((source) =>
      typeof source === "string" ? /^\(\?#u(\d+)\)/.exec(source)?.[1] : undefined,
    );
    return {
      findNextMatchSync(text, start, options) {
        const found = scanner.findNextMatchSync(text, start, options);
        const tag = found === null ? undefined : tags[found.index];
        if (found !== null && tag !== undefined) {
          const unit = Number(tag);
          exercised.add(unit);
          const content = typeof text === "string" ? text : text.content;
          for (const group of expressions[unit]?.groups ?? []) {
            const span = found.captureIndices[group];
            if (span !== undefined && /\S/.test(content.slice(span.start, span.end))) {
              capturesExercised.add(`${unit}:${group}`);
            }
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

const differing: string[] = [];
for (const lang of new Set(inputs.map((input) => input.lang))) {
  const dir = `${root}tests/oracle/${lang}`;
  for (const name of readdirSync(dir).filter((file) => file.endsWith(".txt")).sort()) {
    const committed = readFileSync(`${dir}/${name}`, "utf8");
    const text = committed
      .split("\n")
      .filter((record) => record.startsWith("L "))
      .map((record) => record.slice(2))
      .join("\n");
    if (fixture(highlighter, text, lang) !== committed) differing.push(`${lang}/${name}`);
  }
}
if (differing.length > 0) {
  console.error(`✘ the tagged grammars changed Shiki's scopes in ${differing.join(", ")}`);
  process.exit(1);
}
const strays = [...exercised].filter((unit) => !reachable.has(unit));
if (strays.length > 0) {
  const paths = strays.map((unit) => `${expressions[unit]?.grammar} ${expressions[unit]?.path}`);
  console.error(`✘ exercised but counted unreachable: ${paths.join(", ")}`);
  process.exit(1);
}

const percent = (part: number, whole: number) =>
  whole === 0 ? "-" : `${((100 * part) / whole).toFixed(1)}%`;
console.log("grammar          expressions          captures             unreachable");
for (const name of [...handLexed].sort()) {
  const units = expressions.flatMap((expression, unit) =>
    expression.grammar === name ? [unit] : [],
  );
  const live = units.filter((unit) => reachable.has(unit));
  const hit = live.filter((unit) => exercised.has(unit)).length;
  const groups = live.flatMap((unit) =>
    (expressions[unit]?.groups ?? []).map((group) => `${unit}:${group}`),
  );
  const groupsHit = groups.filter((key) => capturesExercised.has(key)).length;
  console.log(
    [
      name.padEnd(16),
      `${hit}/${live.length}`.padEnd(10),
      percent(hit, live.length).padEnd(10),
      `${groupsHit}/${groups.length}`.padEnd(10),
      percent(groupsHit, groups.length).padEnd(10),
      String(units.length - live.length),
    ].join(" "),
  );
}
