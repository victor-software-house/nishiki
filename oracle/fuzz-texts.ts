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

/** What `oracle/fuzz/<lang>.ts` exports, or nothing where the file is missing. */
export async function extrasOf(lang: string): Promise<Extras> {
  return await import(`./fuzz/${lang}.ts`).catch(() => ({}));
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
  const int = (n: number) => Math.floor(random() * n);
  const pick = <T>(choices: T[]): T => choices[int(choices.length)] as T;
  const mutations = [...MUTATIONS, ...(extras.mutations ?? [])];

  /** `text` with a few insertions, deletions, and repeated, dropped or cut lines. */
  const mutate = (text: string): string => {
    let out = text;
    for (let n = 1 + int(5); n > 0; n--) {
      const op = int(6);
      const at = int(out.length + 1);
      const lines = out.split("\n");
      const k = int(lines.length);
      const line = lines[k] ?? "";
      if (op === 0) out = out.slice(0, at) + pick(mutations) + out.slice(at);
      else if (op === 1) out = out.slice(0, at) + out.slice(at + 1 + int(4));
      else {
        if (op === 2) lines[k] = `${line}\\`;
        else if (op === 3) lines.splice(k, 1);
        else if (op === 4) lines.splice(k, 0, line);
        else lines[k] = line.slice(0, int(line.length + 1));
        out = lines.join("\n");
      }
    }
    return out;
  };

  const sources = texts.map((text) => text.replaceAll("\r\n", "\n").split("\n"));

  const window = (): string => {
    const lines = pick(sources);
    const length = 8 + int(60);
    const start = int(Math.max(1, lines.length - length));
    return mutate(lines.slice(start, start + length).join("\n"));
  };

  const generated = (): string => {
    const pieces = extras.pieces ?? [];
    const out: string[] = [];
    for (let n = 3 + int(25); n > 0; n--) {
      let line = "";
      for (let k = int(14); k > 0; k--) {
        line += pick(pieces);
        if (random() < 0.4) line += " ";
      }
      out.push(line);
    }
    return out.join("\n");
  };

  const markdown = (): string => {
    const fences = extras.fences ?? [lang];
    const out: string[] = [];
    for (let blocks = 1 + int(4); blocks > 0; blocks--) {
      const code = (mode === "mdg" ? generated() : window()).split("\n").slice(0, 25);
      const fence = pick(["```", "```", "~~~", "````"]);
      const info = pick(fences);
      const prefix = pick(["", "", "> ", "  ", "   "]);
      const opener = prefix === "  " ? "- item" : prefix === "   " ? "1. item" : "";
      if (opener) out.push(opener);
      out.push(`${prefix}${fence}${info}`, ...code.map((line) => `${prefix}${line}`));
      out.push(random() < 0.8 ? `${prefix}${fence}` : "text after");
      out.push("", pick(["para", "# head", "- li", ""]));
    }
    return out.join("\n");
  };

  const md = mode.startsWith("md");
  const documents: string[] = [];
  for (let n = 0; n < count; n++) {
    documents.push(md ? markdown() : mode === "gen" ? generated() : window());
  }
  return documents;
}
