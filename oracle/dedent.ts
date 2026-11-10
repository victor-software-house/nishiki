/**
 * Multiline text written at the indentation of the code that builds it, as
 * `indoc!` writes it in Rust.
 *
 * - The line break right after the opening backtick is dropped.
 * - Every line loses the indentation all its lines share.
 *   A closing backtick on a line of its own counts as a line here.
 *   It sits at the text's indentation, as Java's text blocks place it.
 *   Placed further left, it leaves the text that much indentation.
 * - That closing line is dropped.
 *   The line break before it stays.
 *   The text therefore ends with a line break only when one is written before
 *   the backtick.
 * - A line of blanks alone becomes empty and sets no indentation.
 * - An interpolated value goes in as it is, its own line breaks included.
 *   The blanks before a value at the start of a line count as indentation.
 *
 * Write line breaks as line breaks: an escaped `\n` in the text is not one.
 *
 * @example
 * const rust = dedent`
 *   impl Rule {
 *       ${body}
 *   }
 *   `;
 */
export function dedent(strings: TemplateStringsArray, ...values: unknown[]): string {
  const lines = splitLines(strings);
  const indents = lines.filter((line) => line.counts).map((line) => indentOf(line.text));
  const common = indents.length === 0 ? 0 : Math.min(...indents);
  const pieces = strings.map(() => [] as string[]);
  for (const line of lines) {
    pieces[line.piece]!.push(line.startsLine && line.counts ? line.text.slice(common) : line.emptied ? "" : line.text);
  }
  return pieces
    .map((piece) => piece.join("\n"))
    .reduce((text, piece, p) => text + String(values[p - 1]) + piece);
}

/** One fragment of a template piece, between line breaks or values. */
type Fragment = {
  /** The template piece it belongs to. */
  piece: number;
  /** Its text. */
  text: string;
  /** Whether a line starts with it, rather than with a value before it. */
  startsLine: boolean;
  /** Whether its indentation counts toward the common indentation. */
  counts: boolean;
  /** Whether it is a line of blanks alone, or the closing line, and so empty. */
  emptied: boolean;
};

/**
 * The fragments of every piece, with the opening line break dropped.
 * A fragment of blanks before a value is that value's indentation and counts.
 * A closing line of blanks counts, so it can hold the text's left edge.
 */
function splitLines(strings: TemplateStringsArray): Fragment[] {
  const pieces = strings.map((piece) => piece.split("\n"));
  const first = pieces[0]!;
  if (first.length > 1 && first[0] === "") first.shift();
  const last = pieces.length - 1;
  return pieces.flatMap((fragments, p) =>
    fragments.map((text, l) => {
      const startsLine = l > 0 || p === 0;
      const beforeValue = p < last && l === fragments.length - 1;
      const closing = p === last && l === fragments.length - 1 && l > 0;
      const blank = /^[ \t]*$/.test(text);
      const emptied = startsLine && blank && !beforeValue;
      return { piece: p, text, startsLine, counts: startsLine && (!blank || beforeValue || closing), emptied };
    }),
  );
}

/** The width of a line's leading blanks. */
function indentOf(text: string): number {
  return text.match(/^[ \t]*/)![0].length;
}
