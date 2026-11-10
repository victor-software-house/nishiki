// Writes src/astro/scope.rs and src/astro/grammar.rs from the `astro`
// grammar that Shiki 4.6.0 bundles (withastro/language-tools, MIT).
// It writes the scope names.
// For every rule it writes the scopes the rule adds, how it ends and the patterns it tries.
// The patterns keep the grammar's order, with its `include`s flattened.
// The matchers that decide whether a pattern matches are written by hand in src/astro/matchers.rs.
//
// It follows oracle/svelte-grammar.ts, whose grammar Astro's descends from:
//
// 1. An include of another grammar makes its rule embed that language.
//    `source.tsx#object-literal` starts TSX at that entry, through
//    `Grammar::embed_start`.
//    The patterns before the include become the rule's halts.
// 2. A rule's scope name with `$N` repeats begin capture N.
//    The rule carries the group as its payload.
//    A match's scope name with `$N` is an `Sc::Group`.
// 3. A capture group with patterns of its own is a nested list.
//    A group whose one pattern includes `source.js` is lexed again by the
//    JavaScript lexer.
// 4. The grammar's injections add the bodies of `<script>` and `<style>`.
//    The generator writes them as rules of their own.
//    Every rule that can be open inside a `<script>` or `<style>` tag tries
//    them first, as vscode-textmate tries an `L:` injection first.
//    An injection whose grammar nishiki does not lex, Stylus, gets no rule,
//    as vscode-textmate skips it when that grammar is not loaded.
//
// The grammar is edited in five places before it is read; each edit says why.
//
// Run with `bun run oracle/astro-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import astro from "@shikijs/langs@4.6.0/astro";
import html from "@shikijs/langs@4.6.0/html";
import {
  arms,
  groupsCheck,
  ruleCodes,
  scopeFile,
  startTable,
  tooLong,
  writeRust,
} from "./rule-grammar.ts";

type Captures = Record<string, { name?: string; patterns?: Node[] }>;
type Node = {
  name?: string;
  contentName?: string;
  match?: string;
  begin?: string;
  end?: string;
  while?: string;
  captures?: Captures;
  beginCaptures?: Captures;
  endCaptures?: Captures;
  whileCaptures?: Captures;
  patterns?: Node[];
  include?: string;
};
type Grammar = {
  scopeName: string;
  repository: Record<string, Node>;
  patterns: Node[];
  injections: Record<string, Node>;
};

const grammar = (astro as unknown as Grammar[]).find((lang) => lang.scopeName === "source.astro")!;
const repository = grammar.repository;
const htmlRepository = (html as unknown as Grammar[]).find(
  (lang) => lang.scopeName === "text.html.basic",
)!.repository;

