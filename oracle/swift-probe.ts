// Compares the Swift matchers with Oniguruma, pattern by pattern.
//
// usage: bun run oracle/swift-probe.ts <rx.jsonl> <cases per pattern> [pattern id ...]
//
// `rx.jsonl` comes from `SWIFT_RX_OUT=rx.jsonl bun run oracle/swift-grammar.ts`.
// For each pattern it draws lines from the development fixtures, mutated with
// snippets of Swift, and positions in them; asks Oniguruma whether the regular
// expression matches exactly there; asks the matchers through the ignored test
// `swift::tests::probe::matchers_answer_the_probe_cases`; and prints every
// pattern on which they differ, with the first case.
//
// A position can stand after the line feed, one past the text: vscode-textmate
// scans a line until no pattern matches (`_tokenizeString`), and a match that
// takes the line feed leaves its cursor there, where a zero-width pattern such
// as an end can still match. Byte positions past the text map to UTF-16
// positions past it, both ways.
//
// Held-out fixtures are never read: the corpus is the development inputs named
// in SWIFT_CORPUS (comma separated fixture names under tests/oracle/swift/).
import { createOnigurumaEngine } from "@shikijs/engine-oniguruma@4.6.0";
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { tmpdir } from "node:os";
import { join } from "node:path";

const [rxPath, perArg, ...only] = process.argv.slice(2);
const root = new URL("..", import.meta.url).pathname;
const engine = await createOnigurumaEngine(import("@shikijs/engine-oniguruma@4.6.0/wasm-inlined"));
const rows: { id: string; regex: string; groups: number[]; backref: boolean }[] = readFileSync(rxPath!, "utf8")
  .trim()
  .split("\n")
  .map((l) => JSON.parse(l));

const corpusNames = (process.env.SWIFT_CORPUS ??
  "sample,argument-parser-argument,algorithms-chunked,alamofire-af-error,composable-store,string-processing-algorithms-tests").split(",");
const lines: string[] = [];
for (const name of corpusNames)
  for (const l of readFileSync(`${root}tests/oracle/swift/${name}.txt`, "utf8").split("\n"))
    if (l.startsWith("L ") && l.length > 2 && l.length < 240) lines.push(l.slice(2));

let seed = 20261102;
const rnd = () => {
  seed ^= seed << 13;
  seed >>>= 0;
  seed ^= seed >>> 17;
  seed ^= seed << 5;
  seed >>>= 0;
  return seed / 4294967296;
};
const pick = <T,>(a: T[]): T => a[Math.floor(rnd() * a.length)]!;

