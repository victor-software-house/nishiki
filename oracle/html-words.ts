// Writes the words that the big alternations of the `html` grammar spell
// out, expanded and sorted.
// A matcher can then test a whole word against a list.
// - `src/html/words.rs` holds the tag names and attribute names.
// - `src/html_words.rs` holds the event handler names and the character
//   entity names, which other grammars spell the same way.
// Tag names are matched without regard to case, so they are lower-cased here.
// The other names are matched as written.
//
// Run with `bun run oracle/html-words.ts`, and commit the result.
// The script formats what it writes.
// The output is generated: do not edit it.
import html from "@shikijs/langs@4.6.0/html";

import { expand } from "./alternation.ts";
import { writeRust } from "./rule-grammar.ts";

type Node = {
  name?: string;
  match?: string;
  begin?: string;
  patterns?: Node[];
  repository?: Record<string, Node>;
};
const grammar = (html as unknown as (Node & { name: string })[]).find((g) => g.name === "html")!;
const repository = grammar.repository!;

/** The regex of the `index`th pattern of `node`, after checking its scope name. */
function regexAt(node: Node, index: number, name: string): string {
  const pattern = node.patterns![index]!;
  if (pattern.name !== name) throw new Error(`pattern ${index} is ${pattern.name}, not ${name}`);
  return (pattern.match ?? pattern.begin)!;
}

/** The expansion of the text between `prefix` and the last `suffix` of `regex`. */
function words(regex: string, prefix: string, suffix: string, fold: boolean): string[] {
  const start = regex.indexOf(prefix);
  if (start < 0) throw new Error(`${prefix} not in ${regex.slice(0, 60)}`);
  let body = regex.slice(start + prefix.length, regex.lastIndexOf(suffix));
  // A list wrapped twice, as `((a|b))`, is one list.
  while (body.startsWith("(") && body.endsWith(")") && !body.slice(1, -1).includes("(?")) {
    const inner = body.slice(1, -1);
    if (!balanced(inner)) break;
    body = inner;
  }
  const list = expand(body).map((w) => (fold ? w.toLowerCase() : w));
  return [...new Set(list)].sort();
}

/** Whether every `(` of `text` closes within it. */
function balanced(text: string): boolean {
  let depth = 0;
  for (const c of text) {
    if (c === "(") depth++;
    if (c === ")" && --depth < 0) return false;
  }
  return depth === 0;
}

/** The tag names of a start rule `(?i)(<)(NAMES)(?=\s|/?>)`. */
function tags(regex: string): string[] {
  return words(regex, "(?i)(<)(", ")(?=", true);
}

const same = (a: string[], b: string[], what: string): string[] => {
  if (a.join() !== b.join()) throw new Error(`${what}: the start and end lists differ`);
  return a;
};

const valid = repository["tags-valid"]!;
const at = (index: number, name: string) => regexAt(valid, index, name);
const startEnd = (start: number, startName: string, end: number, endName: string, what: string) =>
  same(tags(at(start, startName)), tags(at(end, endName).replace("(</)", "(<)")), what);

const lists: [string, string[]][] = [
  ["METADATA_VOID", tags(at(2, "meta.tag.metadata.$2.void.html"))],
  ["METADATA", startEnd(3, "meta.tag.metadata.$2.start.html", 4, "meta.tag.metadata.$2.end.html", "metadata")],
  ["STRUCTURE_VOID", tags(at(5, "meta.tag.structure.$2.void.html"))],
  ["STRUCTURE", startEnd(6, "meta.tag.structure.$2.start.html", 7, "meta.tag.structure.$2.end.html", "structure")],
  ["INLINE_VOID", tags(at(8, "meta.tag.inline.$2.void.html"))],
  ["INLINE", startEnd(9, "meta.tag.inline.$2.start.html", 10, "meta.tag.inline.$2.end.html", "inline")],
  ["OBJECT_VOID", tags(at(11, "meta.tag.object.$2.void.html"))],
  ["OBJECT", startEnd(12, "meta.tag.object.$2.start.html", 13, "meta.tag.object.$2.end.html", "object")],
  ["DEPRECATED_METADATA_VOID", tags(at(14, "meta.tag.metadata.$2.void.html"))],
  ["DEPRECATED_STRUCTURE", startEnd(15, "meta.tag.structure.$2.start.html", 16, "meta.tag.structure.$2.end.html", "deprecated structure")],
  ["DEPRECATED_INLINE", startEnd(17, "meta.tag.inline.$2.start.html", 18, "meta.tag.inline.$2.end.html", "deprecated inline")],
  ["DEPRECATED_OBJECT_VOID", tags(at(19, "meta.tag.object.$2.void.html"))],
  ["DEPRECATED_OBJECT", startEnd(20, "meta.tag.object.$2.start.html", 21, "meta.tag.object.$2.end.html", "deprecated object")],
  ["UNSUPPORTED", startEnd(22, "meta.tag.other.$2.start.html", 23, "meta.tag.other.$2.end.html", "unsupported")],
];