// Edit 1: the event names and the entity names are html's.
// The matchers test them against the lists in src/html_words.rs, which
// oracle/html-words.ts writes from html's grammar.
{
  const events = repository["attributes-events"]!.begin!;
  const htmlEvents = htmlRepository["attribute"]!.patterns![2]!.begin!;
  const htmlBody = htmlEvents.slice(0, htmlEvents.lastIndexOf("(?!"));
  // Astro's lookahead escapes its backslash: `[-:\\w]` holds `\` and `w`, not `\w`.
  if (events !== `(${htmlBody})(?![-:\\\\w])`)
    throw new Error("attributes-events no longer spells html's event names");
  const entity = repository["entities"]!.patterns![0]!;
  if (entity.match !== htmlRepository["entities"]!.patterns![0]!.match) {
    throw new Error("entities no longer spells html's entity names");
  }
  // Edit 2: the named entity's `;` is group 912, after every group of the
  // name alternation. The alternation's groups become non-capturing, so the
  // `;` is group 3 and the tables index three groups, not 912.
  const head = "(&)(?=[A-Za-z])(";
  const tail = ")(;)";
  if (!entity.match!.startsWith(head) || !entity.match!.endsWith(tail))
    throw new Error("the entity regex changed shape");
  const body = entity.match!.slice(head.length, -tail.length);
  const inner = [...body.matchAll(/\((?!\?)/g)].length;
  if (inner + 3 !== 912 || entity.captures?.["912"] === undefined)
    throw new Error("the entity's `;` is no longer group 912");
  entity.match = `${head}${body.replace(/\((?!\?)/g, "(?:")}${tail}`;
  entity.captures = { "1": entity.captures!["1"]!, "3": entity.captures!["912"]! };
}

// Edit 3: `tags-lang` names `meta.$1.astro` twice and ends at a repeat of
// group 1, which is `script` or `style`. It becomes one rule per tag, with
// static names. Their shared patterns move to `tags-lang-modes`.
{
  const tagsLang = repository["tags-lang"]!;
  if (
    // Spelled in two parts, since the spelling check reads the second half of
    // `script` in the alternation as a typo.
    tagsLang.begin !== `<(s(?:c${"ript"}|tyle))` ||
    tagsLang.end !== "</\\1\\s*>|/>" ||
    tagsLang.name !== "meta.scope.tag.$1.astro meta.$1.astro"
  ) {
    throw new Error("tags-lang changed: check its split by tag");
  }
  repository["tags-lang-modes"] = { patterns: tagsLang.patterns! };
  const split = (tag: string): Node => ({
    ...tagsLang,
    begin: `<(${tag})`,
    end: `</${tag}\\s*>|/>`,
    name: `meta.scope.tag.${tag}.astro meta.${tag}.astro`,
    patterns: [{ include: "#tags-lang-modes" }],
  });
  repository["tags-lang"] = { patterns: [split("script"), split("style")] };
}

// Edit 5: vscode-textmate applies the injections while it lexes a begin
// capture again. In `tags-lang`'s begin capture, `<script` or `<style`, the
// tag's rule is open and no mode is yet, so the body of a tag without `lang`
// opens there when a `>` stands before the capture.
// The body then lexes the rest of the capture, which is always the same text:
// JavaScript reads `<script` as `<` and a name, and CSS reads `<style` as
// plain text.
// Each tag's begin capture tries that first, as a pattern of its own.
{
  const [script, style] = repository["tags-lang"]!.patterns!;
  const start = script!.beginCaptures?.["0"]?.patterns;
  if (start?.length !== 1 || start[0]!.include !== "#tags-start-node") {
    throw new Error("tags-lang's begin capture changed: check the body it opens");
  }
  script!.beginCaptures = {
    "0": {
      patterns: [
        {
          match: "(?<=>)(<)(script)",
          captures: {
            "1": { name: "meta.embedded.block.astro source.js keyword.operator.relational.js" },
            "2": { name: "meta.embedded.block.astro source.js variable.other.readwrite.js" },
          },
        },
        { include: "#tags-start-node" },
      ],
    },
  };
  style!.beginCaptures = {
    "0": {
      patterns: [
        { match: "(?<=>)<style", name: "meta.embedded.block.astro source.css" },
        { include: "#tags-start-node" },
      ],
    },
  };
}

// Edit 4: a backtick attribute value includes TSX's
// `template-substitution-element` and `string-character-escape`, the two
// patterns of TSX's template body, and ends at a repeat of its backtick.
// It becomes one embedding of TSX at that body, `typescript::start_template`,
// ending at a backtick.
{
  const literal = repository["attribute-literal"]!;
  const includes = literal.patterns!.map((p) => p.include).join(" ");
  if (
    literal.begin !== "(`)" ||
    literal.end !== "\\1" ||
    includes !== "source.tsx#template-substitution-element source.tsx#string-character-escape"
  ) {
    throw new Error("attribute-literal changed: check its embedding of the template body");
  }
  literal.end = "`";
  literal.patterns = [{ include: "source.tsx#template-body" }];
}

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

/** The identifier of a scope name; Astro's own names drop `.astro`, and the
 * JavaScript and other names keep their language as a suffix. */
function scopeIdent(name: string): string {
  const parts = name.split(".");
  if (parts[parts.length - 1] === "astro") parts.pop();
  let out = "";
  parts.forEach((part, index) => {
    for (const piece of part.split(/[-_]/)) {
      const capture = piece.charAt(0).toUpperCase() + piece.slice(1);
      out += index === 0 ? (abbreviations[piece] ?? capture) : capture;
    }
  });
  return out;
}

const scopes = new Map<string, string>([["Source", "source.astro"]]);
function scope(name: string): string {
  const ident = scopeIdent(name);
  const known = scopes.get(ident);
  if (known !== undefined && known !== name) throw new Error(`${ident}: ${known} and ${name}`);
  scopes.set(ident, name);
  return ident;
}

/** A scope of a match or a capture: a fixed one, or one built from a capture
 * group's text. */
type Sc = { fixed: string } | { prefix: string; group: number; suffix: string };
function scList(name: string | undefined): Sc[] {
  if (name === undefined) return [];
  return name
    .split(" ")
    .filter((part) => part !== "")
    .map((part): Sc => {
      const m = /^(.*?)\$(\d+)(.*)$/.exec(part);
      if (!m) return { fixed: scope(part) };
      return { prefix: m[1]!, group: Number(m[2]), suffix: m[3]! };
    });
}
/** The scopes of a static name; an empty name adds none, as in vscode-textmate. */
function staticList(name: string | undefined): Sc[] {
  if (name !== undefined && /\$\d/.test(name))
    throw new Error(`${name}: a dynamic name where a static one belongs`);
  return scList(name);
}

/** A rule's name with one scope that repeats begin capture `group`, such as
 * `meta.scope.tag.$1.astro meta.raw.astro`: the static scopes before it,
 * its prefix and suffix, and the static scopes after it. */
type Dynamic = { before: Sc[]; prefix: string; group: number; suffix: string; after: Sc[] };
function dynamicName(name: string): Dynamic | null {
  const parts = name.split(" ").filter((part) => part !== "");
  const at = parts.findIndex((part) => /\$\d/.test(part));
  if (at < 0) return null;
  const m = /^(.*)\$(\d)(.*)$/.exec(parts[at]!)!;
  if (/\$\d/.test(m[1]! + m[3]!)) throw new Error(`${name}: two groups in one scope`);
  if (parts.slice(at + 1).some((part) => /\$\d/.test(part)))
    throw new Error(`${name}: two dynamic scopes`);
  return {
    before: parts.slice(0, at).flatMap((part) => staticList(part)),
    prefix: m[1]!,
    group: Number(m[2]),
    suffix: m[3]!,
    after: parts.slice(at + 1).flatMap((part) => staticList(part)),
  };
}

// Embedded languages.

/** A language a rule hands its text to, and the entry it starts at: null for
 * the language's root, else the name of a `start_*` function of the
 * TypeScript module. */
type Embed = { lang: string; feature: string; entry: string | null };

/** What each include of another grammar stands for: a language and entry,
 * or nothing, for a grammar nishiki does not lex. */
const externals: Record<string, Embed | null> = {
  "source.ts": { lang: "TypeScript", feature: "lang-typescript", entry: null },
  "source.tsx": { lang: "Tsx", feature: "lang-tsx", entry: null },
  "source.tsx#object-literal": { lang: "Tsx", feature: "lang-tsx", entry: "start_object_literal" },
  "source.tsx#template-body": { lang: "Tsx", feature: "lang-tsx", entry: "start_template" },
  "source.js": { lang: "JavaScript", feature: "lang-javascript", entry: null },
  "source.json": { lang: "Json", feature: "lang-json", entry: null },
  "source.css": { lang: "Css", feature: "lang-css", entry: null },
  "source.css.postcss": { lang: "PostCss", feature: "lang-postcss", entry: null },
  "source.css.less": { lang: "Less", feature: "lang-less", entry: null },
  "source.css.scss": { lang: "Scss", feature: "lang-scss", entry: null },
  "source.sass": { lang: "Sass", feature: "lang-sass", entry: null },
  "source.stylus": null,
};

// Rules and patterns.

/** Readable ids for rules whose position would otherwise name them. */
const renames: Record<string, string> = {
  TagsLangAt0: "TagsLangScript",
  TagsLangAt1: "TagsLangStyle",
  TagsLangModesAt0: "LangJson",
  TagsLangModesAt1: "LangModule",
  TagsLangModesAt2: "LangNamed",
  AttributesEventsAt0: "EventEquals",
  EventEqualsAt2: "EventScript",
  EventScriptAt0: "EventScriptBare",
  EventScriptAt1: "EventScriptDouble",
  EventScriptDoubleAt0: "EventScriptDoubleRun",
  EventScriptDoubleAt1: "EventScriptDoubleLineComment",
  EventScriptDoubleAt2: "EventScriptDoubleBlockComment",
  EventScriptAt2: "EventScriptSingle",
  EventScriptSingleAt0: "EventScriptSingleRun",
  EventScriptSingleAt1: "EventScriptSingleLineComment",
  EventScriptSingleAt2: "EventScriptSingleBlockComment",
  AttributesKeyvalueAt0: "KeyvalueEquals",
  AttributesValueAt1: "ValueUnquoted",
  AttributesValueAt2: "ValueQuoted",
  EntitiesAt0: "EntityNamed",
  EntitiesAt1: "EntityDecimal",
  EntitiesAt2: "EntityHexadecimal",
  EntitiesAt3: "EntityAmbiguous",
  TagsNameAt0: "TagNameComponent",
  TagsNameAt1: "TagNameCustom",
  TagsNameAt2: "TagNamePlain",
  CommentsAt0: "CommentsInvalid",
  TextAt0: "Text",
  NestedScriptStartAt0: "ScriptStartInBody",
  NestedStyleStartAt0: "StyleStartInBody",
};

/** Readable ids for nested lists that their first user would name. */
const nestedRenames: Record<string, string> = {
  NestedAttributesEventsGroup0: "NestedAttributeName",
  NestedTagsLangScriptGroup0: "NestedScriptStart",
  NestedTagsLangStyleGroup0: "NestedStyleStart",
};

const pascal = (key: string) =>
  key
    .split(/[-_.]/)
    .map((piece) => piece.charAt(0).toUpperCase() + piece.slice(1))
    .join("");

type Act =
  | { role: "match"; name: Sc[] }
  | { role: "begin"; rule: string }
  | { role: "end" }
  | { role: "while" };
type Pattern = {
  id: string;
  regex: string;
  act: Act;
  captures: [number, Sc[]][];
  nested: [number, string][];
};
type Rule = {
  id: string;
  name: Sc[];
  dynamic: Dynamic | null;
  content: Sc[];
  /** The raw scope names of the rule's name and content, for the check
   * that no rule inside a language tag hides its body. */
  names: string[];
  end: string;
  while: string | null;
  /** The begin capture group that the end pattern repeats. */
  endBackref: number | null;
  /** The begin capture group the rule carries: the one its end or its name
   * repeats. */
  backref: number | null;
  embed: Embed | null;
  /** The patterns, and for an embedding rule, how many come before the
   * embedded grammar; the rest come after it. */
  patterns: string[];
  before: number;
};

const patternById = new Map<string, Pattern>();
const rules = new Map<string, Rule>();

/** A rule with no name, no end and no patterns yet. */
const bare = (id: string): Rule => ({
  id,
  name: [],
  dynamic: null,
  content: [],
  names: [],
  end: "",
  while: null,
  endBackref: null,
  backref: null,
  embed: null,
  patterns: [],
  before: 0,
});

/** The nested lists, by the include or the inline patterns they stand for.
 * Each is a rule with patterns only, which `retokenise` runs over the group. */
const nestedNames = new Map<string, string>();

/** The nested lists of a group that another language lexes again, by the grammar
 * its one pattern includes. The engine defers such a group to that
 * language (`Embeds::lex_nested`), so its rule has no patterns. */
const deferredNames: Record<string, string> = {
  "source.js": "NestedJavaScript",
};

function nestedName(group: string, patterns: Node[], context: string): string {
  const only = patterns.length === 1 ? patterns[0]! : null;
  const key =
    only?.include !== undefined
      ? only.include
      : patterns.map((p) => p.match ?? p.include ?? "?").join(" | ");
  const known = nestedNames.get(key);
  if (known !== undefined) return known;
  const deferred = only?.include === undefined ? undefined : deferredNames[only.include];
  const named =
    only?.include?.startsWith("#") === true
      ? `Nested${pascal(only.include.slice(1))}`
      : (deferred ?? `Nested${context}Group${group}`);
  const name = nestedRenames[named] ?? named;
  nestedNames.set(key, name);
  const rule = bare(name);
  rules.set(name, rule);
  if (deferred !== undefined) return name;
  const members = flatten(patterns, name);
  if (members.some((member) => "embed" in member))
    throw new Error(`${name}: a nested list embeds a grammar`);
  rule.patterns = members.map((member) => (member as { pattern: string }).pattern);
  return name;
}

function captureList(captures: Captures | undefined): [number, Sc[]][] {
  if (!captures) return [];
  return Object.entries(captures)
    .map(([group, value]): [number, Sc[]] => [Number(group), staticList(value.name)])
    .filter(([, names]) => names.length > 0)
    .sort((a, b) => a[0] - b[0]);
}

function nestedList(captures: Captures | undefined, context: string): [number, string][] {
  if (!captures) return [];
  const out: [number, string][] = [];
  for (const [group, value] of Object.entries(captures)) {
    if (value.patterns === undefined) continue;
    out.push([Number(group), nestedName(group, value.patterns, context)]);
  }
  return out.sort((a, b) => a[0] - b[0]);
}

/** One member of a flattened pattern list: a pattern id, or the embedded
 * grammar of an include of another one. */
type Member = { pattern: string } | { embed: Embed };

/** Registers a match or begin rule under `id`; returns the pattern id. */
function register(node: Node, id: string): string {
  const clash = patternById.get(id);
  if (clash && clash.regex !== (node.match ?? node.begin)) throw new Error(`${id} names two rules`);
  if (patternById.has(id)) return id;
  if (node.match !== undefined) {
    // vscode-textmate ignores the patterns of a match rule.
    patternById.set(id, {
      id,
      regex: node.match,
      act: { role: "match", name: scList(node.name) },
      captures: captureList(node.captures),
      nested: nestedList(node.captures, id),
    });
    return id;
  }
  if (node.begin === undefined || (node.end === undefined && node.while === undefined)) {
    throw new Error(`${id}: neither a match nor a begin rule`);
  }
  patternById.set(id, {
    id,
    regex: node.begin,
    act: { role: "begin", rule: id },
    captures: captureList(node.beginCaptures ?? node.captures),
    nested: nestedList(node.beginCaptures ?? node.captures, id),
  });
  const closing = node.end ?? node.while!;
  const closeId = node.end !== undefined ? `${id}End` : `${id}While`;
  const closeCaptures =
    node.end !== undefined
      ? (node.endCaptures ?? node.captures)
      : (node.whileCaptures ?? node.captures);
  patternById.set(closeId, {
    id: closeId,
    regex: closing,
    act: node.end !== undefined ? { role: "end" } : { role: "while" },
    captures: captureList(closeCaptures),
    nested: nestedList(closeCaptures, closeId),
  });
  const dynamic = node.name !== undefined ? dynamicName(node.name) : null;
  const endGroups = [...new Set([...closing.matchAll(/\\([1-9])/g)].map((m) => Number(m[1])))];
  if (endGroups.length > 1) throw new Error(`${id}: the end repeats more than one group`);
  const groups = [...new Set([...endGroups, ...(dynamic === null ? [] : [dynamic.group])])];
  if (groups.length > 1) throw new Error(`${id}: the end and the name repeat different groups`);
  const rule: Rule = {
    ...bare(id),
    name: dynamic === null ? staticList(node.name) : dynamic.before,
    dynamic,
    content: staticList(node.contentName),
    names: [node.name ?? "", node.contentName ?? ""]
      .flatMap((n) => n.split(" "))
      .filter((n) => n !== ""),
    end: node.end !== undefined ? closeId : "",
    while: node.while !== undefined ? closeId : null,
    endBackref: endGroups[0] ?? null,
    backref: groups[0] ?? null,
  };
  rules.set(id, rule);
  const members = flatten(node.patterns ?? [], id);
  const embeds = members.filter((member) => "embed" in member);
  if (embeds.length > 1) throw new Error(`${id}: embeds two grammars`);
  if (embeds.length === 1) {
    if (rule.backref !== null) throw new Error(`${id}: embeds and repeats a group`);
    rule.embed = (embeds[0] as { embed: Embed }).embed;
    rule.before = members.findIndex((member) => "embed" in member);
  }
  rule.patterns = members.flatMap((member) => ("pattern" in member ? [member.pattern] : []));
  return id;
}

/** The members a pattern list expands to, `include`s flattened. */
function flatten(list: Node[], context: string): Member[] {
  const out: Member[] = [];
  list.forEach((node, index) => {
    if (node.include === undefined) {
      out.push(...expand(node, `${context}At${index}`));
      return;
    }
    if (!node.include.startsWith("#")) {
      if (!(node.include in externals))
        throw new Error(`${context}: unknown include ${node.include}`);
      const embed = externals[node.include]!;
      if (embed !== null) out.push({ embed });
      return;
    }
    const key = node.include.slice(1);
    const target = repository[key];
    if (!target) throw new Error(`unknown include ${key}`);
    out.push(...expand(target, pascal(key)));
  });
  return out;
}

function expand(node: Node, id: string): Member[] {
  if (node.match !== undefined || node.begin !== undefined)
    return [{ pattern: register(node, renames[id] ?? id) }];
  if (node.patterns) return flatten(node.patterns, id);
  throw new Error(`${id}: empty rule`);
}

// The injections: the bodies of `<script>` and `<style>`.
// Each body becomes a rule; its matcher checks the tag and the `lang` value
// of the open rules, which stand for the selector.

/** Each body rule and the selectors it stands for. */
const bodies = new Map<string, string[]>();
for (const [selector, injection] of Object.entries(grammar.injections)) {
  const forms = injection.patterns!;
  if (forms.length !== 1) throw new Error(`${selector}: more than one form`);
  const node = forms[0]!;
  const include = node.patterns![0]!.include!;
  const embed = externals[include];
  if (embed === undefined) throw new Error(`${selector}: unknown include ${include}`);
  if (embed === null) continue; // a grammar the bundle lacks
  const id = `Body${embed.lang}`;
  bodies.set(id, [...(bodies.get(id) ?? []), selector]);
  register(node, id);
}

const rootMembers = flatten(grammar.patterns, "Root");
if (rootMembers.some((member) => "embed" in member)) throw new Error("the root embeds a grammar");
rules.set("Root", {
  ...bare("Root"),
  patterns: rootMembers.map((member) => (member as { pattern: string }).pattern),
});

/** Whether a rule's own scopes hold a `source` scope, which hides the
 * injections' selectors from the rules inside it (`- (meta source)`). */
const hides = (id: string) => rules.get(id)!.names.some((name) => name.startsWith("source."));

// The rules that can be open inside a `<script>` or `<style>` tag with no
// rule that hides the selectors around them: the tag's rules and every rule
// their patterns open, short of a rule that hides them.
// The injections' selectors can hold in each of them.
const inside = new Set<string>();
const queue = ["TagsLangScript", "TagsLangStyle"];
while (queue.length > 0) {
  const id = queue.pop()!;
  if (inside.has(id)) continue;
  const rule = rules.get(id);
  if (rule === undefined) throw new Error(`no rule ${id}`);
  inside.add(id);
  if (hides(id)) continue;
  for (const pattern of rule.patterns) {
    const act = patternById.get(pattern)!.act;
    if (act.role === "begin") queue.push(act.rule);
  }
}
/** The rules that try the bodies first: those inside a language tag whose
 * text no `source` scope of their own hides from the selectors.
 * Those are the rules that lex Astro's own patterns, and the backtick value,
 * which embeds TSX's template body with no `source` scope; its halts hold
 * where a body opens. */
const tryBodies = [...inside].filter((id) => !bodies.has(id) && !hides(id));
if (tryBodies.some((id) => rules.get(id)!.embed !== null && id !== "AttributeLiteral")) {
  throw new Error("an embedding rule other than the backtick value would try the bodies");
}
for (const id of tryBodies) {
  const rule = rules.get(id)!;
  rule.patterns = [...bodies.keys(), ...rule.patterns];
}
/** The end patterns that give way to a body: an `L:` injection wins a tie
 * with the rule's end, which the engine tries first. */
const yieldsToBodies = new Set(
  tryBodies.map((id) => rules.get(id)!.end).filter((end) => end !== ""),
);

// Output.

const scText = (list: Sc[]) =>
  list.length
    ? `scopes![${list
        .map((s) =>
          "fixed" in s
            ? `Sc::S(S::${s.fixed})`
            : `Sc::Group(${JSON.stringify(s.prefix)}, ${s.group}, ${JSON.stringify(s.suffix)})`,
        )
        .join(", ")}]`
    : "Sc::NO_SCOPES";

const ruleList = [...rules.values()];
const patternList = [...patternById.values()];
if (ruleList.length > 256) throw new Error(`${ruleList.length} rules do not fit one byte`);

let g = `//! The grammar's rules as tables. Generated by \`oracle/astro-grammar.ts\`
//! from the \`astro\` grammar Shiki 4.6.0 bundles; do not edit.
//!
//! A [\`Pattern\`] is one regular expression of the grammar; its doc comment shows
//! which.
//! Each rule lists the patterns it tries, in the grammar's order with its
//! \`include\`s flattened.
//! So the first that matches at the earliest position wins, as in a
//! \`TextMate\` engine.
//!
//! A rule that includes another grammar embeds that language; the patterns
//! it lists before the include are its halts.

use super::engine::Astro;
use super::scope::S;
use crate::rules::{self, embedded};
use crate::scope::scopes;

pub(crate) type Act = rules::Act<Astro>;
pub(crate) type PatternDefinition = rules::PatternDefinition<Astro>;
pub(crate) type RuleDefinition = rules::RuleDefinition<Astro>;
pub(crate) use crate::rules::Start;

/// A scope of a rule or a capture group.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Sc {
    /// A fixed scope.
    S(S),
    /// A scope made of a prefix, the text of a capture group of the match and
    /// a suffix.
    Group(&'static str, u8, &'static str),
}

impl Sc {
    /// The code that keys a list of scopes made with scopes!.
    /// A fixed scope's list has one.
    /// A scope a capture group spells has none.
    /// Its lists are never cached.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless \`repr(u16)\` enum has no \`From\` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        match self {
            Sc::S(scope) => scope as u16,
            Sc::Group(..) => u16::MAX,
        }
    }

    /// The empty scope list.
    pub(crate) const NO_SCOPES: crate::scope::Scopes<Sc> = &crate::scope::Keyed::new([], []);
}

/// The start function of a TSX entry, when TSX is built.
macro_rules! entry {
    ($start:ident) => {{
        #[cfg(feature = "lang-tsx")]
        let start: Option<fn(&mut crate::arena::Arena, u32) -> u32> = Some(crate::typescript::$start);
        #[cfg(not(feature = "lang-tsx"))]
        let start = None;
        start
    }};
}

`;

g += `/// A rule of the grammar that can be open at the end of a line.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub(crate) enum Rule {\n`;
for (const r of ruleList) g += `    ${r.id},\n`;
g += `}\n\nimpl Rule {\n    /// Every rule, in code order.\n    pub(crate) const ALL: [Rule; ${ruleList.length}] = [\n`;
for (const r of ruleList) g += `        Rule::${r.id},\n`;
g += `    ];\n\n${tooLong}    pub(crate) fn definition(self) -> RuleDefinition {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    `RuleDefinition {\n                name: ${scText(r.name)},\n                content: ${scText(r.content)},\n                end: ${r.end ? `Some(Pattern::${r.end})` : "None"},\n                backref: ${r.endBackref === null ? "None" : `Some(${r.endBackref})`},\n                patterns: &[${r.patterns.map((p) => `Pattern::${p}`).join(", ")}],\n            }`,
  ]),
  "Rule",
);
/** The features Cargo.toml defines: an embedded language whose feature is
 * not there yet embeds nothing. */
const features = new Set(
  [...(await Bun.file(`${root_dir}Cargo.toml`).text()).matchAll(/^([a-z_-]+) = /gm)].map(
    (m) => m[1]!,
  ),
);
for (const r of ruleList) {
  if (r.embed !== null && !features.has(r.embed.feature)) {
    console.warn(
      `${r.id}: no feature ${r.embed.feature} in Cargo.toml yet; the rule embeds nothing`,
    );
  }
}
g += `        }\n    }\n\n    /// What a rule carries from line to line.\n    /// A rule that embeds a language carries that language's state, when its\n    /// feature is on.\n    /// A rule whose end or name repeats a begin capture group carries that\n    /// group.\n    #[inline]\n    pub(crate) fn payload(self) -> rules::Payload {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    r.embed && features.has(r.embed.feature)
      ? `embedded!(${r.embed.lang}, "${r.embed.feature}").map_or(rules::Payload::None, rules::Payload::Embed)`
      : r.backref === null
        ? "rules::Payload::None"
        : `rules::Payload::Backref(${r.backref})`,
  ]),
  "Rule",
);
g += `        }\n    }\n\n    /// The TSX entry an embedding rule starts at; none for a language's root.\n    pub(crate) fn embed_start(self) -> Option<fn(&mut crate::arena::Arena, u32) -> u32> {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [
    r.id,
    r.embed?.entry ? `entry!(${r.embed.entry})` : "None",
  ]),
  "Rule",
);
g += `        }\n    }\n\n    /// The prefix of the name scope that repeats the rule's payload.\n    pub(crate) fn name_delimiter(self) -> Option<&'static str> {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [r.id, r.dynamic ? `Some("${r.dynamic.prefix}")` : "None"]),
  "Rule",
);
g += `        }\n    }\n\n    /// The text after the payload in that name scope.\n    pub(crate) fn name_suffix(self) -> &'static str {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [r.id, `"${r.dynamic?.suffix ?? ""}"`]),
  "Rule",
);
g += `        }\n    }\n\n    /// The static name scopes after that name scope.\n    pub(crate) fn name_after(self) -> crate::scope::Scopes<Sc> {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [r.id, scText(r.dynamic?.after ?? [])]),
  "Rule",
);
g += `        }\n    }\n\n    /// Whether one of the rule's own scopes, a \`source\` scope, keeps the\n    /// bodies' injections out of the rules inside it.\n    pub(crate) fn hides_bodies(self) -> bool {\n        match self {\n`;
g += arms(
  ruleList.map((r): [string, string] => [r.id, hides(r.id) ? "true" : "false"]),
  "Rule",
);
g += `        }\n    }\n}\n\n`;

