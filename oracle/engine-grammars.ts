// Writes tests/engine/grammars/: the Shiki grammars the engine's tests link
// into tables, with what each table loads and where each grammar comes from.
//
// Usage: bun oracle/engine-grammars.ts
//
// It writes:
// - `<name>.json`: each grammar, as Shiki 4.6.0 ships it, indented two spaces;
// - `tables.json`: per table, its root scope, the grammars it links and the
//   languages it hands regions to;
// - `SOURCES.md`: each grammar's source, commit and licence.
//
// It also writes the grammars that `codegen/` reads and no table links: the
// grammars it reads word lists and matchers from, and the grammars of Markdown's fences.
//
// A table is a Shiki language id, with the languages it may embed joined by
// `+`, then the languages it hands regions to, each after `>`. A table loads
// the grammars Shiki registers for each language joined by `+`. A handed
// language's grammar is loaded too, without the grammars it registers. Its
// rules then serve includes of one of its rules, such as HAML's
// `source.ruby#escaped_char`.
//
// A grammar that `oracle/licences.ts` refuses stays out of the repository,
// tests included. A handed language's refused grammar is left out, so an
// include of one of its rules matches nothing. The run fails on any other
// grammar that `oracle/licences.ts` refuses or lacks. A table or a word list
// reads only grammars that `allowed` lists: a grammar in `testData` may be
// committed for a check, never compiled into a table the crate ships.
import { bundledLanguages } from "shiki@4.6.0";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { allowed, refused, testData } from "./licences.ts";

/** One registered grammar, as Shiki ships it. */
type ShikiGrammar = { name: string; scopeName: string } & Record<string, unknown>;

/** What a table links, as `tables.json` records it. */
type Table = {
  /** The table's name: its first language. */
  name: string;
  /** The root scope of the first language's grammar. */
  root: string;
  /** The grammars it links, by name. */
  grammars: string[];
  /** The languages it hands regions to, by Shiki id. */
  handOffs: string[];
};

/** The tables the engine's tests run. */
const tables = [
  "asm",
  "common-lisp",
  "elixir>html",
  "gdscript",
  "graphql",
  "haml>ruby>javascript>css>markdown>sass>coffee",
  "html>javascript>css",
  "javascript",
  "jinja",
  "make",
  [
    "markdown",
    "typescript",
    "tsx",
    "javascript",
    "shellscript",
    "python",
    "rust",
    "yaml",
    "json",
    "jsonc",
    "jsonl",
    "make",
    "html-derivative",
    "html",
    "css",
  ].join("+"),
  "matlab",
  "objective-c",
  "objective-cpp",
  "python",
  "rust",
  "shellscript",
  "tex>r",
  "toml",
  "typescript",
  "vb",
  "vue>css>javascript>typescript>json>html>html-derivative",
  "yaml",
];

const out = fileURLToPath(new URL("../tests/engine/grammars/", import.meta.url));

/** The grammars Shiki registers for `id`, the language's own last. */
async function registrations(id: string): Promise<ShikiGrammar[]> {
  const loader = bundledLanguages[id as keyof typeof bundledLanguages];
  if (!loader) throw new Error(`Shiki has no language ${id}`);
  return ((await loader()) as { default: ShikiGrammar[] }).default;
}

const grammars = new Map<string, ShikiGrammar>();
const linked: Table[] = [];
for (const spec of tables) {
  const [embedding, ...handOffs] = spec.split(">");
  const ids = embedding.split("+");
  const names: string[] = [];
  const add = (grammar: ShikiGrammar) => {
    if (!names.includes(grammar.name)) names.push(grammar.name);
    grammars.set(grammar.name, grammar);
  };
  let root = "";
  for (const id of ids) {
    const registered = await registrations(id);
    for (const grammar of registered) {
      if (!handOffs.includes(grammar.name)) add(grammar);
    }
    if (root === "") root = registered[registered.length - 1].scopeName;
  }
  for (const id of handOffs) {
    const own = (await registrations(id)).find((grammar) => grammar.name === id);
    if (!own) throw new Error(`${spec}: Shiki registers no grammar named ${id}`);
    if (!refused[id]) add(own);
  }
  linked.push({ name: ids[0], root, grammars: names, handOffs });
}

