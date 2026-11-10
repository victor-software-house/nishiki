/** Every Unicode scalar value, ascending: every code point but the surrogates. */
export function scalarValues(): number[] {
  const points: number[] = [];
  for (let cp = 0; cp <= 0x10ffff; cp++) if (cp < 0xd800 || cp > 0xdfff) points.push(cp);
  return points;
}

/** Code points written as one string, with the code point at each UTF-16 index. */
export interface PointText {
  text: string;
  /** Both indices of a code point beyond U+FFFF give that code point. */
  pointAt: number[];
}

/** The code points as one string, for a scanner to search. */
export function pointText(points: readonly number[]): PointText {
  let text = "";
  const pointAt: number[] = [];
  for (const cp of points) {
    text += String.fromCodePoint(cp);
    pointAt.push(cp);
    if (cp > 0xffff) pointAt.push(cp);
  }
  return { text, pointAt };
}
