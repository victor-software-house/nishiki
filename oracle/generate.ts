// Writes the Shiki oracle: every input in inputs.ts, tokenized by Shiki with
// its per-line limits off, as one fixture per input under tests/oracle/.
//
// A fixture holds, per source line, `L <text>` and then one
// `T <byte end> <scope> <scope> ...` per TextMate token, outermost scope first.
// Byte ends are UTF-8 offsets into the line, as nishiki's tokens report them.
import { createHighlighter } from "shiki@4.6.0";
import { inputs, type Input } from "./inputs.ts";

const root = new URL("..", import.meta.url).pathname;

/** The input's text, with CRLF read as LF, as Shiki splits lines. */
async function fetchText(input: Input): Promise<string> {
  const response = await fetch(input.url);
  if (!response.ok) {
    throw new Error(`${input.url}: HTTP ${response.status}`);
  }
  return (await response.text()).replaceAll("\r\n", "\n");
}

const languages = [...new Set(inputs.map((input) => input.lang))];
const highlighter = await createHighlighter({
  langs: languages,
  themes: ["github-dark"],
});
const encoder = new TextEncoder();

for (const input of inputs) {
  const text = await fetchText(input);
  const lines = highlighter.codeToTokens(text, {
    lang: input.lang,
    theme: "github-dark",
    includeExplanation: "scopeName",
    tokenizeMaxLineLength: 0,
    tokenizeTimeLimit: 0,
  }).tokens;
  const sourceLines = text.split("\n");
  if (sourceLines.length !== lines.length) {
    throw new Error(
      `${input.name}: ${sourceLines.length} lines, Shiki gave ${lines.length}`,
    );
  }
  const out: string[] = [];
  lines.forEach((tokens, index) => {
    out.push(`L ${sourceLines[index]}`);
    let end = 0;
    for (const token of tokens) {
      for (const part of token.explanation ?? []) {
        end += encoder.encode(part.content).length;
        const scopes = part.scopes.map((scope) => scope.scopeName).join(" ");
        out.push(`T ${end} ${scopes}`);
      }
    }
  });
  const path = `${root}tests/oracle/${input.lang}/${input.name}.txt`;
  await Bun.write(path, `${out.join("\n")}\n`);
  console.log(`✔ ${input.lang}/${input.name}: ${lines.length} lines`);
}
