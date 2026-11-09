// Every pattern of the shaderlab grammar, written for nishiki.
Shader "Custom/Coverage"
{
    Properties
    {
        _MainTex ("Albedo", 2D) = "white" {}
        _Color ("Color", Color) = (1, 1, 1, 1)
        _Glossiness ("Smoothness", Range(0.0, 1)) = 0.5
        _Count ("Count", Int) = 3
        _Scale ("Scale", Float) = 1.5
        _Offset ("Offset", Vector) = (0, 0, 0, 0)
        _Volume ("Volume", 3D) = "" {}
        _Sky ("Sky", Cube) = "" {}
        _Any ("Any", Any) = "" {}
        [NoScaleOffset] _BumpMap ("Normal", 2D) = "bump" {}
        [Toggle(_EMISSION)] _Emission ("Emission", Float) = 0
        [Enum(UnityEngine.Rendering.CullMode)] _Cull ("Cull", Float) = 2
        [PowerSlider(3.0)] _Shininess ("Shininess", Range(0.01, 1)) = 0.08
    }
    Category
    {
        SubShader
        {
            Tags { "Queue"="Transparent" "RenderType"="Opaque" }
            LOD 200
            Name "MAIN"
            Cull [_Cull]
            ZWrite On
            ZTest LEqual
            Offset -1, -1
            Blend SrcAlpha OneMinusSrcAlpha
            BlendOp Add
            ColorMask RGB
            AlphaToMask Off
            Lighting Off
            Fog { Mode Exp2 Density 0.5 Color (1, 1, 1, 1) }
            Stencil
            {
                Ref 2 ReadMask 255 WriteMask 255
                Comp Always CompBack Less CompFront Greater
                Pass Replace Fail Keep ZFail IncrSat
            }
            Material { Diffuse [_Color] Ambient (1, 1, 1, 1) Shininess 0.7 Specular (1, 1, 1, 1) Emission (0, 0, 0, 0) }
            SeparateSpecular On
            SetTexture [_MainTex] { ConstantColor (1, 1, 1, 1) Combine texture * primary double, texture Quad }
            SetTexture [_BumpMap] { Combine previous * constant, previous Alpha }
            AlphaTest Greater 0.5
            ColorMaterial AmbientAndDiffuse
            BindChannels { Bind "Vertex", vertex Bind "Normal", normal Bind "TexCoord0", texcoord0 Bind "TexCoord1", texcoord1 Bind "Tangent", tangent }
            Fog { Mode Global } Fog { Mode Linear } Fog { Mode Exp }
            Blend One Zero, DstColor DstAlpha
            Blend OneMinusSrcColor SrcColor, OneMinusDstColor OneMinusDstAlpha
            BlendOp Sub, RevSub BlendOp Min BlendOp Max BlendOp LogicalClear
            BlendOp LogicalSet BlendOp LogicalCopyInverted BlendOp LogicalCopy BlendOp LogicalNoop
            BlendOp LogicalInvert BlendOp LogicalAnd BlendOp LogicalNand BlendOp LogicalOr BlendOp LogicalNor
            BlendOp LogicalXor BlendOp LogicalEquiv BlendOp LogicalAndReverse BlendOp LogicalAndInverted
            BlendOp LogicalOrReverse BlendOp LogicalOrInverted
            Comp GEqual Comp Equal Comp NotEqual Comp Never Pass Zero Pass DecrSat Pass Invert Pass IncrWrap Pass DecrWrap
            Combine Lerp(texture) previous
            Cull Back Cull Front ColorMask A ColorMask BGR
            UsePass "Specular/FORWARD"
            GrabPass { "_GrabTexture" }
            Dependency "BaseMapShader" = "Diffuse"
            Matrix [_Matrix]

            CGPROGRAM
            #pragma vertex vert
            #pragma fragment frag
            #include "UnityCG.cginc"

            struct v2f { float4 pos : SV_POSITION; fixed4 color : COLOR; fixed2x2 m; };

            v2f vert (appdata_base v)
            {
                v2f o;
                o.pos = mul(UNITY_MATRIX_MVP, v.vertex);
                float3 world = mul(unity_ObjectToWorld, v.vertex).xyz;
                float3 cam = _WorldSpaceCameraPos + _ProjectionParams.xyz;
                float t = _Time.y + _SinTime.x + unity_DeltaTime.z;
                float3 l = _WorldSpaceLightPos0.xyz * _LightColor0.rgb;
                float3 a = unity_AmbientSky.rgb + unity_FogColor.rgb;
                float f = unity_LODFade.x;
                o.color = fixed4(1, 1, 1, 1); fixed3 c = 0; fixed f1;
                return o;
            }

            #if defined(SHADER_API_D3D11) || defined(SHADER_API_GLES3)
            #if SHADER_TARGET >= 30 && UNITY_VERSION >= 560
            #ifdef UNITY_UV_STARTS_AT_TOP
            #ifdef UNITY_PASS_FORWARDBASE
            #endif
            /* a comment with ENDCG inside */
            appdata_full full; appdata_tan tan; appdata_img img;
            ENDCG
        }

        Pass
        {
            HLSLPROGRAM
            struct Input { float2 uv_MainTex; };
            void surf (Input IN, inout SurfaceOutputStandard o) { SurfaceOutput s; SurfaceOutputStandardSpecular ss; }
            float4 frag () : SV_Target { return 0; } ENDHLSL
        }
        CGINCLUDE
        float shared;
        ENDCG
        HLSLINCLUDE
        float other;
        ENDHLSL
    }
    Fallback "Diffuse"
    CustomEditor "MyEditor"
}
