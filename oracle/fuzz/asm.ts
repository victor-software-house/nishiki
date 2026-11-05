// x86 assembly for the differential fuzzer (fuzz.ts) and the Oniguruma probe
// (onig.ts): pieces of assembly to build documents from, texts to insert,
// the info strings of Markdown fences, and the probe's snippets.

/** Pieces that random documents are made of. */
export const pieces: string[] = [
  "section .text", "section  .data", "segment .bss", ".rodata", ".textual", "global _start", ".globl main", "extern printf",
  "required x", "_start:", "main:", ".loop:", "..@local:", "..@x", ".1:", ".12ab", "1bad:", "$sym", "$1", "$1_ff", "$0.5p3",
  "mov eax, ebx", "MOV RAX, [RBX+8]", "movzx ecx, byte [rsi]", "lea rdi, [rel msg]", "push rbp", "pop rbp", "ret", "call printf",
  "jmp .loop", "cmovne eax, edx", "vaddps zmm0{k1}{z}, zmm1, zmm2", "vpxord zmm3, zmm4, [rax]{1to16}", "{vex3} vpaddd ymm0, ymm1, ymm2",
  "{evex} vmovdqu32 zmm0, [rdi]", "vcvtph2ps xmm0, xmm1", "vscatterdpd [rax+zmm1*8]{k1}, zmm2", "rep movsb", "lock cmpxchg [rdi], ecx",
  "xacquire lock add [rax], 1", "fld st0", "fstp st1", "movq mm0, mm1", "movdqa xmm15, xmm0", "kmovw k1, eax", "bndmk bnd0, [rax]",
  "mov cr0, eax", "mov dr7, eax", "lgdt [gdtr]", "ltr ax", "db 0x1F, 1Fh, 0h1F, $1F", "dw 101b, 0b101, 0y1_0, 1y", "dd 17o, 0o17, 0q17, 17q",
  "dq 123, 0d123, 123d, 0t9, 9t", "dt 1.5, 1., 1.5e3, 1e10, 0d1.5, 1_000.5e-3", "dd 0x1.8p3, 0b1.1p2, 0o1.7p1, $1.8p3, 0x1p3",
  "dt 12p3, 12p, 1_2p", "db 'a', \"str\", `back`", "db \"unterminated", "times 10 db 0", "resb 64", "incbin \"file.bin\"",
  "equ 5", "x equ 1 dup 2", "strict near far abs rel nosplit", "a16 o32 a64", "sbyte word dword qword tword oword yword zword",
  "__?FILE?__", "__?LINE?__", "__?NASM_MAJOR?__", "___?NASM_PATCHLEVEL?__", "__?USE_ALTREG?__", "__?ALIGN_NOPS?__", "__?ALIGNMODE?__",
  "__?PASS?__", "__?utf16le?__", "__?float32?__", "__?Infinity?__", "__?QNaN?__", "__FILE__", "__USE_ALTREG__", "__ALIGN_X__",
  "__utf16__", "__NASM_VER__", "___NASM_PATCHLEVEL__", "__PASS__", "Inf", "NaN", "QNaN", "float32", "float80m", "ilog2e", "ilog2cw",
  "..start", "..got", "..gotpcrel", "..plt", "..tlsie", "..imagebase", "; comment", "  ; indented", "x  # hash comment", "# start",
  "#x", "/* block */", "/* open", "close */", "#if 0", "#endif", "%if 0", "  %endif", "#error stop", "%warning \\", "#include \"a.h\"",
  "%include <x.inc> // c", "#include", "%define X 1", "%define F(x) %strlen(x) \\", "%xdefine Y %eval(1 + %count(a))", "%macro m 1",
  "%endmacro", "%ifdef X", "%elifdef Y", "%else", "%pragma x", "%assign i i+1", "%rep 4", "%endrep", "%push ctx", "%undef X",
  "%defstr s x", "%deftok t x", "%use altreg", "%ifusing x", "%abs(1)", "%sel(1, a, b)", "%tok(\"x\")", "%cond(1, a)", "%isdef(x)",
  "ſhl eax, 1", "Kand k1, k2", "ﬁld st0", "fiﬆp st0", "ﬆrict", "ﬂd1", "movsß", "movß", "é:", "x.y:", "a-b:", "label_é:",
];

/** Texts that mutation inserts. */
export const mutations: string[] = [
  ";", "#", " # ", "%", "/*", "*/", "\"", "'", "`", "\\", "\\\n", ":", ".", "..", "..@", "$", "{", "}", "(", ")", "[", "]", ",",
  "0x", "0b", "_", "e", "p", "h", " ", "\t", " ", "　", "ſ", "K", "ß", "é", "mov ", "%define ", "__?",
];

/** Info strings of Markdown fences, which Shiki's Markdown grammar does not
 * hand to assembly. */
export const fences: string[] = ["asm"];

/** What `onig.ts` draws on: snippets it inserts into fixture lines. */
export const probe = {
  snippets: [
    ";", " # ", "#", "%", "/*", "*/", "\"", "'", "`", "\\", ":", ".", "..", "..@", "$", "$1", "$1_", "{k1}", "{z}", "{1to8}", "{rn-sae}",
    "{vex2}", "0x1F", "1Fh", "0b1.1p2", "1.5e3", "1.", "12p", "0d1.5", "0t9", "17q", "101b", "$1.8p3", "__?FILE?__", "__?USE_X?__",
    "__USE_X__", "__ALIGN_Y__", "..got", "..plt", "section ", ".text", ".rodata", "global ", ".globl", "extern", "%define ", "#if 0",
    "%endif", "#include ", "%strlen(", "%eval(", "mov", "MOV", "eax", "zmm31", "xmm16", "st", "cr8", "ſ", "K", "ß", "ﬁ", "ﬆ", "ﬂ", "é",
    " ", "　", "İ", "ı",
  ],
};
