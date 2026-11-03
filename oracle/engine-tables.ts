// Compiles Shiki's grammar for a language into the tables of nishiki's
// TextMate engine, as one Rust source file.
//
// Usage: bun oracle/engine-tables.ts <out dir> <spec>...
//
// A spec is a Shiki language id, or an id with the languages it may embed
// joined by `+`, such as `markdown+rust+shellscript`: the grammar of the
// first, with the others available to its includes, as Shiki loads them.
//
// The tables hold what the engine needs and nothing it can work out later: the
// rules with their includes already followed, the patterns as text (compiled on
// first use), and the scope names interned. A rule whose patterns all include a
// grammar that is not loaded is dropped, as vscode-textmate drops it.
import { bundledLanguages } from "shiki@4.6.0";
import { mkdirSync, writeFileSync } from "node:fs";

type Json = Record<string, unknown>;
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
const idx = (n: number) => (n === NONE ? "usize::MAX" : String(n));

const outDir = process.argv[2];
const ids = process.argv.slice(3);
if (!outDir || ids.length === 0) throw new Error("usage: engine-tables.ts <out dir> <language id>...");

/** Every grammar a language needs, by scope name. */
async function load(spec: string): Promise<{ docs: Map<string, Json>; scope: string }> {
  const docs = new Map<string, Json>();
  let scope = "";
  for (const id of spec.split("+")) {
    const loader = bundledLanguages[id as keyof typeof bundledLanguages];
    if (!loader) throw new Error(`Shiki has no language ${id}`);
    const registrations = ((await loader()) as { default: Json[] }).default;
    for (const grammar of registrations) docs.set(String(grammar.scopeName), grammar);
    if (scope === "") scope = String(registrations[registrations.length - 1].scopeName);
  }
  return { docs, scope };
}

class Builder {
  rules: Rule[] = [];
  patterns: string[] = [];
  patternIds = new Map<string, number>();
  names: string[] = [];
  nameIds = new Map<string, number>();
  roots = new Map<string, number>();
  repo = new Map<string, number>();

