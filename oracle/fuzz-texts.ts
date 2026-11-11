// The documents of a differential corpus, as text.
// oracle/fuzz.ts writes them as fixtures, and oracle/coverage.ts tokenizes them in its own process.
// The same arguments give the same documents.
import { readdirSync, readFileSync } from "node:fs";

/** What `oracle/fuzz/<lang>.ts` adds for one language. */
export type Extras = { pieces?: string[]; mutations?: string[]; fences?: string[] };

/** The kinds of document:
 * - `win`: windows of the source texts, mutated;
 * - `gen`: lines of random pieces;
 * - `mdw`, `mdg`: the documents of `win` or `gen` in Markdown fences of the language,
 *   bare, in block quotes and in list items. */
export type Mode = "win" | "gen" | "mdw" | "mdg";

const root = new URL("..", import.meta.url).pathname;

/** What `oracle/fuzz/<lang>.ts` exports, or nothing where the file is missing.
 * Only a missing file reads as nothing; an error in the file stops the run. */
export async function extrasOf(lang: string): Promise<Extras> {
  const path = `${root}oracle/fuzz/${lang}.ts`;
  return (await Bun.file(path).exists()) ? await import(path) : {};
}

/** The text of every fixture of `lang`, in name order: `readdirSync` order
 * varies between file systems, and the windows drawn depend on it. */
export function fixtureTexts(lang: string): string[] {
  const dir = `${root}tests/oracle/${lang}`;
  return readdirSync(dir)
    .filter((name) => name.endsWith(".txt"))
    .sort()
    .map((name) =>
      readFileSync(`${dir}/${name}`, "utf8")
        .split("\n")
        .filter((record) => record.startsWith("L "))
        .map((record) => record.slice(2))
        .join("\n"),
    );
}

/** Texts that break most grammars somewhere: brackets, quotes, comment
 * markers, escapes, a line continuation, and characters outside ASCII. */
const MUTATIONS = [
  "(",
  ")",
  "{",
  "}",
  "[",
  "]",
  "<",
  ">",
  ";",
  ",",
  ":",
  "::",
  '"',
  "'",
  "`",
  "\\",
  "/*",
  "*/",
  "//",
  "#",
  "=",
  " ",
  "\t",
  "é",
  "日",
  "٣",
  " ",
  "́",
  "🦀",
  "\\\n",
];

/** `count` documents of `mode` for `lang`, drawn at `seed` from `texts`. */
export function fuzzTexts(
  lang: string,
  mode: Mode,
  seed: number,
  count: number,
  extras: Extras,
  texts: string[],
): string[] {
  let state = (seed * 2654435761) >>> 0 || 1;
  const random = () => {
    state ^= state << 13;
    state >>>= 0;
    state ^= state >>> 17;
    state ^= state << 5;
    state >>>= 0;
    return state / 4294967296;
  };
  /** A whole number from 0 up to, but not including, `limit`. */
  const below = (limit: number) => Math.floor(random() * limit);
  const pick = <T>(choices: T[]): T => choices[below(choices.length)] as T;
  const mutations = [...MUTATIONS, ...(extras.mutations ?? [])];

  /** `text` with a few insertions, deletions, and repeated, dropped or cut lines. */
  const mutate = (text: string): string => {
    let mutated = text;
    for (let edits = 1 + below(5); edits > 0; edits--) {
      const edit = below(6);
      const at = below(mutated.length + 1);
      const lines = mutated.split("\n");
      const lineIndex = below(lines.length);
      const line = lines[lineIndex] ?? "";
      if (edit === 0) mutated = mutated.slice(0, at) + pick(mutations) + mutated.slice(at);
      else if (edit === 1) mutated = mutated.slice(0, at) + mutated.slice(at + 1 + below(4));
      else {
        if (edit === 2) lines[lineIndex] = `${line}\\`;
        else if (edit === 3) lines.splice(lineIndex, 1);
        else if (edit === 4) lines.splice(lineIndex, 0, line);
        else lines[lineIndex] = line.slice(0, below(line.length + 1));
        mutated = lines.join("\n");
      }
    }
    return mutated;
  };

  const sources = texts.map((text) => text.replaceAll("\r\n", "\n").split("\n"));

  const window = (): string => {
    const lines = pick(sources);
    const length = 8 + below(60);
    const start = below(Math.max(1, lines.length - length));
    return mutate(lines.slice(start, start + length).join("\n"));
  };

  const generated = (): string => {
    const pieces = extras.pieces ?? [];
    const lines: string[] = [];
    for (let linesLeft = 3 + below(25); linesLeft > 0; linesLeft--) {
      let line = "";
      for (let piecesLeft = below(14); piecesLeft > 0; piecesLeft--) {
        line += pick(pieces);
        if (random() < 0.4) line += " ";
      }
      lines.push(line);
    }
    return lines.join("\n");
  };

  const markdown = (): string => {
    const fences = extras.fences ?? [lang];
    const lines: string[] = [];
    for (let blocks = 1 + below(4); blocks > 0; blocks--) {
      const code = (mode === "mdg" ? generated() : window()).split("\n").slice(0, 25);
      const fence = pick(["```", "```", "~~~", "````"]);
      const info = pick(fences);
      const prefix = pick(["", "", "> ", "  ", "   "]);
      const opener = prefix === "  " ? "- item" : prefix === "   " ? "1. item" : "";
      if (opener) lines.push(opener);
      lines.push(`${prefix}${fence}${info}`, ...code.map((line) => `${prefix}${line}`));
      lines.push(random() < 0.8 ? `${prefix}${fence}` : "text after");
      lines.push("", pick(["para", "# head", "- li", ""]));
    }
    return lines.join("\n");
  };

  const inMarkdown = mode.startsWith("md");
  const documents: string[] = [];
  while (documents.length < count) {
    documents.push(inMarkdown ? markdown() : mode === "gen" ? generated() : window());
  }
  return documents;
}
