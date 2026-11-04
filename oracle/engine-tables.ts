// Compiles Shiki's grammar for a language into the tables of nishiki's
// TextMate engine, as one Rust source file.
//
// Usage: bun oracle/engine-tables.ts [--json] <out dir> <spec>...
//
// A table for a directory under `src/` ships in the crate and is written as
// `<out dir>/grammar.rs`, which the language's module includes, as in
// `bun oracle/engine-tables.ts src/powershell powershell`; any other
// directory gets `<id>.rs`, as the test tables in `tests/engine/` are.
//
// With `--json` it writes `<id>.json` instead, the same tables as data, for
// the survey test that runs the engine on many grammars without compiling
// them in (src/textmate/survey.rs).
//
// A spec is a Shiki language id, or an id with the languages it may embed
// joined by `+`, such as `markdown+rust+shellscript`: the grammar of the
// first, with the others available to its includes, as Shiki loads them. `*`
// stands for every language the oracle's inputs use, which is the set the
// oracle's Shiki highlighter loads.
//
// `~` followed by a language id leaves that language out, even when the
// registration of a language in the spec brings it along, as Julia's brings
// C++ and R: `julia~cpp~r` loads Julia with Python, JavaScript and SQL only.
// A rule that includes a language left out is dropped, as Shiki drops it for
// a grammar it has not loaded. A grammar the crate must not bundle stays out
// of a shipped table this way.
//
// `>` followed by a language id hands that language a region, as in
// `haml>ruby>javascript`. `oracle/hand-lexed.ts` lists the languages it takes.
// An include of the language's whole grammar becomes a hand-off. The grammar
// is neither loaded nor compiled. The list the include stood in holds the
// rules before the include, then the rules after it. The hand-off names that
// list. It also names the two parts as lists of their own.
//
// The tables hold what the engine needs and nothing it can work out later:
// - the rules, with their includes already followed;
// - the patterns as text, compiled on first use;
// - the scope names, interned.
// A rule whose patterns all include a grammar that is not loaded is dropped,
// as vscode-textmate drops it.
// The compiled patterns and the lists' scanners go in cells.
// Each grammar allocates its cells on first use.
//
// Every grammar the Rust tables hold needs its licence in `oracle/licences.ts`;
// the run fails, naming the grammar, when one is missing, or when a table the
// crate ships holds a refused one.
import { bundledLanguages } from "shiki@4.6.0";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { inputs } from "./inputs.ts";
import { handLexed } from "./hand-lexed.ts";
import { allowed, refused } from "./licences.ts";
import { writeRust } from "./rule-grammar.ts";

type Json = Record<string, unknown>;
/** The repositories an include can name, innermost last: a rule without a
 * begin or match may carry its own, which shadows the outer ones. */
type Env = Json[];
type Cap = { index: number; name: string[]; contentName: string[]; retokenize: number };
type Rule = {
  kind: "Match" | "BeginEnd" | "BeginWhile" | "Container";
  name: string[];
  contentName: string[];
  begin: number;
  end: number;
  while: number;
  captures: Cap[];
  endCaptures: Cap[];
  patterns: number[];
  applyEndLast: boolean;
  /** Includes the rule's list had that could not be resolved. */
  missing: boolean;
};

const NONE = 0xffffffff;
/** Where the ids of hand-offs start in a list: above every rule's. */
const HAND_OFF = 1 << 30;
const idx = (n: number) => (n === NONE ? "usize::MAX" : String(n));

const args = process.argv.slice(2);
const asJson = args[0] === "--json";
if (asJson) args.shift();
const outDir = args[0];
const ids = args.slice(1);
if (!outDir || ids.length === 0) throw new Error("usage: engine-tables.ts <out dir> <language id>...");

