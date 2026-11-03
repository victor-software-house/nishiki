// Writes the Shiki oracle: every input in inputs.ts, tokenized by Shiki with
// its per-line limits off, as one fixture per input under tests/oracle/.
//
// A fixture holds, per source line, `L <text>` and then one
// `T <byte end> <scope> <scope> ...` per TextMate token, outermost scope first.
// Byte ends are UTF-8 offsets into the line, as nishiki's tokens report them.
//
// For each theme in `themes`, it also writes the theme file to
// tests/oracle/themes/<theme>.json and, per input, Shiki's colours to
// `<name>.<theme>.colors`: one line per source line, each token as
// `<byte end>:<colour>:<font style bits>`.
import { bundledLanguages, bundledThemes, createHighlighter } from "shiki@4.6.0";
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

/** Themes that exercise every part of rule matching: github-dark, the
 * default; one-light, with descendant and `>` selectors; catppuccin-mocha,
 * with many font styles. */
const themes = ["github-dark", "one-light", "catppuccin-mocha"] as const;

// `ORACLE_LANG=<id>` writes the fixtures of one language only.
const only = Bun.env["ORACLE_LANG"];
const selected = only === undefined ? inputs : inputs.filter((input) => input.lang === only);
/** Each input language's grammar, and the grammars it embeds that are input
 * languages too. A grammar for a language nishiki does not lex is left out,
 * so a fence or embedding in that language stays plain, as nishiki leaves it,
 * whichever input language's grammar would pull it in. Every input language
 * is loaded, also under `ORACLE_LANG`, so one language's fixtures come out as
 * a full run writes them. */
const inputLangs = new Set(inputs.map((input) => input.lang));
const loaded = (
  await Promise.all([...inputLangs].map(async (lang) => (await bundledLanguages[lang]()).default))
).flat();
const lexed = loaded.filter(
  (grammar) =>
    inputLangs.has(grammar.name) || grammar.aliases?.some((alias) => inputLangs.has(alias)),
);
const lexedNames = new Set(lexed.map((grammar) => grammar.name));
const languages = [...new Map(lexed.map((grammar) => [grammar.name, grammar])).values()].map(
  (grammar) => ({
    ...grammar,
    embeddedLangs: grammar.embeddedLangs?.filter((name) => lexedNames.has(name)),
  }),
);
const highlighter = await createHighlighter({
  langs: languages,
  themes: [...themes],
});
const encoder = new TextEncoder();

for (const theme of themes) {
  const raw = (await bundledThemes[theme]()).default;
  await Bun.write(`${root}tests/oracle/themes/${theme}.json`, `${JSON.stringify(raw, null, 2)}\n`);
}

for (const input of selected) {
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
  for (const theme of themes) {
    const coloured = highlighter.codeToTokens(text, {
      lang: input.lang,
      theme,
      tokenizeMaxLineLength: 0,
      tokenizeTimeLimit: 0,
    }).tokens;
    const rows = coloured.map((tokens) => {
      let end = 0;
      return tokens
        .map((token) => {
          end += encoder.encode(token.content).length;
          return `${end}:${token.color}:${token.fontStyle ?? 0}`;
        })
        .join(" ");
    });
    await Bun.write(
      `${root}tests/oracle/${input.lang}/${input.name}.${theme}.colors`,
      `${rows.join("\n")}\n`,
    );
  }
  console.log(`✔ ${input.lang}/${input.name}: ${lines.length} lines`);
}
