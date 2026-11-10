/**
 * Multiline text written at the indentation of the code that builds it.
 *
 * It plays the part `indoc!` plays in Rust, with two differences:
 *
 * - `indoc!` measures the indentation without the first line.
 *   `dedent` measures every line that is not blank, the first included.
 * - `dedent` also measures a closing backtick on a line of its own, as Java's text blocks do.
 *
 * - The first line is dropped when it holds only blanks.
 * - Every line loses the indentation all its lines share.
 *   A closing backtick on a line of its own counts as a line here.
 *   It sits at the text's indentation.
 *   Placed further left, it leaves the text that much indentation.
 * - That closing line is dropped.
 *   The line break before it stays.
 *   The text therefore ends with a line break only when one is written before
 *   the backtick.
 * - A line of blanks alone becomes empty and sets no indentation.
 * - An interpolated value goes in as it is.
 *   A value that starts a line counts that line's blanks as indentation.
 *   Each later non-empty line of the value gets the same indentation, once the margin is removed.
 *   A value whose first line is empty leaves its line empty, without blanks.
 * - It throws when one line's indentation does not start with the shortest one,
 *   such as a tab against spaces.
 *
 * The tag reads the text as cooked, so an escaped `\n` is a line break like a written one.
 * Write line breaks as line breaks.
 *
 * @example
 * const rust = dedent`
 *   impl Rule {
 *       ${body}
 *   }
 *   `;
 */
export function dedent(strings: TemplateStringsArray, ...values: Values): string {
  return dedentPieces(strings, values);
}

/** The values a `dedent` template interpolates. */
type Values = readonly (string | number)[];

/** The text `dedent` builds from template pieces and the values between them. */
export function dedentPieces(pieces: readonly string[], values: Values): string {
  const fragments = splitFragments(pieces);
  const indents = fragments
    .filter((fragment) => fragment.setsIndent)
    .map((fragment) => indentOf(fragment.text));
  const margin = commonMargin(indents);
  const texts = pieces.map(() => [] as string[]);
  const valueIndents: string[] = [];
  for (const fragment of fragments) {
    const text = dedented(fragment, margin);
    texts[fragment.piece]!.push(text);
    if (fragment.beforeValue) valueIndents[fragment.piece] = startsValueLine(fragment) ? text : "";
  }
  return texts.map((lines) => lines.join("\n")).reduce((text, piece, index) => {
    const value = String(values[index - 1]);
    const indent = valueIndents[index - 1]!;
    const before = value.split("\n")[0] === "" ? text.slice(0, text.length - indent.length) : text;
    return before + indented(value, indent) + piece;
  });
}

/** One fragment of a template piece, between line breaks or values. */
type Fragment = {
  /** The template piece it belongs to. */
  piece: number;
  /** Its text. */
  text: string;
  /** Whether a line starts with it, rather than with a value before it. */
  startsLine: boolean;
  /** Whether a value follows it. */
  beforeValue: boolean;
  /** Whether its indentation counts toward the indentation every line loses. */
  setsIndent: boolean;
  /** Whether it is a line of blanks alone, or the closing line, and so empty. */
  blankLine: boolean;
};

/**
 * The fragments of every piece, with a blank first line dropped.
 * A fragment of blanks before a value is that value's indentation and sets indentation.
 * A closing line of blanks sets indentation too, so it can hold the text's left edge.
 */
function splitFragments(pieces: readonly string[]): Fragment[] {
  const split = pieces.map((piece) => piece.split("\n"));
  const first = split[0]!;
  if (first.length > 1 && isBlank(first[0]!)) first.shift();
  const last = split.length - 1;
  return split.flatMap((lines, piece) =>
    lines.map((text, line) => {
      const startsLine = line > 0 || piece === 0;
      const beforeValue = piece < last && line === lines.length - 1;
      const closing = piece === last && line === lines.length - 1 && line > 0;
      const blank = isBlank(text);
      return {
        piece,
        text,
        startsLine,
        beforeValue,
        setsIndent: startsLine && (!blank || beforeValue || closing),
        blankLine: startsLine && blank && !beforeValue,
      };
    }),
  );
}

/** Whether a value follows this fragment at the start of its line, after blanks only. */
function startsValueLine(fragment: Fragment): boolean {
  return fragment.startsLine && isBlank(fragment.text);
}

/** A fragment's text once the margin is removed. */
function dedented(fragment: Fragment, margin: string): string {
  if (fragment.blankLine) return "";
  if (fragment.setsIndent) return fragment.text.slice(margin.length);
  return fragment.text;
}

/** The indentation every line shares: the shortest, which every other must start with. */
function commonMargin(indents: readonly string[]): string {
  if (indents.length === 0) return "";
  const shortest = indents.reduce((least, indent) =>
    indent.length < least.length ? indent : least,
  );
  if (indents.some((indent) => !indent.startsWith(shortest))) {
    throw new Error("dedent: the lines mix tabs and spaces in their indentation");
  }
  return shortest;
}

/** A value with every later non-empty line indented by `indent`. */
function indented(value: string, indent: string): string {
  return value
    .split("\n")
    .map((line, index) => (index === 0 || line === "" ? line : indent + line))
    .join("\n");
}

/** Whether a line holds only blanks. */
function isBlank(text: string): boolean {
  return /^[ \t]*$/.test(text);
}

/** A line's leading blanks. */
function indentOf(text: string): string {
  return text.match(/^[ \t]*/)![0];
}
