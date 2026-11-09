// Tests of the generator library's pattern analysis, run by `mise run check:rule-grammars`.
import { expect, test } from "bun:test";

import { canMatchEmpty, startOf, takesOneBlank } from "./rule-grammar.ts";

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

test("a leading group's alternatives each run on with the rest of the pattern", () => {
  expect(takesOneBlank("(;|(^|\\s)#\\s).*$")).toBe(true);
  expect(takesOneBlank("(;|#)\\s.*$")).toBe(false);
});

test("a quantified leading group does not distribute", () => {
  expect(takesOneBlank("(a|\\s)+x")).toBe(false);
});

test("a leading group with an anchor counts only through its blank branch", () => {
  expect(takesOneBlank("(^|\\s)#\\s")).toBe(true);
  expect(takesOneBlank("(^|a)#\\s")).toBe(false);
});

test("a match of only a lookahead, or an optional atom, can match the empty text", () => {
  expect(canMatchEmpty("(;)|(?=[)}])")).toBe(true);
  expect(canMatchEmpty("a?")).toBe(true);
  expect(canMatchEmpty("(;)|[)}]")).toBe(false);
  expect(canMatchEmpty("\\b\\w+")).toBe(false);
});

test("an anchor, a negative lookahead or an optional atom can match where no byte follows", () => {
  for (const regex of ["$", "\\z", "(?=$)", "(?!x)", "a?", "(;)|(?=[)}]|$)", "(\\w*)\\1"]) {
    expect(startOf(regex).atEnd).toBe(true);
  }
});

test("a lookahead that needs a byte cannot match where no byte follows", () => {
  for (const regex of ["(?=<<)", "(?=\\n)", "(?=[)}])", "\\b\\w+", "(?<=\\{)(\\s+)"]) {
    expect(startOf(regex).atEnd).toBe(false);
  }
});

test("a lookbehind that needs a byte cannot match at the start of the line", () => {
  for (const regex of ["(?<=\\.)foo", "(?<=\\w)x", "(?<=ab)x", "(?:(?<=,)|(?<=;))x"]) {
    expect(startOf(regex).behind).toBe(true);
  }
  for (const regex of ["(?<=^|\\s)x", "(?<=\\.)?x", "x|(?<=\\.)y", "(?<!\\.)x"]) {
    expect(startOf(regex).behind).toBe(false);
  }
});

test("a lookbehind of one character with no word character cannot match inside a word", () => {
  for (const regex of ["(?<=\\.)foo", "(?<=[,(])\\s*x", "(?<=\\{)(\\s+)"]) {
    expect(startOf(regex).wb).toBe(true);
  }
  for (const regex of ["(?<=\\w)x", "(?<=[a.])x", "(?<=\\W)x", "(?<=..)x", "(?<=.)x"]) {
    expect(startOf(regex).wb).toBe(false);
  }
});
