// Where a Rust string literal continues on the next line through a backslash line break.
// CODING_STANDARDS 3.10: a reason or message fits on one line, and multiline text is an
// indoc! or raw string block.
// The scan walks the file once, so a backslash in a comment, a raw string or a char
// literal is not read as a string's.

/** The lines of `text` whose string literal ends in a backslash line break, as `file:line: text`. */
export function continuedLiterals(file: string, text: string): string[] {
  const lines = text.split("\n");
  const found = new Set<number>();
  let line = 0;
  let i = 0;
  const at = (offset: number): string => text[i + offset] ?? "";
  const step = (): void => {
    if (text[i] === "\n") line += 1;
    i += 1;
  };
  while (i < text.length) {
    if (at(0) === "/" && at(1) === "/") {
      while (i < text.length && at(0) !== "\n") step();
    } else if (at(0) === "/" && at(1) === "*") {
      let depth = 0;
      do {
        if (at(0) === "/" && at(1) === "*") {
          depth += 1;
          step();
        } else if (at(0) === "*" && at(1) === "/") {
          depth -= 1;
          step();
        }
        step();
      } while (i < text.length && depth > 0);
    } else if (/^b?r#*"/.test(text.slice(i, i + 260)) && !/\w/.test(text[i - 1] ?? "")) {
      const open = /^b?r(#*)"/.exec(text.slice(i, i + 260));
      const close = `"${open?.[1] ?? ""}`;
      for (let k = 0; k < (open?.[0].length ?? 1); k += 1) step();
      while (i < text.length && text.slice(i, i + close.length) !== close) step();
      for (let k = 0; k < close.length; k += 1) step();
    } else if (at(0) === '"') {
      step();
      while (i < text.length && at(0) !== '"') {
        if (at(0) === "\\") {
          if (at(1) === "\n") found.add(line);
          step();
        }
        step();
      }
      step();
    } else if (at(0) === "'") {
      const literal = /^'(?:\\(?:u\{[0-9a-fA-F]+\}|x[0-9a-fA-F]{2}|.)|[^\\'\n])'/u.exec(
        text.slice(i, i + 16),
      );
      const length = literal?.[0].length ?? 1;
      for (let k = 0; k < length; k += 1) step();
    } else {
      step();
    }
  }
  return [...found].map((index) => `${file}:${index + 1}: ${(lines[index] ?? "").trim()}`);
}
