// The oracle's fixture format, shared by the scripts that write fixtures.
//
// A fixture holds, per source line, `L <text>` and then one
// `T <byte end> <scope> <scope> ...` per TextMate token, outermost scope first.
// Byte ends are UTF-8 offsets into the line, as nishiki's tokens report them.
import type { createHighlighter } from "shiki@4.6.0";

/** A highlighter that Shiki makes. */
export type Highlighter = Awaited<ReturnType<typeof createHighlighter>>;

const encoder = new TextEncoder();

/** Shiki's scopes for `text` in `lang` as a fixture, with its per-line limits
 * off. `text` has LF line ends. */
export function fixture(highlighter: Highlighter, text: string, lang: string): string {
  const lines = highlighter.codeToTokens(text, {
    lang,
    theme: highlighter.getLoadedThemes()[0],
    includeExplanation: "scopeName",
    tokenizeMaxLineLength: 0,
    tokenizeTimeLimit: 0,
  }).tokens;
  const sourceLines = text.split("\n");
  if (sourceLines.length !== lines.length) {
    throw new Error(`${sourceLines.length} lines, Shiki gave ${lines.length}`);
  }
  const out: string[] = [];
  lines.forEach((tokens, index) => {
    out.push(`L ${sourceLines[index]}`);
    let end = 0;
    for (const token of tokens) {
      for (const part of token.explanation ?? []) {
        end += encoder.encode(part.content).length;
        out.push(`T ${end} ${part.scopes.map((scope) => scope.scopeName).join(" ")}`);
      }
    }
  });
  return `${out.join("\n")}\n`;
}
