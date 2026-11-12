// Writes src/css/scope.rs and src/css/grammar.rs from the `css` grammar
// that Shiki 4.6.0 bundles (microsoft/vscode, MIT): the scope names, and
// for every rule the scopes it adds, how it ends and the patterns it tries, in
// the grammar's order with its `include`s flattened. The matchers that decide
// whether a pattern matches are written by hand in src/css/matchers.
//
// One more rule comes from the `vue` grammar (vuejs/language-tools, MIT):
// the string of Vue's `style` attribute, whose patterns include this grammar's.
// A Vue component lexes that string from it.
//
// Run with `bun run oracle/css-grammar.ts` and commit the result; the
// output is generated, do not edit it.
import css from "@shikijs/langs@4.6.0/css";
import vue from "@shikijs/langs@4.6.0/vue";
import {
  arms,
  groupsCheck,
  ruleCodes,
  scopeFile,
  startTable,
  tooLong,
  writePatterns,
  writeRust,
} from "./rule-grammar.ts";

type Captures = Record<string, { name?: string; patterns?: unknown[] }>;
export type Node = {
  name?: string;
  contentName?: string;
  match?: string;
  begin?: string;
  end?: string;
  captures?: Captures;
  beginCaptures?: Captures;
  endCaptures?: Captures;
  patterns?: Node[];
  include?: string;
};
type CaptureNode = { name?: string; patterns?: Node[] };

const grammar = (css as unknown as { repository: Record<string, Node>; patterns: Node[] }[])[0]!;
export const repository = grammar.repository;

/** Replaces `${n:/downcase}` in a scope name. */
const downcase = (name: string | undefined, group: number, word: string) =>
  name?.replace(`\${${group}:/downcase}`, word.toLowerCase());

/** The units of the grammar's numeric values, sorted and lowered. */
export let units: string[] = [];

/**
 * The grammar names a scope after the text a group matched, as in
 * \`keyword.other.unit.${2:/downcase}.css\`.
 * The at-rule keywords of \`font-features\` become one pattern each, with its own name.
 * The expansion keeps the grammar's order.
 *
 * The numeric units stay one pattern, with no fixed name for the unit.
 * Its matcher names the unit from the table of units and their scopes.
 * Every unit is letters only and ends at \`\\b\`.
 * So at most one unit matches after a number, and their order does not matter.
 */
function expandDynamic(): void {
  const fonts = repository["font-features"]!;
  const begin = fonts.begin!;
  const alternatives = /\(\(@\)\(([^)]*)\)\)/.exec(begin)![1]!.split("|");
  const template = { ...fonts };
  // One entry for each at-rule keyword, listed as the grammar's single entry.
  repository["font-features"] = {
    patterns: alternatives.map((word) => ({
      ...template,
      begin: begin.replace(`(${alternatives.join("|")})`, `(${word})`),
      beginCaptures: {
        "1": { name: downcase(template.beginCaptures!["1"]!.name, 3, word)! },
        "2": template.beginCaptures!["2"]!,
      },
      name: downcase(template.name, 3, word),
      patterns: (JSON.parse(JSON.stringify(template)) as Node).patterns,
    })),
  };
  const numeric = repository["numeric-values"]!.patterns![1]!;
  const regex = numeric.match!;
  const marker = "(?:(%)|(";
  const at = regex.indexOf(marker);
  const prefix = regex.slice(0, at);
  const alternation = regex.slice(at + marker.length, regex.lastIndexOf(")\\b)?"));
  units = alternation
    .split("|")
    .map((word) => word.toLowerCase())
    .sort();
  for (const word of units) {
    if (!/^[a-z]+$/.test(word)) throw new Error(`unit ${word} is not letters only`);
  }
  repository["numeric-values"]!.patterns = [
    repository["numeric-values"]!.patterns![0]!,
    {
      match: `${prefix}(?:(%))`,
      name: numeric.name,
      captures: { "1": { name: "keyword.other.unit.percentage.css" } },
    },
    { match: `${prefix}(?:(${alternation})\\b)`, name: numeric.name },
    { match: prefix, name: numeric.name },
  ];
}
expandDynamic();

/**
 * The first capturing group of `regex`, outside character classes: where it starts and ends,
 * and what is inside.
 */
