// Writes the Shiki oracle: every input in inputs.ts, tokenized by Shiki with
// its per-line limits off, as one fixture per input under tests/oracle/.
//
// fixture.ts describes the format.
//
// For each theme in `themes`, it also writes the theme file to
// tests/oracle/themes/<theme>.json and, per input, Shiki's colours to
// `<name>.<theme>.colors`: one line per source line, each token as
// `<byte end>:<colour>:<font style bits>`.
import { bundledThemes, createHighlighter } from "shiki@4.6.0";
import { fixture } from "./fixture.ts";
import { grammars } from "./grammars.ts";
import { inputs, type Input } from "./inputs.ts";
import { sourceText } from "./sources.ts";

const root = new URL("..", import.meta.url).pathname;

/** The input's text, read from the mirror unless it was imported, with CRLF
 * read as LF, as Shiki splits lines. */
async function inputText(input: Input): Promise<string> {
  const text = "text" in input ? input.text : await sourceText(input.url);
  return text.replaceAll("\r\n", "\n");
}

/** Themes that exercise every part of rule matching: github-dark, the
 * default; one-light, with descendant and `>` selectors; catppuccin-mocha,
 * with many font styles. */
const themes = ["github-dark", "one-light", "catppuccin-mocha"] as const;

// `ORACLE_LANG=<id>` writes the fixtures of one language only.
const only = Bun.env["ORACLE_LANG"];
const selected = only === undefined ? inputs : inputs.filter((input) => input.lang === only);
/** A highlighter on every input language, also under `ORACLE_LANG`, so
 * one language's fixtures come out as a full run writes them. The run makes
 * one per input language and disposes it after that language. Disposing it
 * frees the scanners the language compiled in Oniguruma's memory. One
 * highlighter for every language kept all of them. Once the process held
 * about 2.3 GB, Shiki gave each later line one token with only the root
 * scope. */
const newHighlighter = () => createHighlighter({ langs: grammars, themes: [...themes] });
const encoder = new TextEncoder();

// Loading a theme adds its default colours as its first rule. Shiki adds
// the rule to the bundled theme object itself. The theme files hold each
// theme as a highlighter loads it, so one loads them before they are
// written.
(await newHighlighter()).dispose();
for (const theme of themes) {
  const raw = (await bundledThemes[theme]()).default;
  await Bun.write(`${root}tests/oracle/themes/${theme}.json`, `${JSON.stringify(raw, null, 2)}\n`);
}

for (const group of Map.groupBy(selected, (input) => input.lang).values()) {
  const highlighter = await newHighlighter();
  for (const input of group) {
    const text = await inputText(input);
    const scopes = fixture(highlighter, text, input.lang);
    const path = `${root}tests/oracle/${input.lang}/${input.name}.txt`;
    await Bun.write(path, scopes);
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
    console.log(`✔ ${input.lang}/${input.name}: ${text.split("\n").length} lines`);
  }
  highlighter.dispose();
}
