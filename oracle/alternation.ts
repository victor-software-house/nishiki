// The strings a regular expression that spells out a finite set of words
// matches, shared by the generators that turn a grammar's big alternations
// into word lists.

/** The strings a regular expression made of literals, classes, groups, `|` and `?` spells. */
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
      let item = atom();
      for (;;) {
        if (src[pos] === "?") {
          pos++;
          if (src[pos] === "?") pos++;
          item = ["", ...item];
        } else break;
      }
      out = out.flatMap((a) => item.map((b) => a + b));
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
