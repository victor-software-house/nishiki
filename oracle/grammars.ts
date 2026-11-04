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
 * them: the C++ lexer covers `cpp-macro`, the grammar of a `#define`'s body. */
const partOf: Record<string, string> = { "cpp-macro": "cpp" };
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
export const grammars = [...new Map(lexed.map((grammar) => [grammar.name, grammar])).values()].map(
  (grammar) => ({
    ...grammar,
    embeddedLangs: grammar.embeddedLangs?.filter((name) => lexedNames.has(name)),
  }),
);
