# Oracle inputs

`mise run oracle` fetches each input below at its pinned commit and writes Shiki's scopes for it to `<language>/<name>.txt`.
Each fixture repeats its input's text line by line.
The fixtures are test data: `Cargo.toml` leaves `tests/` out of the published crate.

| Fixture | Source | Licence |
|:--|:--|:--|
| `rust/sample` | [`samples/rust.sample`][rust-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `rust/ripgrep-searcher` | [`crates/searcher/src/searcher/mod.rs`][ripgrep-searcher] in BurntSushi/ripgrep | [Unlicense or MIT][ripgrep-licence] |
| `rust/serde-de` | [`serde_core/src/de/mod.rs`][serde-de] in serde-rs/serde | [MIT or Apache-2.0][serde-licence] |
| `markdown/sample` | [`samples/markdown.sample`][markdown-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `markdown/tokio-readme` | [`README.md`][tokio-readme] in tokio-rs/tokio | [MIT][tokio-licence] |
| `markdown/serde-readme` | [`README.md`][serde-readme] in serde-rs/serde | [MIT or Apache-2.0][serde-licence] |
| `markdown/book-ownership` | [`src/ch04-01-what-is-ownership.md`][book-ownership] in rust-lang/book | [MIT or Apache-2.0][book-licence] |
| `markdown/book-strings` | [`src/ch08-02-strings.md`][book-strings] in rust-lang/book | [MIT or Apache-2.0][book-licence] |
| `markdown/clap-readme` | [`README.md`][clap-readme] in clap-rs/clap | [MIT or Apache-2.0][clap-licence] |
| `markdown/zod-readme` | [`packages/zod/README.md`][zod-readme] in colinhacks/zod | [MIT][zod-licence] |
| `markdown/ky-readme` | [`readme.md`][ky-readme] in sindresorhus/ky | [MIT][ky-licence] |
| `markdown/zod-api` | [`packages/docs/content/api.mdx`][zod-api] in colinhacks/zod | [MIT][zod-licence] |
| `markdown/nvm-readme` | [`README.md`][nvm-readme] in nvm-sh/nvm | [MIT][nvm-licence] |
| `markdown/pyenv-readme` | [`README.md`][pyenv-readme] in pyenv/pyenv | [MIT][pyenv-licence] |
| `markdown/cargo-release` | [`doc/contrib/src/process/release.md`][cargo-release] in rust-lang/cargo | [MIT or Apache-2.0][cargo-licence] |
| `markdown/eslint-no-restricted-imports` | [`docs/src/rules/no-restricted-imports.md`][eslint-imports] in eslint/eslint | [MIT][eslint-licence] |
| `markdown/eslint-mcp` | [`docs/src/use/mcp.md`][eslint-mcp] in eslint/eslint | [MIT][eslint-licence] |
| `typescript/sample` | [`samples/typescript.sample`][ts-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `typescript/vue-effect` | [`packages/reactivity/src/effect.ts`][vue-effect] in vuejs/core | [MIT][vue-licence] |
| `typescript/vue-api-create-app` | [`packages/runtime-core/src/apiCreateApp.ts`][vue-app] in vuejs/core | [MIT][vue-licence] |
| `typescript/zod-classic-schemas` | [`packages/zod/src/v4/classic/schemas.ts`][zod-schemas] in colinhacks/zod | [MIT][zod-licence] |
| `typescript/ky-core` | [`source/core/Ky.ts`][ky-core] in sindresorhus/ky | [MIT][ky-licence] |
| `shellscript/sample` | [`samples/shellscript.sample`][shell-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `shellscript/nvm` | [`nvm.sh`][nvm] in nvm-sh/nvm | [MIT][nvm-licence] |
| `shellscript/ruby-build` | [`bin/ruby-build`][ruby-build] in rbenv/ruby-build | [MIT][ruby-build-licence] |
| `shellscript/pyenv-init` | [`libexec/pyenv-init`][pyenv-init] in pyenv/pyenv | [MIT][pyenv-licence] |
| `json/sample` | [`samples/json.sample`][json-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `json/vue-package` | [`package.json`][vue-package] in vuejs/core | [MIT][vue-licence] |
| `json/zod-package` | [`packages/zod/package.json`][zod-package] in colinhacks/zod | [MIT][zod-licence] |
| `json/vscode-typescript-grammar` | [`extensions/typescript-basics/syntaxes/TypeScript.tmLanguage.json`][vscode-ts-grammar] in microsoft/vscode | [MIT][vscode-licence] |
| `jsonc/sample` | [`samples/jsonc.sample`][jsonc-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `jsonc/vscode-launch` | [`.vscode/launch.json`][vscode-launch] in microsoft/vscode | [MIT][vscode-licence] |
| `jsonl/sample` | [`samples/jsonl.sample`][jsonl-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `jsonl/rag-output` | [`output.jsonl`][rag-output] in jxnl/n-levels-of-rag | [MIT][rag-licence] |
| `jsonl/rag-output-50` | [`output-50.jsonl`][rag-output-50] in jxnl/n-levels-of-rag | [MIT][rag-licence] |

Each fixture also has one `.colors` file per theme below: Shiki's colour and font style for each token.

| Theme | Source | Licence |
|:--|:--|:--|
| `themes/github-dark` | [primer/github-vscode-theme][github-theme], through `@shikijs/themes` 4.6.0 | [MIT][github-theme-licence] |
| `themes/one-light` | [akamud/vscode-theme-onelight][one-light], through `@shikijs/themes` 4.6.0 | [MIT][one-light-licence] |
| `themes/catppuccin-mocha` | [catppuccin/vscode][catppuccin], through `@shikijs/themes` 4.6.0 | [MIT][catppuccin-licence] |

`ripgrep-searcher` and `serde-de` are held out: they were not among the inputs the Rust lexer was written against.
`markdown/book-ownership`, `markdown/book-strings` and `markdown/clap-readme` are held out in the same way: they were added after the other Markdown inputs passed.
`markdown/ky-readme` and `markdown/zod-api` are held out for the TypeScript fences, and `markdown/zod-readme` is the development input for them.
`zod-classic-schemas` and `ky-core` are held out for the TypeScript lexer the same way.
`ruby-build` and `pyenv-init` are held out for shell, for the same reason.
`markdown/nvm-readme` is the development input for shell fences and `markdown/pyenv-readme` is held out.
`json/sample`, `json/vue-package`, `jsonc/sample`, `jsonl/sample` and `jsonl/rag-output` are the development inputs for JSON.
`json/zod-package`, `json/vscode-typescript-grammar`, `jsonc/vscode-launch` and `jsonl/rag-output-50` are held out.
`markdown/eslint-no-restricted-imports` is the development input for JSON fences and `markdown/eslint-mcp` is held out.
No input has a `jsonc` or `jsonl` fence, so those two spellings are covered by unit tests only.

[rust-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/rust.sample
[shiki-samples-licence]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/LICENSE
[ripgrep-searcher]: https://github.com/BurntSushi/ripgrep/blob/91f5d22ef48bde459d90a8c7dbeab416e97d0f2b/crates/searcher/src/searcher/mod.rs
[ripgrep-licence]: https://github.com/BurntSushi/ripgrep/blob/91f5d22ef48bde459d90a8c7dbeab416e97d0f2b/COPYING
[serde-de]: https://github.com/serde-rs/serde/blob/0db9b3b504bcb0a7fa9cf001429869519afd3e9a/serde_core/src/de/mod.rs
[ts-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/typescript.sample
[vue-effect]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages/reactivity/src/effect.ts
[vue-app]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages/runtime-core/src/apiCreateApp.ts
[vue-licence]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/LICENSE
[zod-schemas]: https://github.com/colinhacks/zod/blob/2554b70c3d4a80273381e61d527d820720626440/packages/zod/src/v4/classic/schemas.ts
[zod-licence]: https://github.com/colinhacks/zod/blob/2554b70c3d4a80273381e61d527d820720626440/LICENSE
[zod-readme]: https://github.com/colinhacks/zod/blob/2554b70c3d4a80273381e61d527d820720626440/packages/zod/README.md
[ky-readme]: https://github.com/sindresorhus/ky/blob/3541888878c4d303a8bc2ad61815f4e048f55a5d/readme.md
[zod-api]: https://github.com/colinhacks/zod/blob/2554b70c3d4a80273381e61d527d820720626440/packages/docs/content/api.mdx
[ky-core]: https://github.com/sindresorhus/ky/blob/3541888878c4d303a8bc2ad61815f4e048f55a5d/source/core/Ky.ts
[ky-licence]: https://github.com/sindresorhus/ky/blob/3541888878c4d303a8bc2ad61815f4e048f55a5d/license
[serde-licence]: https://github.com/serde-rs/serde/blob/0db9b3b504bcb0a7fa9cf001429869519afd3e9a/LICENSE-MIT
[github-theme]: https://github.com/primer/github-vscode-theme/tree/cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf
[github-theme-licence]: https://github.com/primer/github-vscode-theme/blob/cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf/LICENSE
[one-light]: https://github.com/akamud/vscode-theme-onelight/tree/5866e900db932d580e978a58db42f65cde07998b
[one-light-licence]: https://github.com/akamud/vscode-theme-onelight/blob/5866e900db932d580e978a58db42f65cde07998b/LICENSE
[catppuccin]: https://github.com/catppuccin/vscode/tree/befc9e6fc41980f4241408f7049755d47c06ff45
[catppuccin-licence]: https://github.com/catppuccin/vscode/blob/befc9e6fc41980f4241408f7049755d47c06ff45/LICENSE
[markdown-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/markdown.sample
[tokio-readme]: https://github.com/tokio-rs/tokio/blob/b499257ef62bd631b83a988d371f4330f545670c/README.md
[tokio-licence]: https://github.com/tokio-rs/tokio/blob/b499257ef62bd631b83a988d371f4330f545670c/LICENSE
[serde-readme]: https://github.com/serde-rs/serde/blob/0db9b3b504bcb0a7fa9cf001429869519afd3e9a/README.md
[book-ownership]: https://github.com/rust-lang/book/blob/2771df165f092a4fbb9f46b5b2e7450f19010adb/src/ch04-01-what-is-ownership.md
[book-strings]: https://github.com/rust-lang/book/blob/2771df165f092a4fbb9f46b5b2e7450f19010adb/src/ch08-02-strings.md
[book-licence]: https://github.com/rust-lang/book/blob/2771df165f092a4fbb9f46b5b2e7450f19010adb/LICENSE-MIT
[clap-readme]: https://github.com/clap-rs/clap/blob/db30b39a55c188763666268447050a0dacba3bf0/README.md
[clap-licence]: https://github.com/clap-rs/clap/blob/db30b39a55c188763666268447050a0dacba3bf0/LICENSE-MIT
[shell-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/shellscript.sample
[cargo-release]: https://github.com/rust-lang/cargo/blob/6238f6fd61bffc158aabeb77b779fc16d6e7b5fc/doc/contrib/src/process/release.md
[cargo-licence]: https://github.com/rust-lang/cargo/blob/6238f6fd61bffc158aabeb77b779fc16d6e7b5fc/LICENSE-MIT
[nvm]: https://github.com/nvm-sh/nvm/blob/5e38bc0caed342356ffd26eb21e0e9b995ac228b/nvm.sh
[nvm-readme]: https://github.com/nvm-sh/nvm/blob/5e38bc0caed342356ffd26eb21e0e9b995ac228b/README.md
[pyenv-readme]: https://github.com/pyenv/pyenv/blob/e6669f0a84df60daba8672c3b3f0caebd0fdcef8/README.md
[nvm-licence]: https://github.com/nvm-sh/nvm/blob/5e38bc0caed342356ffd26eb21e0e9b995ac228b/LICENSE.md
[ruby-build]: https://github.com/rbenv/ruby-build/blob/4ef3159f94dd66c8325efb42db2faeb7d5732db0/bin/ruby-build
[ruby-build-licence]: https://github.com/rbenv/ruby-build/blob/4ef3159f94dd66c8325efb42db2faeb7d5732db0/LICENSE
[pyenv-init]: https://github.com/pyenv/pyenv/blob/e6669f0a84df60daba8672c3b3f0caebd0fdcef8/libexec/pyenv-init
[pyenv-licence]: https://github.com/pyenv/pyenv/blob/e6669f0a84df60daba8672c3b3f0caebd0fdcef8/LICENSE
[eslint-imports]: https://github.com/eslint/eslint/blob/11c66ba08381cb7cff7679d662c9f6654b6fe712/docs/src/rules/no-restricted-imports.md
[eslint-mcp]: https://github.com/eslint/eslint/blob/11c66ba08381cb7cff7679d662c9f6654b6fe712/docs/src/use/mcp.md
[eslint-licence]: https://github.com/eslint/eslint/blob/11c66ba08381cb7cff7679d662c9f6654b6fe712/LICENSE
[json-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/json.sample
[jsonc-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/jsonc.sample
[jsonl-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/jsonl.sample
[vue-package]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/package.json
[zod-package]: https://github.com/colinhacks/zod/blob/2554b70c3d4a80273381e61d527d820720626440/packages/zod/package.json
[vscode-ts-grammar]: https://github.com/microsoft/vscode/blob/a4e689ae13ef7a1fe9b515656b51c9be7c274294/extensions/typescript-basics/syntaxes/TypeScript.tmLanguage.json
[vscode-launch]: https://github.com/microsoft/vscode/blob/a4e689ae13ef7a1fe9b515656b51c9be7c274294/.vscode/launch.json
[vscode-licence]: https://github.com/microsoft/vscode/blob/a4e689ae13ef7a1fe9b515656b51c9be7c274294/LICENSE.txt
[rag-output]: https://github.com/jxnl/n-levels-of-rag/blob/2ce1106b0163d11fcf7959187b42f0d3a67fbe05/output.jsonl
[rag-output-50]: https://github.com/jxnl/n-levels-of-rag/blob/2ce1106b0163d11fcf7959187b42f0d3a67fbe05/output-50.jsonl
[rag-licence]: https://github.com/jxnl/n-levels-of-rag/blob/2ce1106b0163d11fcf7959187b42f0d3a67fbe05/LICENSE