/** Every grammar a language needs, by scope name. */
async function load(spec: string, handed: string[]): Promise<{ docs: Map<string, Json>; scope: string }> {
  const docs = new Map<string, Json>();
  let scope = "";
  const [included, ...excluded] = spec.split("~");
  const left = new Set([...excluded, ...handed]);
  const ids = included.split("+").flatMap((id) => (id === "*" ? [...new Set(inputs.map((i) => i.lang))] : [id]));
  for (const id of ids) {
    const loader = bundledLanguages[id as keyof typeof bundledLanguages];
    if (!loader) throw new Error(`Shiki has no language ${id}`);
    const registrations = ((await loader()) as { default: Json[] }).default;
    for (const grammar of registrations) {
      if (!left.has(String(grammar.name))) docs.set(String(grammar.scopeName), grammar);
    }
    if (scope === "") scope = String(registrations[registrations.length - 1].scopeName);
  }
  return { docs, scope };
}

/** The language a spec compiles: its first id. */
const base = (spec: string) => spec.split(/[+~>]/)[0];

class Builder {
  rules: Rule[] = [];
  patterns: string[] = [];
  patternIds = new Map<string, number>();
  names: string[] = [];
  nameIds = new Map<string, number>();
  roots = new Map<string, number>();
  /** Each raw rule object is compiled once, as in vscode-textmate. */
  compiled = new Map<object, number>();
  /** The Shiki names of the grammars whose rules the tables hold. */
  used = new Set<string>();

  /** The `Language` variant of each hand-off, by its id less `HAND_OFF`. */
  handOffs: string[] = [];

  constructor(
    private docs: Map<string, Json>,
    private base: string,
    /** The `Language` variant of each handed grammar, by root scope. */
    private handed: Map<string, string>,
  ) {}

  /** The id that stands in a list for a hand-off to `variant`. */
  handOff(variant: string): number {
    let index = this.handOffs.indexOf(variant);
    if (index < 0) {
      index = this.handOffs.length;
      this.handOffs.push(variant);
    }
    return HAND_OFF + index;
  }

  pattern(source: string): number {
    let id = this.patternIds.get(source);
    if (id === undefined) {
      id = this.patterns.length;
      this.patterns.push(source);
      this.patternIds.set(source, id);
    }
    return id;
  }

  /** A rule or capture name as the names it adds: one table index per scope. */
  scopes(text: unknown): string[] {
    return typeof text === "string" ? text.split(/\s+/).filter((s) => s.length > 0) : [];
  }

  reserve(kind: Rule["kind"]): number {
    this.rules.push({
      kind,
      name: [],
      contentName: [],
      begin: NONE,
      end: NONE,
      while: NONE,
      captures: [],
      endCaptures: [],
      patterns: [],
      applyEndLast: false,
      missing: false,
    });
    return this.rules.length - 1;
  }

  root(scope: string): number | undefined {
    const known = this.roots.get(scope);
    if (known !== undefined) return known;
    const doc = this.docs.get(scope);
    if (!doc) return undefined;
    this.used.add(String(doc.name));
    const id = this.reserve("Container");
    this.roots.set(scope, id);
    const env: Env = [(doc.repository as Json | undefined) ?? {}];
    const [patterns, missing] = this.list(scope, (doc.patterns as Json[]) ?? [], env);
    this.rules[id].patterns = patterns;
    this.rules[id].missing = missing;
    return id;
  }

  /** The rule of a raw rule object, compiled on first use. */
  entry(scope: string, value: Json, env: Env): number {
    const known = this.compiled.get(value);
    if (known !== undefined) return known;
    const id = this.reserve("Container");
    this.compiled.set(value, id);
    this.rule(scope, value, id, env);
    return id;
  }

  include(scope: string, target: string, env: Env): number | undefined {
    const variant = this.handed.get(target);
    if (variant !== undefined) return this.handOff(variant);
    if (target === "$self") return this.root(scope);
    if (target === "$base") return this.root(this.base);
    if (target.startsWith("#")) {
      const name = target.slice(1);
      for (let i = env.length - 1; i >= 0; i--) {
        const found = env[i][name] as Json | undefined;
        if (found) return this.entry(scope, found, env);
      }
      return undefined;
    }
    const hash = target.indexOf("#");
    if (hash >= 0) {
      const other = target.slice(0, hash);
      const doc = this.docs.get(other);
      const repository = doc?.repository as Json | undefined;
      const found = repository?.[target.slice(hash + 1)] as Json | undefined;
      if (!found) return undefined;
      this.used.add(String(doc?.name));
      return this.entry(other, found, [repository as Json]);
    }
    return this.root(target);
  }

