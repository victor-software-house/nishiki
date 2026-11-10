// Writes what the Shiki fidelity check compares, for every language Shiki
// bundles.
//
// Usage: bun oracle/fidelity.ts <out dir> [<language id>...]
//
// With no language id, it writes every language in Shiki's
// `bundledLanguagesInfo`. The out directory then holds:
// - `grammars/<name>.json`: each grammar Shiki loads, as Shiki ships it;
// - `samples/<id>.txt`: Shiki's scopes for the language's sample, in the
//   fixture format of `fixture.ts`;
// - `tables/<id>.json`: the engine's tables, from `engine-tables.ts --json`;
// - `languages.json`: per language, its root scope, the grammars it loads,
//   whether Shiki has a sample for it, and why its tables failed, if they did.
//
// Each language gets a highlighter of its own, loaded with its grammar and
// the grammars it embeds, as `createHighlighter({ langs: [id] })` loads them.
// The engine's tables load the same set.
//
// Nothing it writes is committed. Grammars whose licence is unknown are read
// here for testing only, as invariant 2 allows.
import { bundledLanguages, bundledLanguagesInfo, createHighlighter } from "shiki@4.6.0";
import { mkdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { fixture } from "./fixture.ts";
import { samples } from "./inputs.ts";

/** One language's entry in `languages.json`. */
type Language = {
  id: string;
  scope: string;
  grammars: string[];
  sample: boolean;
  /** Why `engine-tables.ts` failed, or null when it wrote the tables. */
  tables: string | null;
};

const [out, ...only] = process.argv.slice(2);
if (!out) throw new Error("usage: fidelity.ts <out dir> [<language id>...]");
for (const dir of ["grammars", "samples", "tables"]) mkdirSync(`${out}/${dir}`, { recursive: true });

const ids = only.length > 0 ? only : bundledLanguagesInfo.map((info) => info.id);
const engineTables = fileURLToPath(new URL("engine-tables.ts", import.meta.url));

/** The sample's text with LF line ends, or null when Shiki has none. */
async function sample(id: string): Promise<string | null> {
  const response = await fetch(`${samples}/${id}.sample`);
  if (response.status === 404) return null;
  if (!response.ok) throw new Error(`${id}.sample: HTTP ${response.status}`);
  return (await response.text()).replaceAll("\r\n", "\n");
}

/** Writes the engine's tables for `id`, and returns why they failed, or null. */
async function tables(id: string): Promise<string | null> {
  const child = Bun.spawn([process.execPath, engineTables, "--json", `${out}/tables`, id], {
    stdout: "ignore",
    stderr: "pipe",
  });
  const [code, stderr] = await Promise.all([child.exited, new Response(child.stderr).text()]);
  if (code === 0) return null;
  const error = stderr.split("\n").find((line) => line.startsWith("error:") || line.includes("Error:"));
  return (error ?? stderr.trim().split("\n").at(-1) ?? `exit ${code}`).trim();
}

const languages: Language[] = [];
for (const id of ids) {
  const loader = bundledLanguages[id as keyof typeof bundledLanguages];
  if (!loader) throw new Error(`Shiki has no language ${id}`);
  const registrations = (await loader()).default;
  for (const grammar of registrations) {
    await Bun.write(`${out}/grammars/${grammar.name}.json`, JSON.stringify(grammar));
  }
  const text = await sample(id);
  if (text !== null) {
    const highlighter = await createHighlighter({ langs: [id], themes: ["github-dark"] });
    await Bun.write(`${out}/samples/${id}.txt`, fixture(highlighter, text, id));
    highlighter.dispose();
  }
  const failure = await tables(id);
  languages.push({
    id,
    scope: String(registrations[registrations.length - 1].scopeName),
    grammars: registrations.map((grammar) => grammar.name),
    sample: text !== null,
    tables: failure,
  });
  console.log(`${failure === null ? "✔" : "✘"} ${id}${text === null ? ": no sample" : ""}${failure ? `: ${failure}` : ""}`);
}
await Bun.write(`${out}/languages.json`, `${JSON.stringify(languages, null, 2)}\n`);
