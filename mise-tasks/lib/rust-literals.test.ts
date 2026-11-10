// Tests of the backslash line break check, run by `mise run test:scripts`.
// Each test plants Rust source with one kind of backslash at a line's end.
import { expect, test } from "bun:test";

import { continuedLiterals } from "./rust-literals.ts";

const FILE = "src/planted.rs";

/** Rust source of `lines`. */
function source(lines: string[]): string {
  return [...lines, ""].join("\n");
}

test("a reason that continues on the next line fails", () => {
  const text = source([
    "#[expect(",
    "    dead_code,",
    '    reason = "only the tests use it, \\',
    '              with the engine on"',
    ")]",
  ]);
  expect(continuedLiterals(FILE, text)).toEqual([`${FILE}:3: reason = "only the tests use it, \\`]);
});

test("a byte string that continues fails", () => {
  const text = source(['static WORDS: &[u8] = b"\\', "ab\\", 'cd";']);
  expect(continuedLiterals(FILE, text)).toEqual([
    `${FILE}:1: static WORDS: &[u8] = b"\\`,
    `${FILE}:2: ab\\`,
  ]);
});

test("an escaped backslash at a line's end passes", () => {
  const text = source(['let text = "#define X \\\\', '    1";']);
  expect(continuedLiterals(FILE, text)).toEqual([]);
});

test("a backslash in a raw string passes", () => {
  const text = source(['let text = r#"a "quoted" \\', 'b"#;', 'let next = "c";']);
  expect(continuedLiterals(FILE, text)).toEqual([]);
});

test("a backslash in a comment passes", () => {
  const text = source(["// a line that ends in \\", "/* and \\", '"not a string */', "fn f() {}"]);
  expect(continuedLiterals(FILE, text)).toEqual([]);
});

test("a quote char literal opens no string", () => {
  const text = source(["let quote = '\"';", 'let lifetime: &\'static str = "one line";', "// \\"]);
  expect(continuedLiterals(FILE, text)).toEqual([]);
});

test("a string after an escaped quote char literal is still read", () => {
  const text = source(["let quote = '\\'';", 'let reason = "a \\', 'b";']);
  expect(continuedLiterals(FILE, text)).toEqual([`${FILE}:2: let reason = "a \\`]);
});
