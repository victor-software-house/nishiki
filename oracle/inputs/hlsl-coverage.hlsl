// Every pattern of the hlsl grammar, written for nishiki.
/* A block comment
   over two lines */
#define WIDTH 1.5f
  #  ifdef DEBUG
#ifx elif
#pragma pack_matrix(row_major)
#include "common.hlsli"

typedef float4 Color;
cbuffer Constants : register(b0)
{
    float4x4 world : packoffset(c0);
    float4 reg : register; float4 off : packoffset;
    min16float2 scale;
    uint3 size;
    double2x3 grid;
    unsigned int count;
    vector v; matrix m; dword d; void nothing;
};

struct VSOut
{
    float4 position : SV_Position;
    centroid float2 uv : TEXCOORD0;
    nointerpolation float3 normal : NORMAL1;
    linear float4 color :COLOR;
    noperspective float depth : SV_Depth;
    sample float fog : FOG;
    float4 target : SV_Target7;
    uint id : SV_DispatchThreadID;
    float cover : SV_InnerCoverage;
    float binormal : binormal2;
};

Texture2D<float4> albedo : register(t0);
Texture2DMSArray<float4> msaa;
RWTexture2D<float4> output;
RWStructuredBuffer<uint> counters;
RasterizerOrderedTexture2D<float4> rov;
AppendStructuredBuffer<uint> queue;
ByteAddressBuffer raw;
SamplerState linearClamp;
SamplerComparisonState shadowCmp;
sampler2D legacy; samplerCUBE cube; texture2D oldTex; textureCUBE oldCube;
string label = "a \"quoted\" name\\";
groupshared float cache[64];
static const snorm float half_ = 0.5;
unorm float other = .25f;
globallycoherent RWBuffer<uint> flags;
interface ILight { float3 Shade(); };
class Point : ILight { float3 Shade() { return 0; } };
namespace Lights { tbuffer Extra { float e; }; }

[numthreads(8, 8, 1)]
void main(uint3 tid : SV_DispatchThreadID, in float a, out float b, inout float c)
{
    float x = 1.0 + 2. + 3.5F + 4 + 5f + 0x1F + 0XAB + a.5 + 1.5x;
    bool ok = true && !false;
    if (x > 0) { return; } else { discard; }
    for (int i = 0; i < 4; i++) { continue; }
    while (ok) { break; }
    do { x = max(x, 1); } while (false);
    switch (i) { case 0: break; default: break; }
    float y = ok ? x : color;
    output[tid.xy] = albedo.Sample(linearClamp, float2(0, 0));
    precise float p = mul(world, x);
    export extern volatile uniform shared row_major column_major inline float q;
}

[maxvertexcount(3)]
void gs(triangle VSOut input[3], inout TriangleStream<VSOut> stream, point float a, line float b, lineadj float c, triangleadj float d)
{
    LineStream<VSOut> lines; PointStream<VSOut> points;
    InputPatch<VSOut, 3> ip; OutputPatch<VSOut, 3> op; ConstantBuffer<VSOut> cb; ConsumeStructuredBuffer<uint> cs;
}

BlendState AlphaBlend
{
    AlphaToCoverageEnable = FALSE;
    BlendEnable[0] = TRUE;
    SrcBlend = SRC_ALPHA;
    DestBlend = inv_src_alpha;
    BlendOp = ADD;
    SrcBlendAlpha = ONE; DestBlendAlpha = Zero; BlendOpAlpha = max;
    RenderTargetWriteMask[0] = 0x0F;
};

DepthStencilState DepthOn
{
    DepthEnable = TRUE;
    DepthWriteMask = ALL;
    DepthFunc = LESS_EQUAL;
    StencilEnable = FALSE; StencilReadMask = 0xFF; StencilWriteMask = 0xFF;
    FrontFaceStencilFail = KEEP; FrontFaceStencilZFail = INCR_SAT; FrontFaceStencilPass = REPLACE; FrontFaceStencilFunc = ALWAYS;
    BackFaceStencilFail = DECR; BackFaceStencilZFail = INVERT; BackFaceStencilPass = KEEP; BackFaceStencilFunc = NEVER;
};

RasterizerState Wire
{
    FillMode = WIREFRAME; CullMode = NONE; FrontCounterClockwise = FALSE;
    DepthBias = 0; DepthBiasClamp = 0; SlopeScaleDepthBias = 0;
    ZClipEnable = TRUE; ScissorEnable = FALSE; MultiSampleEnable = FALSE; AntiAliasedLineEnable = FALSE;
};

SamplerState Aniso
{
    Filter = ANISOTROPIC; AddressU = WRAP; AddressV = MIRROR_ONCE; AddressW = BORDER;
    MipLODBias = 0; MaxAnisotropy = 16; ComparisonFunc = GREATER; BorderColor = float4(0, 0, 0, 0);
    MinLOD = 0; MaxLOD = 0;
};

technique11 Render
{
    pass P0
    {
        SetVertexShader(CompileShader(vs_5_0, main()));
        SetPixelShader(compile ps_5_0 main());
        SetBlendState(AlphaBlend, float4(0, 0, 0, 0), 0xFFFFFFFF);
        SetRasterizerState(NULL);
    }
}
technique10 Old { pass {} }
Technique Upper { pass {} }
technique Plain { pass {} }
