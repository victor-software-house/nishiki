// The oracle's fixture format, shared by the scripts that write fixtures.
//
// A fixture holds, per source line, `L <text>` and then one
// `T <byte end> <scope> <scope> ...` per TextMate token, outermost scope first.
// Byte ends are UTF-8 offsets into the line, as nishiki's tokens report them.
import type { createHighlighter, GrammarState } from "shiki@4.6.0";

/** A highlighter that Shiki makes. */
export type Highlighter = Awaited<ReturnType<typeof createHighlighter>>;

const encoder = new TextEncoder();

/** Lines Shiki tokenizes per call. One call on the largest fixture held
 * about a gigabyte of tokens. Each block starts from the grammar state the
 * block before it ended in. Every line gets the scopes one call would give
 * it. */
const blockLines = 1000;

/** Shiki's scopes for `text` in `lang` as a fixture, one block of lines at
 * a time. Shiki's per-line limits are off. Each part holds whole records and
 * ends with a newline. The parts joined are the fixture. `text` has LF line
 * ends. */
export function* fixtureParts(
  highlighter: Highlighter,
  text: string,
  lang: string,
): Generator<string> {
  const sourceLines = text.split("\n");
  let grammarState: GrammarState | undefined;
  for (let first = 0; first < sourceLines.length; first += blockLines) {
    const block = sourceLines.slice(first, first + blockLines);
    const result = highlighter.codeToTokens(block.join("\n"), {
      lang,
      theme: highlighter.getLoadedThemes()[0],
      includeExplanation: "scopeName",
      tokenizeMaxLineLength: 0,
      tokenizeTimeLimit: 0,
      grammarState,
    });
    grammarState = result.grammarState;
    if (block.length !== result.tokens.length) {
      throw new Error(
        `lines ${first + 1} to ${first + block.length}: Shiki gave ${result.tokens.length} lines`,
      );
    }
    const out: string[] = [];
    result.tokens.forEach((tokens, index) => {
      out.push(`L ${block[index]}`);
      let end = 0;
      for (const token of tokens) {
        for (const part of token.explanation ?? []) {
          end += encoder.encode(part.content).length;
          out.push(`T ${end} ${part.scopes.map((scope) => scope.scopeName).join(" ")}`);
        }
      }
    });
    yield `${out.join("\n")}\n`;
  }
}

/** Shiki's scopes for `text` in `lang` as a fixture: `fixtureParts` joined. */
export const fixture = (highlighter: Highlighter, text: string, lang: string): string =>
  [...fixtureParts(highlighter, text, lang)].join("");
