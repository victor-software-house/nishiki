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
  asm: {
    words: ["mov", "MOV", "movzx", "lea", "push", "pop", "call", "ret", "jmp", "jne", "cmovne", "add", "sub", "xor", "int", "syscall", "nop", "rep", "lock", "vaddps", "vpxord", "vcvtph2ps", "vscatterdpd", "kmovw", "fld", "fstp", "eax", "rax", "rbx", "rsp", "r8d", "r15b", "al", "ah", "cs", "ds", "xmm0", "ymm15", "zmm31", "xmm16", "mm0", "st0", "st", "k1", "cr0", "cr8", "dr7", "bnd0", "rip", "eflags", "byte", "word", "dword", "qword", "db", "dw", "dd", "dq", "resb", "times", "equ", "incbin", "strict", "near", "rel", "a16", "o32", "section", "segment", ".text", ".data", ".rodata", ".bss", "global", ".globl", "extern", "main", "_start", "x", "1", "0x1F", "1Fh", "$1F", "101b", "0b101", "17o", "0q17", "123d", "0t9", "1.5", "1.", "1e10", "1.5e3", "0x1.8p3", "$1.8p3", "12p3", "Inf", "NaN", "float32", "ilog2e", "__?FILE?__", "__?USE_X?__", "__FILE__", "__USE_X__", "..start", "..got", "\u017fhl", "\u212aand", "é"],
    marks: [",", ":", ".", "..", "..@", "[", "]", "+", "-", "*", "(", ")", "{", "}", "{k1}", "{z}", "{1to8}", "{rn-sae}", "{vex3}", "$", "$$", "%", "%%", "\"", "'", "`", ";", " # ", "#", "/*", "*/", "\\", "\u00a0"],
    heads: ["", "", " ", "    ", "\t", "; ", "# ", "%define X ", "%macro m 1", "%if 0", "#if 0", "%endif", "#include ", "%include \"", "label:", ".loop:", "section .text", "global "],
  },
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
    words: ["defun", "DEFUN", "defmacro", "defvar", "defparameter", "defconstant", "defclass", "defmethod", "defgeneric", "defstruct", "defpackage", "in-package", "define-condition", "define-compiler-macro", "lambda", "let", "let*", "flet", "labels", "if", "when", "unless", "cond", "case", "loop", "for", "in", "collect", "do", "progn", "setf", "setq", "return-from", "block", "handler-case", "format", "car", "cdr", "cons", "list", "mapcar", "funcall", "apply", "nil", "NIL", "t", "&optional", "&rest", "&key", "&body", "&aux", ":keyword", ":test", "*special*", "+const+", "foo", "x", "1", "-1", "1.5", "1/2", "#x1F", "#b101", "1d0", "é", "string", "integer", "error", "simple-error", "\u017fetq", "cla\u00df", "\u212aeyword", "\ufb01rst", "*print-base*", "+limit+", "pkg:sym", "pkg::*v*", "#:g", "#1=", "#1#", "#*101", "#2A", "#36rZ", "1d0", ".5", "1.", "**", "///", "reinitialize-instance", "satisfies", "macrolet", "unwind-protect", "the"],
    marks: ["(", ")", "'", "`", ",", ",@", "#'", "#\\a", "#\\Space", "#(", "#p\"a\"", "#+sbcl", "#-(or a b)", "#.", "|a b|", "\"", "\\\"", "\"~a~%\"", "\"~{~a~^, ~}\"", ";", ";;", "#|", "|#", ":", "::", ".", "~", "~'", "~/f/", "~:@", "~1,2", "\"~'\"a\"", "'a:*b*", "(defun (f)", "(defun ,", "#\\(", "\u00a0", "\u3000"],
    heads: ["", "", " ", "  ", "; ", ";; ", "(", "(defun f (", "#| "],
  },
  elixir: {
    words: ["defmodule", "defprotocol", "defimpl", "def", "defp", "defmacro", "defmacrop", "defguard", "defdelegate", "do", "end", "do:", "fn", "when", "case", "cond", "with", "if", "unless", "receive", "after", "try", "rescue", "catch", "else", "raise", "alias", "import", "require", "use", "quote", "unquote", "and", "or", "not", "in", "nil", "true", "false", "__MODULE__", "__ENV__", "Foo.Bar", "Enum.map", "x", "_x", "_", "f!", "f?", "name=", "é", "@doc", "@moduledoc", "@spec", "@x", ":ok", ":\"a b\"", ":'a'", ":+", ":===", "key:", "?a", "?\\n", "?\\x41", "1", "0x1F", "0b101", "0o17", "1_000", "1.5e3", "&1", "^x", "~s(a)", "~S|a|", "~r/a/i", "~w[a b]", "~H\"\"\"", "~L\"\"\"", "<div>", "{@x}", "<%= x %>"],
    marks: ["(", ")", "[", "]", "{", "}", "%{", "<<", ">>", ",", ";", ".", "..", "//", ":", "::", "=", "==", "===", "!=", "<>", "++", "--", "|>", "<-", "->", "=>", "\\\\", "&", "&&", "&&&", "||", "|||", "!", "^^^", "~~~", "<<<", "<~>", "\"", "\"\"\"", "'", "'''", "#{", "\\n", "\\\"", "#", "##", "|", "/"],
    heads: ["", "", " ", "  ", "# ", "## ", "def ", "defp ", "defmodule ", "  @doc ", "@doc \"\"\"", "\"\"\"", "fn ", "  (", "case ", "~H\"\"\""],
  },
  gdscript: {
    words: ["extends", "class_name", "class", "func", "static", "var", "const", "signal", "enum", "setget", "get", "set", "onready", "export", "tool", "await", "yield", "if", "elif", "else", "for", "in", "while", "match", "case", "when", "break", "continue", "pass", "return", "and", "or", "not", "is", "as", "self", "super", "preload", "load", "null", "true", "false", "PI", "TAU", "INF", "NAN", "Vector2", "Vector2i", "Vector3", "Color", "Array", "Dictionary", "Node", "Node2D", "String", "int", "float", "bool", "void", "NodePath", "get_node", "has_node", "get_node_or_null", "find_node", "new", "x", "_x", "MyClass", "MY_CONST", "E_num", "é", "@export", "@onready", "@tool", "@export_range", "1", "0x1F", "0b101", "1_000", "1.5", ".5", "1.e5", ".e5", "1e10", "-3"],
    marks: ["(", ")", "[", "]", "{", "}", ",", ";", ":", ":=", "=", "==", "!=", "<", ">", "<=", ">=", "->", "+", "-", "*", "/", "%", "**", "+=", "-=", "&", "|", "^", "~", "<<", ">>", "&&", "||", "!", ".", "$", "$\"a\"", "%Unique", "&\"name\"", "^\"path\"", "@\"x\"", "\"", "'", "\"\"\"", "'''", "r\"", "\\n", "\\\"", "{0}", "{name!r:>10}", "{a:{w}}", "%5.2f", "%(k)s", "#", "##", "#region", "#endregion", "\\"],
    heads: ["", "", "\t", "  ", "# ", "func f(", "var a = ", "\tvar s = \"", "class A:", "match x:", "\t1:", "x = \\"],
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
  matlab: {
    words: ["function","end","if","elseif","else","for","parfor","while","switch","case","otherwise","try","catch","break","continue","return","spmd","global","persistent","classdef","properties","methods","events","enumeration","arguments","(Access = private)","(SetAccess = protected)","hold","on","off","format","long","disp","ans","eps","i","j","inf","Inf","NaN","nan","nargin","nargout","varargin","varargout","pi","true","false","x","Foo","a.b","obj.prop","fFoo","nargoutx","ansx","1","1.5",".5","1e-3","2.5E+10","3i","4j","1e","0x1F","é","double","mustBePositive","{mustBeNumeric}"],
    marks: ["'","''",".'","x&'","x('","x*'","x+'","x,'","x-'","x.'","x/'","x:'","x;'","x<'","x='","x>'","x['","x\\'","x^'","x{'","x|'","x~'","x]'","x)'","x}'"," '","\"","\"\"","'it''s'","\"a \"\" b\"","%d","%5.2f","%%","\\n","(",")","[","]","{","}","a(end)","x{1}","+","-","*","/","\\","^",".*","./",".^","==","~=","<=",">=","&&","||","&","|","~",":","@","@(x)",",",";","="," = "," == "," + ","...","... comment","%","% c","%%","%{","%}","!","<","<<"],
    heads: ["",""," ","  ","% ","%% ","%%\t","%{","%}","  %{","  %}","!","function ","function [a, b] = ","function y = ","classdef ","classdef (Sealed) A < handle ","properties ","methods ","end","if ","for ","while ","switch ","case ","otherwise","try","catch ","hold ","format ","disp ","global ","x = ","y(1) = "],
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
  coffee: {
    words: ["x", "@x", "$y", "_z", "Foo", "FOO_BAR", "console", "log", "Math", "abs", "Array", "isArray", "Object", "keys", "class", "extends", "new", "super", "this", "if", "unless", "then", "else", "for", "own", "in", "of", "when", "while", "return", "yield", "await", "and", "or", "not", "is", "isnt", "true", "yes", "off", "null", "undefined", "typeof", "delete", "var", "require", "setTimeout", "onClick", "getElementById", "plugins", "refresh", "1", "0x1F", "0o17", "0b10", "1.5e3", ".5", "3.", "1..", "2e", "é"],
    marks: ["(", ")", "[", "]", "{", "}", ",", ".", "..", "...", "::", "?", "?.", ":", ";", "=", "==", "!=", "->", "=>", "+=", "or=", "&&=", ">>>=", "<<", "+", "-", "*", "/", "%", "&", "|", "^", "~", "!", "@", "#", "###", "#{", "\"", "'", "'''", "\"\"\"", "`", "\\", "///", "/a+/g", "<div", "</div>", "/>", "-1"],
    heads: ["", "", " ", "  ", "# ", "### ", "x = ", "f = (a) ->", "class A", "  b: ->"],
  },
  haml: {
    words: ["%p", "%div", "%a", "%br", "%script", "%html", "%my-tag", "%svg:rect", ".lead", ".a-b", "#main", "#x_1", "x", "@user", "@@count", "$g", "item", "items", "each", "do", "end", "if", "else", "elseif", "foreach", "function", "use", "try", "catch", "die", "require_once", "link_to", "render", "true", "nil", "1", "1.5", ":sym", "a:", "href", "class", "data", "é", ":ruby", ":opal", ":javascript", ":css", ":sass", ":scss", ":style", ":styles", ":coffee", ":coffeescript", ":plain", ":php", ":markdown"],
    marks: ["{", "}", "(", ")", "[", "]", ",", "|", " |", "=", "==", "!=", "!==", "&=", "&amp=", "~", "-", "- ", "-#", "/", "/[if IE]", "/*", "!!!", "!!! 5", "#{", "#{}", "#@", "#@@", "#$", "\"", "\\x41", "\\n", "'", "=>", "do |x|", "{ |x|", "# c", ":", ".", "&", "!"],
    heads: ["", "", " ", "  ", "    ", "\t", "%p", "%a{", ".", "#", "- ", "= ", "!!!", "-#", "/ ", ":ruby", ":javascript", ":sass", ":css", ":markdown", ":plain", ":coffee", "==", "%p(", "%div["],
  },
  pascal: {
    words: ["program", "unit", "uses", "interface", "implementation", "begin", "end", "BEGIN", "End", "var", "const", "type", "class", "record", "procedure", "function", "constructor", "destructor", "property", "read", "write", "forward", "external", "attribute", "overload", "if", "then", "else", "for", "to", "downto", "do", "while", "repeat", "until", "case", "of", "try", "except", "finally", "raise", "exit", "and", "or", "not", "div", "mod", "shl", "as", "asc", "export", "exports", "require", "requires", "and_then", "self", "result", "Result", "nil", "true", "False", "Assert", "Integer", "string", "file", "Boolean", "TFoo", "TFoo.Bar", "x", "_y", "ſelf", "é", "1", "12ab", "1.5", ".5", "2.5e-3", "1E+9", "$FF", "$1234567890ABCDEF0", "#13", "#10"],
    marks: ["(", ")", "[", "]", ",", ".", "..", ":", ";", ":=", "=", "<>", "<=", "+", "-", "*", "/", "^", "@", "&", "'", "''", "'it''s'", "{", "}", "{$", "{$IFDEF X}", "(*", "*)", "//", "#", "$"],
    heads: ["", "", " ", "  ", "\t", "// ", "{ ", "(* ", "{$", "procedure ", "function F; ", "begin"],
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
  hack: {
    words: ["function", "async", "await", "class", "abstract", "final", "interface", "trait", "enum", "enum class", "module", "newtype", "type", "typedef", "extends", "implements", "namespace", "use", "use type", "use namespace", "as", "?as", "is", "const", "public", "private", "protected", "internal", "static", "return", "yield", "new", "clone", "if", "else", "elseif", "foreach", "while", "switch", "case", "default", "try", "catch", "finally", "throw", "invariant", "vec", "dict", "keyset", "Vector", "Map", "Set", "Pair", "shape", "tuple", "darray", "int", "?int", "string", "num", "arraykey", "mixed", "nonnull", "void", "noreturn", "this", "Awaitable", "Foo\\Bar", "\\HH\\Lib\\Str", "Str\\format", "C\\count", "A::B", "A::class", "self::", "parent::", "static::", "$this", "$x", "$$", "$_GET", "true", "NULL", "PHP_EOL", "__LINE__", "__construct", "0x1F", "0b1", "1_000", "1.5e3", "x", "\u017ftatic", "\u00ff"],
    marks: ["(", ")", "[", "]", "{", "}", ",", ";", ":", "::", "?", "??", "?->", "->", "==>", "|>", "=>", "=", "==", "===", "!==", "<>", "<=>", "<", ">", "+", "-", "*", "**", "/", "%", ".", "...", "&&", "||", "!", "@", "<<", ">>", "<<__Memoize>>", "\"", "'", "`", "\"{$x}\"", "\"$a->b\"", "'SELECT a FROM t'", "\"SELECT * FROM t WHERE a = '$b'\"", "re\"/a+/i\"", "\\n", "\\x41", "<<<EOT", "<<<'EOT'", "EOT;", "<div>", "</div>", "<x:y a={1} />", "&amp;", "<!--", "-->", "//", "#", "/*", "*/", "/**", "@param", "{@link a}", "?>", "<?hh"],
    heads: ["", "", " ", "    ", "\t", "// ", " * ", "<?hh ", "EOT", "<div>", "function ", "class ", "case ", "return <", "$x = <"],
  },
  hcl: {
    words: ["resource", "variable", "locals", "module", "data", "dynamic", "content", "provider", "terraform", "for", "for_each", "in", "if", "else", "endif", "endfor", "true", "false", "null", "nullable", "trueish", "any", "string", "number", "bool", "list", "set", "map", "tuple", "object", "x", "a-b", "a-", "_x", "1abc", "é", "var.name", "local.tags", "each.key", "aws_subnet.this[*].id", "a.0.b", "a.*.b", "max(", "merge(", "provider::aws::arn_parse(", "a::b::c::d(", "1", "1.5", "1e5", "1.5e-3", "0x1", "<<EOT", "<<-EOT", "EOT", "  EOT", "\"web\"", "\"a${b}c\"", "\"%{if a}b%{endif}\"", "\"\\n\\u00e9\\x\"", "\"$${x}\""],
    marks: ["(", ")", "[", "]", "{", "}", "[for", "{for", "[ for", ",", ".", "...", ":", "?", "=", "==", "!=", "=>", "<", ">", "<=", ">=", "+", "-", "*", "/", "%", "&&", "||", "!", "\"", "${", "%{", "~ ", " ~", "\\", "#", "//", "/*", "*/", "(k) =", "(a.b) :", "\"k\" =", "a =", "a ="],
    heads: ["", "", " ", "  ", "    ", "# ", "// ", "/* ", "resource \"a\" \"b\" {", "locals {", "x = ", "x = {", "x = [", "  for ", "  ", "}", "EOT", "(", "\"k\" = "],
  },
  r: {
    words: ["function", "if", "else", "for", "in", "while", "repeat", "break", "next", "return", "library", "require", "stop", "try", "tryCatch", "switch", "UseMethod", "setClass", "R6Class", "TRUE", "FALSE", "NULL", "NA", "NA_integer_", "Inf", "NaN", "x", "df", "my.var", ".hidden", "_x", "é", "αβ", "Ⅻ", "x\u0301", "a_b", "dplyr::filter", "base:::sum", "pkg.name::f", "f(", "c(", "mean(x)", "1", "1L", "2.5", ".5", "1e10", "1e", "1e+", "2i", "0x1F", "0xFFp2L", "`odd name`", "`a\\`b`", "@examples", "@examplesIf", "@param", "@field", "@slot", "@return", "@export", "@@", "\\code{x}", "\\alpha", "**bold**", "*it*", "__b__", "_i_", "`code`", "[stats::median()]", "[base:::sum]", "[f]", "<https://x.org>"],
    marks: ["(", ")", "[", "]", "[[", "]]", "{", "}", ",", ";", "=", "==", "!=", "<-", "<<-", "->", "->>", ":=", "|>", "=>", "%in%", "%>%", "%%", "%", "+", "-", "*", "**", "/", "^", "!", "&", "&&", "|", "||", "~", "?", "@", "$", ":", "::", ":::", "<", ">", "<=", ">=", "\\", "\\(x)", "\"", "'", "\\n", "\\x41", "\\101", "\\u00e9", "\\U{1F600}", "\\q", "r\"(", ")\"", "R\"--[", "]--\"", "r'{", "}'", "#", "#'", "```", "```r", "x = 1", "y=2"],
    heads: ["", "", " ", "  ", "    ", "# ", "#' ", "#' @examples", "#' @examplesIf x", "#' @param x ", "#' ```", "  #' ", "##' ", "function(", "if (", "x <- "],
  },
  perl: {
    words: ["my", "our", "local", "sub", "package", "use", "require", "return", "if", "elsif", "else", "unless", "while", "until", "for", "foreach", "last", "next", "redo", "die", "eval", "do", "and", "or", "not", "xor", "print", "printf", "open", "close", "split", "join", "map", "grep", "sort", "keys", "push", "shift", "bless", "ref", "defined", "wantarray", "STDIN", "STDERR", "ARGV", "Foo::Bar", "x", "$x", "@x", "%x", "$#x", "${x}", "$_", "@_", "$1", "$&", "$!", "$x->{a}", "$a[0]", "1", "0x1F", "1_000", "3.14", "é", "__END__", "__DATA__", "__FILE__", "__PACKAGE__", "BEGIN", "END", "format", "qw(a b)", "q{a}", "qq{$x}", "qr/a/i", "qx{ls}", "m{a}", "s/a/b/g", "s{a}{b}x", "tr/a/b/", "y/a/b/"],
    marks: ["(", ")", "[", "]", "{", "}", "<", ">", ",", ";", ":", "::", "?", "=", "==", "=~", "!~", "<=>", "->", "=>", "+", "-", "*", "**", "/", "//", "%", ".", "..", "...", "&", "&&", "|", "||", "^", "~", "!", "\\", "-e", "x 3", "\"", "'", "`", "\"$x @y\"", "'a\\'b'", "\\n", "\\x41", "<<EOT", "<<~EOT", "<<'EOT'", "<<\"SQL\"", "<<HTML", "EOT", "SQL", "HTML", "#", "=pod", "=cut", "B<b>", "C<< c >>", "&/a/", "(/a/", "{/a/", "|/a/", "~/a/", "if /a/", "unless /a/", "x /a/", "/a/"],
    heads: ["", "", " ", "    ", "\t", "# ", "=head1 ", "=item ", "EOT", "SQL", "HTML", "  EOT", "sub ", "package ", "my $x = ", "/a/ ", "__END__"],
  },
  mermaid: {
    words: ["A", "B", "Foo", "x", "id1", "a-b", "a.b", "end", "class", "classDef", "click", "callback", "subgraph", "direction", "TD", "LR", "participant", "actor", "as", "Note", "note", "left of", "right of", "over", "activate", "loop", "alt", "else", "opt", "option", "par", "box", "transparent", "state", "fork", "join", "<<fork>>", "<<interface>>", "namespace", "map", "int", "String", "List~int~", "title", "section", "dateFormat", "axisFormat", "excludes", "weekends", "todayMarker", "tickInterval", "1day", "crit", "done", "after", "commit", "id", "tag", "type", "NORMAL", "branch", "merge", "order", "checkout", "cherry-pick", "group", "service", "junction", "in", "requirement", "element", "risk", "low", "verifymethod", "test", "satisfies", "contains", "x-axis", "y-axis", "quadrant-1", "line", "bar", "::icon", "PK", "FK", "string", "1", "0.5", "386"],
    marks: ["(", ")", "((", "))", "[", "]", "[[", "]]", "([", "])", "{", "}", "{{", "}}", ">", "<", "-->", "---", "-.->", "==>", "--o", "--x", "<--", "-- ", " --", "|", "|o", "||", "o{", "}|", "..", "..>", "<|--", "*--", "->>", "-->>", "-x", "-)", "+", "-", "#", "~", ":", ":::", ",", ";", "\"", "\"1\"", "\"*\"", "`", "@{", "%%", "[*]", "1+", "0+", "only one", "zero or more", "one or many"],
    heads: ["", "", " ", "  ", "    ", "%% ", "graph TD", "flowchart LR", "classDiagram", "sequenceDiagram", "stateDiagram-v2", "erDiagram", "gantt", "gitGraph", "mindmap", "pie", "quadrantChart", "requirementDiagram", "journey", "xychart-beta", "architecture-beta", "C4Context", "C4Component", "```", "```mermaid", "::: mermaid", ":::"],
  },
  vb: {
    words: ["Sub", "Function", "End", "Class", "Property", "Dim", "ReDim", "Const", "Private", "Public", "Default", "If", "Then", "Else", "ElseIf", "Else If", "End If", "Select", "Case", "For", "Each", "To", "Next", "While", "Wend", "Do", "Until", "Loop", "With", "Exit", "Return", "Call", "Set", "Let", "Get", "New", "As", "Mod", "And", "Not", "Or", "Xor", "Empty", "Nothing", "Null", "True", "False", "Response", "Request", "Session", "Server", "Application", "Buffer", "Write", "CreateObject", "Len", "Mid", "vbCrLf", "vbTab", "Integer", "String", "x", "_x", "foo7", "Foo", "Key", "aſ", "ſize", "Claß", "ﬁx", "é", "1", "1.5", ".5", "1.", "1e10", "0xFF", "12UL", "$name"],
    marks: ["(", ")", ",", ".", "=", "<>", "<", ">", "<=", ">=", "&", "+", "-", "*", "/", "\\", "^", ":", "\"", "\"\"", "'", "!", "$", "%", "?:", "++", "===", "\t", "  "],
    heads: ["", "", "\t", "\t\t", "  ", "    ", "' ", "Sub f(", "Dim ", "If ", "\"", "  ' "],
  },
  jinja: {
    words: ["block", "endblock", "extends", "if", "elif", "else", "endif", "for", "in", "endfor", "recursive", "set", "macro", "endmacro", "call", "endcall", "filter", "endfilter", "include", "import", "from", "as", "with", "without", "context", "raw", "endraw", "is", "not", "and", "or", "true", "false", "none", "loop", "super", "self", "varargs", "kwargs", "x", "_x", "x1", "1abc", "items", "upper", "default", "range", "\u00e9", "1", "1.5", "\"s\"", "'s'", "@/re/"],
    marks: ["{{", "}}", "{{-", "-}}", "{%", "%}", "{%-", "-%}", "{#", "#}", "{#-", "-#}", "(", ")", "[", "]", "{", "}", ",", ".", ":", "|", "=", "==", "!=", "<", ">", "<=", ">=", "=>", "+", "-", "*", "**", "/", "//", "%", "\"", "'", "\\n", "\\\\", "\\x41", "\\u00e9", "\\", "@/", "<div>", "</div>"],
    heads: ["", "", " ", "  ", "\t", "{{ ", "{% ", "{# ", "{% raw %}", "<p>", "text "],
  },
  tex: {
    words: ["\\def","\\edef","\\let","\\relax","\\iffalse","\\ifx","\\ifnum","\\ifdefined","\\else","\\fi","\\unless","\\csname","\\endcsname","\\expandafter","\\catcode","\\@ifnextchar","\\ExplSyntaxOn","\\cs_new:Npn","\\tl_set:Nn","\\__my_fn:nn",".key_name:n","\\l_my_tl","\\alpha","\\beta","\\leq","\\leftarrow","\\infty","\\sum","\\int","\\bogoplus","\\sin","\\log","\\lim","\\frac","\\left","\\right","\\bigl","\\Bigg","\\langle","\\rVert","\\text","\\mbox","\\Sexpr","\\verb","\\begin","\\end","\\name","\\a","\\b","\\@","\\,","\;","x","y","é","1","3.14",".5"],
    marks: ["$","$$","\\$","\\\\","{","}","\\{","\\}","[","]","(",")","\\(","\\)","^","_","^^","__","+","-","*","/","=","`","`\\@","%","% c","%:","&","#","#1","~"],
    heads: ["",""," ","  ","% ","  % ","%!TEX root = ","\\iffalse","\\fi","\\ifx\\a\\b","$","$$","\\catcode`\\@=11 "],
  },
  latex: {
    words: ["\\begin{document}","\\end{document}","\\section","\\subsection*","\\label","\\ref","\\eqref","\\crefrange","\\cite","\\citep","\\textcite","\\parencite","\\bibentry","\\input","\\emph","\\textbf","\\texttt","\\footnote","\\marginpar","\\href","\\url","\\hyperref","\\newcommand","\\renewcommand*","\\usepackage","\\documentclass","\\item","\\newline","\\textdollar","\\l_my_tl","\\q_stop","\\verb","\\Verb*","\\mint","\\mintinline","\\pyinline","\\py","\\pyc","\\jl","\\directlua","\\cacheMeCode","\\addplot","gnuplot","\\beginsong","\\endsong","\\begin{minted}","\\end{minted}","\\begin{pycode}","\\end{pycode}","\\begin{juliacode}","\\end{juliacode}","\\begin{luacode}","\\end{luacode}","\\begin{verbatim}","\\end{verbatim}","\\begin{equation}","\\end{equation}","\\begin{align*}","\\end{align*}","\\begin{tabular}","\\end{tabular}","\\begin{itemize}","\\end{itemize}","\\begin{gnuplot}","\\end{gnuplot}","\\begin{CacheMeCode}","\\end{CacheMeCode}","\\begin{terminal}","\\end{terminal}","\\begin{x}","\\end{x}","\\alpha","\\frac","python","bash","js","x","é","1"],
    marks: ["$","$$","\\(","\\)","\\[","\\]","{","}","[","]","(",")","<",">","|","+","!","&","\\&","\\\\","%","% c","^","_","\\^","~",",",":","*","=","};"],
    heads: ["",""," ","  ","% ","\\begin{minted}{python}","\\begin{pycode}","\\begin{verbatim}","\\begin{equation}","\\begin{tabular}{cc}","\\item ","$"],
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
