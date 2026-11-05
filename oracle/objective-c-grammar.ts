// Writes src/objective_c/scope.rs, src/objective_c/grammar.rs and src/objective_c/words.rs from the `objective-c` grammar that
// Shiki 4.6.0 bundles (microsoft/vscode `extensions/objective-c`, MIT, from jeff-hykin/better-objc-syntax).
// It runs the generator in `oracle/rule-generator.ts`.
// The matchers that decide whether a pattern matches are written by hand in src/objective_c.
//
// The grammar's `c_lang` entry carries a repository of its own.
// Where Markdown embeds Objective-C, the engine resolves `$base` to the grammar's root, as the generator does.
//
// Run with `bun run oracle/objective-c-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import langs from "@shikijs/langs@4.6.0/objective-c";

import { type Config, type GrammarJson, generate, type WordList } from "./rule-generator.ts";

/** The `objective-c` grammar Shiki bundles. */
export const objectiveC: Config = {
  grammar: (langs as unknown as GrammarJson[]).find((lang) => lang.scopeName === "source.objc")!,
  generator: "objective-c-grammar.ts",
  id: "objective-c",
  suffix: "objc",
  dir: "objective_c",
  engine: "ObjectiveC",
  table: "OBJECTIVE_C",
  prefixes: { c_lang: "C" },
  literal: [],
  words: ({ between, inside }) => {
    const lists: WordList[] = [
      { name: "LEOPARD_FUNCTION", doc: "The Cocoa functions Leopard added, of `AnonymousPattern18`.", body: inside("AnonymousPattern18", "(\\s*)\\b(", ")\\b") },
      { name: "FUNCTION", doc: "The Cocoa functions of `AnonymousPattern19`.", body: inside("AnonymousPattern19", "(\\s*)\\b(", ")\\b") },
      { name: "LEOPARD_CLASS", doc: "The Cocoa classes Leopard added, of `AnonymousPattern20`.", body: inside("AnonymousPattern20", "\\b", "\\b") },
      { name: "CLASS", doc: "The Cocoa classes of `AnonymousPattern21`.", body: inside("AnonymousPattern21", "\\b", "\\b") },
      { name: "LEOPARD_TYPE", doc: "The Cocoa types Leopard added, of `AnonymousPattern22`.", body: inside("AnonymousPattern22", "\\b", "\\b") },
      { name: "QUARTZ_CLASS", doc: "The Quartz classes of `AnonymousPattern23`.", body: inside("AnonymousPattern23", "\\b", "\\b") },
      { name: "QUARTZ_TYPE", doc: "The Quartz types of `AnonymousPattern24`.", body: inside("AnonymousPattern24", "\\b", "\\b") },
      { name: "TYPE", doc: "The Cocoa types of `AnonymousPattern25`.", body: inside("AnonymousPattern25", "\\b", "\\b") },
      { name: "CONSTANT", doc: "The Cocoa constants of `AnonymousPattern26`.", body: inside("AnonymousPattern26", "\\b", "\\b") },
      { name: "LEOPARD_NOTIFICATION", doc: "The notifications Leopard added, of `AnonymousPattern27`.", body: inside("AnonymousPattern27", "\\b", "\\b") },
      { name: "NOTIFICATION", doc: "The notifications of `AnonymousPattern28`.", body: inside("AnonymousPattern28", "\\b", "\\b") },
      { name: "LEOPARD_CONSTANT", doc: "The Cocoa constants Leopard added, of `AnonymousPattern29`.", body: inside("AnonymousPattern29", "\\b", "\\b") },
      { name: "COCOA_CONSTANT", doc: "The Cocoa constants of `AnonymousPattern30`.", body: inside("AnonymousPattern30", "\\b", "\\b") },
      { name: "PROTOCOL", doc: "The Cocoa protocols of `ProtocolListAt0`.", body: inside("ProtocolListAt0", "\\b", "\\b") },
      { name: "C99_FUNCTION", doc: "The C library functions of `CFunctionsAt0`.", body: inside("CFunctionsAt0", "(\\s*)\\b(", ")\\b") },
      { name: "MAC_TYPE", doc: "The Mac Classic types of `CLangAt29`.", body: inside("CLangAt29", "\\b(", ")\\b") },
      { name: "STORAGE", doc: "The storage types of `CStorageTypesAt1`.", body: inside("CStorageTypesAt1", "(?-im:(?<!\\w)(?:", ")(?!\\w))") },
      { name: "MEMBER_EXCLUDED", doc: "The words a member name of `CMemberAccess` may not start with.", body: between("CMemberAccess", "\\b((?!", ")[A-Z_a-z]") },
      { name: "CALL_EXCLUDED", doc: "The words before `(` that are no function call to `CLangAt33`.", body: between("CLangAt33", "(?!\\s*(?:", ")\\s*\\()") },
      { name: "DEFINE_EXCLUDED", doc: "The words before `(` that are no call to `CPreprocessorRuleDefineLineContentsAt4`.", body: between("CPreprocessorRuleDefineLineContentsAt4", "(?!(?:", ")\\s*\\()") },
      { name: "FUNCTION_EXCLUDED", doc: "The words before `(` that are no call to `CCFunctionCall`, `CFunctionCallInnardsAt5` and `CBlockInnardsAt6`.", body: between("CCFunctionCall", "(?!(?:", ")\\s*\\()") },
    ];
    for (const id of ["CFunctionCallInnardsAt5"]) {
      if (between(id, "(?!(?:", ")\\s*\\()") !== lists.at(-1)!.body) throw new Error(`${id}: another excluded list`);
    }
    return lists;
  },
};

if (import.meta.main) await generate(objectiveC);
