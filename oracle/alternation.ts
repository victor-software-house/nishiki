// The strings a regular expression that spells out a finite set of words
// matches, shared by the generators that turn a grammar's big alternations
// into word lists.

/** The strings a regular expression made of literals, classes, groups, `|` and `?` spells,
 * in the order a backtracking matcher tries them: a greedy `?` tries its atom first, a lazy
 * `??` the empty string first. */
export function expand(src: string): string[] {
  let pos = 0;
  function alternation(): string[] {
    let out = sequence();
    while (src[pos] === "|") {
      pos++;
      out = out.concat(sequence());
    }
    return out;
  }
  function sequence(): string[] {
    let out = [""];
    while (pos < src.length && src[pos] !== "|" && src[pos] !== ")") {
      let strings = atom();
      for (;;) {
        if (src[pos] === "?") {
          pos++;
          if (src[pos] === "?") {
            pos++;
            strings = ["", ...strings];
          } else strings = [...strings, ""];
        } else if (src[pos] === "{") {
          // A greedy `{m,n}` tries the most repetitions first.
          const close = src.indexOf("}", pos);
          const [lo, hi] = src.slice(pos + 1, close).split(",").map(Number) as [number, number];
          pos = close + 1;
          const once = strings;
          const times = (n: number): string[] => (n === 0 ? [""] : times(n - 1).flatMap((a) => once.map((b) => a + b)));
          strings = [];
          for (let n = hi; n >= lo; n--) strings.push(...times(n));
        } else break;
      }
      out = out.flatMap((a) => strings.map((b) => a + b));
    }
    return out;
  }
  function atom(): string[] {
    const c = src[pos]!;
    if (c === "(") {
      pos++;
      if (src.startsWith("?:", pos)) pos += 2;
      const inner = alternation();
      if (src[pos] !== ")") throw new Error(`unclosed group in ${src}`);
      pos++;
      return inner;
    }
    if (c === "[") {
      pos++;
      const chars: string[] = [];
      while (src[pos] !== "]") {
        const lo = src[pos++]!;
        if (src[pos] === "-" && src[pos + 1] !== "]") {
          pos++;
          const hi = src[pos++]!;
          for (let code = lo.charCodeAt(0); code <= hi.charCodeAt(0); code++) chars.push(String.fromCharCode(code));
        } else chars.push(lo);
      }
      pos++;
      return chars;
    }
    if (c === "\\") {
      pos += 2;
      return [src[pos - 1]!];
    }
    pos++;
    return [c];
  }
  const out = alternation();
  if (pos !== src.length) throw new Error(`stopped at ${pos} in ${src}`);
  return out;
}
