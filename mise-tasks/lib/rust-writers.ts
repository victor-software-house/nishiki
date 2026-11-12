// The Bun scripts that write Rust, and the Rust files they write.
// check:line-width leaves both alone, and check:literals leaves the scripts alone,
// until rule-table-codegen moves each writer to Rust (CODING_STANDARDS 3.10).

/** The text of a comment line, without its marker, or the line when it is code. */
export function commentText(line: string): string {
  return line.trim().replace(/^(\/\/[/!]?|#|\*)\s?/, "");
}

/** The Bun script a generated Rust file names as its writer, when its header says not to edit it. */
export function writerOf(text: string): string | undefined {
  const header = text.split("\n").slice(0, 12).map(commentText).join(" ");
  if (!/do not edit|regenerate with/i.test(header)) return undefined;
  return /oracle\/[\w/-]+\.ts/.exec(header)?.[0];
}

/**
 * The files to leave alone, out of `texts`, which maps each tracked file to its text:
 * each Bun script a generated file names in its header or that calls `writeRust`,
 * and each Rust file such a script writes.
 */
export function rustWriterFiles(texts: Map<string, string>): Set<string> {
  const found = new Set<string>();
  for (const [file, text] of texts) {
    const writer = file.endsWith(".rs") ? writerOf(text) : undefined;
    if (writer !== undefined) found.add(writer).add(file);
    if (file.endsWith(".ts") && /\bwriteRust\(/.test(text)) found.add(file);
  }
  return found;
}
