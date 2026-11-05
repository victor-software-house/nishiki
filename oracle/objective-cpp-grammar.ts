// Writes src/objective_cpp/scope.rs, src/objective_cpp/grammar.rs and src/objective_cpp/words.rs from the `objective-cpp`
// grammar that Shiki 4.6.0 bundles (microsoft/vscode `extensions/objective-c`, MIT, from jeff-hykin/better-objcpp-syntax).
// It runs the generator of `oracle/objective-c-grammar.ts`.
//
// The grammar repeats every expression of the `objective-c` grammar.
// The Objective-C matchers answer those, so `Pat::expr` sends them to `crate::objective_c::Rx`.
// The rest, mostly from the grammar's `cpp_lang` and `cpp_lang_newish` entries, are this module's own `Rx`.
//
// The `c_lang`, `cpp_lang` and `cpp_lang_newish` entries carry repositories of their own.
// An include inside one looks in its repository first, as vscode-textmate's does.
//
// Run with `bun run oracle/objective-cpp-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import langs from "@shikijs/langs@4.6.0/objective-cpp";

import { type Config, type GrammarJson, generate, objectiveC } from "./objective-c-grammar.ts";

export const objectiveCpp: Config = {
  grammar: (langs as unknown as GrammarJson[]).find((lang) => lang.scopeName === "source.objcpp")!,
  generator: "objective-cpp-grammar.ts",
  id: "objective-cpp",
  suffix: "objcpp",
  dir: "objective_cpp",
  engine: "ObjectiveCpp",
  table: "OBJECTIVE_CPP",
  prefixes: { c_lang: "C", cpp_lang: "Cpp", cpp_lang_newish: "New" },
  // The namespace rule names capture 1 of its end, which has no group 2.
  // Shiki shows the name as written, on the bracket the end's lookahead takes.
  literal: ["keyword.control.namespace.$2.objcpp"],
  shared: { path: "crate::objective_c::Rx", ids: await generate(objectiveC, false) },
  // The end `\)\2(\3)"` repeats group 2 or group 3 of the begin, whichever took the delimiter.
  // The begin's matcher records it as group 4.
  rawDelimiter: { end: '\\)\\2(\\3)"', group: 4 },
  // Its own expressions hold no long alternation, so it writes no word table.
  words: () => [],
};

if (import.meta.main) await generate(objectiveCpp);
