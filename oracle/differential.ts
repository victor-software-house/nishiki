// Random documents for a language, tokenized by Shiki into oracle-format
// fixtures, for comparing a lexer with Shiki on text no fixture holds.
//
// usage: bun run oracle/differential.ts <lang> <seed> <documents> <lines> <out dir> [fence]
// With `fence`, each document sits in a Markdown fence of the language and the
// fixtures are Markdown's, which tests the language as Markdown embeds it.
// Compare each `<out dir>/<seed>-<n>.txt` with the lexer the way
// tests/it/oracle.rs compares a fixture (write a small example that reads the
// `L`/`T` records, or point a copy of the oracle test at the directory).
// Shiki loads every input grammar, as generate.ts does.
// An injection such as Svelte's SCSS body then applies as in the fixtures.
import { createHighlighter } from "shiki@4.6.0";
import { mkdirSync, writeFileSync } from "node:fs";
import { grammars } from "./grammars.ts";

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
  "common-lisp": {
    words: ["defun", "DEFUN", "defmacro", "defvar", "defparameter", "defconstant", "defclass", "defmethod", "defgeneric", "defstruct", "defpackage", "in-package", "define-condition", "define-compiler-macro", "lambda", "let", "let*", "flet", "labels", "if", "when", "unless", "cond", "case", "loop", "for", "in", "collect", "do", "progn", "setf", "setq", "return-from", "block", "handler-case", "format", "car", "cdr", "cons", "list", "mapcar", "funcall", "apply", "nil", "NIL", "t", "&optional", "&rest", "&key", "&body", "&aux", ":keyword", ":test", "*special*", "+const+", "foo", "x", "1", "-1", "1.5", "1/2", "#x1F", "#b101", "1d0", "é", "string", "integer", "error", "simple-error"],
    marks: ["(", ")", "'", "`", ",", ",@", "#'", "#\\a", "#\\Space", "#(", "#p\"a\"", "#+sbcl", "#-(or a b)", "#.", "|a b|", "\"", "\\\"", "\"~a~%\"", "\"~{~a~^, ~}\"", ";", ";;", "#|", "|#", ":", "::", "."],
    heads: ["", "", " ", "  ", "; ", ";; ", "(", "(defun f (", "#| "],
  },
  haskell: {
    words: ["module", "where", "import", "qualified", "as", "hiding", "class", "instance", "data", "newtype", "type", "family", "role", "nominal", "deriving", "stock", "anyclass", "via", "pattern", "foreign", "export", "ccall", "safe", "unsafe", "forall", "∀", "let", "in", "do", "mdo", "rec", "proc", "case", "of", "if", "then", "else", "infixl", "infixr", "Foo", "M.Map", "Data.List", "x", "x'", "_y", "f#", "1", "0x1F", "0b101", "1.5e3", "0x1.8p3", "'c'", "'\\n'", "'\\x41'", "\"str\"", "\"a\\\"b\"", "#label", "@Int", "@'[]", "LANGUAGE", "GADTs", "INLINE", "SPECIALISE", "language", "ſafe"],
    marks: ["(", ")", "[", "]", "{", "}", ",", ";", "=", "|", "::", "∷", "->", "→", "<-", "=>", "⇒", "\\", "λ", "@", "!", "~", "$", "$$", "..", ".", ":", ":+", "`", "'", "(#", "#)", "(# #)", "(,)", "()", "[]", "[|", "|]", "[e||", "||]", "[expr|", "{-", "-}", "{-#", "#-}", "{-@", "@-}", "--", "-- |", "-- ^", "-<", "-<<", "+", "<>", "#"],
    heads: ["", "", " ", "  ", "-- ", "{- ", "{-# ", "module ", "import ", "data ", "class ", "instance ", "type ", "newtype ", "  | ", "  , ", "  deriving ", "foreign import ccall ", "pattern ", "infixl 6 ", "#if ", "f :: ", ":: "],
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
  "objective-c": {
    words: ["@interface", "@implementation", "@end", "@protocol", "@property", "@synthesize", "@dynamic", "@class", "@selector", "@encode", "@try", "@catch", "@finally", "@throw", "@synchronized", "@private", "@required", "nonatomic", "strong", "weak", "copy", "readonly", "getter", "nullable", "self", "super", "_cmd", "nil", "Nil", "YES", "NO", "id", "id<NSCopying>", "instancetype", "IBOutlet", "IBAction", "SEL", "BOOL", "NSString", "NSArray", "NSObject", "NSRect", "NSOrderedSame", "NSApp", "NSLog", "NSMakeRange", "NSStringFromClass", "CGRect", "CALayer", "NSWindowDidResizeNotification", "NS_AVAILABLE_IOS", "NS_DURING", "kCFNull", "gFoo", "sBar", "size_t", "int8_t", "uint", "pthread_t", "OSStatus", "my_t", "struct", "enum", "union", "typedef", "const", "static", "inline", "__block", "switch", "case", "default", "break", "return", "if", "else", "for", "in", "while", "do", "sizeof", "int", "void", "char", "unsigned", "long", "printf", "memcpy", "Foo", "x", "obj", "integer", "foo_bar", "1", "0x1F", "0x1.8p3", "1.5f", "1e-3", "0b101", "017", "10UL", "1'000", "'a'", "é", "defined", "NULL", "predicateWithFormat:", "argumentArray:", "AND", "CONTAINS", "SUBQUERY"],
    marks: ["(", ")", "[", "]", "{", "}", "<", ">", ":", ";", ",", ".", "->", "^", "*", "&", "&&", "||", "=", "==", "+=", "<<=", "?", "!", "~", "...", "@\"", "\"", "\\\"", "\\n", "\\x41", "%@", "%d", "%1$@", "%-5.2f", "%lu", "%%", "\" PRIu64", "@[", "@{", "@(", "//", "/*", "*/", "\\", "#", "<<", ">>", "NSPredicate predicateWithFormat:"],
    heads: ["", "", " ", "    ", "// ", "// = ", "/* = ", "#import ", "#include <", "#define ", "#if 0", "#if 1", "#ifdef ", "#elif ", "#else", "#endif", "#pragma mark ", "#error ", "#line ", "#undef ", "- (void)", "+ (instancetype)", "- (id)init", "@interface A : NSObject ", "@implementation A", "switch (x) ", "case 1: "],
  },
  "objective-cpp": {
    words: ["@interface", "@implementation", "@end", "@protocol", "@property", "@synthesize", "@class", "@selector", "@autoreleasepool", "@try", "@catch", "@finally", "nonatomic", "strong", "weak", "readonly", "self", "super", "nil", "YES", "NO", "id", "instancetype", "NSString", "NSArray", "BOOL", "namespace", "using", "template", "typename", "class", "struct", "public", "private", "virtual", "override", "const", "constexpr", "auto", "static_cast", "std::vector", "std::string", "nullptr", "true", "new", "delete", "return", "if", "else", "for", "while", "int", "void", "char", "unsigned", "Foo", "x", "1", "0x1F", "1.5f", "'a'", "é", "#import", "#include", "#define", "#if", "#endif", "NS_ASSUME_NONNULL_BEGIN", "this", "friend", "explicit", "final", "noexcept", "mutable", "thread_local", "decltype", "wchar_t", "typeid", "and_eq", "bitor", "dynamic_cast", "reinterpret_cast", "throw", "catch", "fFoo", "mBar", "protected", "public:", "using namespace", "extern \"C\"", "a::b::c", "~Foo(", "A::~A(", "u8\"", "L\"", "u\"x\"", "R\"xy(", ")xy\"", "R\"abcdefghijklmnopq(", ")abcdefghijklmnopq\"", "u8R\"(", "\"\\u00e9\\x41\\7\""],
    marks: ["(", ")", "[", "]", "{", "}", "<", ">", "::", ":", ";", ",", ".", "->", "^", "*", "&", "&&", "=", "==", "+", "-", "@\"", "\"", "\\\"", "@[", "@{", "@(", "R\"(", ")\"", "//", "/*", "*/", "#", "<<", ">>", "~"],
    heads: ["", "", " ", "    ", "// ", "#import ", "- (void)", "+ (instancetype)", "@interface A : NSObject ", "template <"],
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
  svelte: {
    words: ["div", "span", "p", "a", "button", "input", "slot", "svelte:head", "svelte:window", "svelte:component", "svelte:fragment", "Foo", "Foo.Bar", "my-element", "class", "id", "href", "value", "on:click", "on:click|preventDefault", "bind:value", "bind:this", "class:active", "style:color", "use:action", "transition:fade", "in:fly", "out:slide", "animate:flip", "let:item", "--color", "generics", "lang", "type", "module", "context", "x", "item", "items", "count", "$count", "$$props", "$$restProps", "$$slots", "$state", "$derived", "$props", "$effect", "1", "1.5", "10px", "100%", "true", "null", "await", "then", "catch", "as", "each", "if", "else", "key", "snippet", "render", "html", "debug", "const", "import", "export", "let", "const x", "function", "return", "=>", "é"],
    marks: ["<", ">", "</", "/>", "=", "\"", "'", "`", "{", "}", "{ ", "{#if ", "{:else}", "{:else if ", "{/if}", "{#each ", "{/each}", "{#await ", "{:then ", "{:catch ", "{/await}", "{#key ", "{/key}", "{#snippet ", "{/snippet}", "{@html ", "{@render ", "{@debug ", "{@const ", "{@attach ", "(", ")", "[", "]", ",", ":", ";", ".", "...", "|", "!", "?", "+", "-", "*", "/", "&&", "||", "==", "<!--", "-->", "@component", "//", "/*", "*/", "\\n", "${", "#"],
    heads: ["", "", "  ", "\t", "<script>", "<script lang=\"ts\">", "<script context=\"module\">", "</script>", "<style>", "<style lang=\"scss\">", "<style lang=\"less\">", "</style>", "<template>", "</template>", "<div ", "<!-- ", "<!-- @component ", "{#if ", "{/if}", "{#each ", "{/each}", "{@html "],
  },
  swift: {
    words: ["func", "init", "class", "struct", "enum", "protocol", "extension", "actor", "typealias", "operator", "infix", "import", "let", "var", "if", "else", "guard", "switch", "case", "default", "for", "in", "while", "return", "throws", "try", "await", "async", "where", "is", "as?", "self", "Self", "nil", "true", "some", "any", "static", "public", "private", "@available", "@objc", "@MainActor", "#if", "#else", "#endif", "#available", "Int", "String", "Array", "Foo", "x", "_", "é", "$0", "print", "1", "0x1F", "1_000", "2e10", "`class`", "\\(x)", "\\.self", "swift(>=5.9)", "os(macOS)"],
    marks: ["(", ")", "[", "]", "{", "}", "<", ">", ",", ".", "...", "..<", "?", "!", "??", "?.", ":", ";", "=", "==", "+", "-", "*", "/", "&+", "->", "=>", "@", "#", "\\", "\"", "\"\"\"", "#\"", "\"#", "##\"", "\\(", "/*", "*/", "///", "//", "#/", "/#", "/a+/", "#/a|b/#", "(?i)", "|>"],
    heads: ["", "", " ", "    ", "// ", "} ", "case "],
  },
  php: {
    words: ["function", "fn", "class", "abstract", "final", "readonly", "interface", "trait", "enum", "case", "extends", "implements", "namespace", "use", "as", "insteadof", "const", "public", "private", "protected", "static", "private(set)", "new", "clone", "instanceof", "return", "yield", "yield from", "if", "else", "elseif", "foreach", "while", "match", "switch", "default", "try", "catch", "finally", "throw", "require_once", "include", "echo", "print", "array", "list", "int", "?string", "float", "bool", "mixed", "void", "never", "self", "parent", "true", "NULL", "__DIR__", "PHP_EOL", "E_ALL", "T_STRING", "strlen", "json_encode", "array_map", "mb_strlen", "preg_match", "ArrayIterator", "XMLDiff\\DOM", "OCI-Lob", "Foo", "Foo\\Bar", "\\Foo", "x", "\u00e9t\u00e9", "1", "0x1F", "0b101", "0o17", "017", "1_000.5e3", ".5", "$x", "$this", "$_GET", "$GLOBALS", "$$x", "Attribute", "Override", "goto", "and", "xor"],
    marks: ["(", ")", "[", "]", "{", "}", ",", ";", ":", "::", "?", "?:", "??", "??=", "->", "?->", "=>", "=", "==", "===", "!=", "!==", "<>", "<=>", "<=", ">=", "<", ">", "+", "-", "*", "**", "/", "%", ".", ".=", "...", "&", "&&", "|", "||", "^", "~", "!", "@", "<<", ">>", "#[", "\"", "'", "`", "\"{$x}\"", "\"$a->b\"", "'SELECT a FROM t'", "\"SELECT * FROM t WHERE a = '$b'\"", "'/a+[b]/i'", "\"/\\d+/\"", "\\n", "\\x41", "\\u{1F}", "<<<EOT", "<<<'EOT'", "<<<SQL", "<<<HTML", "<<<JSON", "EOT;", "SQL;", "HTML;", "JSON;", "//", "#", "/*", "*/", "/**", "@param", "@return", "{@link a}", "?>", "<?php"],
    heads: ["", "", " ", "    ", "// ", " * ", "<?php ", "EOT", "SQL", "HTML", "JSON", "<div>", "function ", "class ", "case "],
  },
  mermaid: {
    words: ["A", "B", "Foo", "x", "id1", "a-b", "a.b", "end", "class", "classDef", "click", "callback", "subgraph", "direction", "TD", "LR", "participant", "actor", "as", "Note", "note", "left of", "right of", "over", "activate", "loop", "alt", "else", "opt", "option", "par", "box", "transparent", "state", "fork", "join", "<<fork>>", "<<interface>>", "namespace", "map", "int", "String", "List~int~", "title", "section", "dateFormat", "axisFormat", "excludes", "weekends", "todayMarker", "tickInterval", "1day", "crit", "done", "after", "commit", "id", "tag", "type", "NORMAL", "branch", "merge", "order", "checkout", "cherry-pick", "group", "service", "junction", "in", "requirement", "element", "risk", "low", "verifymethod", "test", "satisfies", "contains", "x-axis", "y-axis", "quadrant-1", "line", "bar", "::icon", "PK", "FK", "string", "1", "0.5", "386"],
    marks: ["(", ")", "((", "))", "[", "]", "[[", "]]", "([", "])", "{", "}", "{{", "}}", ">", "<", "-->", "---", "-.->", "==>", "--o", "--x", "<--", "-- ", " --", "|", "|o", "||", "o{", "}|", "..", "..>", "<|--", "*--", "->>", "-->>", "-x", "-)", "+", "-", "#", "~", ":", ":::", ",", ";", "\"", "\"1\"", "\"*\"", "`", "@{", "%%", "[*]", "1+", "0+", "only one", "zero or more", "one or many"],
    heads: ["", "", " ", "  ", "    ", "%% ", "graph TD", "flowchart LR", "classDiagram", "sequenceDiagram", "stateDiagram-v2", "erDiagram", "gantt", "gitGraph", "mindmap", "pie", "quadrantChart", "requirementDiagram", "journey", "xychart-beta", "architecture-beta", "C4Context", "C4Component", "```", "```mermaid", "::: mermaid", ":::"],
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

const highlighter = await createHighlighter({ langs: grammars, themes: ["github-dark"] });
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