function firstGroup(regex: string): { start: number; end: number; body: string } {
  let inClass = false;
  for (let i = 0; i < regex.length; i++) {
    const c = regex[i]!;
    if (c === "\\") i++;
    else if (inClass) inClass = c !== "]";
    else if (c === "[") inClass = true;
    else if (c === "(" && regex[i + 1] !== "?") {
      const end = regex.indexOf(")", i);
      return { start: i, end: end + 1, body: regex.slice(i + 1, end) };
    }
  }
  throw new Error(`no group in ${regex}`);
}

/** A pattern named after its first group (`$1`) becomes one pattern per alternative. */
function expandBackrefs(list: Node[]): Node[] {
  return list.flatMap((node) => {
    if (node.patterns) node.patterns = expandBackrefs(node.patterns);
    if (node.match === undefined || !node.name?.includes("$1")) return [node];
    const group = firstGroup(node.match);
    return group.body.split("|").map((word) => ({
      ...node,
      match: `${node.match!.slice(0, group.start)}(${word})${node.match!.slice(group.end)}`,
      name: node.name!.replace("$1", word),
    }));
  });
}
for (const entry of Object.values(repository))
  if (entry.patterns) entry.patterns = expandBackrefs(entry.patterns);
const root = { patterns: grammar.patterns } as Node;

const root_dir = new URL("..", import.meta.url).pathname;

// Scope names.

const abbreviations: Record<string, string> = {
  punctuation: "P",
  keyword: "Kw",
  constant: "Const",
  invalid: "Inv",
  storage: "Stor",
  entity: "Ent",
  variable: "Var",
  support: "Supp",
  string: "Str",
  comment: "Cmt",
  meta: "Meta",
};

function scopeIdent(name: string): string {
  const parts = name.split(".");
  let suffix = "";
  if (parts[parts.length - 1] === "css") parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const capture = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? capture) : capture;
    }
  });
  return out + suffix;
}

export const scopes = new Map<string, string>([["Source", "source.css"]]);
function scope(name: string): string {
  const ident = scopeIdent(name);
  const known = scopes.get(ident);
  if (known !== undefined && known !== name) throw new Error(`${ident}: ${known} and ${name}`);
  scopes.set(ident, name);
  return ident;
}
function scopeList(name: string | undefined): string[] {
  return name === undefined ? [] : name.split(" ").map(scope);
}

/** The scope of each of \`units\`, in the same order. */
export const unitScopes = units.map((word) => scope(`keyword.other.unit.${word}.css`));

// Rules and patterns.

// The grammar spells `character` wrong in one key (split here so that the
// spelling check passes); the Rust names do not.
export const pascal = (key: string) =>
  key
    .split(/[-_.]/)
    .map((piece) => piece.charAt(0).toUpperCase() + piece.slice(1))
    .join("");

export type Pattern = {
  id: string;
  regex: string;
  act: { role: "match"; name: string[] } | { role: "begin"; rule: string } | { role: "end" };
  captures: [number, string[]][];
  /** Capture groups that the grammar tokenizes with patterns of their own. */
  nested: [number, string][];
  /** The key of the repository entry it comes from, for flavours. */
  key: string;
};
export type Rule = {
  id: string;
  key: string;
  name: string[];
  content: string[];
  end: string;
  patterns: string[];
};

export const patternById = new Map<string, Pattern>();
export const rules = new Map<string, Rule>();

/** The capture groups whose text the grammar tokenizes with patterns,
 * each with the name of its nested list. */
function nestedList(captures: Captures | undefined): [number, string][] {
  if (!captures) return [];
  const out: [number, string][] = [];
  for (const [group, value] of Object.entries(captures)) {
    const first = (value.patterns?.[0] ?? null) as { match?: string; include?: string } | null;
    if (first === null) continue;
    const name =
      first.include === "#escapes"
        ? "Escapes"
        : first.include === "#comment-block"
          ? "CommentBlock"
          : null;
    if (name === null) throw new Error(`capture ${group}: unknown nested patterns`);
    out.push([Number(group), name]);
  }
  return out.sort((a, b) => a[0] - b[0]);
}

