// Writes src/classes/table.rs: the characters that Oniguruma's `\d`, `\w`, `\s`, `[[:alpha:]]`,
// `\p{upper}`, `\p{L}`, `\p{N}` and `\p{M}` match, as Shiki 4.6.0 uses them in the `json`,
// `typescript` and `python` grammars.
// Rust's `char` methods differ from them (`\d` is only the decimal digits, `\w` includes marks,
// `[[:alpha:]]` follows an older Unicode), so the table is measured rather than derived:
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
// It records those five sets in oracle/facts/classes.json, for a Rust writer.
//
// No bundled grammar reads `\p{L}`, `\p{N}` or `\p{M}` alone,
// so those three are read from Shiki's own Oniguruma engine:
// one scan of a text of every code point for each, with the pattern `\p{X}+`.
// So are the three unions the `scala` grammar's names and operators read: `\p{Lt}\p{Lu}`,
// `\p{Lo}\p{Nl}\p{Ll}` and `\p{Sm}\p{So}`.
// So are the two the `haskell` grammar reads beside `\p{Lt}\p{Lu}`: `\p{Ll}` and `\p{S}\p{P}`.
// So are the three the `r` grammar's identifiers read beside `\p{L}` and `\d`: `\p{Nl}`,
// `\p{Mn}\p{Mc}` and `\p{Pc}`.
import { createOnigurumaEngine } from "@shikijs/engine-oniguruma@4.6.0";
import { createHighlighter } from "shiki@4.6.0";
import { dedent } from "./dedent.ts";
import { type ClassesFacts, rangeNotation, shikiGrammars, writeFacts } from "./facts.ts";

const root = new URL("..", import.meta.url).pathname;
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

const engine = await createOnigurumaEngine(import("@shikijs/engine-oniguruma@4.6.0/wasm-inlined"));

/** Whether each code point matches `\p{name}`, from one scan of a text of
 * every point with the engine Shiki runs grammars on. */
function property(name: string): boolean[] {
  const offsets: number[] = [];
  let text = "";
  for (const cp of points) {
    offsets.push(text.length);
    text += String.fromCodePoint(cp);
  }
  offsets.push(text.length);
  const index = new Map(offsets.map((offset, i) => [offset, i]));
  const scanner = engine.createScanner([`\\p{${name}}+`]);
  const string = engine.createString(text);
  const flags = points.map(() => false);
  for (let from = 0; ;) {
    const match = scanner.findNextMatchSync(string, from);
    if (!match) break;
    const { start, end } = match.captureIndices[0]!;
    for (let i = index.get(start)!; offsets[i]! < end; i++) flags[i] = true;
    from = end;
  }
  return flags;
}

const letters = property("L");
const numbers = property("N");
const marks = property("M");

/** Whether each code point is in any of the categories `names`. */
function any(...names: string[]): boolean[] {
  const each = names.map(property);
  return points.map((_, i) => each.some((flags) => flags[i]));
}
const titleUppers = any("Lt", "Lu");
const lowerOthers = any("Lo", "Nl", "Ll");
const symbols = any("Sm", "So");
const lowers = property("Ll");
const titles = property("Lt");
const symbolsOrPunctuation = any("S", "P");
const letterNumbers = property("Nl");
const nonenclosingMarks = any("Mn", "Mc");
const connectors = property("Pc");

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

function table(name: string, doc: string, flags: boolean[], cfg = ""): string {
  const rows = ranges(flags)
    .map(([lo, hi]) => `    (0x${lo.toString(16)}, 0x${hi.toString(16)}),`)
    .join("\n");
  return dedent`
    /// ${doc}
    ${cfg}const ${name}: &[(u32, u32)] = &[
    ${rows}
    ];
    `;
}

