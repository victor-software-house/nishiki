// Compiles Shiki's grammar for a language into the tables of nishiki's
// TextMate engine, as JSON. It is the oracle for the Rust compiler, which
// writes every table the engine runs.
//
// Usage: bun oracle/engine-tables.ts [--every] <out dir> <spec>...
//
// It writes `<id>.json`, the tables as data.
// Beside it, `<id>.sources.jsonl` holds the grammars the tables came from:
// a header line with the root scope and the handed languages, then one
// grammar a line. The Rust compiler's comparison test links them and checks
// its tables equal these (`mod survey` in src/textmate/compile/tests.rs).
// A spec it refuses writes `<id>.error` and the run goes on.
// `--every` adds every language Shiki bundles to the specs.
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
// a grammar it has not loaded.
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
// - the patterns as text;
// - the scope names, interned.
// A rule whose patterns all include a grammar that is not loaded is dropped,
// as vscode-textmate drops it.
import { bundledLanguages, bundledLanguagesInfo } from "shiki@4.6.0";
import { mkdirSync, writeFileSync } from "node:fs";
import { inputs } from "./inputs.ts";
import { handLexed } from "./hand-lexed.ts";

type Json = Record<string, unknown>;
/** The repositories an include can name, innermost last: a rule without a
 * begin or match may carry its own, which shadows the outer ones. */
type Repositories = Json[];
type Capture = { index: number; name: string[]; contentName: string[]; retokenize: number };
type Rule = {
  matching: "Match" | "BeginEnd" | "BeginWhile" | "Container";
  name: string[];
  contentName: string[];
  begin: number;
  end: number;
  while: number;
  captures: Capture[];
  endCaptures: Capture[];
  patterns: number[];
  applyEndLast: boolean;
  /** Includes the rule's list had that could not be resolved. */
  missing: boolean;
};

const NONE = 0xffffffff;
/** Where the ids of hand-offs start in a list: above every rule's. */
const HAND_OFF = 1 << 30;

const args = process.argv.slice(2);
const every = args[0] === "--every";
if (every) args.shift();
const outDir = args[0];
const ids = [...args.slice(1), ...(every ? bundledLanguagesInfo.map((info) => info.id) : [])];
if (!outDir || ids.length === 0) throw new Error("usage: engine-tables.ts <out dir> <language id>...");

