// Oniguruma's answer for one case of a regex probe, in the form the regex
// tests read: each group's UTF-8 start and end, `x` for a group that did not
// take part, `-` when nothing matched.
//
// Shiki's engine gives a group that did not take part 4294967295 as its start
// and end on ASCII text. On other text it gives the text's length instead.
// A group that took part with no characters at the text's end looks the same,
// so such an answer is ambiguous, and `answer` returns undefined for it.

/** One group of a match, as Shiki's engine gives it, in UTF-16 units. */
type Group = { start: number; end: number };

/** The start and end Shiki's engine gives a group that did not take part. */
const unset = 0xffffffff;

/** `index`, a UTF-16 offset in `text`, as a UTF-8 offset. */
export const utf8 = (text: string, index: number) => Buffer.byteLength(text.slice(0, index));

/** The answer for a match of `groups` in `text`, or undefined when it is
 * ambiguous. */
export function answer(groups: Group[] | undefined, text: string): string | undefined {
  if (groups === undefined) return "-";
  if (
    /[^\x00-\x7f]/.test(text) &&
    groups.slice(1).some(({ start, end }) => start === text.length && end === text.length)
  ) {
    return undefined;
  }
  return groups
    .map(({ start, end }) => (start === unset || end < start ? "x" : `${utf8(text, start)},${utf8(text, end)}`))
    .join(";");
}
