// Counts the grammars of Shiki 4.6.0 that use `injections` or `injectionSelector`,
// and the patterns of all grammars that use backreferences in end/while.
import { bundledLanguages } from "shiki@4.6.0";

const seen = new Set<string>();
let total = 0, withInj = 0, withSel = 0, bytes = 0;
const names: string[] = [];
for (const [id, load] of Object.entries(bundledLanguages)) {
  const mod = await (load as () => Promise<{ default: unknown }>)();
  for (const g of mod.default as Array<Record<string, unknown>>) {
    const scope = String(g.scopeName);
    if (seen.has(scope)) continue;
    seen.add(scope);
    total++;
    const text = JSON.stringify(g);
    bytes += text.length;
    if (g.injections) { withInj++; names.push(scope); }
    if (g.injectionSelector) withSel++;
  }
}
console.log({ grammars: total, withInjections: withInj, withInjectionSelector: withSel, jsonBytes: bytes });
console.log(names.join(" "));