  constructor(
    private docs: Map<string, Json>,
    private base: string,
  ) {}

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
    const id = this.reserve("Container");
    this.roots.set(scope, id);
    const [patterns, missing] = this.list(scope, (doc.patterns as Json[]) ?? []);
    this.rules[id].patterns = patterns;
    this.rules[id].missing = missing;
    return id;
  }

  repository(scope: string, name: string): number | undefined {
    const key = `${scope}\u0000${name}`;
    const known = this.repo.get(key);
    if (known !== undefined) return known;
    const entry = ((this.docs.get(scope)?.repository as Json | undefined)?.[name]) as Json | undefined;
    if (!entry) return undefined;
    const id = this.reserve("Container");
    this.repo.set(key, id);
    this.rule(scope, entry, id);
    return id;
  }

  include(scope: string, target: string): number | undefined {
    if (target === "$self") return this.root(scope);
    if (target === "$base") return this.root(this.base);
    if (target.startsWith("#")) return this.repository(scope, target.slice(1));
    const hash = target.indexOf("#");
    if (hash >= 0) {
      const other = target.slice(0, hash);
      return this.docs.has(other) ? this.repository(other, target.slice(hash + 1)) : undefined;
    }
    return this.root(target);
  }

  /** The rules of a list, and whether any were dropped. */
  list(scope: string, list: Json[]): [number[], boolean] {
    const out: number[] = [];
    for (const pattern of list) {
      let id: number | undefined;
      if (typeof pattern.include === "string") {
        id = this.include(scope, pattern.include);
        if (id === undefined) continue;
      } else {
        id = this.rule(scope, pattern, undefined);
      }
      const rule = this.rules[id];
      if (rule.kind !== "Match" && rule.missing && rule.patterns.length === 0) continue;
      out.push(id);
    }
    return [out, out.length !== list.length];
  }

  captures(scope: string, value: unknown): Cap[] {
    if (typeof value !== "object" || value === null) return [];
    const out: Cap[] = [];
    for (const [key, raw] of Object.entries(value as Json)) {
      const index = Number(key);
      if (!Number.isInteger(index)) continue;
      const capture = raw as Json;
      out.push({
        index,
        name: this.scopes(capture.name),
        contentName: this.scopes(capture.contentName),
        retokenize: capture.patterns ? this.rule(scope, capture, undefined, true) : NONE,
      });
    }
    return out.sort((a, b) => a.index - b.index);
  }

  rule(scope: string, value: Json, into: number | undefined, container = false): number {
    const kind: Rule["kind"] = container
      ? "Container"
      : value.begin !== undefined
        ? value.while !== undefined
          ? "BeginWhile"
          : "BeginEnd"
        : value.match !== undefined
          ? "Match"
          : "Container";
    const id = into ?? this.reserve(kind);
    const rule = this.rules[id];
    rule.kind = kind;
    rule.name = this.scopes(value.name);
    rule.contentName = this.scopes(value.contentName);
    rule.applyEndLast = value.applyEndPatternLast === 1 || value.applyEndPatternLast === true;
    if (kind === "Match") {
      rule.begin = this.pattern(String(value.match));
      rule.captures = this.captures(scope, value.captures);
    } else if (kind === "BeginEnd" || kind === "BeginWhile") {
      rule.begin = this.pattern(String(value.begin));
      rule.captures = this.captures(scope, value.beginCaptures ?? value.captures);
      if (kind === "BeginEnd") {
        rule.end = this.pattern(String(value.end));
        rule.endCaptures = this.captures(scope, value.endCaptures ?? value.captures);
      } else {
        rule.while = this.pattern(String(value.while));
        rule.endCaptures = this.captures(scope, value.whileCaptures ?? value.captures);
      }
    }
    if (kind !== "Match") {
      const [patterns, missing] = this.list(scope, (value.patterns as Json[]) ?? []);
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
        const rule = this.rules[id];
        if (rule.kind === "Container") {
          if (seen.has(id)) continue;
          seen.add(id);
          expand(rule.patterns, out, seen);
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
    else out += ch;
  }
  return out + '"';
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

mkdirSync(outDir, { recursive: true });
for (const id of ids) {
  const { docs, scope } = await load(id);
  const builder = new Builder(docs, scope);
  const root = builder.root(scope);
  if (root === undefined) throw new Error(`${id}: no root rule`);
  builder.flatten();
  const refs = (list: string[]) => `&[${list.map((n) => builder.nameRef(n)).join(", ")}]`;
  const caps = (list: Cap[]) =>
    `&[${list
      .map(
        (c) =>
          `Cap { index: ${c.index}, name: ${refs(c.name)}, content_name: ${refs(c.contentName)}, retokenize: ${idx(c.retokenize)} }`,
      )
      .join(", ")}]`;
  const rules = builder.rules
    .map(
      (r) =>
        `    Rule { kind: RuleKind::${r.kind}, name: ${refs(r.name)}, content_name: ${refs(r.contentName)}, begin: ${idx(r.begin)}, end: ${idx(r.end)}, until: ${idx(r.while)}, captures: ${caps(r.captures)}, end_captures: ${caps(r.endCaptures)}, patterns: &[${r.patterns.join(", ")}], apply_end_last: ${r.applyEndLast} },`,
    )
    .join("\n");
  const patterns = builder.patterns
    .map((p) => `    PatternSlot::new(${rustString(p)}),`)
    .join("\n");
  const names = builder.names.map((n) => `    ${rustString(n)},`).join("\n");
  const source = `//! The tables of Shiki 4.6.0's \`${id}\` grammar, made by \`oracle/engine-tables.ts\`.
//! Do not edit: regenerate with \`bun oracle/engine-tables.ts\`.

use crate::textmate::grammar::{Cap, Grammar, PatternSlot, Rule, RuleKind};

static NAMES: &[&str] = &[
${names}
];

static PATTERNS: [PatternSlot; ${builder.patterns.length}] = [
${patterns}
];

static RULES: &[Rule] = &[
${rules}
];

/// The grammar.
pub(crate) static GRAMMAR: Grammar = Grammar {
    scope: ${rustString(scope)},
    names: NAMES,
    rules: RULES,
    patterns: &PATTERNS,
    root: ${root},
};
`;
  writeFileSync(`${outDir}/${id.split("+")[0]}.rs`, source);
  console.log(`${id}: ${builder.rules.length} rules, ${builder.patterns.length} patterns, ${builder.names.length} names`);
}
