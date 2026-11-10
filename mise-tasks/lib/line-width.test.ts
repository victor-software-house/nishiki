// Tests of the line bound, run by `mise run test:scripts`.
// Each test plants a long Rust doc line in a matchers module.
import { expect, test } from "bun:test";

import { longLines, WIDTH } from "./line-width.ts";

const MATCHERS = "src/go/matchers/types.rs";

/** A regex with no whitespace, longer than a line. */
const regex = `\\b(?:${Array.from({ length: 20 }, (_, i) => `word${i}`).join("|")})\\b`;

/** A Rust doc comment of `lines`, followed by the item it documents. */
function documented(lines: string[]): string {
  return [...lines.map((line) => `    /// ${line}`), "    fn item() {}", ""].join("\n");
}

test("the planted regex is past the bound", () => {
  expect(regex.length).toBeGreaterThan(WIDTH);
});

test("a long matcher regex in a text block passes", () => {
  const text = documented(["The regex:", "", "```text", regex, "```"]);
  expect(longLines(MATCHERS, text)).toEqual([]);
});

test("a long matcher regex in a code span passes", () => {
  expect(longLines(MATCHERS, documented([`\`${regex}\`,`, "for `Words`."]))).toEqual([]);
});

test("a long doctest line fails, even in a matchers module", () => {
  const call = `let found = matcher.find(r"${regex}");`;
  const text = documented(["```", call, "```"]);
  expect(longLines(MATCHERS, text)).toEqual([`${MATCHERS}:2: ${call.length + 8} characters`]);
});

test("a long line of prose after a code span fails", () => {
  const prose = `\`${regex}\`, for \`Words\`.`;
  const text = documented([prose]);
  expect(longLines(MATCHERS, text)).toEqual([`${MATCHERS}:1: ${prose.length + 8} characters`]);
});