g += `/// A pattern of the grammar.\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n#[repr(u16)]\npub(crate) enum Pattern {\n`;
const docs = (s: string) => {
  const one = s.replace(/\n/g, "\\n");
  return one.length > 56 ? `…` : one;
};
for (const p of patternList) {
  g += `    /// The ${p.act.role} \`${docs(p.regex).replaceAll("`", "'")}\`.\n    ${p.id},\n`;
}
g += `}\n\nimpl Pattern {\n    /// Every pattern, for the tool that compares the matchers with Oniguruma.\n    #[cfg(test)]\n    pub(crate) const ALL: [Pattern; ${patternList.length}] = [\n${patternList.map((p) => `        Pattern::${p.id},\n`).join("")}    ];\n\n${tooLong}    pub(crate) fn definition(self) -> PatternDefinition {\n        match self {\n`;
g += arms(
  patternList.map((p): [string, string] => {
    const act =
      p.act.role === "match"
        ? `Act::Match(${scText(p.act.name)})`
        : p.act.role === "begin"
          ? `Act::Begin(Rule::${p.act.rule})`
          : p.act.role === "end"
            ? "Act::End"
            : "Act::Match(Sc::NO_SCOPES)";
    const captures = p.captures.map(([n, names]) => `(${n}, ${scText(names)})`).join(", ");
    const nested = p.nested.map(([n, name]) => `(${n}, Rule::${name})`).join(", ");
    return [
      p.id,
      `PatternDefinition {\n                act: ${act},\n                captures: &[${captures}],\n                nested: &[${nested}],\n            }`,
    ];
  }),
  "Pattern",
);
g += `        }\n    }\n\n    /// Whether the pattern is the end of a rule inside a \`<script>\` or\n    /// \`<style>\` tag, which gives way to a body that opens at the same place.\n    pub(crate) fn yields_to_bodies(self) -> bool {\n        match self {\n`;
g += arms(
  patternList.map((p): [string, string] => [p.id, yieldsToBodies.has(p.id) ? "true" : "false"]),
  "Pattern",
);
g += `        }\n    }\n}\n`;