  /** The rules of a list, and whether any were dropped. */
  list(scope: string, list: Json[], env: Env): [number[], boolean] {
    const out: number[] = [];
    for (const pattern of list) {
      let id: number | undefined;
      if (typeof pattern.include === "string") {
        id = this.include(scope, pattern.include, env);
        if (id === undefined) continue;
        if (id >= HAND_OFF) {
          out.push(id);
          continue;
        }
      } else {
        id = this.rule(scope, pattern, undefined, env);
      }
      const rule = this.rules[id];
      if (rule.kind !== "Match" && rule.missing && rule.patterns.length === 0) continue;
      out.push(id);
    }
    return [out, out.length !== list.length];
  }

  captures(scope: string, value: unknown, env: Env): Cap[] {
    if (typeof value !== "object" || value === null) return [];
    const out: Cap[] = [];
    for (const [key, raw] of Object.entries(value as Json)) {
      const index = Number(key);
      if (!Number.isInteger(index)) continue;
      if (typeof raw !== "object" || raw === null) continue;
      const capture = raw as Json;
      out.push({
        index,
        name: this.scopes(capture.name),
        contentName: this.scopes(capture.contentName),
        retokenize: capture.patterns ? this.rule(scope, capture, undefined, env, true) : NONE,
      });
    }
    return out.sort((a, b) => a.index - b.index);
  }

  rule(scope: string, value: Json, into: number | undefined, outer: Env, container = false): number {
    // As Shiki reads them: an empty `match` is no match, an empty `while` is
    // none, and an empty or missing `end` never matches.
    const kind: Rule["kind"] = container
      ? "Container"
      : value.match
        ? "Match"
        : value.begin !== undefined
          ? value.while
            ? "BeginWhile"
            : "BeginEnd"
          : "Container";
    const env: Env =
      kind === "Container" && value.repository ? [...outer, value.repository as Json] : outer;
    const id = into ?? this.reserve(kind);
    const rule = this.rules[id];
    rule.kind = kind;
    rule.name = this.scopes(value.name);
    rule.contentName = this.scopes(value.contentName);
    rule.applyEndLast = value.applyEndPatternLast === 1 || value.applyEndPatternLast === true;
    if (kind === "Match") {
      rule.begin = this.pattern(String(value.match));
      rule.captures = this.captures(scope, value.captures, env);
    } else if (kind === "BeginEnd" || kind === "BeginWhile") {
      rule.begin = this.pattern(String(value.begin));
      rule.captures = this.captures(scope, value.beginCaptures ?? value.captures, env);
      if (kind === "BeginEnd") {
        rule.end = value.end ? this.pattern(String(value.end)) : NONE;
        rule.endCaptures = this.captures(scope, value.endCaptures ?? value.captures, env);
      } else {
        rule.while = this.pattern(String(value.while));
        rule.endCaptures = this.captures(scope, value.whileCaptures ?? value.captures, env);
      }
    }
    if (kind !== "Match") {
      const [patterns, missing] = this.list(scope, (value.patterns as Json[]) ?? [], env);
      rule.patterns = patterns;
      rule.missing = missing;
    }
    return id;
  }

  /** Follows the includes of every rule, leaving only rules that match. */
  flatten(): void {
    const flat = new Map<number, number[]>();
    const expand = (list: number[], out: number[], seen: Set<number>) => {
      for (const id of list) {
        if (id < HAND_OFF && this.rules[id].kind === "Container") {
          if (seen.has(id)) continue;
          seen.add(id);
          expand(this.rules[id].patterns, out, seen);
        } else {
          out.push(id);
        }
      }
    };
    for (let id = 0; id < this.rules.length; id++) {
      const rule = this.rules[id];
      const out: number[] = [];
      expand(rule.patterns, out, new Set([id]));
      flat.set(id, out);
    }
    for (let id = 0; id < this.rules.length; id++) this.rules[id].patterns = flat.get(id) ?? [];
  }

