// Counts the regular-expression features each grammar uses.
import { readdirSync, readFileSync } from "node:fs";
const root = new URL("..", import.meta.url).pathname;
const features: Record<string, RegExp> = {
  lookahead: /\(\?[=!]/,
  lookbehind: /\(\?<[=!]/,
  backref: /\\[1-9]|\\k</,
  subexp_call: /\\g</,
  atomic: /\(\?>/,
  possessive: /(?:[*+?]|\})\+/,
  anchor_G: /\\G/,
  anchor_A: /\\A/,
  hex_h: /\\h/,
  unicode_prop: /\\p\{/i,
  named_group: /\(\?<[A-Za-z_]/,
  inline_flags: /\(\?[a-zA-Z-]+[:)]/,
  extended_x: /\(\?[a-z]*x[a-z]*[:)]/,
  conditional: /\(\?\(/,
  absence: /\(\?~/,
  keep_K: /\\K/,
  posix_class: /\[:[a-z]+:\]/,
  class_intersection: /&&/,
  eol_dollar: /\$/,
  lazy: /[*+?}]\?/,
};
const out: string[] = [];
const total: Record<string, number> = {};
for (const file of readdirSync(`${root}grammars`)) {
  const g = JSON.parse(readFileSync(`${root}grammars/${file}`, "utf8"));
  const patterns = new Set<string>();
  const walk = (node: unknown) => {
    if (Array.isArray(node)) return node.forEach(walk);
    if (node && typeof node === "object") {
      for (const [k, v] of Object.entries(node)) {
        if (["match", "begin", "end", "while"].includes(k) && typeof v === "string") patterns.add(v);
        else walk(v);
      }
    }
  };
  walk(g);
  const counts: Record<string, number> = {};
  for (const p of patterns) for (const [name, re] of Object.entries(features)) if (re.test(p)) { counts[name] = (counts[name] ?? 0) + 1; total[name] = (total[name] ?? 0) + 1; }
  out.push(`${file}: ${patterns.size} patterns; ${Object.entries(counts).map(([k, v]) => `${k}=${v}`).join(" ")}`);
}
console.log(out.join("\n"));
console.log("TOTAL", JSON.stringify(total));
