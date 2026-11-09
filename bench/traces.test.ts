// Tests of the trace reader and the diff parser, run by `mise run check:traces`.
import { expect, test } from "bun:test";

import { hunks, split } from "./traces.ts";

test("an added line whose text starts with ++ stays in its hunk", () => {
  const diff = ["diff --git a/x b/x", "--- a/x", "+++ b/x", "@@ -2,0 +3,2 @@", "++= 1", "+++ y", ""].join("\n");
  expect(hunks(diff)).toEqual([{ start: 2, count: 0, added: ["+= 1", "++ y"] }]);
});

test("the file header before the first hunk adds no line", () => {
  const diff = ["--- a/x", "+++ b/x", "@@ -1 +1 @@", "-old", "+new", ""].join("\n");
  expect(hunks(diff)).toEqual([{ start: 0, count: 1, added: ["new"] }]);
});

test("a last row with no line feed ends the header without looping", () => {
  expect(() => split("kind edits\nlanguage rust", /^base \d+$/)).toThrow("the trace has no body");
});

test("a header that no row ends is an error, not a loop", () => {
  expect(() => split("kind edits\nlanguage rust\n", /^base \d+$/)).toThrow("the trace has no body");
});

test("a body that starts on the last row, with no line feed, is found", () => {
  const { fields, rest } = split("kind edits\nbase 0", /^base \d+$/);
  expect(fields.get("kind")).toBe("edits");
  expect(rest).toBe("base 0");
});
