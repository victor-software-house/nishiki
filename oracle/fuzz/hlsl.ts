// HLSL for the differential fuzzer (fuzz.ts) and the Oniguruma harness
// (onig.ts): pieces of HLSL to build documents from, texts to insert, the
// info strings of Markdown fences, and the texts the harness probes
// expressions with.
// The semantic snippets put zero, one and two blanks after a `:`, and the
// number snippets put a word character after each place a `\b` is tried.

/** Pieces that random documents are made of. */
export const pieces: string[] = ["cbuffer Constants : register(b0)","{","};","}","struct VSOut","float4 position : SV_Position;","float2 uv : TEXCOORD0;","float3 normal :NORMAL1;","float4 color :  COLOR;","uint id : SV_DispatchThreadID;","float4 target : SV_Target7;","Texture2D<float4> albedo : register(t0);","RWStructuredBuffer<uint> counters;","SamplerState linearClamp;","sampler2D legacy;","[numthreads(8, 8, 1)]","void main(uint3 tid : SV_DispatchThreadID)","float x = 1.0 + 2. + .5f + 3F + 0x1F;","float y = 1.5x + a.5;","bool ok = true && !false;","if (x > 0) { return; } else { discard; }","for (int i = 0; i < 4; i++) { continue; }","x = max(x, 1); y = max;","v = c ? a : color;","output[tid.xy] = albedo.Sample(linearClamp, uv);","#define WIDTH 1.5f","  #  ifdef DEBUG","#ifx","#endif","#include \"common.hlsli\"","// comment","/* block","comment */","/* one */ x;","string s = \"a \\\"b\\\" c\\\\\";","BlendState B { BlendEnable = TRUE; SrcBlend = SRC_ALPHA; BlendOp = add; };","DepthStencilState D { DepthFunc = LESS_EQUAL; DepthWriteMask = ALL; };","RasterizerState R { FillMode = WIREFRAME; CullMode = NONE; };","SamplerState S { Filter = ANISOTROPIC; AddressU = WRAP; };","technique11 T { pass P0 { SetPixelShader(compile ps_5_0 main()); } }","groupshared float cache[64];","static const snorm float h = 0.5;","typedef float4 Color;","é = 1;","x = ſv_depth;","","  "];

/** Texts that mutation inserts. */
export const mutations: string[] = ["\"","\\","/*","*/","//","#",":",": ","(",")","{","}",";",".","f","0x","é"];

/** Info strings of Markdown fences. Shiki's Markdown grammar has no HLSL fence. */
export const fences: string[] = ["hlsl"];

/** For the Oniguruma harness. */
export const probe = {
  /** Texts inserted into fixture lines. */
  snippets: [":COLOR",": COLOR",":  COLOR",":\u00a0COLOR",": color2",": TEXCOORD12",": TEXCOORD1x",": SV_Target7",": SV_Target8",": sv_position",": ſv_depth",": SV_InnerCoverage","x:fog","1.","1.5","1.5f","1.5fx","1.5x","1.x","1. ",".5",".5f",".5fx","a.5","3f","3fx","0x1F","0x1G","0XAB","true","truex","TRUE","max","max(","max (","Max","maxx","float4(","float4x4","Texture2D(","register(","register","#if","#ifx","  #  define","#\u00a0if","x #if","\"a\\\"b\"","\"a\\","/* x */","// x","é","x\u00a0y"] as string[],
  /** Texts tried at 0 for an expression that starts with `\\G`. */
  anchored: [""] as string[],
  /** Whole texts tried at 0. */
  whole: ["#define X 1","  #  ifdef X","#ifx","#pragma once","# include \"a\"","x #if","/* a */","// a","\"a\"","float4 p : SV_Position;","max(a, b)","technique11 T"] as string[],
};
