# Engine test grammars

`oracle/engine-grammars.ts` writes this directory; nobody edits it by hand.
Each grammar is the file Shiki 4.6.0 ships, taken from the repository and commit below.
The grammars are test data: `Cargo.toml` leaves `tests/` out of the published crate.

| Grammar | Source | Licence |
|:--|:--|:--|
| `asm` | [13xforever/x86_64-assembly-vscode at 5c1958f4283d260f5dc88fa4f5f31c620e061a5a][asm-source] | [MIT][asm-licence] |
| `coffee` | [microsoft/vscode at ff58f5083f57b50e6a100ab60454dfb72fe21e80][coffee-source] | [MIT][coffee-licence] |
| `common-lisp` | [qingpeng9802/vscode-common-lisp at 79d04dc9ca80d98e65c4f28b3d67a24a7829b18f][common-lisp-source] | [MIT][common-lisp-licence] |
| `css` | [microsoft/vscode at af600487b1e94374d9f48f57cbf2cad24656b07f][css-source] | [MIT][css-licence] |
| `elixir` | [elixir-editors/elixir-tmbundle at 43c8cd957d5ac6e1abbd8730fc7a08c81a6e76c9][elixir-source] | [Apache-2.0][elixir-licence] |
| `gdscript` | [godotengine/godot-vscode-plugin at 3f2860fd76c591a6010f0d35220490933e41d01d][gdscript-source] | [MIT][gdscript-licence] |
| `graphql` | [prisma-labs/vscode-graphql at 4569aef2364b42b6acd8188e4a6ca9b0ba3c6a1d][graphql-source] | [MIT][graphql-licence] |
| `haml` | [karuna/haml-vscode at aa45415aefd691af0a501ea4f98e434490623e0d][haml-source] | [MIT][haml-licence] |
| `html` | [microsoft/vscode at 45324363153075dab0482312ae24d8c068d81e4f][html-source] | [TextMate bundle licence][html-licence] |
| `html-derivative` | [microsoft/vscode at 4064f172528461afc6944383d07521f6a30da89c][html-derivative-source] | [TextMate bundle licence][html-derivative-licence] |
| `javascript` | [microsoft/vscode at 210541906e5a96ab39f9c753f921b1bd35f4138b][javascript-source] | [MIT][javascript-licence] |
| `jinja` | [samuelcolvin/jinjahtml-vscode at be729c88f723602e7b5df2e9dc5e8c016c31c38f][jinja-source] | [MIT][jinja-licence] |
| `jinja-html` | [samuelcolvin/jinjahtml-vscode at be729c88f723602e7b5df2e9dc5e8c016c31c38f][jinja-html-source] | [MIT][jinja-html-licence] |
| `json` | [microsoft/vscode at d6af4893ed9a3545163a4cb748fa5548bd1e51a5][json-source] | [MIT][json-licence] |
| `jsonc` | [microsoft/vscode at 0adddd82816db666c52038e8277312dc55da2756][jsonc-source] | [MIT][jsonc-licence] |
| `jsonl` | [microsoft/vscode at 0adddd82816db666c52038e8277312dc55da2756][jsonl-source] | [MIT][jsonl-licence] |
| `jsx` | [microsoft/vscode at 210541906e5a96ab39f9c753f921b1bd35f4138b][jsx-source] | [MIT][jsx-licence] |
| `make` | [microsoft/vscode at 91db8ea85fee30ebb3c7a5e7939aa2a8d79811c5][make-source] | [MIT][make-licence] |
| `markdown` | [microsoft/vscode at 6d8ab9737d58fc5eaf07e2ae6553b38183a5de47][markdown-source] | [MIT][markdown-licence] |
| `markdown-vue` | [vuejs/language-tools at 784dd56026e07f1dadb9ef2498418eccbb3dee29][markdown-vue-source] | [MIT][markdown-vue-licence] |
| `matlab` | [mathworks/MATLAB-Language-grammar at 69dbf20c1eb19efd629c0438a2a493e60617e8de][matlab-source] | [BSD-2-Clause][matlab-licence] |
| `objective-c` | [microsoft/vscode at bd1abe08c38bb5deccf65610f8f3c03d2338d20d][objective-c-source] | [MIT][objective-c-licence] |
| `objective-cpp` | [microsoft/vscode at bd1abe08c38bb5deccf65610f8f3c03d2338d20d][objective-cpp-source] | [MIT][objective-cpp-licence] |
| `python` | [microsoft/vscode at cf4c9e469d521fa5f33353737e8157eb0789ad02][python-source] | [MIT][python-licence] |
| `ruby` | [microsoft/vscode at 0ac234bbd0b6dd67efa40d1b593f9808f9dfdaf4][ruby-source] | [MIT][ruby-licence] |
| `rust` | [microsoft/vscode at af600487b1e94374d9f48f57cbf2cad24656b07f][rust-source] | [MIT][rust-licence] |
| `sass` | [TheRealSyler/vscode-sass-indented at f0e50cfaca4b64a1d49f8d938f188d6fd0bcfb6f][sass-source] | [MIT][sass-licence] |
| `shellscript` | [microsoft/vscode at 9473445f7d3dcb5c579f42ece8b6c18c43c63ed3][shellscript-source] | [MIT][shellscript-licence] |
| `tex` | [microsoft/vscode at 609f17a76734f092792a33c891af40bbcaea0c64][tex-source] | [MIT][tex-licence] |
| `toml` | [textmate/toml.tmbundle at e82b64c1e86396220786846201e9aa3f0a2d9ca2][toml-source] | [TextMate bundle licence][toml-licence] |
| `tsx` | [microsoft/vscode at 210541906e5a96ab39f9c753f921b1bd35f4138b][tsx-source] | [MIT][tsx-licence] |
| `typescript` | [microsoft/vscode at 210541906e5a96ab39f9c753f921b1bd35f4138b][typescript-source] | [MIT][typescript-licence] |
| `vb` | [microsoft/vscode at 35d97bc7e439fce0f50f42074041ab2d8571b20a][vb-source] | [TextMate bundle licence][vb-licence] |
| `vue` | [vuejs/language-tools at cd7a6e53d58111640f116d1e19f1c1cf06c7157d][vue-source] | [MIT][vue-licence] |
| `vue-directives` | [vuejs/language-tools at 784dd56026e07f1dadb9ef2498418eccbb3dee29][vue-directives-source] | [MIT][vue-directives-licence] |
| `vue-interpolations` | [vuejs/language-tools at 784dd56026e07f1dadb9ef2498418eccbb3dee29][vue-interpolations-source] | [MIT][vue-interpolations-licence] |
| `vue-sfc-style-variable-injection` | [vuejs/language-tools at e85556a2e4bf0d6c981bdbd95c8a0b1e85e7ae8a][vue-sfc-style-variable-injection-source] | [MIT][vue-sfc-style-variable-injection-licence] |
| `yaml` | [textmate/yaml.tmbundle at e54ceae3b719506dba7e481a77cea4a8b576ae46][yaml-source] | [MIT][yaml-licence] |

