// Writes the grammars of Shiki 4.6.0 that the prototype runs, as JSON under
// grammars/. Licence data comes from licences.ts.
import { bundledLanguages } from "shiki@4.6.0";
import { mkdirSync, writeFileSync } from "node:fs";

const root = new URL("..", import.meta.url).pathname;
mkdirSync(`${root}grammars`, { recursive: true });
const wanted = ["rust", "markdown", "typescript", "shellscript"];
for (const id of wanted) {
  const module = await bundledLanguages[id as keyof typeof bundledLanguages]();
  const registrations = module.default as unknown as Array<Record<string, unknown>>;
  registrations.forEach((grammar, index) => {
    const scope = String(grammar.scopeName);
    writeFileSync(`${root}grammars/${scope}.json`, JSON.stringify(grammar));
    console.log(id, index, scope, JSON.stringify(grammar).length);
  });
}
