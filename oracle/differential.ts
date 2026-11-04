// Random documents for a language, tokenized by Shiki into oracle-format
// fixtures, for comparing a lexer with Shiki on text no fixture holds.
//
// usage: bun run oracle/differential.ts <lang> <seed> <documents> <lines> <out dir> [fence]
// With `fence`, each document sits in a Markdown fence of the language and the
// fixtures are Markdown's, which tests the language as Markdown embeds it.
// Compare each `<out dir>/<seed>-<n>.txt` with the lexer the way
// tests/it/oracle.rs compares a fixture (write a small example that reads the
// `L`/`T` records, or point a copy of the oracle test at the directory).
import { createHighlighter } from "shiki@4.6.0";
import { mkdirSync, writeFileSync } from "node:fs";

const [lang, seedArg, docsArg, linesArg, out, mode] = process.argv.slice(2);
const fence = mode === "fence";
let seed = Number(seedArg);
const rnd = () => {
  seed = (seed * 1664525 + 1013904223) >>> 0;
  return seed / 4294967296;
};
const pick = <T,>(a: T[]): T => a[Math.floor(rnd() * a.length)]!;

/** Fragments to glue into lines: words, marks and line starts. */
const pools: Record<string, { words: string[]; marks: string[]; heads: string[] }> = {
  kotlin: {
    words: ["fun", "class", "interface", "object", "val", "var", "import", "package", "if", "else", "when", "in", "is", "as", "by", "get", "set", "this", "super", "private", "open", "data", "suspend", "true", "false", "null", "Foo", "x", "String", "List", "foo", "1", "0x1F", "1.5f", "@Foo", "@file:JvmName", "@param", "{@link a.B#c(d)}", "`a b`"],
    marks: ["(", ")", "[", "]", "{", "}", "<", ">", ",", ".", "?.", "::", ":", "?", "=", "==", "+", "-", "->", "\"", "\"\"\"", "'", "\"${x}\"", "$x", "/*", "*/", "/**", "//"],
    heads: ["", "", " ", "    ", " * ", "// "],
  },
  dart: {
    words: ["import", "export", "library", "part of", "as", "show", "hide", "class", "extends", "extension type", "mixin", "with", "implements", "factory", "get", "set", "final", "const", "late", "var", "void", "int", "bool", "dynamic", "if", "else", "for", "in", "switch", "case", "when", "return", "async", "async*", "sync*", "await", "yield", "yield*", "try", "on", "catch", "throw", "new", "is", "is!", "this", "super", "true", "null", "Foo", "_Bar", "\$x", "x", "foo", "1", "1.5e3", "0x1F", "1.x", ".5", "@override", "[Foo]", "`code`", "extends"],
    marks: ["(", ")", "[", "]", "{", "}", "<", ">", "<T>(", "<List<int>>(", ",", ".", "?.", "!", "?", ":", ";", "=", "==", "=>", "+", "-", "~/", ">>>=", "&&", "\"", "'", "\"\"\"", "'''", "r\"", "r'", "\"\${x}\"", "'\$x'", "\\n", "/*", "*/", "/**", "/**/", "//", "///", "```"],
    heads: ["", "", " ", "  ", " * ", "/// ", "///     ", "// ", "#!"],
  },
  powershell: {
    words: ["function", "filter", "param", "begin", "process", "end", "if", "elseif", "else", "foreach", "where", "switch", "return", "class", "hidden", "static", "using namespace", "Get-ChildItem", "Where-Object", "ForEach-Object", "Write-Host", "Microsoft.PowerShell.Utility\\Get-Date", "cmd.exe", "$x", "$_", "$PSItem", "$true", "$env:Path", "${env:Path}", "$global:a", "@args", "$Error.Count", "$a.b.c", "x", "Foo", "1", "1.5mb", "0x1Fl", "0b101", "1e3", "10kb", "[int]", "[Parameter(Mandatory)]", "[CmdletBinding()]", "-eq", "-notlike", "-cin", "-is", "-and", "-not", "-join", "-band", "-f", "-Path", "--%"],
    marks: ["(", ")", "[", "]", "{", "}", "@{", "@(", "$(", "${", ",", ".", "..", ";", "|", "||", "&&", "&", "=", "+=", "!", "?", "%", "2>&1", ">>", "<", "`", "`n", "`u{1F600}", "`u{", "\"", "'", "''", "\"\"", "@\"", "\"@", "@'", "'@", "#", "<#", "#>", ".SYNOPSIS", ".PARAMETER Name x", "#requires -Version 7"],
    heads: ["", "", " ", "    ", "# ", "function ", "@\"", "\"@", ".EXAMPLE"],
  },
  scala: {
    words: ["package", "import", "export", "given", "using", "as", "object", "class", "case class", "trait", "enum", "extends", "with", "derives", "def", "val", "var", "type", "new", "this", "super", "if", "then", "else", "match", "case", "for", "yield", "while", "do", "try", "catch", "finally", "return", "throw", "inline", "transparent", "opaque", "private[this]", "protected", "lazy", "implicit", "override", "@volatile", "extension", "end", "end if", "end Foo", "classOf", "isInstanceOf", "true", "null", "Foo", "x", "_x", "op_+", "é", "café", "`my var`", "'sym", "1", "1.5e3", "0x1F", "0b101", ".5f", "10L", "s", "f", "raw", "ns:tag", "&amp;"],
    marks: ["(", ")", "[", "]", "{", "}", "()", ",", ".", ";", ":", "::", "=", "==", "=>", "<-", "->", "<:", ">:", "<%", "+", "-", "*", "/", "&&", "||", "!", "∘", "\\", "_*", "\"", "\"\"\"", "s\"", "raw\"", "f\"\"\"", "'", "'a'", "'\\n'", "$x", "${", "$$", "$1", "/*", "*/", "/**", "/**/", "//", "<a>", "</a>", "<br></br>", "<p:x a=\"1\">", "/>", "'{", "'["],
    heads: ["", "", " ", "  ", " * ", "// ", "//> using ", "#!", "end "],
  },
  nix: {
    words: ["let", "in", "rec", "with", "inherit", "inherit (x)", "if", "then", "else", "assert", "or", "import", "builtins", "builtins.map", "true", "null", "throw", "x", "x'", "foo-bar", "pkgs.lib", "a.b.c", "1", "0", "./a/b.nix", "~/x", "../x", "<nixpkgs>", "<a/b>", "https://x.org/a?b=c", "x:", "x@", "@args", "..."],
    marks: ["(", ")", "[", "]", "{", "}", "{ }", ",", ".", ";", ":", "?", "@", "=", "==", "!=", "!", "<", ">=", "->", "&&", "||", "//", "++", "+", "-", "*", "/", "|>", "<|", "\"", "\\\"", "''", "'''", "''$", "''\\n", "${", "\"${x}\"", "/*", "*/", "**/", "#"],
    heads: ["", "", " ", "  ", "# ", "/* ", "''", "{"],
  },
  swift: {
    words: ["func", "init", "class", "struct", "enum", "protocol", "extension", "actor", "typealias", "operator", "infix", "import", "let", "var", "if", "else", "guard", "switch", "case", "default", "for", "in", "while", "return", "throws", "try", "await", "async", "where", "is", "as?", "self", "Self", "nil", "true", "some", "any", "static", "public", "private", "@available", "@objc", "@MainActor", "#if", "#else", "#endif", "#available", "Int", "String", "Array", "Foo", "x", "_", "é", "$0", "print", "1", "0x1F", "1_000", "2e10", "`class`", "\\(x)", "\\.self", "swift(>=5.9)", "os(macOS)"],
    marks: ["(", ")", "[", "]", "{", "}", "<", ">", ",", ".", "...", "..<", "?", "!", "??", "?.", ":", ";", "=", "==", "+", "-", "*", "/", "&+", "->", "=>", "@", "#", "\\", "\"", "\"\"\"", "#\"", "\"#", "##\"", "\\(", "/*", "*/", "///", "//", "#/", "/#", "/a+/", "#/a|b/#", "(?i)", "|>"],
    heads: ["", "", " ", "    ", "// ", "} ", "case "],
  },
};
const pool = pools[lang!];
if (!pool) throw new Error(`no fragment pool for ${lang}`);