[asm-source]: https://github.com/13xforever/x86_64-assembly-vscode/tree/5c1958f4283d260f5dc88fa4f5f31c620e061a5a
[asm-licence]: https://github.com/13xforever/x86_64-assembly-vscode/blob/5c1958f4283d260f5dc88fa4f5f31c620e061a5a/LICENSE.txt
[coffee-source]: https://github.com/microsoft/vscode/tree/ff58f5083f57b50e6a100ab60454dfb72fe21e80
[coffee-licence]: https://github.com/atom/language-coffee-script/blob/HEAD/LICENSE.md
[common-lisp-source]: https://github.com/qingpeng9802/vscode-common-lisp/tree/79d04dc9ca80d98e65c4f28b3d67a24a7829b18f
[common-lisp-licence]: https://github.com/qingpeng9802/vscode-common-lisp/blob/79d04dc9ca80d98e65c4f28b3d67a24a7829b18f/LICENSE
[css-source]: https://github.com/microsoft/vscode/tree/af600487b1e94374d9f48f57cbf2cad24656b07f
[css-licence]: https://github.com/microsoft/vscode-css/blob/HEAD/LICENSE.txt
[elixir-source]: https://github.com/elixir-editors/elixir-tmbundle/tree/43c8cd957d5ac6e1abbd8730fc7a08c81a6e76c9
[elixir-licence]: https://github.com/elixir-editors/elixir-tmbundle/blob/43c8cd957d5ac6e1abbd8730fc7a08c81a6e76c9/LICENSE
[gdscript-source]: https://github.com/godotengine/godot-vscode-plugin/tree/3f2860fd76c591a6010f0d35220490933e41d01d
[gdscript-licence]: https://github.com/godotengine/godot-vscode-plugin/blob/3f2860fd76c591a6010f0d35220490933e41d01d/LICENSE
[graphql-source]: https://github.com/prisma-labs/vscode-graphql/tree/4569aef2364b42b6acd8188e4a6ca9b0ba3c6a1d
[graphql-licence]: https://github.com/prisma-labs/vscode-graphql/blob/4569aef2364b42b6acd8188e4a6ca9b0ba3c6a1d/LICENSE
[haml-source]: https://github.com/karuna/haml-vscode/tree/aa45415aefd691af0a501ea4f98e434490623e0d
[haml-licence]: https://github.com/karuna/haml-vscode/blob/aa45415aefd691af0a501ea4f98e434490623e0d/LICENSE.md
[html-source]: https://github.com/microsoft/vscode/tree/45324363153075dab0482312ae24d8c068d81e4f
[html-licence]: https://github.com/textmate/html.tmbundle/blob/HEAD/README.mdown
[html-derivative-source]: https://github.com/microsoft/vscode/tree/4064f172528461afc6944383d07521f6a30da89c
[html-derivative-licence]: https://github.com/textmate/html.tmbundle/blob/HEAD/README.mdown
[javascript-source]: https://github.com/microsoft/vscode/tree/210541906e5a96ab39f9c753f921b1bd35f4138b
[javascript-licence]: https://github.com/microsoft/TypeScript-TmLanguage/blob/HEAD/LICENSE.txt
[jinja-source]: https://github.com/samuelcolvin/jinjahtml-vscode/tree/be729c88f723602e7b5df2e9dc5e8c016c31c38f
[jinja-licence]: https://github.com/samuelcolvin/jinjahtml-vscode/blob/be729c88f723602e7b5df2e9dc5e8c016c31c38f/LICENSE
[jinja-html-source]: https://github.com/samuelcolvin/jinjahtml-vscode/tree/be729c88f723602e7b5df2e9dc5e8c016c31c38f
[jinja-html-licence]: https://github.com/samuelcolvin/jinjahtml-vscode/blob/be729c88f723602e7b5df2e9dc5e8c016c31c38f/LICENSE
[json-source]: https://github.com/microsoft/vscode/tree/d6af4893ed9a3545163a4cb748fa5548bd1e51a5
[json-licence]: https://github.com/microsoft/vscode-JSON.tmLanguage/blob/HEAD/LICENSE
[jsonc-source]: https://github.com/microsoft/vscode/tree/0adddd82816db666c52038e8277312dc55da2756
[jsonc-licence]: https://github.com/microsoft/vscode-JSON.tmLanguage/blob/HEAD/LICENSE
[jsonl-source]: https://github.com/microsoft/vscode/tree/0adddd82816db666c52038e8277312dc55da2756
[jsonl-licence]: https://github.com/microsoft/vscode-JSON.tmLanguage/blob/HEAD/LICENSE
[jsx-source]: https://github.com/microsoft/vscode/tree/210541906e5a96ab39f9c753f921b1bd35f4138b
[jsx-licence]: https://github.com/microsoft/TypeScript-TmLanguage/blob/HEAD/LICENSE.txt
[make-source]: https://github.com/microsoft/vscode/tree/91db8ea85fee30ebb3c7a5e7939aa2a8d79811c5
[make-licence]: https://github.com/fadeevab/make.tmbundle/blob/HEAD/LICENSE.md
[markdown-source]: https://github.com/microsoft/vscode/tree/6d8ab9737d58fc5eaf07e2ae6553b38183a5de47
[markdown-licence]: https://github.com/microsoft/vscode-markdown-tm-grammar/blob/HEAD/LICENSE.txt
[markdown-vue-source]: https://github.com/vuejs/language-tools/tree/784dd56026e07f1dadb9ef2498418eccbb3dee29
[markdown-vue-licence]: https://github.com/vuejs/language-tools/blob/784dd56026e07f1dadb9ef2498418eccbb3dee29/LICENSE
[matlab-source]: https://github.com/mathworks/MATLAB-Language-grammar/tree/69dbf20c1eb19efd629c0438a2a493e60617e8de
[matlab-licence]: https://github.com/mathworks/MATLAB-Language-grammar/blob/69dbf20c1eb19efd629c0438a2a493e60617e8de/license.txt
[objective-c-source]: https://github.com/microsoft/vscode/tree/bd1abe08c38bb5deccf65610f8f3c03d2338d20d
[objective-c-licence]: https://github.com/jeff-hykin/better-objc-syntax/blob/HEAD/LICENSE
[objective-cpp-source]: https://github.com/microsoft/vscode/tree/bd1abe08c38bb5deccf65610f8f3c03d2338d20d
[objective-cpp-licence]: https://github.com/jeff-hykin/better-objcpp-syntax/blob/HEAD/LICENSE
[python-source]: https://github.com/microsoft/vscode/tree/cf4c9e469d521fa5f33353737e8157eb0789ad02
[python-licence]: https://github.com/MagicStack/MagicPython/blob/HEAD/LICENSE
[ruby-source]: https://github.com/microsoft/vscode/tree/0ac234bbd0b6dd67efa40d1b593f9808f9dfdaf4
[ruby-licence]: https://github.com/Shopify/ruby-lsp/blob/HEAD/LICENSE.txt
[rust-source]: https://github.com/microsoft/vscode/tree/af600487b1e94374d9f48f57cbf2cad24656b07f
[rust-licence]: https://github.com/dustypomerleau/rust-syntax/blob/HEAD/LICENSE.txt
[sass-source]: https://github.com/TheRealSyler/vscode-sass-indented/tree/f0e50cfaca4b64a1d49f8d938f188d6fd0bcfb6f
[sass-licence]: https://github.com/TheRealSyler/vscode-sass-indented/blob/f0e50cfaca4b64a1d49f8d938f188d6fd0bcfb6f/LICENSE
[shellscript-source]: https://github.com/microsoft/vscode/tree/9473445f7d3dcb5c579f42ece8b6c18c43c63ed3
[shellscript-licence]: https://github.com/jeff-hykin/better-shell-syntax/blob/HEAD/LICENSE
[tex-source]: https://github.com/microsoft/vscode/tree/609f17a76734f092792a33c891af40bbcaea0c64
[tex-licence]: https://github.com/jlelong/vscode-latex-basics/blob/HEAD/LICENSE.txt
[toml-source]: https://github.com/textmate/toml.tmbundle/tree/e82b64c1e86396220786846201e9aa3f0a2d9ca2
[toml-licence]: https://github.com/textmate/toml.tmbundle/blob/e82b64c1e86396220786846201e9aa3f0a2d9ca2/README.mdown
[tsx-source]: https://github.com/microsoft/vscode/tree/210541906e5a96ab39f9c753f921b1bd35f4138b
[tsx-licence]: https://github.com/microsoft/TypeScript-TmLanguage/blob/HEAD/LICENSE.txt
[typescript-source]: https://github.com/microsoft/vscode/tree/210541906e5a96ab39f9c753f921b1bd35f4138b
[typescript-licence]: https://github.com/microsoft/TypeScript-TmLanguage/blob/HEAD/LICENSE.txt
[vb-source]: https://github.com/microsoft/vscode/tree/35d97bc7e439fce0f50f42074041ab2d8571b20a
[vb-licence]: https://github.com/textmate/asp.vb.net.tmbundle/blob/HEAD/README.mdown
[vue-source]: https://github.com/vuejs/language-tools/tree/cd7a6e53d58111640f116d1e19f1c1cf06c7157d
[vue-licence]: https://github.com/vuejs/language-tools/blob/cd7a6e53d58111640f116d1e19f1c1cf06c7157d/LICENSE
[vue-directives-source]: https://github.com/vuejs/language-tools/tree/784dd56026e07f1dadb9ef2498418eccbb3dee29
[vue-directives-licence]: https://github.com/vuejs/language-tools/blob/784dd56026e07f1dadb9ef2498418eccbb3dee29/LICENSE
[vue-interpolations-source]: https://github.com/vuejs/language-tools/tree/784dd56026e07f1dadb9ef2498418eccbb3dee29
[vue-interpolations-licence]: https://github.com/vuejs/language-tools/blob/784dd56026e07f1dadb9ef2498418eccbb3dee29/LICENSE
[vue-sfc-style-variable-injection-source]: https://github.com/vuejs/language-tools/tree/e85556a2e4bf0d6c981bdbd95c8a0b1e85e7ae8a
[vue-sfc-style-variable-injection-licence]: https://github.com/vuejs/language-tools/blob/e85556a2e4bf0d6c981bdbd95c8a0b1e85e7ae8a/LICENSE
[yaml-source]: https://github.com/textmate/yaml.tmbundle/tree/e54ceae3b719506dba7e481a77cea4a8b576ae46
[yaml-licence]: https://github.com/textmate/yaml.tmbundle/blob/e54ceae3b719506dba7e481a77cea4a8b576ae46/Syntaxes/YAML-license.txt
