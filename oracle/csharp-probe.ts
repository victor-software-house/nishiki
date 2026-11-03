// Compares the C# matchers with Oniguruma, the regular expression engine
// Shiki runs, one pattern at a time.
//
// For every pattern of the grammar it builds cases from the development
// fixtures' lines, mutated with C# fragments, asks Oniguruma whether the
// pattern's expression matches at a position and where its capture groups
// are, runs the matchers on the same cases through the ignored test
// `matchers_answer_the_probe_cases` of src/csharp/tests.rs, and lists the
// patterns where the two differ.
//
// Run `CSHARP_DUMP=<patterns.jsonl> bun run oracle/csharp-grammar.ts` first,
// then `bun run oracle/csharp-probe.ts <patterns.jsonl> <per pattern> [ids]`.
import { createOnigurumaEngine } from "@shikijs/engine-oniguruma@4.6.0";
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";

const [dump, perArg, ...only] = process.argv.slice(2);
const root = new URL("..", import.meta.url).pathname;
const scratch = Bun.env["CSHARP_PROBE_DIR"] ?? "/tmp";
const engine = await createOnigurumaEngine(import("@shikijs/engine-oniguruma@4.6.0/wasm-inlined"));
const rows: { id: string; regex: string; groups: number[] }[] = readFileSync(dump!, "utf8")
  .trim()
  .split("\n")
  .map((line) => JSON.parse(line));

const lines: string[] = [];
const fixtures = `${root}tests/oracle/csharp`;
for (const file of readdirSync(fixtures)) {
  if (!file.endsWith(".txt")) continue;
  for (const record of readFileSync(`${fixtures}/${file}`, "utf8").split("\n")) {
    if (!record.startsWith("L ")) continue;
    const text = record.slice(2);
    if (text.length > 0 && text.length < 200) lines.push(text);
  }
}

let seed = 12345;
const random = () => {
  seed ^= seed << 13;
  seed >>>= 0;
  seed ^= seed >>> 17;
  seed ^= seed << 5;
  seed >>>= 0;
  return seed / 4294967296;
};
const pick = <T>(items: T[]): T => items[Math.floor(random() * items.length)]!;

const snippets = [
  "int x = 5;", "var y = new List<int>();", "public static void Main(string[] args)", "string? name,", "List<Dictionary<string, int>> map;",
  "(int a, int b) t =", "ref readonly int Foo(", "Foo.Bar<T>.Baz x;", "global::System.String s", "int[] a, int[,] b", "T? v", "A::B c",
  "public int Prop { get; set; }", "int this[int i] =>", "event EventHandler<Args> Changed;", "IFoo.Bar<T>(", "static implicit operator int(", "operator ==(",
  "public override bool Equals(object? obj)", "async Task<int> RunAsync()", "foreach (var (k, v) in dict)", "foreach (ref readonly T x in xs)",
  "using var f = File.Open(", "using static System.Math;", "using A = System.Text;", "global using System;", "extern alias Lib;", "namespace A.B.C;",
  "class C<T> : Base<T>, IFoo where T : class, new()", "record struct P(int X, int Y);", "enum E : byte { A = 1, B }", "delegate void H(int x);", "interface I<in T, out U>",
  "[assembly: Foo]", "[return: NotNull]", "[Obsolete(\"x\", true)]", "x?.y?[0]", "a ?? b", "x is not null", "x is { Length: > 0 } y", "x switch { 1 => 2, _ => 3 }",
  "case int n when n > 0:", "default:", "(Foo)bar", "(int)x", "x as T[]", "new Foo(1, 2)", "new[] { 1, 2 }", "new Foo { A = 1 }", "stackalloc int[4]",
  "from x in xs where x > 1 select x", "join y in ys on x equals y", "orderby x descending", "let z = 1", "group x by y into g",
  "$\"a {b} c\"", "$@\"a {b}\"", "@\"a\"\"b\"", "\"\"\"raw\"\"\"", "$$\"\"\"{{x}}\"\"\"", "'a'", "'\\n'", "\"\\u00e9\\x41\"",
  "0x1F", "0b1010_1010", "1_000_000", "1.5e-3f", ".5", "10UL", "0xFFFFu", "3.14m", "1e", "1__0", "0x", "08",
  "/// <summary>x</summary>", "/// <param name=\"x\" />", "/** doc */", "/** doc", " * line", " */", "// c", "/* c */", "<![CDATA[x]]>", "&amp; &#x41; &#65;",
  "#if DEBUG", "#region Foo", "#pragma warning disable 1591, 1573", "#nullable enable", "#define X", "#elif A && !B", "#line 5 \"f.cs\"", "#:package Foo@1.0",
  "x => x + 1", "(a, b) => a + b", "async (x) => await x", "static delegate (int x) { }", "delegate { }", "default(int)", "typeof(int)", "nameof(x)", "checked(x)",
  "a.b.c(", "a<b>(", "a.b<c,d>(", "base(x)", "this(x)", "yield return x;", "yield break;", "await x", "lock (x)", "fixed (int* p = a)", "unsafe { }",
  "if (x)", "while (true)", "else if", "catch (Exception e) when (e is X)", "const int N = 1;", "int local() =>", "~Foo()", "Foo() : base()", "int* p", "ref int r",
  "where T : struct", "where T : notnull", "T[] arr", "x[0].y", "x ? y : z", "x?.Foo", "x!.y", "a += b", "a >>>= 2", "a ??= b", "a => b", "-> x",
  "params int[] xs", "this T x", "out var z", "in int x", "int x = 1, y = 2;", "var (a, b) = t;", "(a, b) = (b, a);", "{", "}", "(", ")", "[", "]", "<", ">", ";", ",", ".", ":", "::",
  "  ", "\t", "é", "日本語", "x y",
];
const mutate = (text: string): string => {
  let out = text;
  const count = Math.floor(random() * 3);
  for (let i = 0; i < count; i++) {
    const at = Math.floor(random() * (out.length + 1));
    out = out.slice(0, at) + pick(snippets) + (random() < 0.5 ? " " : "") + out.slice(at);
  }
  return out.length < 240 ? out : text;
};