  /** The table index of a scope name; the top bit marks a name built from captures. */
  nameRef(name: string): number {
    let id = this.nameIds.get(name);
    if (id === undefined) {
      id = this.names.length;
      this.names.push(name);
      this.nameIds.set(name, id);
    }
    return name.includes("$") ? id | 0x8000 : id;
  }
}

function rustString(text: string): string {
  let out = '"';
  for (const ch of text) {
    const code = ch.codePointAt(0) as number;
    if (ch === "\\") out += "\\\\";
    else if (ch === '"') out += '\\"';
    else if (ch === "\n") out += "\\n";
    else if (ch === "\r") out += "\\r";
    else if (ch === "\t") out += "\\t";
    else if (code < 0x20 || code === 0x7f) out += `\\u{${code.toString(16)}}`;
    // A character that NFC changes, such as U+F900, or a combining mark after
    // a range's dash, would leave the literal non-NFC, which clippy refuses.
    else if (ch.normalize("NFC") !== ch || /\p{M}/u.test(ch)) out += `\\u{${code.toString(16)}}`;
    else out += ch;
  }
  return out + '"';
}

/** Whether `source` uses `\G`, which matches where the last match ended. */
function usesLastMatchEnd(source: string): boolean {
  for (let i = 0; i + 1 < source.length; i++) {
    if (source[i] === "\\") {
      if (source[i + 1] === "G") return true;
      i++;
    }
  }
  return false;
}

function hasBackrefs(source: string): boolean {
  for (let i = 0; i + 1 < source.length; i++) {
    if (source[i] === "\\") {
      if (/[1-9]/.test(source[i + 1])) return true;
      i++;
    }
  }
  return false;
}

/** Whether the tables ship in the crate: written under `src/`. */
const shipped = resolve(outDir).startsWith(fileURLToPath(new URL("../src/", import.meta.url)));

/** Fails unless every grammar the tables hold has its licence, read at its
 * source, in `oracle/licences.ts`. Tables under `src/` ship in the crate, so
 * each of their grammars must also be allowed, with its origin in the notices
 * the crate ships. The JSON tables feed a survey and are not kept, so they
 * skip the gate. */
function gate(spec: string, used: Set<string>) {
  const notices = readFileSync(new URL("../THIRD-PARTY-NOTICES.md", import.meta.url), "utf8");
  const problems = [...used].sort().flatMap((name) => {
    const reason = refused[name];
    if (reason) return shipped ? [`${name} is never bundled: ${reason}`] : [];
    const entry = allowed[name];
    if (!entry) return [`${name} has no licence in oracle/licences.ts`];
    if (shipped && !notices.includes(`https://github.com/${entry.origin}`))
      return [`${name}: ${entry.origin} is not in THIRD-PARTY-NOTICES.md`];
    return [];
  });
  if (problems.length > 0) throw new Error(`${spec}: ${problems.join("; ")}`);
}