/**
 * The grammars `codegen/` reads that no table links: word lists, matchers and
 * Svelte's injections into CoffeeScript.
 */
const codegenGrammars = ["c", "less", "proto", "sql", "svelte"];
/**
 * The grammars of Markdown's fences that no table links, which
 * `codegen/src/fence_stops.rs` checks.
 * `groovy` stays out: `oracle/licences.ts` refuses it.
 * `r` is not listed: TeX's table hands regions to it, so it is written already.
 */
const fenceGrammars = [
  "abap",
  "bat",
  "bibtex",
  "clojure",
  "cpp",
  "csharp",
  "dart",
  "diff",
  "docker",
  "erlang",
  "fsharp",
  "git-commit",
  "git-rebase",
  "go",
  "handlebars",
  "ini",
  "java",
  "julia",
  "latex",
  "log",
  "lua",
  "perl",
  "php",
  "powershell",
  "pug",
  "raku",
  "regexp",
  "rst",
  "scala",
  "scss",
  "swift",
  "xml",
  "xsl",
];
for (const id of [...codegenGrammars, ...fenceGrammars]) {
  const own = (await registrations(id)).find((grammar) => grammar.name === id);
  if (!own) throw new Error(`Shiki registers no grammar named ${id}`);
  grammars.set(own.name, own);
}

const both = Object.keys(testData).filter((name) => allowed[name]);
if (both.length > 0) {
  throw new Error(`both allowed and testData in oracle/licences.ts: ${both.sort().join(", ")}`);
}
/** The grammars a table links or `codegen/` reads. */
const tabled = [...linked.flatMap((table) => table.grammars), ...codegenGrammars];
const tabledTestData = tabled.filter((name) => testData[name]);
if (tabledTestData.length > 0) {
  throw new Error(
    `a table or word list reads testData grammars: ${tabledTestData.sort().join(", ")}`,
  );
}
const unlicensed = [...grammars.keys()].filter(
  (name) => refused[name] || !(allowed[name] ?? testData[name]),
);
if (unlicensed.length > 0) {
  throw new Error(`not allowed in oracle/licences.ts: ${unlicensed.sort().join(", ")}`);
}
/** The licence record of a committed grammar. */
const licensed = (name: string) => allowed[name] ?? testData[name];

rmSync(out, { recursive: true, force: true });
mkdirSync(out, { recursive: true });
const names = [...grammars.keys()].sort();
for (const name of names) {
  writeFileSync(`${out}${name}.json`, `${JSON.stringify(grammars.get(name), null, 2)}\n`);
}
writeFileSync(`${out}tables.json`, `${JSON.stringify(linked, null, 2)}\n`);

const rows = names.map((name) => {
  const { bundled, licence } = licensed(name);
  return (
    `| \`${name}\` | [${bundled.replace("@", " at ")}][${name}-source] | ` +
    `[${licence}][${name}-licence] |`
  );
});
const links = names.flatMap((name) => {
  const { bundled, origin, file } = licensed(name);
  const [repository, commit] = bundled.split("@");
  const licenceCommit = origin === repository ? commit : "HEAD";
  return [
    `[${name}-source]: https://github.com/${repository}/tree/${commit}`,
    `[${name}-licence]: https://github.com/${origin}/blob/${licenceCommit}/${file}`,
  ];
});
writeFileSync(
  `${out}SOURCES.md`,
  [
    "# Engine test grammars",
    "",
    "`oracle/engine-grammars.ts` writes this directory; nobody edits it by hand.",
    "Each grammar is the file Shiki 4.6.0 ships, taken from the repository and commit below.",
    "The grammars are test data: `Cargo.toml` leaves `tests/` out of the published crate.",
    "",
    "| Grammar | Source | Licence |",
    "|:--|:--|:--|",
    ...rows,
    "",
    ...links,
    "",
  ].join("\n"),
);
console.log(`✔ ${names.length} grammars for ${linked.length} tables`);
