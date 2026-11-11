// Measures the characters that Oniguruma's `\d`, `\w`, `\s`, `[[:alpha:]]` and `\p{upper}`
// match beyond ASCII, as Shiki 4.6.0 uses them in the `json`, `typescript` and `python`
// grammars.
// Rust's `char` methods differ from them (`\d` is only the decimal digits, `\w` includes marks,
// `[[:alpha:]]` follows an older Unicode), so the sets are measured rather than derived:
// one line per character, tokenized by the grammar, and read back from its scopes.
//
//   `[1X]`     X is a digit when `1X` is one number token (`\d`);
//   `[trueX]`  X is a word character when `true` is not a constant (`\w`);
//   `[X]`      X is white space when no invalid scope covers it (`\s`);
//   `let Xa;`  X is a letter when it starts the variable's name (`[[:alpha:]]`,
//              in `typescript`; the `a` keeps a capital from naming a constant);
//   `XX`       X is a capital when the pair is a constant name (`\p{upper}`, in
//              `python`).
//
// It records those five sets and the points it probed in oracle/facts/classes.json.
// codegen/src/classes.rs writes src/classes/table.rs from them, and from the
// Unicode categories that oracle/unicode-tables.ts records, limited to the same points.
import { createHighlighter } from "shiki@4.6.0";
import { type ClassesFacts, rangeNotation, shikiGrammars, writeFacts } from "./facts.ts";

const highlighter = await createHighlighter({
  langs: ["json", "typescript", "python"],
  themes: ["github-dark"],
});
const enc = new TextEncoder();

/** The code points that can be letters, marks, digits or spaces: U+0080 to
 * the end of plane 3, where the last letters are, and the variation selectors
 * of plane 14, which are marks. No other plane has any. */
const points: number[] = [];
for (let cp = 0x80; cp < 0x34000; cp++) {
  if (cp < 0xd800 || cp > 0xdfff) points.push(cp);
}
for (let cp = 0xe0000; cp < 0xe0200; cp++) points.push(cp);

/** The tokens' scope names per line, with byte ends. */
function probe(make: (ch: string) => string, lang = "json") {
  const text = points.map((cp) => make(String.fromCodePoint(cp))).join("\n");
  const lines = highlighter.codeToTokens(text, {
    lang,
    theme: "github-dark",
    includeExplanation: "scopeName",
    tokenizeMaxLineLength: 0,
    tokenizeTimeLimit: 0,
  }).tokens;
  return lines.map((tokens) => {
    const parts: { end: number; scopes: string[] }[] = [];
    let end = 0;
    for (const token of tokens) {
      for (const part of token.explanation ?? []) {
        end += enc.encode(part.content).length;
        parts.push({ end, scopes: part.scopes.map((s) => s.scopeName) });
      }
    }
    return parts;
  });
}

/** The scopes of the token that covers byte `at`. */
function at(parts: { end: number; scopes: string[] }[], byte: number) {
  return parts.find((part) => part.end > byte)?.scopes ?? [];
}

const digits = probe((ch) => `[1${ch}]`).map((parts) =>
  at(parts, 2).includes("constant.numeric.json"),
);
const words = probe((ch) => `[true${ch}]`).map(
  (parts) => !at(parts, 1).includes("constant.language.json"),
);
const spaces = probe((ch) => `[${ch}]`).map(
  (parts, i) =>
    !at(parts, 1).some((scope) => scope.startsWith("invalid.")) && points[i] !== undefined,
);

const alphas = probe((ch) => `let ${ch}a;`, "typescript").map((parts) =>
  at(parts, 4).includes("variable.other.readwrite.ts"),
);
const uppers = probe((ch) => `${ch}${ch}`, "python").map((parts) =>
  parts.some((part) => part.scopes.includes("constant.other.caps.python")),
);

function ranges(flags: boolean[]): [number, number][] {
  const out: [number, number][] = [];
  flags.forEach((on, i) => {
    if (!on) return;
    const cp = points[i]!;
    const last = out[out.length - 1];
    if (last && last[1] + 1 === cp) last[1] = cp;
    else out.push([cp, cp]);
  });
  return out;
}

/** The facts classes.json records: the five sets read by tokenizing, and the points probed. */
const facts: ClassesFacts = {
  measured: shikiGrammars(["json", "typescript", "python"]),
  script: "oracle/classes.ts",
  points: ranges(points.map(() => true)).map(rangeNotation),
  sets: Object.entries({ digits, words, spaces, alphas, uppers }).map(([key, flags]) => ({
    key,
    ranges: ranges(flags).map(rangeNotation),
  })),
};
writeFacts("classes", facts);
console.log([digits, words, spaces, alphas, uppers].map((flags) => ranges(flags).length).join(" "));
