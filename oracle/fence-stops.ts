// Checks `Language::pops_fence` in src/language.rs against Shiki's grammars.
//
// A grammar whose root holds a `match` that can match the empty text can stop
// there without advancing.
// vscode-textmate then pops the innermost rule, which in a Markdown fence is
// the fence's body.
// Markdown lexes the fences of the languages `pops_fence` names so that they
// can report it.
// This script lists the grammars that Shiki's Markdown fences include and
// that can stop at their root, and fails when the list differs from the
// languages `pops_fence` names.
//
//   bun run oracle/fence-stops.ts
//
// `mise run check:rule-grammars` runs it.
import { bundledLanguages } from "shiki@4.6.0";
import { canMatchEmpty } from "./rule-grammar.ts";

type Rule = { include?: string; match?: string; begin?: string; patterns?: Rule[] };
type Grammar = Rule & { name: string; scopeName: string; repository?: Record<string, Rule> };

const languages = bundledLanguages as Record<string, () => Promise<{ default: Grammar[] }>>;

/** The grammars `name` loads: its own, and those it embeds. */
async function load(name: string): Promise<Grammar[]> {
  return (await languages[name]!()).default;
}

/** The scopes that `rule` and the rules inside it include from another
 * grammar. */
function includes(rule: unknown, into: Set<string>): void {
  if (rule === null || typeof rule !== "object") return;
  const include = (rule as Rule).include;
  if (include !== undefined && /^(source|text)\./.test(include)) into.add(include.split("#")[0]!);
  for (const value of Object.values(rule)) includes(value, into);
}

/** Whether a root rule of `grammar` is a `match` that can match the empty
 * text. It follows `$self`, `$base` and the grammar's own repository. */
function stopsAtRoot(grammar: Grammar): boolean {
  const seen = new Set<Rule>();
  const stops = (rules: Rule[] | undefined): boolean =>
    (rules ?? []).some((rule) => {
      if (seen.has(rule)) return false;
      seen.add(rule);
      if (rule.include === "$self" || rule.include === "$base") return stops(grammar.patterns);
      if (rule.include?.startsWith("#")) {
        const entry = grammar.repository?.[rule.include.slice(1)];
        if (entry === undefined) return false;
        return entry.match !== undefined || entry.begin !== undefined ? stops([entry]) : stops(entry.patterns);
      }
      if (rule.include !== undefined || rule.begin !== undefined) return false;
      return rule.match !== undefined ? canMatchEmpty(rule.match) : stops(rule.patterns);
    });
  return stops(grammar.patterns);
}

const markdown = (await load("markdown")).find((grammar) => grammar.name === "markdown")!;
const fenced = new Set<string>();
for (const [key, rule] of Object.entries(markdown.repository ?? {})) {
  if (key.startsWith("fenced_code_block")) includes(rule, fenced);
}
const grammars = new Map<string, Grammar>();
for (const name of Object.keys(languages)) {
  for (const grammar of await load(name)) grammars.set(grammar.scopeName, grammar);
}
const stopping = [...fenced]
  .map((scope) => grammars.get(scope))
  .filter((grammar): grammar is Grammar => grammar !== undefined && stopsAtRoot(grammar))
  .map((grammar) => grammar.name)
  .sort();

const source = await Bun.file(new URL("../src/language.rs", import.meta.url)).text();
const start = source.indexOf("fn pops_fence");
const body = source.slice(start, source.indexOf("\n    }\n", start));
const named = [
  ...new Set(
    [...body.matchAll(/Language::(\w+)/g)].map(([, variant]) =>
      variant!.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase(),
    ),
  ),
].sort();

if (stopping.join() !== named.join()) {
  console.error(`fence grammars that can stop at their root: ${stopping.join(", ") || "none"}`);
  console.error(`languages Language::pops_fence names: ${named.join(", ") || "none"}`);
  process.exit(1);
}
console.log(`✔ fence grammars that can stop at their root: ${stopping.join(", ")}`);
