// Tests of the generator library's pattern analysis, run by `mise run check:rule-grammars`.
import { expect, test } from "bun:test";

import { takesOneBlank } from "./rule-grammar.ts";

test("a single blank, then a character that is not a blank, takes one blank", () => {
  expect(takesOneBlank("\\s-\\s")).toBe(true);
  expect(takesOneBlank("\\s+x")).toBe(false);
});

test("a line feed after the blank counts as a character that is not a blank", () => {
  expect(takesOneBlank(".\\n")).toBe(true);
});

test("a negative lookbehind whose body cannot match a blank is stepped over", () => {
  expect(takesOneBlank("(\\)|(?<!\\.\\.\\.).\\n)")).toBe(true);
});

test("a negative lookbehind whose body can match a blank is not stepped over", () => {
  expect(takesOneBlank("(?<! ).\\n")).toBe(false);
  expect(takesOneBlank("(?<!\\s\\s).\\n")).toBe(false);
  expect(takesOneBlank("(?<!(a)\\1).\\n")).toBe(false);
});
