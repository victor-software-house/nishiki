---
nishiki: patch
---

Highlight ShaderLab with a lexer of its own, on by default and in `all`, as `Language::ShaderLab` and the names `shaderlab` and `shader`.
Its `CGPROGRAM` and `HLSLPROGRAM` blocks go to the HLSL lexer, with Unity's own names.