function captureList(captures: Captures | undefined): [number, string[]][] {
  if (!captures) return [];
  return Object.entries(captures)
    .map(([group, value]): [number, string[]] => [Number(group), scopeList(value.name)])
    .filter(([, names]) => names.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

/** Registers a match or begin/end rule under `id`; returns the pattern id. */
function register(node: Node, id: string, key: string): string {
  const clash = patternById.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (node.match !== undefined) {
    if (!patternById.has(id)) {
      patternById.set(id, {
        id,
        regex: node.match,
        act: { role: "match", name: scopeList(node.name) },
        captures: captureList(node.captures),
        nested: nestedList(node.captures),
        key,
      });
    }
    return id;
  }
  if (node.begin !== undefined && node.end !== undefined) {
    if (!patternById.has(id)) {
      patternById.set(id, {
        id,
        regex: node.begin,
        act: { role: "begin", rule: id },
        captures: captureList(node.beginCaptures ?? node.captures),
        nested: nestedList(node.beginCaptures ?? node.captures),
        key,
      });
      patternById.set(`${id}End`, {
        id: `${id}End`,
        regex: node.end,
        act: { role: "end" },
        captures: captureList(node.endCaptures ?? node.captures),
        nested: nestedList(node.endCaptures ?? node.captures),
        key,
      });
      const rule: Rule = {
        id,
        key,
        name: scopeList(node.name),
        content: scopeList(node.contentName),
        end: `${id}End`,
        patterns: [],
      };
      rules.set(id, rule);
      rule.patterns = flatten(node.patterns ?? [], id);
    }
    return id;
  }
  throw new Error(`${id}: neither a match nor a begin/end rule`);
}

/** The pattern ids a pattern list expands to, `include`s flattened. */
function flatten(list: Node[], context: string): string[] {
  const out: string[] = [];
  list.forEach((node, index) => {
    if (node.include === "$self") {
      out.push(...flatten(root.patterns!, "Root"));
    } else if (node.include !== undefined) {
      if (!node.include.startsWith("#")) throw new Error(`external include ${node.include}`);
      const key = node.include.slice(1);
      const target = repository[key];
      if (!target) throw new Error(`unknown include ${key}`);
      out.push(...expand(target, pascal(key), key));
    } else {
      out.push(...expand(node, `${context}At${index}`, context));
    }
  });
  return out;
}

export function expand(node: Node, id: string, key: string): string[] {
  if (node.match !== undefined || node.begin !== undefined) return [register(node, id, key)];
  if (node.patterns) return flatten(node.patterns, id);
  throw new Error(`${id}: empty rule`);
}

const rootPatterns = flatten(root.patterns!, "Root");
rules.set("Root", {
  id: "Root",
  key: "",
  name: [],
  content: [],
  end: "",
  patterns: rootPatterns,
});

/**
 * The patterns of the string in Vue's `style` attribute, with their includes of
 * this grammar made local.
 * They are `rule-list-innards` but for three entries, and their property value
 * ends before a quote, not before `)` or `}`.
 */
function vueStyleAttribute(): Node[] {
  const grammars = vue as unknown as { scopeName: string; repository: Record<string, Node> }[];
  const grammar = grammars.find((g) => g.scopeName === "text.html.vue")!;
  const string = grammar.repository["vue-directives-style-attr"]!.patterns![0]!;
  return JSON.parse(JSON.stringify(string.patterns), (key, value: unknown) =>
    key === "include" && typeof value === "string" && value.startsWith("source.css#")
      ? value.slice("source.css".length)
      : value,
  ) as Node[];
}

// Output, when run rather than imported by another generator.
if (import.meta.main) {
  // The rule of Vue's `style` attribute, which the SCSS generator does not copy.
  rules.set("VueStyleAttribute", {
    id: "VueStyleAttribute",
    key: "",
    name: [],
    content: [],
    end: "",
    patterns: flatten(vueStyleAttribute(), "VueStyleAttribute"),
  });

  // Output.

  const list = (names: string[]) =>
    names.length ? `scopes![${names.map((n) => `S::${n}`).join(", ")}]` : "S::NO_SCOPES";

  let g = `//! The grammar's rules as tables. Generated by \`oracle/css-grammar.ts\` from
//! the \`css\` grammar Shiki 4.6.0 bundles; do not edit.
//! The rule \`VueStyleAttribute\` and the rules it opens come from the \`vue\`
//! grammar.
//! They read the string of Vue's \`style\` attribute.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.

use super::engine::Css;
use super::scope::S;
use crate::scope::scopes;
use crate::rules;

pub(crate) type Act = rules::Act<Css>;
pub(crate) type PatternDefinition = rules::PatternDefinition<Css>;
pub(crate) type RuleDefinition = rules::RuleDefinition<Css>;
pub(crate) use crate::rules::Start;

/// The patterns a capture group is tokenized with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Nested {
    /// The escapes of a name.
    Escapes,
    /// A block comment.
    CommentBlock,
}

`;

  /** Match arms of \`type::Id\`s, with the ids that share a body joined by \`|\`. */
  const ruleList = [...rules.values()];
  const patternList = [...patternById.values()];

  g += `/// A rule of the grammar that can be open at the end of a line.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u8)]\n#[expect(\n    clippy::enum_variant_names,\n    reason = "the names are the grammar's rule names"\n)]\npub(crate) enum Rule {\n`;
  for (const r of ruleList) g += `    ${r.id},\n`;
  g += `}\n\nimpl Rule {\n    /// Every rule, in code order.\n    pub(crate) const ALL: [Rule; ${ruleList.length}] = [\n`;
  for (const r of ruleList) g += `        Rule::${r.id},\n`;
  g += `    ];\n\n${tooLong}    pub(crate) fn definition(self) -> RuleDefinition {\n        match self {\n`;
  g += arms(
    ruleList.map((r): [string, string] => [
      r.id,
      `RuleDefinition {\n                name: ${list(r.name)},\n                content: ${list(r.content)},\n                end: ${r.end ? `Some(Pattern::${r.end})` : "None"},\n                backref: None,\n                patterns: &[${r.patterns.map((p) => `Pattern::${p}`).join(", ")}],\n            }`,
    ]),
    "Rule",
  );
  g += `        }\n    }\n}\n\n`;

  g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pattern {\n`;
  const docs = (s: string) => {
    const one = s.replace(/\n/g, "\\n");
    return one.length > 56 ? `…` : one;
  };
  for (const p of patternList) {
    const role = p.act.role === "end" ? "end" : p.act.role === "begin" ? "begin" : "match";
    g += `    /// The ${role} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
  }
  g += `}\n\nimpl Pattern {\n${tooLong}    pub(crate) fn definition(self) -> PatternDefinition {\n        match self {\n`;
  g += arms(
    patternList.map((p): [string, string] => {
      const act =
        p.act.role === "match"
          ? `Act::Match(${list(p.act.name)})`
          : p.act.role === "begin"
            ? `Act::Begin(Rule::${p.act.rule})`
            : "Act::End";
      const captures = p.captures.map(([n, names]) => `(${n}, ${list(names)})`).join(", ");
      const nested = p.nested.map(([n, name]) => `(${n}, Nested::${name})`).join(", ");
      return [
        p.id,
        `PatternDefinition {\n                act: ${act},\n                captures: &[${captures}],\n                nested: &[${nested}],\n            }`,
      ];
    }),
    "Pattern",
  );
  g += `        }\n    }\n}\n`;

  g += `
/// The units of the grammar's numeric values, sorted and lowered.
/// The unit pattern's matcher looks up the word after a number here once.
pub(crate) const UNITS: [&str; ${units.length}] = [${units.map((u) => `"${u}"`).join(", ")}];

/// The length of the longest of [\`UNITS\`].
/// A longer word is none of them.
pub(crate) const LONGEST_UNIT: usize = ${Math.max(...units.map((u) => u.length))};

/// The scope that names each of [\`UNITS\`], in the same order.
pub(crate) const UNIT_SCOPES: [S; ${units.length}] = [${unitScopes.map((u) => `S::${u}`).join(", ")}];
`;
  g += startTable(patternList);
  g += ruleCodes("Css", "CSS");
  g += groupsCheck(
    "Css",
    patternList.flatMap((p) => [...p.captures.map(([n]) => n), ...p.nested.map(([n]) => n)]),
  );

  await writePatterns("css", patternList);

  await writeRust(`${root_dir}src/css/grammar.rs`, g);

  // Scope table.
  const s = scopeFile(
    "css-grammar.ts",
    "CSS",
    scopes,
    `    #[expect(
        clippy::enum_variant_names,
        reason = "the names are the scope names, and the unit s ends in S"
    )]
`,
  );
  await writeRust(`${root_dir}src/css/scope.rs`, s);
  if (Bun.env.CSS_DUMP)
    await Bun.write(
      Bun.env.CSS_DUMP,
      JSON.stringify(
        patternList.map((p) => ({
          id: p.id,
          role: p.act.role,
          regex: p.regex,
          captures: p.captures,
          nested: p.nested,
        })),
        null,
        0,
      ),
    );
  console.log(`${ruleList.length} rules, ${patternList.length} patterns, ${scopes.size} scopes`);
}
