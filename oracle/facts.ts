// The fact files under oracle/facts/: what Shiki and Oniguruma answer, recorded
// as formatted JSON for the Rust writers to read.
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";

/** The directory of the fact files. */
const directory = new URL("facts/", import.meta.url).pathname;

/** The version of Shiki that `package.json` pins, which every script imports. */
const shikiVersion: string = JSON.parse(
  readFileSync(new URL("../package.json", import.meta.url).pathname, "utf8"),
).dependencies.shiki;

/** What answered, when it is Oniguruma as Shiki bundles it. */
export const bundledOniguruma = `Oniguruma, as shiki ${shikiVersion} bundles it`;

/** What answered, when it is Shiki tokenizing with its grammars. */
export function shikiGrammars(names: readonly string[]): string {
  const last = names.at(-1) ?? "";
  const listed = names.length < 2 ? last : `${names.slice(0, -1).join(", ")} and ${last}`;
  return `shiki ${shikiVersion}, tokenizing with its ${listed} grammars`;
}

/** A code point in Unicode's notation: at least four hex digits, upper case. */
export function codePointNotation(cp: number): string {
  return cp.toString(16).toUpperCase().padStart(4, "0");
}

/** A range of code points in Unicode's notation: `0041..005A`, or `00AA` alone. */
export function rangeNotation([first, last]: readonly [number, number]): string {
  if (first === last) return codePointNotation(first);
  return `${codePointNotation(first)}..${codePointNotation(last)}`;
}

/** What every fact file starts with: what answered, and the script that asked. */
interface Provenance {
  measured: string;
  script: string;
}

/** A set of code points by its key, as ranges in Unicode's notation. */
export interface SetFact {
  key: string;
  ranges: string[];
}

/** unicode-sets.json: every set the engine's `lookup` names. */
export interface UnicodeSetsFacts extends Provenance {
  sets: SetFact[];
}

/** case-folds.json: the orbits of `(?i)`, and the folds of more than one character. */
export interface CaseFoldsFacts extends Provenance {
  /** Every orbit of more than one, each ascending, sorted by its first member. */
  orbits: string[][];
  /** Every character whose fold is a string, by character, with that string's code points. */
  folds: { char: string; fold: string[] }[];
}

/** classes.json: the classes read by tokenizing, over the code points probed. */
export interface ClassesFacts extends Provenance {
  points: string[];
  sets: SetFact[];
}

/** Each fact file by its name. */
interface FactFiles {
  "unicode-sets": UnicodeSetsFacts;
  "case-folds": CaseFoldsFacts;
  classes: ClassesFacts;
}

/** Writes `facts` to `oracle/facts/<name>.json`, indented two spaces. */
export function writeFacts<Name extends keyof FactFiles>(name: Name, facts: FactFiles[Name]): void {
  mkdirSync(directory, { recursive: true });
  writeFileSync(`${directory}${name}.json`, `${JSON.stringify(facts, null, 2)}\n`);
}
