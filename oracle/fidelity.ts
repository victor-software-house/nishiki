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
// A language's input is Shiki's sample for it.
// `standIns` in inputs.ts names an input for each language Shiki has no
// sample for.
// `extraInputs` adds inputs that reach rules the samples miss.
// Every third-party input is read from the mirror, through sources.ts.
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
import { samples, standIns } from "./inputs.ts";
import { sourceText } from "./sources.ts";

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

const ids = only.length > 0 ? only : bundledLanguagesInfo.map((language) => language.id);
const repository = fileURLToPath(new URL("..", import.meta.url));

/** Each input to check: its language, its file name in `samples/`, and its
 * URL or path. */
const checkedInputs = ids.flatMap((id) => [
  { id, sample: id, input: standIns[id] ?? `${samples}/${id}.sample` },
  ...(extraInputs[id] ?? []).map((path) => ({
    id,
    sample: path
      .split("/")
      .at(-1)!
      .replace(/\.[^.]*$/, ""),
    input: path,
  })),
]);

/** The text at `input`, a URL read from the mirror or a path in this
 * repository, with CRLF read as LF, as Shiki splits lines. */
async function inputText(input: string): Promise<string> {
  const text = input.startsWith("https://")
    ? await sourceText(input)
    : await Bun.file(`${repository}${input}`).text();
  return text.replaceAll("\r\n", "\n");
}

const checked: CheckedInput[] = [];
for (const { id, sample, input } of checkedInputs) {
  const text = await inputText(input);
  const loader = bundledLanguages[id as keyof typeof bundledLanguages];
  if (!loader) throw new Error(`Shiki has no language ${id}`);
  const registrations = (await loader()).default;
  for (const grammar of registrations) {
    await Bun.write(`${out}/grammars/${grammar.name}.json`, JSON.stringify(grammar));
  }
  const highlighter = await createHighlighter({ langs: [id], themes: ["github-dark"] });
  await Bun.write(`${out}/samples/${sample}.txt`, fixture(highlighter, text, id));
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
