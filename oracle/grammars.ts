// The grammars the oracle loads into Shiki, shared by the scripts that
// tokenize its inputs, so every script sees the same embeddings.
import { bundledLanguages } from "shiki@4.6.0";
import { inputs } from "./inputs.ts";

/** Each input language's grammar, and the grammars it embeds that are input
 * languages too or that nishiki lexes as part of one. A grammar for a
 * language nishiki does not lex is left out, so a fence or embedding in that
 * language stays plain, as nishiki leaves it, whichever input language's
 * grammar would pull it in. */
const inputLangs = new Set(inputs.map((input) => input.lang));
/** Grammars that nishiki lexes as part of the input language that embeds
 * them: the C++ lexer covers `cpp-macro`, the grammar of a `#define`'s body,
 * and the Markdown lexer covers `markdown-vue`, Vue's fence in Markdown. */
const partOf: Record<string, string> = { "cpp-macro": "cpp", "markdown-vue": "vue" };
const loaded = (
  await Promise.all([...inputLangs].map(async (lang) => (await bundledLanguages[lang]()).default))
).flat();
const lexed = loaded.filter(
  (grammar) =>
    inputLangs.has(grammar.name) ||
    grammar.aliases?.some((alias) => inputLangs.has(alias)) ||
    inputLangs.has(partOf[grammar.name] ?? ""),
);
const lexedNames = new Set(lexed.map((grammar) => grammar.name));
/** The scopes of the grammars left out. No rule can include them here. */
const leftOut = new Set(
  loaded.filter((grammar) => !lexedNames.has(grammar.name)).map((grammar) => grammar.scopeName),
);
type Rule = { include?: string; patterns?: Rule[]; repository?: Record<string, Rule> };
/** The oracle's one grammar edit.
 * C++'s raw strings `R"re(`, `R"_r(`, `R"regex(`, `R"glsl(` and `R"GLSL(`
 * have rules whose only pattern includes the regex or GLSL grammar.
 * nishiki does not lex those grammars.
 * vscode-textmate skips a rule whose patterns include only missing grammars.
 * With those grammars left out, it would read these strings as other raw
 * strings.
 * Shiki with its bundled grammars keeps the rules.
 * nishiki lexes them with their rules and plain text inside, as the C++
 * variant's documentation says.
 * So in C++'s `string_context`, an include of a grammar left out becomes no
 * pattern at all. */
function keepRawStringRules(grammar: Rule & { name: string }): Rule & { name: string } {
  const context = grammar.repository?.string_context;
  if (!(grammar.name === "cpp" || grammar.name === "cpp-macro") || context === undefined) return grammar;
  const patterns = context.patterns?.map((rule) =>
    rule.patterns?.length && rule.patterns.every((inner) => leftOut.has(inner.include ?? ""))
      ? { ...rule, patterns: [] }
      : rule,
  );
  return { ...grammar, repository: { ...grammar.repository, string_context: { ...context, patterns } } };
}
export const grammars = [...new Map(lexed.map((grammar) => [grammar.name, grammar])).values()].map(
  (grammar) => ({
    ...keepRawStringRules(grammar),
    embeddedLangs: grammar.embeddedLangs?.filter((name) => lexedNames.has(name)),
  }),
);