const SNIPPETS = [
  "func f(", "func <T>(", "init?(", "init(", "deinit {", "subscript(", "class A: B {", "struct S<T: P> where T.E == Int {", "enum E: Int {", "case a(Int), b", "case .x:", "extension Array where Element: Equatable {",
  "protocol P: AnyObject {", "associatedtype T", "typealias A = B<C>", "actor A {", "macro m() = #externalMacro(", "@available(iOS 13, *)", "@objc(foo:)", "@MainActor", "@propertyWrapper", "@resultBuilder",
  "#if os(macOS)", "#elseif swift(>=5.9)", "#else", "#endif", "#warning(\"x\")", "#sourceLocation(file: \"a\", line: 1)", "import Foundation", "import struct A.B", "@testable import C",
  "let x = 1", "var y: Int? = nil", "let (a, b) = (1, 2)", "if let x = y {", "guard case .a = b else {", "for i in 0..<10 {", "while true {", "repeat {", "switch x {", "default:", "defer {", "do {", "catch let e {",
  "try? f()", "try! g()", "await h()", "async let a = b", "throws -> Int", "rethrows", "inout x", "some P", "any P", "borrowing", "consuming", "~Copyable",
  "\"a\\(b)c\"", "\"\"\"", "#\"raw\"#", "##\"x\"##", "\"\\u{1F600}\"", "'c'", "0x1F", "0b101", "0o17", "1_000", "1.5e3", "0x1.8p3", ".5", "1.",
  "/// doc", "// c", "/* c */", "/* /* n */ */", "/**/", "#/a+/#", "/[a-z]+/", "/x/.ignoresCase()", "#/\\d{3}/#",
  "a ?? b", "a?.b", "a!.c", "a... b", "..<", "...", "===", "!==", "~=", "&+", "&-", "&*", "<<", ">>", "->", "=>", "?", "!", "$0", "$1", "\\.self", "\\Foo.bar", "#selector(a)", "#keyPath(b)",
  "`class`", "`a b`", "é", "日本語", "_", "__x", "x.0", "a.b.c", "a?.b?.c", ".init(", ".a.b", "[Int: String]", "[[Int]]", "(Int) -> Void", "(a: Int, b: String)", "Array<Int>", "Optional<String>", "A.B<C>.D",
  "weak var x", "unowned let y", "lazy var z", "static func", "class func", "override init", "required init", "convenience init", "final class", "open class", "public private(set) var", "internal", "fileprivate", "mutating func", "nonisolated", "indirect enum", "@escaping", "@autoclosure", "@Sendable", "@inlinable", "@discardableResult",
  "where T: Hashable", "T: Comparable & Hashable", "<T, U>", "<T: P>", "Self", "self", "super", "nil", "true", "false", "Any", "AnyObject", "Void", "Int", "String", "Bool", "print(", "assert(", "fatalError(", "min(", "zip(", "repeat each T", "each x",
  "@_specialize(", "@_silgen_name(\"a\")", "precedencegroup P {", "higherThan: A", "associativity: left", "infix operator +++", "prefix operator !", "postfix operator ++", "operator <=>",
  "{ $0 + 1 }", "{ (a: Int) -> Int in", "{ [weak self] in", "}", "{", "(", ")", "[", "]", ",", ";", ":", ".", "@", "#", "\\", "\"", "'", "`",
];
const mutate = (t: string) => {
  let out = t;
  const n = Math.floor(rnd() * 3);
  for (let i = 0; i < n; i++) {
    const at = Math.floor(rnd() * (out.length + 1));
    out = out.slice(0, at) + pick(SNIPPETS) + (rnd() < 0.5 ? " " : "") + out.slice(at);
  }
  return out.length < 260 ? out : t;
};

const per = Number(perArg);
const targets = rows.filter((r) => !r.backref && (only.length === 0 || only.includes(r.id)));
const scanners = new Map<string, any>();
const find = (regex: string, text: string, p: number, g: boolean) => {
  const rx = g ? regex : regex.replaceAll("\\G", "\uFFFF");
  let sc = scanners.get(rx);
  if (!sc) {
    sc = engine.createScanner([rx]);
    scanners.set(rx, sc);
  }
  // Positions are UTF-16 here; the text is converted to bytes below.
  return sc.findNextMatchSync(engine.createString(text + "\n"), p);
};

type Case = { rx: string; text: string; p: number; g: boolean };
const cases: Case[] = [];
const enc = new TextEncoder();
/** The byte offset of the UTF-16 offset `i` of `text`. */
const byteAt = (text: string, i: number) =>
  // Oniguruma sees the text and a line feed: an end past the text is a match
  // that took the line feed, which the matchers report as `lim() + 1`.
  i > text.length ? enc.encode(text).length + (i - text.length) : enc.encode(text.slice(0, i)).length;
