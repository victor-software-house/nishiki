// Measures how much of each grammar that a hand-written lexer follows the
// oracle fixtures exercise, as openspec/changes/oracle-coverage/design.md
// defines it (sections 2 and 3).
//
//   bun run oracle/coverage.ts [report]
//   bun run oracle/coverage.ts --pick <grammar> <lang> <file or directory>…
//
// The report goes to tests/oracle/COVERAGE.md unless another path is given,
// as the coverage check gives one to compare with the committed file.
//
// `--pick` ranks candidate inputs for closing a grammar's gaps, and writes no
// report. It reads each candidate as `<lang>`, every file of a directory
// included, and repeatedly takes the one that exercises the most units of
// `<grammar>` that neither the fixtures nor an earlier pick exercise. It
// prints each pick with the units it adds, then the units still left.
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
// reaches, to tests/oracle/COVERAGE.md. Every loaded grammar must be listed
// as hand-lexed, so a new hand lexer cannot drop out of the count. Each row of
// `languages!` in src/language.rs must name the root scope of Shiki's grammar
// for its id, since the linker hands an include of that scope to the row's
// lexer. The run fails after writing the report when a waiver in
// oracle/coverage-waivers.ts is wrong, or when a grammar listed complete there
// has a unit that neither a fixture nor a waiver covers.
import { mkdtempSync, readdirSync, readFileSync, rmSync, statSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createHighlighter, type RegexEngine } from "shiki@4.6.0";
import { createOnigurumaEngine } from "shiki@4.6.0/engine/oniguruma";
import { handLexed as handOffs } from "./hand-lexed.ts";
import { fixtureParts, type Highlighter } from "./fixture.ts";
import { grammars } from "./grammars.ts";
import { complete, waivers } from "./coverage-waivers.ts";
import { inputs } from "./inputs.ts";
import { dedent } from "./dedent.ts";

const picking = process.argv[2] === "--pick";
const reportPath = picking ? undefined : process.argv[2];
const [pickGrammar, pickLang, ...pickPaths] = picking ? process.argv.slice(3) : [];
if (picking && (pickGrammar === undefined || pickLang === undefined || pickPaths.length === 0)) {
  throw new Error("usage: bun run oracle/coverage.ts --pick <grammar> <lang> <file or directory>…");
}

/** The grammars a hand-written lexer follows, by Shiki's name, and the
 * lexer that follows each. */
