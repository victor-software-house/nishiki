// Writes what the Shiki fidelity check compares, for every language Shiki
// bundles.
//
// Usage: bun oracle/fidelity.ts <out dir> [<language id>...]
//
// With no language id, it writes every language in Shiki's
// `bundledLanguagesInfo`. The out directory then holds:
// - `grammars/<name>.json`: each grammar Shiki loads, as Shiki ships it;
// - `samples/<sample>.txt`: Shiki's scopes for each input, in the fixture
//   format of `fixture.ts`;
// - `languages.json`: per input, its language, its root scope, the grammars it
//   loads, and where the input came from.
//
// A language's input is Shiki's sample for it. `standIns` names an input for
// each language Shiki has no sample for. `extraInputs` adds inputs that reach
// rules the samples miss.
//
// Each language gets a highlighter of its own, loaded with its grammar and
// the grammars it embeds, as `createHighlighter({ langs: [id] })` loads them.
// The fidelity check loads the same set through nishiki's public API.
//
// Nothing it writes is committed. Grammars whose licence is unknown are read
// here for testing only, as invariant 2 allows.
import { bundledLanguages, bundledLanguagesInfo, createHighlighter } from "shiki@4.6.0";
import { mkdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { fixture } from "./fixture.ts";
import { inputs, samples } from "./inputs.ts";

/** One input's entry in `languages.json`. */
type CheckedInput = {
  id: string;
  /** The name of its file in `samples/`. */
  sample: string;
  scope: string;
  grammars: string[];
  /** The URL of its input, or its path in this repository. */
  input: string;
};

const vscodeFixtures =
  "https://raw.githubusercontent.com/microsoft/vscode/d5b3834b4da716541a3a5cb8483cf3675cbdd29f/extensions/vscode-colorize-tests/test/colorize-fixtures";
const h5bpIndex = inputs.find((input) => input.lang === "html" && input.name === "h5bp-index");
if (h5bpIndex === undefined || !("url" in h5bpIndex)) throw new Error("inputs.ts has no html/h5bp-index");

/** The input of each language Shiki 4.6.0 has no sample for.
 * `tests/oracle/SOURCES.md` records their licences. */
const standIns: Record<string, string> = {
  "git-commit": `${vscodeFixtures}/COMMIT_EDITMSG`,
  "git-rebase": `${vscodeFixtures}/git-rebase-todo`,
  "html-derivative": h5bpIndex.url,
  "nextflow-groovy": `${vscodeFixtures}/test.groovy`,
};

/** Inputs beyond each language's own, by language.
 * Each is a path in this repository.
 * - `csharp-coverage.cs` holds C# documentation comments that open with `/**`, which Shiki's
 *   C# sample lacks. Their rule needs the absent operator `(?~…)`. */
const extraInputs: Record<string, string[]> = {
  csharp: ["oracle/inputs/csharp-coverage.cs"],
};

const [out, ...only] = process.argv.slice(2);
if (!out) throw new Error("usage: fidelity.ts <out dir> [<language id>...]");
for (const dir of ["grammars", "samples"]) mkdirSync(`${out}/${dir}`, { recursive: true });

const ids = only.length > 0 ? only : bundledLanguagesInfo.map((info) => info.id);
const repository = fileURLToPath(new URL("..", import.meta.url));

/** Each input to check: its language, its file name in `samples/`, and its
 * URL or path. */
const checkedInputs = ids.flatMap((id) => [
  { id, sample: id, input: standIns[id] ?? `${samples}/${id}.sample` },
  ...(extraInputs[id] ?? []).map((path) => ({
    id,
    sample: path.split("/").at(-1)!.replace(/\.[^.]*$/, ""),
    input: path,
  })),
]);

/** The text at `input`, a URL or a path in this repository, with CRLF read as
 * LF, as Shiki splits lines. */
async function fetchText(input: string): Promise<string> {
  if (!input.startsWith("https://")) return (await Bun.file(`${repository}${input}`).text()).replaceAll("\r\n", "\n");
  const response = await fetch(input);
  if (!response.ok) throw new Error(`${input}: HTTP ${response.status}`);
  return (await response.text()).replaceAll("\r\n", "\n");
}

/** Runs `task` on every item, `width` at a time, and keeps the order. */
async function pool<T, R>(items: T[], width: number, task: (item: T) => Promise<R>): Promise<R[]> {
  const results: R[] = new Array(items.length);
  let next = 0;
  const worker = async () => {
    while (next < items.length) {
      const at = next++;
      results[at] = await task(items[at]);
    }
  };
  await Promise.all(Array.from({ length: width }, worker));
  return results;
}

const texts = await pool(checkedInputs, 16, (checkedInput) => fetchText(checkedInput.input));
const checked: CheckedInput[] = [];
for (const [at, { id, sample, input }] of checkedInputs.entries()) {
  const loader = bundledLanguages[id as keyof typeof bundledLanguages];
  if (!loader) throw new Error(`Shiki has no language ${id}`);
  const registrations = (await loader()).default;
  for (const grammar of registrations) {
    await Bun.write(`${out}/grammars/${grammar.name}.json`, JSON.stringify(grammar));
  }
  const highlighter = await createHighlighter({ langs: [id], themes: ["github-dark"] });
  await Bun.write(`${out}/samples/${sample}.txt`, fixture(highlighter, texts[at], id));
  highlighter.dispose();
  checked.push({
    id,
    sample,
    scope: String(registrations[registrations.length - 1].scopeName),
    grammars: registrations.map((grammar) => grammar.name),
    input,
  });
  console.log(`✔ ${sample}`);
}
await Bun.write(`${out}/languages.json`, `${JSON.stringify(checked, null, 2)}\n`);
