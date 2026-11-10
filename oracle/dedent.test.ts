// Tests of `dedent`, run by `mise run test:scripts`.
import { expect, test } from "bun:test";

import { dedent } from "./dedent.ts";

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
  expect(ends).toBe("one\ntwo\n");
  expect(runsOn).toBe("one\ntwo");
});

test("a closing backtick left of the text keeps the text's own indentation", () => {
  const text = dedent`
        }
    }
  `;
  expect(text).toBe(["      }", "  }", ""].join("\n"));
});

test("a blank line becomes empty and sets no indentation", () => {
  const text = dedent`
    one

    two
    `;
  const blanksInside = ["    one", "  ", "    two", "    "].join("\n");
  expect(text).toBe("one\n\ntwo\n");
  expect(dedent([`\n${blanksInside}`] as unknown as TemplateStringsArray)).toBe("one\n\ntwo\n");
});

test("an interpolated value with line breaks keeps its own indentation", () => {
  const body = ["fn a() {}", "  fn b() {}"].join("\n");
  const text = dedent`
    impl Rule {
    ${body}
    }
    `;
  expect(text).toBe(["impl Rule {", "fn a() {}", "  fn b() {}", "}", ""].join("\n"));
});

test("the blanks before a value at the start of a line count as indentation", () => {
  const text = dedent`
    match self {
        ${"arm"}
    }`;
  expect(text).toBe(["match self {", "    arm", "}"].join("\n"));
});

test("text after a value on the same line is not dedented", () => {
  const text = dedent`
    [
    ${"a\n"}        b
    ]`;
  expect(text).toBe(["[", "a", "        b", "]"].join("\n"));
});

test("text on the opening line has no indentation, so nothing is dropped", () => {
  const text = dedent`one
    two`;
  expect(text).toBe("one\n    two");
});