const handLexed = new Map([
  ["asm", "Assembly"],
  ["astro", "Astro"],
  ["bat", "Batch"],
  ["blade", "Blade"],
  ["c", "C"],
  ["cmake", "CMake"],
  ["coffee", "CoffeeScript"],
  ["common-lisp", "Common Lisp"],
  ["cpp", "C++"],
  ["cpp-macro", "C++"],
  ["csharp", "C#"],
  ["css", "CSS"],
  ["dart", "Dart"],
  ["diff", "diff"],
  ["docker", "Dockerfile"],
  ["elixir", "Elixir"],
  ["erb", "ERB"],
  ["gdscript", "GDScript"],
  ["go", "Go"],
  ["graphql", "GraphQL"],
  ["hack", "Hack"],
  ["haml", "HAML"],
  ["haskell", "Haskell"],
  ["hcl", "HCL"],
  ["hlsl", "HLSL"],
  ["html", "HTML"],
  ["html-derivative", "HTML"],
  ["ini", "INI"],
  ["java", "Java"],
  ["javascript", "TypeScript"],
  ["jinja", "Jinja"],
  ["json", "JSON"],
  ["jsonc", "JSON"],
  ["jsonl", "JSON"],
  ["jsx", "TypeScript"],
  ["julia", "Julia"],
  ["kotlin", "Kotlin"],
  ["latex", "LaTeX"],
  ["less", "Less"],
  ["lua", "Lua"],
  ["make", "Make"],
  ["markdown", "Markdown"],
  ["matlab", "MATLAB"],
  ["mermaid", "Mermaid"],
  ["markdown-vue", "Markdown"],
  ["nix", "Nix"],
  ["objective-c", "Objective-C"],
  ["objective-cpp", "Objective-C++"],
  ["pascal", "Pascal"],
  ["perl", "Perl"],
  ["php", "PHP"],
  ["postcss", "PostCSS"],
  ["powershell", "PowerShell"],
  ["proto", "Protobuf"],
  ["python", "Python"],
  ["r", "R"],
  ["ruby", "Ruby"],
  ["rust", "Rust"],
  ["sass", "Sass"],
  ["scala", "Scala"],
  ["scss", "SCSS"],
  ["shaderlab", "ShaderLab"],
  ["shellscript", "shell"],
  ["sql", "SQL"],
  ["svelte", "Svelte"],
  ["swift", "Swift"],
  ["tex", "TeX"],
  ["toml", "TOML"],
  ["tsx", "TypeScript"],
  ["typescript", "TypeScript"],
  ["vb", "Visual Basic"],
  ["vue", "Vue"],
  ["xml", "XML"],
  ["yaml", "YAML"],
  ["zig", "Zig"],
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
/** A grammar. An injection grammar, such as `markdown-vue`, has an
 * `injectionSelector`, and its root patterns are the injection. */
type Grammar = Rule & { name: string; scopeName: string; injectionSelector?: string };
/** Whether `grammar` is only an injection grammar.
 * Mermaid's grammar is an injection grammar and an input language too.
 * Its root patterns are then also scanned as a rule list.
 * So only a grammar that is not an input language counts as an injection. */
const injectionOnly = (grammar: Grammar) =>
  grammar.injectionSelector !== undefined && !inputs.some((input) => input.lang === grammar.name);

/** An expression of a hand-lexed grammar, and the groups of it that count. */
type Expression = { grammar: string; path: string; groups: number[] };
/** Where a rule stands: its grammar, and the repositories around it, the
 * innermost last, against which its `#name` includes resolve. */
type Place = { grammar: Grammar; repositories: Record<string, Rule>[] };

const roles = ["match", "begin", "end", "while"] as const;
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
const capturesOf = (rule: Rule, role: (typeof roles)[number]): Captures | undefined =>
  ({
    match: rule.captures,
    begin: rule.beginCaptures ?? rule.captures,
    end: rule.endCaptures ?? rule.captures,
    while: rule.whileCaptures ?? rule.captures,
  })[role];

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
  for (const role of roles) {
    const source = rule[role];
    if (source === undefined || !tag) continue;
    const groups = groupsIn(capturesOf(rule, role))
      .filter(([, capture]) => capture.name !== undefined || capture.patterns !== undefined)
      .map(([group]) => group);
    units.push(expressions.length);
    rule[role] = `(?#u${expressions.length})${source}`;
    expressions.push({ grammar: place.grammar.name, path: under(path, role), groups });
  }
  unitsOf.set(rule, units);
  const inner: Place = rule.repository
    ? { grammar: place.grammar, repositories: [...place.repositories, rule.repository] }
    : place;
  rule.patterns?.forEach((child, index) =>
    walk(child, under(path, `patterns[${index}]`), inner, tag),
  );
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

const loaded = structuredClone(grammars) as unknown as Grammar[];
for (const name of handLexed.keys()) {
  if (!loaded.some((grammar) => grammar.name === name)) {
    throw new Error(`${name}: listed as hand-lexed, but the oracle does not load it`);
  }
}
const unlisted = loaded.filter((grammar) => !handLexed.has(grammar.name));
if (unlisted.length > 0) {
  const names = unlisted.map((grammar) => grammar.name).join(", ");
  throw new Error(`${names}: loaded, but not listed as hand-lexed`);
}
for (const [id, { scope }] of Object.entries(handOffs)) {
  const own = loaded.find((grammar) => grammar.name === id)?.scopeName ?? "not loaded";
  if (own !== scope) {
    throw new Error(`${id}: src/language.rs names the scope ${scope}, Shiki's grammar ${own}`);
  }
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

/** The units a root reaches: each input language's grammar, every loaded
 * grammar's injections, and every grammar that is only an injection,
 * through patterns, captures and includes. */
const reachable = new Set<number>();
const seen = new Set<Rule>();
const todo: Rule[] = loaded.filter((grammar) =>
  inputs.some((input) => input.lang === grammar.name),
);
for (const grammar of loaded) todo.push(...Object.values(grammar.injections ?? {}));
todo.push(...loaded.filter(injectionOnly));
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
  const selector = grammar.injectionSelector;
  if (selector !== undefined && injectionOnly(grammar)) {
    grammar.patterns = [{ match: `(?#i${priorities.length})(?!)` }, ...(grammar.patterns ?? [])];
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

const tagged = (source: string | RegExp, letter: "u" | "i") =>
  typeof source === "string"
    ? new RegExp(`^\\(\\?#${letter}(\\d+)\\)`).exec(source)?.[1]
    : undefined;
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
/** A highlighter on the tagged grammars. The run makes one per input
 * language. It disposes the highlighter after that language. Disposing it
 * frees the scanners the language compiled. One highlighter for every
 * language kept all of them, and the run outgrew the 4 GB Coverage runner.
 * The run also collects garbage after each fixture and each fuzz batch.
 * Bun's collector runs late, and the garbage of several large fixtures piled
 * up before it ran. */
const newHighlighter = () =>
  createHighlighter({
    langs: loaded as unknown as typeof grammars,
    themes: ["github-dark"],
    engine,
  });

/** The text of a file in the fixture format, from its `L` records. It reads
 * the records in place. It makes no array of every record. */
function textOf(records: string): string {
  const lines: string[] = [];
  for (let at = 0; at < records.length;) {
    const newline = records.indexOf("\n", at);
    const end = newline < 0 ? records.length : newline;
    if (records.startsWith("L ", at)) lines.push(records.slice(at + 2, end));
    at = end + 1;
  }
  return lines.join("\n");
}
/** Tokenizes `text` as `lang` on the tagged grammars and counts what it
 * exercises. It tells whether the scopes equal `committed`. It compares one
 * part at a time. */
function agrees(highlighter: Highlighter, text: string, lang: string, committed: string): boolean {
  let at = 0;
  let same = true;
  for (const part of fixtureParts(highlighter, text, lang)) {
    same &&= committed.startsWith(part, at);
    at += part.length;
  }
  settle();
  return same && at === committed.length;
}
/** Tokenizes `text` as `lang` on the tagged grammars and counts what it
 * exercises. */
function exercise(highlighter: Highlighter, text: string, lang: string): void {
  const parts = fixtureParts(highlighter, text, lang);
  let part = parts.next();
  while (part.done !== true) part = parts.next();
  settle();
}

/** The fuzz corpus: per hand-lexed input language, `fuzzCount` mutated
 * windows of its fixtures, and as many generated documents where
 * oracle/fuzz/<lang>.ts has pieces, at seed 1. oracle/fuzz.ts writes the
 * same documents for the same arguments. Only their text is read: fuzz.ts
 * loads one grammar, so its scopes leave other languages' fences plain. */
const fuzzCount = 100;
const corpus = mkdtempSync(join(tmpdir(), "nishiki-coverage-"));
/** Tokenizes the fuzz corpus of `lang`, counting what it exercises. */
async function fuzz(highlighter: Highlighter, lang: string): Promise<void> {
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
      exercise(highlighter, textOf(readFileSync(join(dir, name), "utf8")), lang);
    }
    Bun.gc(true);
  }
}

const differing: string[] = [];
const inputLangs = [...new Set(inputs.map((input) => input.lang))].sort();
for (const lang of inputLangs) {
  const highlighter = await newHighlighter();
  counting = fixtures;
  const dir = `${root}tests/oracle/${lang}`;
  for (const name of readdirSync(dir)
    .filter((file) => file.endsWith(".txt"))
    .sort()) {
    const committed = readFileSync(`${dir}/${name}`, "utf8");
    if (!agrees(highlighter, textOf(committed), lang, committed)) differing.push(`${lang}/${name}`);
    Bun.gc(true);
  }
  if (!picking && handLexed.has(lang)) {
    counting = fuzzed;
    await fuzz(highlighter, lang);
  }
  highlighter.dispose();
  Bun.gc(true);
}
rmSync(corpus, { recursive: true });

if (picking) {
  const highlighter = await newHighlighter();
  const keysOf = (counted: Tally) => [
    ...[...counted.expressions]
      .filter((unit) => expressions[unit]?.grammar === pickGrammar)
      .map(String),
    ...[...counted.captures].filter(
      (key) => expressions[Number(key.split(":")[0])]?.grammar === pickGrammar,
    ),
  ];
  const nameOf = (key: string) => {
    const [unit, group] = key.split(":");
    return `${expressions[Number(unit)]?.path}${group === undefined ? "" : ` group ${group}`}`;
  };
  const all = expressions.flatMap(({ grammar, groups }, unit) =>
    grammar === pickGrammar && reachable.has(unit)
      ? [String(unit), ...groups.map((group) => `${unit}:${group}`)]
      : [],
  );
  if (all.length === 0)
    throw new Error(`${pickGrammar}: not a hand-lexed grammar with reachable units`);
  const have = new Set(keysOf(fixtures));
  const files = pickPaths.flatMap((path) =>
    statSync(path).isDirectory()
      ? readdirSync(path)
          .sort()
          .map((name) => join(path, name))
      : [path],
  );
  const adds = new Map<string, Set<string>>();
  for (const file of files) {
    counting = tally();
    exercise(highlighter, readFileSync(file, "utf8").replaceAll("\r\n", "\n"), pickLang);
    adds.set(file, new Set(keysOf(counting).filter((key) => !have.has(key))));
  }
  console.log(`${pickGrammar}: the fixtures exercise ${have.size} of ${all.length} units`);
  for (;;) {
    const ranked = [...adds].map(([file, keys]) => ({
      file,
      keys: [...keys].filter((key) => !have.has(key)),
    }));
    const best = ranked.reduce((a, b) => (b.keys.length > a.keys.length ? b : a), {
      file: "",
      keys: [] as string[],
    });
    if (best.keys.length === 0) break;
    for (const key of best.keys) have.add(key);
    console.log(`+${best.keys.length}\t${have.size}\t${best.file}`);
    for (const key of best.keys) console.log(`\t\t${nameOf(key)}`);
  }
  const left = all.filter((key) => !have.has(key));
  console.log(`left ${left.length}:`);
  for (const key of left) console.log(`\t${nameOf(key)}`);
  if (differing.length > 0) {
    console.error(`✘ the tagged grammars changed Shiki's scopes in ${differing.join(", ")}`);
    process.exit(1);
  }
  process.exit(0);
}

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
/** The width a listing line keeps to, unless one word is longer. */
const WIDTH = 100;
/** A line's width in characters. */
const width = (line: string) => [...line].length;
/**
 * A note under a listing entry, broken at spaces into lines of at most `WIDTH` characters.
 * Its first line is indented two spaces, the rest four.
 */
function noteLines(note: string): string[] {
  const lines: string[] = [];
  let line = "  ";
  for (const word of note.split(" ")) {
    if (line.trim() !== "" && width(line) + 1 + width(word) > WIDTH) {
      lines.push(line);
      line = `    ${word}`;
    } else {
      line = line.trim() === "" ? `${line}${word}` : `${line} ${word}`;
    }
  }
  return [...lines, line];
}
/** An entry's first line, a unit's path and what follows it, or two lines when that is too wide. */
const headLines = (path: string, rest: string) => {
  const head = `${path}  ${rest}`;
  return width(head) <= WIDTH ? [head] : [path, `  ${rest}`];
};
const sourceOf = new Map<number, string>();
for (const rule of places.keys()) {
  for (const role of roles) {
    const unit = /^\(\?#u(\d+)\)/.exec(rule[role] ?? "")?.[1];
    if (unit !== undefined) sourceOf.set(Number(unit), rule[role] ?? "");
  }
}

/** What fails the gate, in the order found. */
const failures: string[] = [];
const unitAt = new Map(
  expressions.map((expression, unit) => [`${expression.grammar} ${expression.path}`, unit]),
);
/** " group N" after a unit's path for a capture group, or nothing for an expression. */
function groupSuffix(group: number | undefined): string {
  return group === undefined ? "" : ` group ${group}`;
}

/** Each waived unit, by its key, with its reason. */
const waived = new Map<string, string>();
for (const waiver of waivers) {
  const where = `${waiver.grammar} ${waiver.path}${groupSuffix(waiver.group)}`;
  const unit = unitAt.get(`${waiver.grammar} ${waiver.path}`);
  const counted =
    unit !== undefined &&
    reachable.has(unit) &&
    (waiver.group === undefined || (expressions[unit]?.groups ?? []).includes(waiver.group));
  if (!counted) {
    failures.push(`the waiver for ${where} names no unit the report counts`);
    continue;
  }
  const u: Unit = { unit, group: waiver.group };
  if (hitIn(fixtures, u)) failures.push(`${where} is exercised by a fixture: remove its waiver`);
  const pieces = `${root}oracle/fuzz/${waiver.grammar}.ts`;
  const probed = (await Bun.file(pieces).exists()) && (await import(pieces)).probe !== undefined;
  if (waiver.dropped && waiver.probe)
    failures.push(`the waiver for ${where} is for a dropped rule, which no probe answers`);
  if (probed && !waiver.probe && !waiver.dropped)
    failures.push(`the waiver for ${where} names no oracle/onig.ts probe`);
  waived.set(keyOf(u), waiver.reason);
}
const isWaived = (u: Unit) =>
  waived.has(keyOf(u)) || (u.group !== undefined && waived.has(String(u.unit)));
for (const name of complete) {
  if (!handLexed.has(name))
    failures.push(`${name} is listed complete, but is not a hand-lexed grammar`);
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
  const open = missing.filter((u) => !isWaived(u));
  const listed = complete.includes(name);
  if (listed && open.length > 0) {
    const paths = open.map((u) => `${expressions[u.unit]?.path}${groupSuffix(u.group)}`);
    failures.push(
      `${name} is listed complete, but no fixture or waiver covers ${paths.join(", ")}`,
    );
  }
  const state = listed ? "yes" : open.length === 0 ? "ready" : "";
  rows.push(
    `| ${handLexed.get(name)} | \`${name}\` | ${hit} of ${live.length} | ` +
      `${percent(hit, live.length)} | ${groupsHit} of ${groups.length} | ` +
      `${percent(groupsHit, groups.length)} | ${missing.length - open.length} | ` +
      `${fuzzOnly} | ${state} |`,
  );
  const lines = live.flatMap((unit) => {
    const path = expressions[unit]?.path ?? "";
    if (!fixtures.expressions.has(unit)) {
      const fuzz = fuzzed.expressions.has(unit) ? "  fuzz" : "";
      const regex = `${excerpt(sourceOf.get(unit) ?? "")}${fuzz}`;
      const reason = waived.get(String(unit));
      return [
        ...headLines(path, regex),
        ...(reason === undefined ? [] : noteLines(`waived: ${reason}`)),
      ];
    }
    const gaps = groups.filter((u) => u.unit === unit && !hitIn(fixtures, u));
    if (gaps.length === 0) return [];
    const notes = gaps.flatMap((u) => {
      const reason = waived.get(keyOf(u));
      const marks = [
        hitIn(fuzzed, u) ? "fuzz" : "",
        reason === undefined ? "" : `waived: ${reason}`,
      ];
      const note = marks.filter((mark) => mark !== "").join("; ");
      return note === "" ? [] : noteLines(`group ${u.group}: ${note}`);
    });
    return [...headLines(path, `groups ${gaps.map((u) => u.group).join(", ")}`), ...notes];
  });
  if (lines.length > 0) {
    sections.push(dedent`
      ### \`${name}\`

      \`\`\`text
      ${lines.join("\n")}
      \`\`\``);
  }
  const dead = units.filter((unit) => !reachable.has(unit));
  if (dead.length > 0) {
    const lines = dead.map((unit) => expressions[unit]?.path ?? "");
    unreachableSections.push(dedent`
      ### \`${name}\`

      \`\`\`text
      ${lines.join("\n")}
      \`\`\``);
  }
}

const report = dedent`
  # Oracle coverage

  How much of each grammar that a hand-written lexer follows the oracle fixtures exercise.
  \`mise run oracle:coverage\` writes this file
  from [\`oracle/coverage.ts\`](../../oracle/coverage.ts); do not edit it.
  The [oracle-coverage design](../../openspec/changes/oracle-coverage/design.md) defines the units.

  - An expression is exercised when Shiki takes its match on a fixture.
  - A capture is exercised when its group takes part in such a match
    and holds a non-whitespace byte.
  - Fuzz only counts the units that only the fixed fuzz corpus exercises:
    ${fuzzCount} mutated windows per language,
    and ${fuzzCount} generated documents where \`oracle/fuzz/<lang>.ts\` has pieces, at seed 1.
    No CI job compares that corpus with nishiki.
  - Waived counts the units
    that [\`oracle/coverage-waivers.ts\`](../../oracle/coverage-waivers.ts) waives,
    each for a reason given there.
  - Complete reads \`yes\` for a grammar listed complete there,
    whose every unit a fixture or a waiver covers, which the coverage check holds;
    and \`ready\` for one that could be listed.

  | Lexer | Grammar | Expressions | Share | Captures | Share | Waived | Fuzz only | Complete |
  |:--|:--|--:|--:|--:|--:|--:|--:|:--|
  ${rows.join("\n")}

  ## Units no fixture exercises

  Each entry is an expression's path in its grammar, then the start of its regex,
  and \`fuzz\` when the fuzz corpus exercises it.
  The reason follows on indented lines when a waiver covers it.
  An expression the fixtures do exercise is listed with the groups they do not.
  A group the fuzz corpus exercises, or a waiver covers, follows on an indented line.
  The groups of an unexercised expression count as unexercised and are not listed.

  ${sections.join("\n\n")}

  ## Units no root reaches

  Neither an input language's grammar nor an injection reaches these,
  so they are left out of the shares.

  ${unreachableSections.join("\n\n")}
  `;
const out = reportPath ?? `${root}tests/oracle/COVERAGE.md`;
await Bun.write(out, report);
if (failures.length > 0) {
  for (const failure of failures) console.error(`✘ ${failure}`);
  process.exit(1);
}
console.log(`✔ ${out}: ${handLexed.size} grammars`);