const perPattern = Number(perArg);
type Case = { pat: string; text: string; p: number; g: boolean; nl: boolean };
const cases: Case[] = [];
const targets = only.length ? rows.filter((row) => only.includes(row.id)) : rows;
const scanners = new Map<string, any>();
const onigAt = (regex: string, text: string, p: number, nl: boolean, g: boolean) => {
  const source = g ? regex : regex.replaceAll("\\G", "￿");
  let scanner = scanners.get(source);
  if (!scanner) {
    scanner = engine.createScanner([source]);
    scanners.set(source, scanner);
  }
  const match = scanner.findNextMatchSync(engine.createString(nl ? `${text}\n` : text), p);
  return match !== null && match.captureIndices[0].start === p ? match : null;
};
for (const row of targets) {
  const hasG = row.regex.includes("\\G");
  let positives = 0;
  let negatives = 0;
  for (let tries = 0; tries < 20000 && (positives < perPattern / 2 || negatives < perPattern / 2); tries++) {
    const base = pick([...lines, ...snippets]);
    const text = random() < 0.6 ? mutate(base) : base;
    const choice = random();
    let p = Math.floor(random() * (text.length + 2));
    if (choice < 0.15) p = 0;
    else if (choice < 0.3) p = text.length;
    else if (choice < 0.35) p = text.length + 1;
    const nl = random() < 0.85;
    if (!nl && p > text.length) p = text.length;
    // Positions inside a character are not positions of a text.
    if (/[\ud800-\udfff]/.test(text)) continue;
    const g = hasG ? random() < 0.5 : false;
    const hit = onigAt(row.regex, text, p, nl, g) !== null;
    if (hit && positives < perPattern / 2) {
      positives++;
      cases.push({ pat: row.id, text, p, g, nl });
    } else if (!hit && negatives < perPattern / 2) {
      negatives++;
      cases.push({ pat: row.id, text, p, g, nl });
    }
  }
  if (positives < 3) console.log(`note: ${row.id} found only ${positives} positive cases`);
}

// Oniguruma counts positions in UTF-16 code units; the matchers count bytes.
const encoder = new TextEncoder();
const byteOffset = (text: string, unit: number) =>
  encoder.encode(text.slice(0, unit)).length + Math.max(0, unit - text.length);
const casesFile = `${scratch}/csharp-cases.jsonl`;
const answersFile = `${scratch}/csharp-answers.jsonl`;
writeFileSync(
  casesFile,
  cases.map((c) => JSON.stringify({ ...c, p: byteOffset(c.text, c.p) })).join("\n") + "\n",
);
const run = spawnSync(
  "mise",
  ["x", "--", "cargo", "test", "--locked", "--all-features", "--lib", "matchers_answer_the_probe_cases", "--", "--ignored"],
  { cwd: root, env: { ...process.env, CSHARP_PROBE_IN: casesFile, CSHARP_PROBE_OUT: answersFile }, encoding: "utf8" },
);
if (run.status !== 0) {
  console.log(run.stdout.slice(-3000), run.stderr.slice(-3000));
  process.exit(1);
}
const answers = readFileSync(answersFile, "utf8").trim().split("\n").map((line) => JSON.parse(line));
const groupsOf = new Map(rows.map((row) => [row.id, row.groups]));
const regexOf = new Map(rows.map((row) => [row.id, row.regex]));
const stats = new Map<string, { n: number; bad: number; first?: string }>();
cases.forEach((c, i) => {
  const match = onigAt(regexOf.get(c.pat)!, c.text, c.p, c.nl, c.g);
  const got = answers[i];
  const stat = stats.get(c.pat) ?? { n: 0, bad: 0 };
  stat.n++;
  let ok = true;
  let want: { end: number; caps: ([number, number] | null)[] } | null = null;
  if (match !== null) {
    const unit = (position: number) => byteOffset(`${c.text}${c.nl ? "\n" : ""}`, position);
    want = {
      end: unit(match.captureIndices[0].end),
      caps: match.captureIndices.map((x: { start: number; end: number; length: number }) =>
        x.length === 0 && x.start === 0 && x.end === 0 ? null : [unit(x.start), unit(x.end)],
      ),
    };
  }
  if (want === null) ok = got === null;
  else if (got === null) ok = false;
  else {
    ok = got.end === want.end;
    if (ok) {
      for (const group of groupsOf.get(c.pat)!) {
        if (group === 0) continue;
        const w = want.caps[group];
        const g = got.caps[group];
        if (w === null || w === undefined || w[0] >= w[1]) {
          if (g && g[0] < g[1]) ok = false;
          continue;
        }
        if (!g || g[0] !== w[0] || g[1] !== w[1]) ok = false;
      }
    }
  }
  if (!ok) {
    stat.bad++;
    stat.first ??= JSON.stringify({ text: c.text, p: c.p, g: c.g, nl: c.nl, want, got });
  }
  stats.set(c.pat, stat);
});
let bad = 0;
for (const [id, stat] of stats) {
  if (stat.bad) {
    bad++;
    console.log(`${id}: ${stat.bad}/${stat.n} differ; e.g. ${stat.first}`);
  }
}
console.log(`${stats.size} patterns, ${cases.length} cases, ${bad} differ`);
