// Tests of the trace reader and the diff parser, run by `mise run check:traces`.
import { expect, test } from "bun:test";

import { hunks, pushes, split } from "./traces.ts";
import { dedent } from "../oracle/dedent.ts";

test("an added line whose text starts with ++ stays in its hunk", () => {
  const diff = ["diff --git a/x b/x", "--- a/x", "+++ b/x", "@@ -2,0 +3,2 @@", "++= 1", "+++ y", ""].join("\n");
  expect(hunks(diff)).toEqual([{ start: 2, count: 0, added: ["+= 1", "++ y"] }]);
});

test("the file header before the first hunk adds no line", () => {
  const diff = ["--- a/x", "+++ b/x", "@@ -1 +1 @@", "-old", "+new", ""].join("\n");
  expect(hunks(diff)).toEqual([{ start: 0, count: 1, added: ["new"] }]);
});

test("a last row with no line feed ends the header without looping", () => {
  expect(() => split("origin edits\nlanguage rust", /^base \d+$/)).toThrow("the trace has no body");
});

test("a header that no row ends is an error, not a loop", () => {
  expect(() => split(dedent`
    origin edits
    language rust
    `, /^base \d+$/)).toThrow("the trace has no body");
});

test("a body that starts on the last row, with no line feed, is found", () => {
  const { fields, rest } = split("origin edits\nbase 0", /^base \d+$/);
  expect(fields.get("origin")).toBe("edits");
  expect(rest).toBe("base 0");
});

test("deltas that add up to the source make its pushes", () => {
  const bytes = new TextEncoder().encode("aé");
  expect(pushes("t", bytes, dedent`
    deltas
    1 2
    `)).toEqual([{ push: "a" }, { push: "é" }]);
});

test("deltas short of the source are an error", () => {
  const bytes = new TextEncoder().encode("abc");
  expect(() => pushes("t", bytes, dedent`
    deltas
    1 1
    `)).toThrow("t: the deltas do not add up to the source");
});

test("a delta past the source is an error", () => {
  const bytes = new TextEncoder().encode("abc");
  expect(() => pushes("t", bytes, dedent`
    deltas
    2 2
    `)).toThrow("t: a delta is not a length or runs past the source");
});

test("a delta that usize::parse refuses is an error, as in the Rust reader", () => {
  const bytes = new TextEncoder().encode("abc");
  expect(pushes("t", bytes, dedent`
    deltas
    +3
    `)).toEqual([{ push: "abc" }]);
  for (const delta of ["0x3", "0b11", "3e0", "3.0", "-3", "++3"]) {
    expect(() => pushes("t", bytes, dedent`
      deltas
      ${delta}
      `)).toThrow("t: a delta is not a length or runs past the source");
  }
});