for (const r of targets) {
  const hasG = r.regex.includes("\\G");
  let positives = 0;
  let negatives = 0;
  for (let tries = 0; tries < 40000 && (positives < per / 2 || negatives < per / 2); tries++) {
    const base = pick(lines);
    const text = rnd() < 0.5 ? mutate(base) : base;
    const g = hasG && rnd() < 0.6;
    const start = rnd() < 0.2 ? 0 : Math.floor(rnd() * (text.length + 1));
    const m = find(r.regex, text, start, g);
    if (m && positives < per / 2) {
      // A match found by searching from `start` starts at `at`, which is a
      // position where the pattern matches.
      const at = m.captureIndices[0].start;
      cases.push({ rx: r.id, text, p: byteAt(text, at), g: g && at === start });
      positives++;
    }
    if (negatives < per / 2) {
      const p = start;
      const hit = find(r.regex, text, p, g);
      if (!hit || hit.captureIndices[0].start !== p) {
        cases.push({ rx: r.id, text, p: byteAt(text, p), g });
        negatives++;
      }
    }
  }
  if (positives < 3) console.log(`note: ${r.id} found only ${positives} positive cases`);
}

const dir = mkdtempSync(join(tmpdir(), "swift-probe-"));
const casesFile = join(dir, "cases.jsonl");
const answersFile = join(dir, "answers.jsonl");
writeFileSync(casesFile, cases.map((c) => JSON.stringify(c)).join("\n") + "\n");
const run = spawnSync(
  "cargo",
  ["test", "--all-features", "--lib", "swift::tests::probe::matchers_answer_the_probe_cases", "--", "--ignored", "--exact"],
  { cwd: root, env: { ...process.env, SWIFT_PROBE_IN: casesFile, SWIFT_PROBE_OUT: answersFile }, encoding: "utf8" },
);
if (run.status !== 0) {
  console.log(run.stdout.slice(-3000), run.stderr.slice(-3000));
  process.exit(1);
}
const answers = readFileSync(answersFile, "utf8").trim().split("\n").map((l) => JSON.parse(l));
const regexOf = new Map(rows.map((r) => [r.id, r]));
const stats = new Map<string, { n: number; bad: number; first?: string }>();
cases.forEach((c, i) => {
  const row = regexOf.get(c.rx)!;
  // The case's byte position back to UTF-16 for Oniguruma, past the text
  // when the case stands after the line feed.
  const bytes = enc.encode(c.text);
  const p16 =
    c.p > bytes.length
      ? c.text.length + (c.p - bytes.length)
      : new TextDecoder().decode(bytes.slice(0, c.p)).length;
  const m = find(row.regex, c.text, p16, c.g);
  const hit = m && m.captureIndices[0].start === p16 ? m : null;
  const got = answers[i];
  const b = (u: number) => byteAt(c.text, u);
  let ok = true;
  if (!hit) ok = got === null;
  else if (got === null) ok = false;
  else {
    ok = got.end === b(hit.captureIndices[0].end);
    if (ok)
      for (const k of row.groups) {
        const w = hit.captureIndices[k];
        const gg = got.caps[k];
        const wantEmpty = !w || w.length === 0 || w.start === w.end;
        const gotEmpty = !gg || gg[0] >= gg[1];
        if (wantEmpty || gotEmpty) {
          if (wantEmpty !== gotEmpty) ok = false;
        } else if (gg[0] !== b(w.start) || gg[1] !== b(w.end)) ok = false;
      }
  }
  const st = stats.get(c.rx) ?? { n: 0, bad: 0 };
  st.n++;
  if (!ok) {
    st.bad++;
    st.first ??= JSON.stringify({ text: c.text, p: c.p, g: c.g, onig: hit ? [b(hit.captureIndices[0].end), row.groups.map((k) => hit.captureIndices[k] && [b(hit.captureIndices[k].start), b(hit.captureIndices[k].end)])] : null, got });
  }
  stats.set(c.rx, st);
});
let bad = 0;
for (const [id, st] of stats)
  if (st.bad) {
    bad++;
    console.log(`${id}: ${st.bad}/${st.n} differ; e.g. ${st.first}`);
  }
console.log(`${stats.size} patterns, ${bad} differ, ${cases.length} cases`);