const line = () => {
  if (rnd() < 0.05) return "";
  let s = pick(pool.heads);
  for (let i = 0, n = 1 + Math.floor(rnd() * 12); i < n; i++) {
    s += rnd() < 0.55 ? pick(pool.words) : pick(pool.marks);
    if (rnd() < 0.5) s += " ";
  }
  return s;
};

const highlighter = await createHighlighter({
  langs: fence ? [lang!, "markdown"] : [lang!],
  themes: ["github-dark"],
});
const enc = new TextEncoder();
mkdirSync(out!, { recursive: true });
for (let d = 0; d < Number(docsArg); d++) {
  const body = Array.from({ length: Number(linesArg) }, line);
  const lines = fence ? [`\`\`\`${lang}`, ...body, "```"] : body;
  const tokens = highlighter.codeToTokens(lines.join("\n"), {
    lang: fence ? "markdown" : lang!,
    theme: "github-dark",
    includeExplanation: "scopeName",
    tokenizeMaxLineLength: 0,
    tokenizeTimeLimit: 0,
  }).tokens;
  const records: string[] = [];
  tokens.forEach((toks, i) => {
    records.push(`L ${lines[i]}`);
    let end = 0;
    for (const t of toks)
      for (const p of t.explanation ?? []) {
        end += enc.encode(p.content).length;
        records.push(`T ${end} ${p.scopes.map((s) => s.scopeName).join(" ")}`);
      }
  });
  writeFileSync(`${out}/${seedArg}-${d}.txt`, records.join("\n") + "\n");
}
