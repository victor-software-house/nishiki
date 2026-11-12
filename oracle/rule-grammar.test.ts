// Tests of the generator library's pattern analysis.
// `mise run test:scripts` and `mise run check:rule-grammars` run them.
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

test("a lookbehind or \\b after an item that can take the next byte leaves the start free", () => {
  expect(startOf("x?(?<=x)y").behind).toBe(false);
  expect(startOf("\\s*(?<=\\.)x").wb).toBe(true);
  expect(startOf("\\s*(?<=\\.)x").behind).toBe(false);
  expect(startOf("\\s*\\bx").wb).toBe(true);
  expect(startOf("a?\\bx").wb).toBe(false);
  expect(startOf("(?<=\\.)\\s*x").behind).toBe(true);
});

test("a back reference or a call that can match empty can match where no byte follows", () => {
  expect(startOf("(\\w*)\\1", "any").atEnd).toBe(true);
  expect(startOf("(?<n>a*)\\k<n>", "any").atEnd).toBe(true);
  expect(startOf("(?<n>a*)\\g<n>", "any").atEnd).toBe(true);
  expect(startOf("(a*)(?=\\1)", "any").atEnd).toBe(true);
});

test("a back reference in a lookahead that needs a byte cannot match at the end", () => {
  expect(startOf('(?=(?><<[-~]("?)(SQL)\\b\\1))', "any").atEnd).toBe(false);
});

test("a back reference behind a byte the match needs cannot match where no byte follows", () => {
  expect(startOf("<(\\w*)\\1", "any").atEnd).toBe(false);
});

test("a back reference that can repeat an empty capture can match at the end", () => {
  expect(startOf("(a?)\\1", "any").atEnd).toBe(true);
  expect(startOf("(?:x|())\\1", "any").atEnd).toBe(true);
  expect(startOf("(?=(a?))\\1", "any").atEnd).toBe(true);
});

test("every form of a call or a back reference keeps a match where no byte follows", () => {
  expect(startOf("(?<n>a*)\\g'n'", "any").atEnd).toBe(true);
  expect(startOf("(?<n>a*)\\k'n'", "any").atEnd).toBe(true);
  expect(startOf("(a*)\\g<-1>", "any").atEnd).toBe(true);
  expect(startOf("(a*)\\g<1>", "any").atEnd).toBe(true);
  expect(startOf("a*\\g<0>", "any").atEnd).toBe(true);
});

test("a call of the group it sits in is not followed", () => {
  expect(startOf("(?<n>a\\g<n>?b)", "any").atEnd).toBe(false);
  expect(startOf("(?<n>a\\g<n>|)", "any").atEnd).toBe(true);
});

test("a \\G lets a match start anywhere only before the first item that consumes a byte", () => {
  expect(startOf("\\Gx", "any").any).toBe(true);
  expect(startOf("x?\\Gy", "any").any).toBe(true);
  const hack = startOf("(?i)(\\\\)?\\bG(ender\\Gender|lobIterator)\\b", "any");
  expect(hack.any).toBe(false);
  expect(hack.atEnd).toBe(false);
  expect(startOf("a\\Gb", "any").atEnd).toBe(false);
  expect(startOf("x?\\G$", "any").atEnd).toBe(true);
});

test("a named back reference counts as empty where a numbered one does", () => {
  expect(startOf("(?<n>a?)\\k<n>", "any").atEnd).toBe(true);
  expect(startOf("(?=<<(?<n>x?)\\k<n>)", "any").atEnd).toBe(false);
});

test("an item before \\b is safe in a word unless a match starts there and none earlier", () => {
  expect(startOf("a?\\b-").wb).toBe(false);
  expect(startOf("(?:a\\b)?-").wb).toBe(false);
  expect(startOf("(?:([-:\\\\_\\p{L}\\d])*\\\\)?\\b(?i:Add|Get)").wb).toBe(true);
  expect(
    startOf("(')?((?:\\b[\\p{Lu}\\p{Lt}]['_\\p{Ll}\\p{Lu}\\p{Lt}\\d]*\\.)*)\\b([\\p{Lu}\\p{Lt}])")
      .wb,
  ).toBe(true);
  expect(startOf("[\\w-]*\\bx").wb).toBe(true);
  expect(startOf("(\\w*)\\1\\bx", "any").wb).toBe(false);
  expect(startOf("\\G[\\w-]*\\bx").wb).toBe(false);
});

test("a pattern kept out of words for an earlier start is tried where the scan starts", () => {
  expect(startOf("(?:([-:\\\\_\\p{L}\\d])*\\\\)?\\b(?i:Add|Get)").inWordFirst).toBe(true);
  expect(startOf("\\s*\\bx").inWordFirst).toBe(false);
  expect(
    startOf("(')?((?:\\b[\\p{Lu}\\p{Lt}]['_\\p{Ll}\\p{Lu}\\p{Lt}\\d]*\\.)*)\\b([\\p{Lu}\\p{Lt}])")
      .inWordFirst,
  ).toBe(false);
  expect(startOf("a?\\b-").inWordFirst).toBe(false);
});

test("a run after a lookaround cannot move a match inside a word to the word's start", () => {
  expect(startOf("(?=b)\\w*\\b-", "any").wb).toBe(false);
  expect(startOf("(?<=a)\\w*\\b-", "any").wb).toBe(false);
  expect(startOf("(?=b)(?:\\w*)\\b-", "any").wb).toBe(false);
  expect(startOf("(?:(?=b)|-)\\w*\\b-", "any").wb).toBe(false);
  expect(startOf("(?:-?(?=b))\\w*\\b-", "any").wb).toBe(false);
  expect(startOf("-?\\w*\\b-", "any").wb).toBe(true);
  expect(startOf("\\w*\\b-", "any").wb).toBe(true);
  expect(startOf("(?:\\w*)\\b-", "any").wb).toBe(true);
});

test("an unbraced \\x takes at most two hex digits, as Oniguruma's \\xHH does", () => {
  const { set } = startOf("\\x41B");
  const has = (b: number) => ((set[b >> 6]! >> BigInt(b & 63)) & 1n) !== 0n;
  expect(has(0x41)).toBe(true);
  expect(has(0xd0)).toBe(false);
});