/** The attribute that keeps Scala's tables out of builds without Scala. */
const scala = `#[cfg(feature = "lang-scala")]\n`;
/** The attribute that keeps Haskell's tables out of builds without Haskell. */
const haskell = `#[cfg(feature = "lang-haskell")]\n`;
/** The attribute of a table both Scala and Haskell read. */
const scalaOrHaskell = `#[cfg(any(feature = "lang-scala", feature = "lang-haskell"))]\n`;
/** The attribute that keeps the identifier tables out of builds whose
 * languages read none of them. */
const identifier = `#[cfg(feature = "_identifier-classes")]\n`;

const out = dedent`
  //! The characters Oniguruma's \`\\d\`, \`\\w\`, \`\\s\`, \`[[:alpha:]]\`, \`\\p{upper}\`,
  //! \`\\p{L}\`, \`\\p{N}\`, \`\\p{M}\`, \`[\\p{Lt}\\p{Lu}]\`, \`[\\p{Lo}\\p{Nl}\\p{Ll}]\`,
  //! \`[\\p{Sm}\\p{So}]\`, \`\\p{Ll}\`, \`[\\p{S}\\p{P}]\`, \`\\p{Nl}\`, \`[\\p{Mn}\\p{Mc}]\` and
  //! \`\\p{Pc}\` match beyond ASCII, as Shiki 4.6.0 reads them.
  //! Generated by \`oracle/classes.ts\`; do not edit.

  ${table("DIGITS", "The non-ASCII characters `\\d` matches.", digits)}
  ${table("WORDS", "The non-ASCII characters `\\w` matches.", words)}
  ${table("SPACES", "The non-ASCII characters `\\s` matches.", spaces)}
  ${table("ALPHAS", "The non-ASCII characters `[[:alpha:]]` matches.", alphas)}
  ${table("UPPERS", "The non-ASCII characters `\\p{upper}` matches.", uppers)}
  ${table("LETTERS", "The non-ASCII characters `\\p{L}` matches.", letters)}
  ${table("NUMBERS", "The non-ASCII characters `\\p{N}` matches.", numbers)}
  ${table("MARKS", "The non-ASCII characters `\\p{M}` matches.", marks)}
  ${table("TITLE_UPPERS", "The non-ASCII characters `[\\p{Lt}\\p{Lu}]` matches.", titleUppers, scalaOrHaskell)}
  ${table("LOWER_OTHERS", "The non-ASCII characters `[\\p{Lo}\\p{Nl}\\p{Ll}]` matches.", lowerOthers, scala)}
  ${table("SYMBOLS", "The non-ASCII characters `[\\p{Sm}\\p{So}]` matches.", symbols, scala)}
  ${table("LOWERS", "The non-ASCII characters `\\p{Ll}` matches.", lowers, haskell)}
  ${table("TITLES", "The non-ASCII characters `\\p{Lt}` matches.", titles, haskell)}
  ${table("SYMBOLS_OR_PUNCTUATION", "The non-ASCII characters `[\\p{S}\\p{P}]` matches.", symbolsOrPunctuation, haskell)}
  ${table("LETTER_NUMBERS", "The non-ASCII characters `\\p{Nl}` matches.", letterNumbers, identifier)}
  ${table("NONENCLOSING_MARKS", "The non-ASCII characters `[\\p{Mn}\\p{Mc}]` matches.", nonenclosingMarks, identifier)}
  ${table("CONNECTORS", "The non-ASCII characters `\\p{Pc}` matches.", connectors, identifier)}
  /// Whether \`c\` is in the sorted \`table\` of inclusive ranges.
  fn contains(table: &[(u32, u32)], c: char) -> bool {
      let c = u32::from(c);
      table
          .binary_search_by(|&(lo, hi)| {
              if hi < c {
                  std::cmp::Ordering::Less
              } else if lo > c {
                  std::cmp::Ordering::Greater
              } else {
                  std::cmp::Ordering::Equal
              }
          })
          .is_ok()
  }

  /// The regex \`\\d\` for a non-ASCII character.
  pub(super) fn is_digit(c: char) -> bool {
      contains(DIGITS, c)
  }

  /// The regex \`\\w\` for a non-ASCII character.
  pub(super) fn is_word(c: char) -> bool {
      contains(WORDS, c)
  }

  /// The regex \`\\s\` for a non-ASCII character.
  pub(super) fn is_space(c: char) -> bool {
      contains(SPACES, c)
  }

  /// The regex \`[[:alpha:]]\` for a non-ASCII character.
  pub(super) fn is_alpha(c: char) -> bool {
      contains(ALPHAS, c)
  }

  /// The regex \`\\p{upper}\` for a non-ASCII character.
  pub(super) fn is_upper(c: char) -> bool {
      contains(UPPERS, c)
  }

  /// The regex \`\\p{L}\` for a non-ASCII character.
  pub(super) fn is_letter(c: char) -> bool {
      contains(LETTERS, c)
  }

  /// The regex \`\\p{N}\` for a non-ASCII character.
  pub(super) fn is_number(c: char) -> bool {
      contains(NUMBERS, c)
  }

  /// The regex \`\\p{M}\` for a non-ASCII character.
  pub(super) fn is_mark(c: char) -> bool {
      contains(MARKS, c)
  }

  /// The regex \`[\\p{Lt}\\p{Lu}]\` for a non-ASCII character.
  #[cfg(any(feature = "lang-scala", feature = "lang-haskell"))]
  pub(super) fn is_title_or_upper(c: char) -> bool {
      contains(TITLE_UPPERS, c)
  }

  /// The regex \`[\\p{Lo}\\p{Nl}\\p{Ll}]\` for a non-ASCII character.
  #[cfg(feature = "lang-scala")]
  pub(super) fn is_lower_or_other_letter(c: char) -> bool {
      contains(LOWER_OTHERS, c)
  }

  /// The regex \`[\\p{Sm}\\p{So}]\` for a non-ASCII character.
  #[cfg(feature = "lang-scala")]
  pub(super) fn is_symbol(c: char) -> bool {
      contains(SYMBOLS, c)
  }

  /// The regex \`\\p{Ll}\` for a non-ASCII character.
  #[cfg(feature = "lang-haskell")]
  pub(super) fn is_lower(c: char) -> bool {
      contains(LOWERS, c)
  }

  /// The regex \`\\p{Lt}\` for a non-ASCII character.
  #[cfg(feature = "lang-haskell")]
  pub(super) fn is_title(c: char) -> bool {
      contains(TITLES, c)
  }

  /// The regex \`[\\p{S}\\p{P}]\` for a non-ASCII character.
  #[cfg(feature = "lang-haskell")]
  pub(super) fn is_symbol_or_punctuation(c: char) -> bool {
      contains(SYMBOLS_OR_PUNCTUATION, c)
  }

  /// The regex \`\\p{Nl}\` for a non-ASCII character.
  #[cfg(feature = "_identifier-classes")]
  pub(super) fn is_letter_number(c: char) -> bool {
      contains(LETTER_NUMBERS, c)
  }

  /// The regex \`[\\p{Mn}\\p{Mc}]\` for a non-ASCII character.
  #[cfg(feature = "_identifier-classes")]
  pub(super) fn is_nonenclosing_mark(c: char) -> bool {
      contains(NONENCLOSING_MARKS, c)
  }

  /// The regex \`\\p{Pc}\` for a non-ASCII character.
  #[cfg(feature = "_identifier-classes")]
  pub(super) fn is_connector(c: char) -> bool {
      contains(CONNECTORS, c)
  }
  `;
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
await Bun.write(`${root}src/classes/table.rs`, out);
console.log(
  [
    digits,
    words,
    spaces,
    alphas,
    uppers,
    letters,
    numbers,
    marks,
    titleUppers,
    lowerOthers,
    symbols,
    lowers,
    titles,
    symbolsOrPunctuation,
    letterNumbers,
    nonenclosingMarks,
    connectors,
  ]
    .map((flags) => ranges(flags).length)
    .join(" "),
);
