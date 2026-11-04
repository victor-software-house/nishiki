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
  bat: {
    words: ["echo", "ECHO", "echo.", "off", "on", "set", "SET", "setlocal", "EnableDelayedExpansion", "endlocal", "call", "goto", "exit", "if", "not", "exist", "defined", "errorlevel", "else", "for", "in", "do", "rem", "REM.", "copy", "xcopy", "net user", "net", "dir", "find", "NUL", "EQU", "LSS", "x", "name", "value", "1", "-2", "0x1F", "\u017fet", "L\u00df", "/a", "/p", "/A", "%1", "%~dp0", "%*", "%%i", "%%~nxi", "%%~dp$PATH:f", "%path%", "%x:~1,2%", "%x:a=b%", "!x!", "!x:~0,3!", "!x:a=b!", ":label", "::"],
    marks: ["(", ")", ",", ";", "=", "==", "+=", "<<=", "%%=", "&", "&&", "|", "||", "<", ">", ">>", "2>&1", "@", "^", "^^!", "^&", "!", "~", "%", "\"", "'", ":", ":~", "*.txt"],
    heads: ["", "", " ", "  ", "@", "rem ", ":: ", ":", "for %%i in (", "set /a ", "set \"", "echo "],
  },
  julia: {
    words: ["function", "end", "macro", "module", "struct", "mutable struct", "abstract type", "where", "if", "elseif", "else", "for", "outer", "in", "isa", "while", "begin", "let", "do", "try", "catch", "return", "const", "global", "local", "import", "using", "export", "public", "as", "quote", "true", "nothing", "missing", "ARGS", "stdout", "x", "f", "Foo", "α", "∇f", "x′", "f!", "Base.show", "Vector{T}", "Union{Int,Nothing}", "1", "0x1F", "0b101", "1_000.5e-3im", "2π", "Inf32", "NaN", "pi", "@time", "@.", "@show", "@doc", ":sym", "py\"\"\"", "js\"\"\"", "sql\"\"\"", "cxx\"\"\"", "raw\"", "r\"a+\"i", "var\"", "b\"", "doc\"\"\"", "def", "let", "SELECT", "FROM"],
    marks: ["(", ")", "[", "]", "{", "}", ",", ";", ".", "..", "...", ":", "::", "<:", ">:", "=", "==", "===", "!=", "≠", "≤", "+=", ".+", "*", "/", "//", "\\", "^", "÷", "%", "|>", "∘", "->", "=>", "&&", "||", "!", "?", "'", ".'", "$", "$x", "$(", "\"", "\"\"\"", "`", "```", "\\n", "\\\"", "#", "#=", "=#", "TODO"],
    heads: ["", "", " ", "    ", "# ", "\"\"\"", "doc\"\"\"", "function ", "for "],
  },
  kotlin: {
    words: ["fun", "class", "interface", "object", "val", "var", "import", "package", "if", "else", "when", "in", "is", "as", "by", "get", "set", "this", "super", "private", "open", "data", "suspend", "true", "false", "null", "Foo", "x", "String", "List", "foo", "1", "0x1F", "1.5f", "@Foo", "@file:JvmName", "@param", "{@link a.B#c(d)}", "`a b`"],
    marks: ["(", ")", "[", "]", "{", "}", "<", ">", ",", ".", "?.", "::", ":", "?", "=", "==", "+", "-", "->", "\"", "\"\"\"", "'", "\"${x}\"", "$x", "/*", "*/", "/**", "//"],
    heads: ["", "", " ", "    ", " * ", "// "],
  },
  cmake: {
    words: ["cmake_minimum_required", "project", "add_executable", "add_library", "target_link_libraries", "set", "if", "elseif", "else", "endif", "foreach", "function", "macro", "VERSION", "PUBLIC", "PRIVATE", "AND", "OR", "NOT", "STREQUAL", "WIN32", "MSVC", "APPLE", "CMAKE_CXX_FLAGS", "CMAKE_BUILD_TYPE", "Foo_POSTFIX", "OUTPUT_NAME", "COMPILE_DEFINITIONS_<CONFIG>", "BUILD_NAME", "${x}", "$ENV{HOME}", "foo", "1", "TRUE", "on"],
    marks: ["(", ")", "\"", "\\\"", "\\n", "\\", "#", "#[[", "#[=[", "[[", "[=[", "]]", "]=]", "${", "}", ";"],
    heads: ["", "", "  ", "    ", "# "],
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
  zig: {
    words: ["const", "var", "fn", "pub", "inline", "inline fn", "extern", "comptime", "struct", "enum", "union", "error", "if", "else", "while", "for", "switch", "orelse", "try", "catch", "return", "defer", "and", "or", "true", "null", "undefined", "u8", "i32", "u128", "usize", "f64", "c_int", "anytype", "void", "x", "Foo", "foo_bar", "é", "@import", "@intCast", "@\"a b\"", "fn Foo", "fn foo", "fn @\"x\"", "1", "1_000", "0x1F", "0x1.8p3", "0o17", "0b101", "1.5e3", "1e+", "1.e+", "1x", "0xg", "TODO:"],
    marks: ["(", ")", "[", "]", "{", "}", "[*c]", "|", "||", ",", ".", ".*", ".?", ";", ":", "=", "==", "!=", "<", "<<", "<<%=", ">>=", "+%", "-%=", "*", "**", "++", "->", "!", "&", "?", "\"", "\\\"", "\\n", "\\x41", "\\u{1F600}", "\\q", "'a'", "'\\n'", "'\\123'", "'\\128'", "'\\x4'", "\\\\", "//", "///", "//!", "////"],
    heads: ["", "", " ", "    ", "// ", "/// ", "\\\\"],
  },
  postcss: {
    words: ["a", "abbr", "h1", "div", "header", "u", "x", "body", "color", "margin", "display", "block", "red", "inherit", "none", "auto", "true", "from", "to", "not", "in", "and", "screen", "print", "all", "only", "tv", "tty", "url", "format", "attr", "var", "calc", "rgba", "translate", "$x", "$gap-2", "--brand", "--x-y", "&", "&-item", "&_el", "%ph", "#id", ".card", "@media", "@import", "@include", "@mixin", "@function", "@define-mixin", "@custom-media", "@nest", "@apply", "@layer", "!important", "!default", "!global", "!optional", "#fff", "#a1b2c3", "#abcd", "1", "-1", ".5", "1.5", "10px", "2em", "1.5rem", "50%", "90deg", "1s", "3kHz", "é", "{{tag}}", "#{$x}"],
    marks: ["{", "}", "(", ")", "[", "]", ":", "::", ";", ",", ".", "...", "+", " - ", " -$x", " -(", "(-$", "*", "/", ">", "~", "=", "^=", "$=", "*=", "|", "\"", "'", "\"a #{$b} c\"", "'#{1}'", "/*", "*/", "//", "#{", "}}", ":not(", ":hover", "::before", ":-:a", " and(", "),"],
    heads: ["", "", " ", "  ", "// ", "/* ", "a { ", "@media ", ".a:"],
  },
  proto: {
    words: ["syntax", "package", "import", "import weak", "import public", "option", "message", "enum", "service", "rpc", "returns", "stream", "oneof", "extend", "reserved", "to", "max", "optional", "repeated", "required", "map", "int32", "string", "bool", "bytes", ".a.B", "Foo", "x", "FOO_BAR", "é", "true", "1", "-1", "0x1F", "-0x1F", "1.5", "1.5x", ".5", "2e10", "(my.opt)", "(a.b).c", "deprecated"],
    marks: ["{", "}", "(", ")", "[", "]", "<", ">", "=", ";", ",", ":", ".", "\"proto3\"", "\"a.proto\"", "'x'", "\"a\\\"b\"", "/*", "*/", "//"],
    heads: ["", "", " ", "  ", "// ", "message M { ", "enum E { ", "service S { "],
  },
  sass: {
    words: ["$base", "$map", "10px", "1.5em", "50%", "1sec", "-2", ".5", "#fff", "#abcdef", "!default", "!important", "darken(", "url(", "math.div(", "and", "or", "not", "true", "null", "a", "div", "h1", "span", "nav", "&", ".btn", "#main", "%placeholder", ":hover", "::before", "@media", "@include", "@mixin", "@function", "@use", "@import", "@extend", "=m", "+m", "color", "font-size", "--x", "é"],
    marks: ["(", ")", "[", "]", "{", "}", "#{", ",", ".", ":", ";", " - ", "+", "*", "/", "=", "~", "^=", "...", "\"", "'", "\"a#{$b}\"", "/*", "*/", "//", "{{", "}}"],
    heads: ["", "", "  ", "    ", "// ", "/* ", "=", "+", "@"],
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
