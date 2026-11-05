; Written for nishiki: the units of the asm grammar that the third-party
; fixtures leave out.

section .text
segment .bss
.rodata
global _start
.globl main
extern printf
required x

_start:
    ; One word that each register and mnemonic list owns.
    ah cs eflags eip cr0 fpr0 mxcsr zmm0 bnd0 gdt
    dr0 cr8 db0 xmm16 cmova bswap cbw adc aaa and
    rcl seta bsf ja bound cmps in clc lds cpuid
    cldemote rdrand andn fcmovb fbld fadd fabs fcom f2xm1 fld1
    fclex fxrstor movd packssdw paddb pcmpeqb pand pslld emms movaps
    addps cmpps andnps shufps cvtpi2ps ldmxcsr pavgb maskmovq prefetchnta movapd
    addpd andnpd cmppd shufpd cvtdq2pd cvtdq2ps movdq2q paddq lfence addsubpd
    phaddd pabsb dppd movntdqa blendpd pmaxsb roundpd extractps pmovsxbd mpsadbw
    pcmpestri aesdec vbroadcastf128 vaesdec vcmppd vcvtdq2pd vhaddpd vandnpd vmovapd vaddpd
    vpackssdw vpshufb vpabsb vpand vpblendvb vpmovmskb vpinsrb vpsrad vblendpd vpalignr
    vmovd vextractps vmaskmovdqu vcvtph2ps vfmadd132pd vbroadcasti128 vpbroadcastb vpblendd vgatherdd xabort
    sha1msg1 valignd vcvtpd2udq vcvtqq2pd vexpandpd vgetexppd vpblendmd vpermi2d vprold vpsravq
    vrcp14pd vscatterdd vcvtpd2qq vextractf64x2 vpmovb2d vdbpsadbw vpblendmb vpermi2b vpmovb2m vpsllvw
    vpbroadcastmb2q kaddb kunpckbw vexp2pd vgatherpf0dpd vaddph vcvtdq2ph vcvtph2dq vcvtph2psx vcvtsd2sh
    vcvtsh2sd vcvttph2dq vfmadd132ph vfmaddsub132ph vmaxph arpl hlt xgetbv cdqe invept
    getsec capabilities bndcl encls eadd eenter eaug eaccept clrssbsy endbranch
    ldtilecfg clui enqcmd ibhf cvtpd1pi gf2p8affineinvqb hreset incsspd movdir64b pabsq
    rdpid serialize tpause umonitor vbroadcastf32x2 vcompressw vdpbf16ps vextractf32x8 vinsertf32x8 vmaskmov
    vp2intersectd vpbroadcasti32x4 vpcompressb vpdpbusd vpermb vpmadd52huq vpopcntb vprorvd vpshldd vpternlogd
    vpxord vscalefpd wbnoinvd v4fmaddps aesdec128kl blcfill clgi llwpcb movntsd clzero
    extrq vfmaddpd vpcmov vphaddbd vfrczpd femms pavgusb pfcmpeq paddsiw montmul
    xcryptcbc icebp ccmpb cfcmovb cmpbexadd jmpabs cmpeqpd pclmulhqhdq vcmpeq_ospd vpcmpeq
    vpcomeqb

..@local: nop
..@back
.1:
.12ab
1bad:
.$odd:
.@odd:
@weird:
x.y: ret
$sym
    mov rax, $1_ff

    vaddps zmm0{k1}{z}, zmm1, zmm2
    vpxord zmm3, zmm4, [rax]{1to16}
    {vex3} vpaddd ymm0, ymm1, ymm2
    {evex} vmovdqu32 zmm0, [rdi]
    {rex} add eax, 1
    vaddps zmm0, zmm1, zmm2, {rn-sae}
    vaddps zmm0, zmm1, zmm2, {sae}
    rep movsb
    lock cmpxchg [rdi], ecx
    xacquire lock add dword [rax], 1
    nobnd jmp rax
    fld st0
    mov cr0, eax
    mov dr7, eax
    mov cr8, rax
    mov tr6, eax
    lgdt [gdtr]
    bndmk bnd0, [rax]
    movdqa xmm16, xmm0
    vmovdqu64 zmm31, [rsi]
    mov ax, ds
    pushfq
    mov eip, 1

    db 0x1F, 1Fh, 0h1F, $1F, $0
    dw 101b, 0b101, 0y1_0, 1y
    dd 17o, 0o17, 0q17, 17q
    dq 123, 0d123, 123d, 0t9, 9t
    dt 1.5, 1., 1.5e3, 1e10, 0d1.5, 1_000.5e-3
    dd 0x1.8p3, 0b1.1p2, 0o1.7p1, $1.8p3, 0x1p3, 0b1p2, 0o7p1, $1_fp3
    dt 12p3, 12p, 1_2p
    times 10 db 0
    resb 64
    incbin "file.bin"
five equ 5
    strict near far abs rel nosplit
    a16 o32 a64
    sbyte word dword qword tword oword yword zword ddq
    dw __?FILE?__, __?LINE?__, __?BITS?__
    dw __?NASM_MAJOR?__, ___?NASM_PATCHLEVEL?__, __?USE_ALTREG?__
    dw __?ALIGN_NOPS?__, __?ALIGNMODE?__, __?PASS?__, __?DATE_NUM?__
    dw __?utf16le?__, __?float32?__, __?Infinity?__, __?QNaN?__
    dw __FILE__, __USE_ALTREG__, __ALIGN_X__, __utf16__, __NASM_VER__
    dw ___NASM_PATCHLEVEL__, __PASS__, __ALIGNMODE__, __UTC_TIME__
    dd Inf, NaN, QNaN, float32, float80m, ilog2e, ilog2cw
    dd ..start, ..got, ..gotpcrel, ..plt, ..tlsie, ..imagebase

    movl %eax, %ebx   # a hash comment
# a hash comment at the start
/* a block
   comment */
#if 0
skipped
#endif
%if 0
skipped
  %endif
#error stop here \
%warning careful
#include "a.inc"
%include <x.inc> // c
#include \
%define X 1
%define F(x) %strlen(x) \
%xdefine Y %eval(1 + %count(a, b))
%macro m 1
%endmacro
%ifdef X
%elifdef Y
%else
%endif
%pragma x
%line 1
%assign i i+1 \
%rep 4
%endrep
%push ctx
%pop
%undef X
%defstr s x
%deftok t x
%use altreg
%ifusing x
%strlen n "abc"
%define G %abs(%sel(1, a, b)) %tok("x") %cond(1, a) %isdef(x) %num(1)