mkdirSync(outDir, { recursive: true });
for (const id of ids) {
  const [spec, ...handedIds] = id.split(">");
  const handed = new Map<string, string>();
  const features: string[] = [];
  for (const handedId of handedIds) {
    const entry = handLexed[handedId];
    if (!entry) throw new Error(`${id}: ${handedId} is not in oracle/hand-lexed.ts`);
    handed.set(entry.scope, entry.variant);
    features.push(entry.feature);
  }
  const { docs, scope } = await load(spec, handedIds);
  const builder = new Builder(docs, scope, handed);
  const root = builder.root(scope);
  if (root === undefined) throw new Error(`${id}: no root rule`);
  // The grammar's own injections, then those other grammars inject into it.
  const rootDoc = docs.get(scope) as Json;
  const injections: [string, number][] = Object.entries((rootDoc.injections as Json | undefined) ?? {}).map(
    ([selector, value]) => [selector, builder.entry(scope, value as Json, [(rootDoc.repository as Json | undefined) ?? {}])],
  );
  // Grammars that name this one in `injectTo` inject their rules into it,
  // where their `injectionSelector` matches.
  // A grammar that only wraps others, as ts-tags wraps source.ts, takes the
  // injections of those it includes whole.
  const wrapped = new Set(
    ((rootDoc.patterns as Json[] | undefined) ?? [])
      .map((p) => String(p.include ?? ""))
      .filter((include) => include !== "" && !include.startsWith("#") && !include.startsWith("$") && !include.includes("#")),
  );
  for (const [injectScope, doc] of docs) {
    const targets = doc.injectTo as string[] | undefined;
    if (
      injectScope === scope ||
      !doc.injectionSelector ||
      !targets?.some((target) => target === scope || wrapped.has(target))
    )
      continue;
    const injected = builder.root(injectScope);
    if (injected !== undefined) injections.push([String(doc.injectionSelector), injected]);
  }
  if (!asJson) gate(id, builder.used);
  builder.flatten();
  const refs = (list: string[]) => `&[${list.map((n) => builder.nameRef(n)).join(", ")}]`;
  const caps = (list: Cap[]) =>
    `&[${list
      .map(
        (c) =>
          `Cap { index: ${c.index}, name: ${refs(c.name)}, content_name: ${refs(c.contentName)}, retokenize: ${idx(c.retokenize)} }`,
      )
      .join(", ")}]`;
  const listIds = new Map<string, number>();
  const lists: number[][] = [];
  const plain = (rules: number[], handOff: number) => {
    const key = handOff === NONE ? rules.join(",") : `${handOff}>${rules.join(",")}`;
    let id = listIds.get(key);
    if (id === undefined) {
      id = lists.length;
      lists.push(rules);
      listIds.set(key, id);
    }
    return id;
  };
  type HandOffRow = { list: number; variant: string; before: number; after: number };
  const handOffRows: HandOffRow[] = [];
  const handOffIds = new Map<string, number>();
  // A list with a hand-off holds the rules before it, then the rules after it.
  // It stays apart from a plain list with the same rules. The hand-off names
  // the list and its two parts.
  const listOf = (patterns: number[], owner: Rule) => {
    const at = patterns.findIndex((p) => p >= HAND_OFF);
    if (at < 0) return plain(patterns, NONE);
    const before = patterns.slice(0, at);
    const after = patterns.slice(at + 1);
    if (after.some((p) => p >= HAND_OFF)) throw new Error(`${id}: a list with two hand-offs`);
    // The tokenizer finds where these patterns start before the language
    // runs. It searches from the region's start, then past each start. `\G`
    // matches where the last match ended. A search from another place cannot
    // know that place. The generator refuses `\G` in these patterns.
    const halting = [...before, ...after].map((r) => builder.rules[r].begin);
    if (owner.kind === "BeginEnd") halting.push(owner.end);
    const anchored = halting.filter((p) => p !== NONE && usesLastMatchEnd(builder.patterns[p]));
    if (anchored.length > 0)
      throw new Error(`${id}: a hand-off whose halts use \\G: ${anchored.map((p) => builder.patterns[p]).join(", ")}`);
    const row: HandOffRow = {
      list: NONE,
      variant: builder.handOffs[patterns[at] - HAND_OFF],
      before: plain(before, NONE),
      after: plain(after, NONE),
    };
    const key = JSON.stringify(row);
    let handOff = handOffIds.get(key);
    if (handOff === undefined) {
      handOff = handOffRows.length;
      handOffRows.push(row);
      handOffIds.set(key, handOff);
    }
    const list = plain([...before, ...after], handOff);
    handOffRows[handOff].list = list;
    return list;
  };
  const rules = builder.rules
    .map(
      (r) =>
        `    Rule { kind: RuleKind::${r.kind}, name: ${refs(r.name)}, content_name: ${refs(r.contentName)}, begin: ${idx(r.begin)}, end: ${idx(r.end)}, until: ${idx(r.while)}, captures: ${caps(r.captures)}, end_captures: ${caps(r.endCaptures)}, list: ${listOf(r.patterns, r)}, apply_end_last: ${r.applyEndLast} },`,
    )
    .join("\n");
  const listSource = () => lists.map((l) => `    List::new(&[${l.join(", ")}]),`).join("\n");
  const handOffSource = handOffRows
    .map(
      (h) =>
        `    HandOff { list: ${h.list}, language: Language::${h.variant}, before: ${h.before}, after: ${h.after} },`,
    )
    .join("\n");
  const patterns = builder.patterns
    .map((p) => `    Pattern::new(${rustString(p)}),`)
    .join("\n");
  const names = builder.names.map((n) => `    ${rustString(n)},`).join("\n");
  if (asJson) {
    if (handOffRows.length > 0) throw new Error(`${id}: the JSON tables hold no hand-offs`);
    const none = (n: number) => (n === NONE ? -1 : n);
    const cap = (c: Cap) => ({
      index: c.index,
      name: c.name.map((n) => builder.nameRef(n)),
      contentName: c.contentName.map((n) => builder.nameRef(n)),
      retokenize: none(c.retokenize),
    });
    const data = {
      scope,
      root,
      names: builder.names,
      patterns: builder.patterns.map((p) => ({ source: p, backrefs: hasBackrefs(p) })),
      lists,
      injections: injections.map(([selector, rule]) => ({ selector, rule })),
      rules: builder.rules.map((r) => ({
        kind: r.kind,
        name: r.name.map((n) => builder.nameRef(n)),
        contentName: r.contentName.map((n) => builder.nameRef(n)),
        begin: none(r.begin),
        end: none(r.end),
        until: none(r.while),
        captures: r.captures.map(cap),
        endCaptures: r.endCaptures.map(cap),
        list: listOf(r.patterns, r),
        applyEndLast: r.applyEndLast,
      })),
    };
    writeFileSync(`${outDir}/${base(id)}.json`, JSON.stringify(data));
    console.log(`${id}: ${builder.rules.length} rules, ${builder.patterns.length} patterns`);
    continue;
  }
  // A grammar with no capture names no \`Cap\`, and an unused import fails the build.
  const capturing = builder.rules.some((r) => r.captures.length > 0 || r.endCaptures.length > 0);
  const imports = [
    ...(capturing ? ["Cap"] : []),
    "Grammar",
    ...(handOffRows.length > 0 ? ["HandOff"] : []),
    "Injection",
    "List",
    "Pattern",
    "Rule",
    "RuleKind",
  ];
  const handsOff = handOffRows.length > 0;
  const source = `//! The tables of Shiki 4.6.0's \`${id}\` grammar, made by \`oracle/engine-tables.ts\`.
//! Do not edit: regenerate with \`bun oracle/engine-tables.ts\`.
${handsOff ? `//!\n//! It hands regions to languages lexed by hand, so it needs the features ${features.map((f) => `\`${f}\``).join(", ")}.\n` : ""}
use std::sync::OnceLock;

use crate::textmate::grammar::{${imports.join(", ")}};
${handsOff ? "use crate::language::Language;\n" : ""}
static NAMES: &[&str] = &[
${names}
];

static PATTERNS: [Pattern; ${builder.patterns.length}] = [
${patterns}
];

static LISTS: [List; ${lists.length}] = [
${listSource()}
];

static INJECTIONS: &[Injection] = &[
${injections.map(([selector, rule]) => `    Injection { selector: ${rustString(selector)}, rule: ${rule} },`).join("\n")}
];

static RULES: &[Rule] = &[
${rules}
];
${handsOff ? `\nstatic HAND_OFFS: &[HandOff] = &[\n${handOffSource}\n];\n` : ""}
/// The grammar.
pub(crate) static GRAMMAR: Grammar = Grammar {
    scope: ${rustString(scope)},
    names: NAMES,
    rules: RULES,
    lists: &LISTS,
    injections: INJECTIONS,
    patterns: &PATTERNS,
    root: ${root},
    hand_offs: ${handsOff ? "HAND_OFFS" : "&[]"},
    cells: OnceLock::new(),
    full_cells: OnceLock::new(),
};
`;
  await writeRust(`${outDir}/${shipped ? "grammar" : base(id)}.rs`, source);
  console.log(`${id}: ${builder.rules.length} rules, ${builder.patterns.length} patterns, ${builder.names.length} names`);
}