/** Every grammar a language needs, by scope name. */
async function load(
  spec: string,
  handed: string[],
): Promise<{ docs: Map<string, Json>; fragments: Map<string, Json>; scope: string }> {
  const docs = new Map<string, Json>();
  // A handed grammar's repository still serves an include of one of its
  // rules, such as HAML's `source.ruby#escaped_char`.
  const fragments = new Map<string, Json>();
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
  for (const id of handed) {
    const loader = bundledLanguages[id as keyof typeof bundledLanguages];
    if (!loader) throw new Error(`Shiki has no language ${id}`);
    for (const grammar of ((await loader()) as { default: Json[] }).default) {
      if (grammar.name === id) fragments.set(String(grammar.scopeName), grammar);
    }
  }
  return { docs, fragments, scope };
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

  /** The `Language` variant of each hand-off, by its id less `HAND_OFF`. */
  handOffs: string[] = [];

  constructor(
    private docs: Map<string, Json>,
    /** The handed grammars, for an include of one of their rules. */
    private fragments: Map<string, Json>,
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

  reserve(matching: Rule["matching"]): number {
    this.rules.push({
      matching,
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
    const id = this.reserve("Container");
    this.roots.set(scope, id);
    const env: Repositories = [(doc.repository as Json | undefined) ?? {}];
    const [patterns, missing] = this.list(scope, (doc.patterns as Json[]) ?? [], env);
    this.rules[id].patterns = patterns;
    this.rules[id].missing = missing;
    return id;
  }

  /** The rule of a raw rule object, compiled on first use. */
  entry(scope: string, value: Json, env: Repositories): number {
    const known = this.compiled.get(value);
    if (known !== undefined) return known;
    const id = this.reserve("Container");
    this.compiled.set(value, id);
    this.rule(scope, value, id, env);
    return id;
  }

  include(scope: string, target: string, env: Repositories): number | undefined {
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
      const doc = this.docs.get(other) ?? this.fragments.get(other);
      const repository = doc?.repository as Json | undefined;
      const found = repository?.[target.slice(hash + 1)] as Json | undefined;
      if (!found) return undefined;
      return this.entry(other, found, [repository as Json]);
    }
    return this.root(target);
  }

  /** The rules of a list, and whether any were dropped. */
  list(scope: string, list: Json[], env: Repositories): [number[], boolean] {
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
      if (rule.matching !== "Match" && rule.missing && rule.patterns.length === 0) continue;
      out.push(id);
    }
    return [out, out.length !== list.length];
  }

  captures(scope: string, value: unknown, env: Repositories): Capture[] {
    if (typeof value !== "object" || value === null) return [];
    const out: Capture[] = [];
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

  rule(scope: string, value: Json, into: number | undefined, outer: Repositories, container = false): number {
    // As Shiki reads them: an empty `match` is no match, an empty `while` is
    // none, and an empty or missing `end` never matches.
    const matching: Rule["matching"] = container
      ? "Container"
      : value.match
        ? "Match"
        : value.begin !== undefined
          ? value.while
            ? "BeginWhile"
            : "BeginEnd"
          : "Container";
    const env: Repositories =
      matching === "Container" && value.repository ? [...outer, value.repository as Json] : outer;
    const id = into ?? this.reserve(matching);
    const rule = this.rules[id];
    rule.matching = matching;
    rule.name = this.scopes(value.name);
    rule.contentName = this.scopes(value.contentName);
    rule.applyEndLast = value.applyEndPatternLast === 1 || value.applyEndPatternLast === true;
    if (matching === "Match") {
      rule.begin = this.pattern(String(value.match));
      rule.captures = this.captures(scope, value.captures, env);
    } else if (matching === "BeginEnd" || matching === "BeginWhile") {
      rule.begin = this.pattern(String(value.begin));
      rule.captures = this.captures(scope, value.beginCaptures ?? value.captures, env);
      if (matching === "BeginEnd") {
        rule.end = value.end ? this.pattern(String(value.end)) : NONE;
        rule.endCaptures = this.captures(scope, value.endCaptures ?? value.captures, env);
      } else {
        rule.while = this.pattern(String(value.while));
        rule.endCaptures = this.captures(scope, value.whileCaptures ?? value.captures, env);
      }
    }
    if (matching !== "Match") {
      // A rule with no patterns, match or begin and an include holds the
      // include as its one pattern, as vscode-textmate reads it.
      const fallback =
        matching === "Container" && !container && typeof value.include === "string" && value.include
          ? [{ include: value.include }]
          : [];
      const [patterns, missing] = this.list(scope, (value.patterns as Json[]) ?? fallback, env);
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
        if (id < HAND_OFF && this.rules[id].matching === "Container") {
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

function hasBackrefs(source: string): boolean {
  for (let i = 0; i + 1 < source.length; i++) {
    if (source[i] === "\\") {
      if (/[1-9]/.test(source[i + 1])) return true;
      i++;
    }
  }
  return false;
}

// The most memory, in bytes, that the lazy DFA of one of a grammar's scanners
// may hold in a cache (`Grammar::dfa_cache`).
const DFA_CACHE = 1 << 22;

mkdirSync(outDir, { recursive: true });
async function write(id: string) {
  const [spec, ...handedIds] = id.split(">");
  const handed = new Map<string, string>();
  for (const handedId of handedIds) {
    const entry = handLexed[handedId];
    if (!entry) throw new Error(`${id}: ${handedId} is not in oracle/hand-lexed.ts`);
    handed.set(entry.scope, entry.variant);
  }
  const { docs, fragments, scope } = await load(spec, handedIds);
  const header = { root: scope, handed: handedIds };
  const sources = [header, ...docs.values(), ...fragments.values()].map((line) => JSON.stringify(line));
  writeFileSync(`${outDir}/${base(id)}.sources.jsonl`, `${sources.join("\n")}\n`);
  const builder = new Builder(docs, fragments, scope, handed);
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
  builder.flatten();
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
  type PendingHandOff = { list: number; variant: string; before: number; after: number };
  const handOffRows: PendingHandOff[] = [];
  const handOffIds = new Map<string, number>();
  // A list with a hand-off holds the rules before it, then the rules after it.
  // It stays apart from a plain list with the same rules. The hand-off names
  // the list and its two parts.
  // A list that includes several handed grammars hands its region to the
  // language that reads all the others, else to the first. The other
  // includes match nothing.
  const variantOf = (p: number) => builder.handOffs[p - HAND_OFF];
  const readsOf = new Map(Object.values(handLexed).map((h) => [h.variant, h.reads]));
  const halts = new Map(Object.values(handLexed).map((h) => [h.variant, h.halts]));
  const listOf = (patterns: number[], owner: Rule) => {
    const variants = patterns.filter((p) => p >= HAND_OFF).map(variantOf);
    const readsAll = (p: number) => {
      if (p < HAND_OFF) return false;
      const reads = readsOf.get(variantOf(p)) ?? [];
      return variants.every((v) => v === variantOf(p) || reads.includes(v));
    };
    const taker = patterns.findIndex(readsAll);
    const at = taker >= 0 ? taker : patterns.findIndex((p) => p >= HAND_OFF);
    if (at < 0) return plain(patterns, NONE);
    const before = patterns.slice(0, at).filter((p) => p < HAND_OFF);
    const after = patterns.slice(at + 1).filter((p) => p < HAND_OFF);
    const halting = [...before, ...after].map((r) => builder.rules[r].begin);
    if (owner.matching === "BeginEnd") halting.push(owner.end);
    // A lexer that cannot halt would run past these patterns.
    const variant = variantOf(patterns[at]);
    if (!halts.get(variant) && halting.some((p) => p !== NONE)) {
      throw new Error(`${id}: a hand-off to ${variant}, whose lexer cannot halt, has rules or an end`);
    }
    const row: PendingHandOff = {
      list: NONE,
      variant,
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
  // A capture runs the handing loop only when its rule can reach a hand-off,
  // in its own list or through a rule nested in it. The table lists those
  // capture rules, sorted.
  const reaches = (start: number) => {
    const seen = new Set<number>();
    const stack = [start];
    while (stack.length > 0) {
      const id = stack.pop() as number;
      if (seen.has(id)) continue;
      seen.add(id);
      for (const p of builder.rules[id].patterns) {
        if (p >= HAND_OFF) return true;
        stack.push(p);
      }
    }
    return false;
  };
  const handingCaptures = [
    ...new Set(
      builder.rules.flatMap((r) => [...r.captures, ...r.endCaptures].map((c) => c.retokenize)).filter((id) => id !== NONE),
    ),
  ]
    .filter(reaches)
    .sort((a, b) => a - b);
  // The tokenizer looks a list's hand-off up by the list's index, one load a
  // step. A `u8` holds the index, so a grammar has at most 256 hand-offs.
  if (handOffRows.length > 256) throw new Error(`${id}: more than 256 hand-offs`);
  const none = (n: number) => (n === NONE ? -1 : n);
  const cap = (c: Capture) => ({
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
      matching: r.matching,
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
    handOffs: handOffRows.map((h) => ({ language: h.variant, before: h.before, after: h.after })),
    listHandOffs:
      handOffRows.length > 0
        ? lists.map((_, list) => {
            const handOff = handOffRows.findIndex((h) => h.list === list);
            return handOff < 0 ? null : handOff;
          })
        : [],
    handingCaptures,
    dfaCache: DFA_CACHE,
  };
  writeFileSync(`${outDir}/${base(id)}.json`, JSON.stringify(data));
  console.log(`${id}: ${builder.rules.length} rules, ${builder.patterns.length} patterns`);
}

for (const id of ids) {
  try {
    await write(id);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    writeFileSync(`${outDir}/${base(id)}.error`, `${message}\n`);
    console.log(`${id}: refused: ${message}`);
  }
}
