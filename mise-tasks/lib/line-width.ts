// The line bound of check:line-width and the sorts of lines exempt from it.
// CODING_STANDARDS 3.10 sets the bound and its kinds.
// A line counts its characters, not its bytes.
import { commentText } from "./rust-writers.ts";

export const WIDTH = 100;

/** What a test of an unbreakable line sees. */
type LongLine = {
  file: string;
  text: string;
  /**
   * Whether the line is a doc line of a matchers module
   * that starts inside a `text` block or a code span.
   */
  inMatcherDocCode: boolean;
};

/** A sort of line that cannot break, by what it holds. */
type Unbreakable = { name: string; holds: (line: LongLine) => boolean };

const unbreakable: Unbreakable[] = [
  { name: "a URL", holds: ({ text }) => /[a-z]+:\/\/\S/.test(text) || /^\s*url: `/.test(text) },
  {
    name: "an action pinned to a commit",
    holds: ({ text }) => /^\s*(- )?uses: \S+@[0-9a-f]{40}( #.*)?$/.test(text),
  },
  {
    name: "an import",
    holds: ({ file, text }) => file.endsWith(".ts") && /^import .* from "[^"]+"/.test(text),
  },
  {
    name: "a regex literal",
    holds: ({ file, text }) =>
      file.endsWith(".ts") && /^\/.*\/[dgimsuyv]*[;,)]?$/.test(text.trim()),
  },
  {
    name: "a Markdown table row",
    holds: ({ text }) => /^(echo ")?\|.*\|"?$/.test(commentText(text)),
  },
  {
    name: "a matcher's regex in its docs, one unbroken run",
    holds: ({ text, inMatcherDocCode }) => inMatcherDocCode && !/\s/.test(commentText(text)),
  },
  {
    name: "a JSON string",
    holds: ({ file, text }) =>
      file.endsWith(".json") &&
      /^\s*("(?:[^"\\]|\\.)*":\s*)?("(?:[^"\\]|\\.)*",?|[{[])$/.test(text),
  },
  {
    name: "a Swift matcher name",
    // It stays until the Swift generator moves to codegen/ (rule-table-codegen design 2.4).
    holds: ({ file, text }) => file.startsWith("src/swift/") && /\brx_\w{40,}/.test(text),
  },
];

/** A Rust doc line's fence: its run of backticks and its info string. */
const DOC_FENCE = /^\s*\/\/[/!]\s*(`{3,})(\S*)\s*$/;

/** The lines of `text` past `WIDTH` that can break, as `file:line: width` problems. */
export function longLines(file: string, text: string): string[] {
  const rust = file.endsWith(".rs");
  const matchers = /^src\/\w+\/matchers(\.rs$|\/)/.test(file);
  const found: string[] = [];
  let fence: { ticks: string; text: boolean } | undefined;
  let inSpan = false;
  text.split("\n").forEach((line, index) => {
    const doc = rust && /^\s*\/\/[/!]/.test(line);
    const opens = doc ? DOC_FENCE.exec(line) : null;
    const inCode =
      fence !== undefined
        ? fence.text && opens === null
        : inSpan || commentText(line).startsWith("`");
    const inMatcherDocCode = matchers && doc && inCode;
    if (!doc) inSpan = false;
    else if (opens === null) {
      if (fence === undefined) inSpan = inSpan !== (line.split("`").length % 2 === 0);
    } else if (fence === undefined) fence = { ticks: opens[1] ?? "", text: opens[2] === "text" };
    else if (opens[2] === "" && (opens[1] ?? "").length >= fence.ticks.length) fence = undefined;
    const width = [...line].length;
    if (width <= WIDTH) return;
    if (unbreakable.some((sort) => sort.holds({ file, text: line, inMatcherDocCode }))) return;
    found.push(`${file}:${index + 1}: ${width} characters`);
  });
  return found;
}
