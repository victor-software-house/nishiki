// Tests of `dedent`, run by `mise run test:scripts`.
import { expect, test } from "bun:test";

import { dedent, dedentPieces } from "./dedent.ts";

test("the shared indentation and the opening line break are dropped", () => {
  const text = dedent`
    impl Rule {
        fn id() {}
    }`;
  expect(text).toBe(["impl Rule {", "    fn id() {}", "}"].join("\n"));
});

test("a trailing line break is kept only when written", () => {
  const ends = dedent`
    one
    two
    `;
  const runsOn = dedent`
    one
    two`;
  expect(ends).toBe(["one", "two", ""].join("\n"));
  expect(runsOn).toBe(["one", "two"].join("\n"));
});

test("a closing backtick left of the text keeps the text's own indentation", () => {
  const text = dedent`
        }
    }
  `;
  expect(text).toBe(["      }", "  }", ""].join("\n"));
});

test("a closing line right of the text is dropped", () => {
  const pieces = [["", "    a", "        "].join("\n")];
  expect(dedentPieces(pieces, [])).toBe(["a", ""].join("\n"));
});

test("a blank line becomes empty and sets no indentation", () => {
  const text = dedent`
    one

    two
    `;
  const blanksInside = ["", "    one", "  ", "    two", "    "].join("\n");
  expect(text).toBe(["one", "", "two", ""].join("\n"));
  expect(dedentPieces([blanksInside], [])).toBe(["one", "", "two", ""].join("\n"));
});

test("a first line of blanks is dropped like an empty one", () => {
  const pieces = [["   ", "    one", "    two"].join("\n")];
  expect(dedentPieces(pieces, [])).toBe(["one", "two"].join("\n"));
});

test("an empty template gives empty text", () => {
  expect(dedent``).toBe("");
});

test("a single indented line with no line break loses its indentation", () => {
  expect(dedent`    one`).toBe("one");
});

test("an escaped line break is a line break, since the tag reads the cooked text", () => {
  const escaped = dedent`\n    one\n    two`;
  const written = dedent`
    one
    two`;
  expect(escaped).toBe(written);
});

test("indentation that does not start with the shortest one throws", () => {
  const pieces = [["", "\tone", "  two"].join("\n")];
  expect(() => dedentPieces(pieces, [])).toThrow("mix tabs and spaces");
});

test("indentation that starts with the shortest one keeps what follows it", () => {
  const pieces = [["", "\tone", "\t  two"].join("\n")];
  expect(dedentPieces(pieces, [])).toBe(["one", "  two"].join("\n"));
});

test("a value whose first line is empty leaves no blanks on its line", () => {
  const empty = dedent`
    a
      ${""}
    b`;
  const lineFirst = dedent`
    a
      ${["", "x"].join("\n")}
    b`;
  expect(empty).toBe(["a", "", "b"].join("\n"));
  expect(lineFirst).toBe(["a", "", "  x", "b"].join("\n"));
});

test("an empty value before text on its line keeps the line's indentation", () => {
  const prefix = "";
  const code = dedent`
    impl A {
        ${prefix}fn x() {}
    }`;
  expect(code).toBe(["impl A {", "    fn x() {}", "}"].join("\n"));
});

test("an empty value before another value on its line keeps the line's indentation", () => {
  const name = dedent`
    impl A {
        ${""}${"fn x() {}"}
    }`;
  const prefixes = dedent`
    impl A {
        ${""}${""}fn x() {}
    }`;
  expect(name).toBe(["impl A {", "    fn x() {}", "}"].join("\n"));
  expect(prefixes).toBe(["impl A {", "    fn x() {}", "}"].join("\n"));
});

test("empty values alone on their line leave it empty", () => {
  const code = dedent`
    a
      ${""}${""}
    b`;
  expect(code).toBe(["a", "", "b"].join("\n"));
});

test("a value whose first line is empty leaves that line empty, whatever follows it", () => {
  const code = dedent`
    a
        ${["", "x"].join("\n")}y
    b`;
  expect(code).toBe(["a", "", "    xy", "b"].join("\n"));
});

test("a value that starts a line has its later lines indented as that line", () => {
  const body = ["fn a() {}", "", "  fn b() {}"].join("\n");
  const text = dedent`
    impl Rule {
        ${body}
    }
    `;
  expect(text).toBe(["impl Rule {", "    fn a() {}", "", "      fn b() {}", "}", ""].join("\n"));
});

test("a value at the margin keeps its own indentation", () => {
  const body = ["fn a() {}", "  fn b() {}"].join("\n");
  const text = dedent`
    impl Rule {
    ${body}
    }
    `;
  expect(text).toBe(["impl Rule {", "fn a() {}", "  fn b() {}", "}", ""].join("\n"));
});

test("a value after text on its line goes in as it is", () => {
  const text = dedent`
    let x = ${["a", "b"].join("\n")};`;
  expect(text).toBe(["let x = a", "b;"].join("\n"));
});

test("text after a value on the same line is not dedented", () => {
  const text = dedent`
    [
    ${["a", ""].join("\n")}        b
    ]`;
  expect(text).toBe(["[", "a", "        b", "]"].join("\n"));
});

test("text on the opening line has no indentation, so nothing is dropped", () => {
  const text = dedent`one
    two`;
  expect(text).toBe(["one", "    two"].join("\n"));
});

test("numbers go in as their decimal text", () => {
  expect(dedent`
    size ${256}`).toBe("size 256");
});