/** The groups a pattern's tables read: its captures, its nested groups, the
 * group its rule's name or end repeats, and the groups its match name spells. */
const groupsOf = (p: Pattern): number[] => {
  const rule = p.act.role === "begin" ? rules.get(p.act.rule)! : null;
  const named =
    p.act.role === "match" ? p.act.name.flatMap((s) => ("group" in s ? [s.group] : [])) : [];
  return [
    ...new Set([
      ...p.captures.map(([n]) => n),
      ...p.nested.map(([n]) => n),
      ...(rule?.backref != null ? [rule.backref] : []),
      ...named,
    ]),
  ].sort((a, b) => a - b);
};

g += startTable(patternList, "quote");
g += groupsCheck("Astro", patternList.flatMap(groupsOf));
g += ruleCodes("Astro", "ASTRO");

await writeRust(`${root_dir}src/astro/grammar.rs`, g);

const scopeText = scopeFile("astro-grammar.ts", "ASTRO", scopes, "", false);
await writeRust(`${root_dir}src/astro/scope.rs`, scopeText);

// With RX_DUMP set to a file, writes each pattern for oracle/onig.ts. An end
// that repeats a group of its begin is left out: its text comes from the
// line that opened the rule, which a probe case does not have.
if (Bun.env["RX_DUMP"]) {
  await Bun.write(
    Bun.env["RX_DUMP"],
    patternList
      .filter((p) => !(p.act.role === "end" && /\\[1-9]/.test(p.regex)))
      .map((p) =>
        JSON.stringify({ id: p.id, regex: p.regex, groups: groupsOf(p).filter((n) => n > 0) }),
      )
      .join("\n") + "\n",
  );
}
console.log(
  `${ruleList.length} rules, ${patternList.length} patterns, ${scopes.size} scopes, ${tryBodies.length} rules try the bodies`,
);