// The elements of the `svg` and `math` sub-grammars: per category, the list of
// the self-closing rule (`_VOID`) and of the rule that opens an element.
for (const [ns, key, names] of [
  ["SVG", "svg", ["metadata", "structure", "inline", "object", "other"]],
  ["MATH", "math", ["structure", "inline", "object"]],
] as const) {
  const tagsRepo = repository[key]!.patterns![0]!.repository ?? repository[key]!.repository!;
  const patterns = tagsRepo["tags"]!.patterns!;
  // Patterns 0 and 1 are the comment and the CDATA includes.
  names.forEach((cat, i) => {
    const [voidRule, openRule] = [patterns[2 + 2 * i]!, patterns[3 + 2 * i]!];
    for (const [suffix, rule] of [["_VOID", voidRule], ["", openRule]] as const) {
      const expected = `meta.element.${cat}.${key}.$2.html`;
      if (rule.name !== expected) throw new Error(`${key}: ${rule.name}, not ${expected}`);
      lists.push([`${ns}_${cat.toUpperCase()}${suffix}`, tags(rule.begin ?? rule.match!)]);
    }
  });
}

const attributes = (node: Node, key: string): [string, string[]][] => {
  const name = key.toUpperCase();
  const first = node.patterns![0]!.begin!;
  const out: [string, string[]][] = [[`${name}_ATTRIBUTES`, words(first, "(", ")(?![-:\\w])", false)]];
  return out;
};
lists.push(...attributes(repository["attribute"]!, "html"));
const htmlAttribute = repository["attribute"]!.patterns!;
const shared: [string, string[]][] = [
  ["EVENT_HANDLERS", words(htmlAttribute[2]!.begin!, "on(", ")(?![-:\\w])", false)],
];
for (const key of ["svg", "math"]) {
  const repo = repository[key]!.repository!;
  lists.push(...attributes(repo["attribute"]!, key));
}

const entity = repository["entities"]!.patterns![0]!.match!;
shared.push(["ENTITIES", words(entity, "(&)(?=[A-Za-z])(", ")(;)", false)]);

/** The Rust source of `lists`, each a sorted `&[&str]` of `visibility`. */
function rust(header: string, visibility: string, lists: [string, string[]][]): string {
  let out = header;
  for (const [name, list] of lists) {
    out += `${visibility} const ${name}: &[&str] = &[\n${list.map((w) => `    ${JSON.stringify(w)},\n`).join("")}];\n\n`;
  }
  return out.trimEnd() + "\n";
}

const root = new URL("..", import.meta.url).pathname;
await writeRust(
  `${root}src/html/words.rs`,
  rust(
    `//! The tag and attribute names that the big alternations of the \`html\`
//! grammar spell out, sorted. Tag names are lower case. Generated by
//! \`oracle/html-words.ts\`; do not edit.

`,
    "pub(super)",
    lists,
  ),
);
await writeRust(
  `${root}src/html_words.rs`,
  rust(
    `//! The event handler and character entity names that the big alternations
//! of the \`html\` grammar spell out, sorted, for every grammar that spells
//! them the same way. Generated by \`oracle/html-words.ts\`; do not edit.

`,
    "pub(crate)",
    shared,
  ),
);
console.log([...lists, ...shared].map(([n, l]) => `${n} ${l.length}`).join(", "));
