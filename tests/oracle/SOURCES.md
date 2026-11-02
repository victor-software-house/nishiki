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
| `markdown/fastapi-readme` | [`README.md`][fastapi-readme] in tiangolo/fastapi | [MIT][fastapi-licence] |
| `markdown/rich-readme` | [`README.md`][rich-readme] in Textualize/rich | [MIT][rich-licence] |
| `markdown/jekyll-front-matter` | [`docs/_docs/front-matter.md`][jekyll-front-matter] in jekyll/jekyll | [MIT][jekyll-licence] |
| `markdown/traefik-docker` | [`docs/content/getting-started/docker.md`][traefik-docker] in traefik/traefik | [MIT][traefik-licence] |
| `toml/sample` | [`samples/toml.sample`][toml-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `toml/cargo-manifest` | [`Cargo.toml`][cargo-manifest] in rust-lang/cargo | [MIT or Apache-2.0][cargo-licence] |
| `toml/tokio-manifest` | [`tokio/Cargo.toml`][tokio-manifest] in tokio-rs/tokio | [MIT][tokio-licence] |
| `toml/burntsushi-example` | [`_example/example.toml`][burntsushi-example] in BurntSushi/toml | [MIT][burntsushi-toml-licence] |
| `toml/ripgrep-manifest` | [`Cargo.toml`][ripgrep-manifest] in BurntSushi/ripgrep | [Unlicense or MIT][ripgrep-licence] |
| `toml/clap-manifest` | [`Cargo.toml`][clap-manifest] in clap-rs/clap | [MIT or Apache-2.0][clap-licence] |
| `toml/poetry-pyproject` | [`pyproject.toml`][poetry-pyproject] in python-poetry/poetry | [MIT][poetry-licence] |
| `toml/ruff-pyproject` | [`pyproject.toml`][ruff-pyproject] in astral-sh/ruff | [MIT][ruff-licence] |
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
| `python/sample` | [`samples/python.sample`][python-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `python/requests-models` | [`src/requests/models.py`][requests-models] in psf/requests | [Apache-2.0][requests-licence] |
| `python/black-strings` | [`src/black/strings.py`][black-strings] in psf/black | [MIT][black-licence] |
| `python/flask-helpers` | [`src/flask/helpers.py`][flask-helpers] in pallets/flask | [BSD-3-Clause][flask-licence] |
| `python/rich-console` | [`rich/console.py`][rich-console] in Textualize/rich | [MIT][rich-licence] |
| `python/fastapi-routing` | [`fastapi/routing.py`][fastapi-routing] in tiangolo/fastapi | [MIT][fastapi-licence] |
| `python/pytest-python` | [`src/_pytest/python.py`][pytest-python] in pytest-dev/pytest | [MIT][pytest-licence] |
| `yaml/sample` | [`samples/yaml.sample`][yaml-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `yaml/checkout-test` | [`.github/workflows/test.yml`][checkout-test] in actions/checkout | [MIT][checkout-licence] |
| `yaml/petstore-openapi` | [`src/main/resources/openapi.yaml`][petstore-openapi] in swagger-api/swagger-petstore | [Apache-2.0][petstore-licence] |
| `yaml/airflow-compose` | [`airflow-core/docs/howto/docker-compose/docker-compose.yaml`][airflow-compose] in apache/airflow | [Apache-2.0][airflow-licence] |
| `yaml/ripgrep-ci` | [`.github/workflows/ci.yml`][ripgrep-ci] in BurntSushi/ripgrep | [Unlicense or MIT][ripgrep-licence] |
| `yaml/prometheus-config` | [`documentation/examples/prometheus.yml`][prometheus-config] in prometheus/prometheus | [Apache-2.0][prometheus-licence] |
| `make/sample` | [`samples/make.sample`][make-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `make/nvm` | [`Makefile`][nvm-make] in nvm-sh/nvm | [MIT][nvm-make-licence] |
| `make/cjson` | [`Makefile`][cjson-make] in DaveGamble/cJSON | [MIT][cjson-licence] |
| `make/prometheus-common` | [`Makefile.common`][prometheus-make] in prometheus/prometheus | [Apache-2.0][prometheus-make-licence] |
| `make/node` | [`Makefile`][node-make] in nodejs/node | [MIT][node-licence] |
| `make/minikube` | [`Makefile`][minikube-make] in kubernetes/minikube | [Apache-2.0][minikube-licence] |
| `make/moby` | [`Makefile`][moby-make] in moby/moby | [Apache-2.0][moby-licence] |
| `make/zlib` | [`Makefile.in`][zlib-make] in madler/zlib | [zlib][zlib-licence] |
| `make/helm` | [`Makefile`][helm-make] in helm/helm | [Apache-2.0][helm-licence] |
| `markdown/click-man-readme` | [`README.md`][click-man-readme] in click-contrib/click-man | [MIT][click-man-licence] |
| `markdown/httparser-readme` | [`README.md`][httparser-readme] in antlabs/httparser | [Apache-2.0][httparser-licence] |
| `markdown/latexrun-readme` | [`README.md`][latexrun-readme] in aclements/latexrun | [MIT][latexrun-licence] |
| `markdown/device-virtual-readme` | [`README.md`][device-virtual-readme] in edgexfoundry/device-virtual-go | [Apache-2.0][device-virtual-licence] |
| `javascript/sample` | [`samples/javascript.sample`][js-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `javascript/express-application` | [`lib/application.js`][express-application] in expressjs/express | [MIT][express-licence] |
| `jsx/sample` | [`samples/jsx.sample`][jsx-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `jsx/preact-keys` | [`test/browser/keys.test.jsx`][preact-keys] in preactjs/preact | [MIT][preact-licence] |
| `tsx/sample` | [`samples/tsx.sample`][tsx-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `tsx/shadcn-sidebar` | [`apps/v4/registry/new-york-v4/ui/sidebar.tsx`][shadcn-sidebar] in shadcn-ui/ui | [MIT][shadcn-licence] |
| `javascript/express-response` | [`lib/response.js`][express-response] in expressjs/express | [MIT][express-licence] |
| `javascript/axios-xhr` | [`lib/adapters/xhr.js`][axios-xhr] in axios/axios | [MIT][axios-licence] |
| `jsx/preact-render` | [`compat/test/browser/render.test.jsx`][preact-render] in preactjs/preact | [MIT][preact-licence] |
| `tsx/shadcn-chart` | [`apps/v4/registry/new-york-v4/ui/chart.tsx`][shadcn-chart] in shadcn-ui/ui | [MIT][shadcn-licence] |
| `tsx/shadcn-combobox` | [`apps/v4/registry/new-york-v4/ui/combobox.tsx`][shadcn-combobox] in shadcn-ui/ui | [MIT][shadcn-licence] |
| `markdown/shadcn-data-table` | [`apps/v4/content/docs/components/radix/data-table.mdx`][shadcn-data-table] in shadcn-ui/ui | [MIT][shadcn-licence] |
| `markdown/shadcn-message-scroller` | [`apps/v4/content/docs/components/radix/message-scroller.mdx`][shadcn-message-scroller] in shadcn-ui/ui | [MIT][shadcn-licence] |
| `markdown/preact-readme` | [`README.md`][preact-readme] in preactjs/preact | [MIT][preact-licence] |

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
`python/sample`, `python/requests-models` and `python/black-strings` are the development inputs for Python.
`python/flask-helpers`, `python/rich-console`, `python/fastapi-routing` and `python/pytest-python` are held out.
`markdown/fastapi-readme` is the development input for Python fences and `markdown/rich-readme` is held out.
`markdown/nvm-readme` is the development input for shell fences and `markdown/pyenv-readme` is held out.
`json/sample`, `json/vue-package`, `jsonc/sample`, `jsonl/sample` and `jsonl/rag-output` are the development inputs for JSON.
`json/zod-package`, `json/vscode-typescript-grammar`, `jsonc/vscode-launch` and `jsonl/rag-output-50` are held out.
`markdown/eslint-no-restricted-imports` is the development input for JSON fences and `markdown/eslint-mcp` is held out.
No input has a `jsonc` or `jsonl` fence, so those two spellings are covered by unit tests only.
The YAML lexer was written from the YAML 1.2 specification and from Shiki's output alone, as invariant 2 allows for a grammar with no licence, and it was checked against a local set of YAML files and fuzzed against Shiki before these inputs were added.
`yaml/sample`, `yaml/checkout-test` and `yaml/petstore-openapi` are its development inputs, and `markdown/jekyll-front-matter` is the development input for front matter and YAML fences.
`yaml/airflow-compose`, `yaml/ripgrep-ci`, `yaml/prometheus-config` and `markdown/traefik-docker` are held out.
Their first scores, before any fix, were 100% for the three YAML files and 64 of 3,517 bytes different for `traefik-docker`, all of them on the line after a `\` in a shell fence, which the fix for shell fences (nishiki-archive#54) makes match.
`make/sample`, `make/nvm`, `make/cjson` and `make/prometheus-common` are the development inputs for Makefiles.
`make/node`, `make/minikube`, `make/moby`, `make/zlib` and `make/helm` are held out.
`markdown/click-man-readme` and `markdown/httparser-readme` are the development inputs for `makefile` fences, and `markdown/latexrun-readme` and `markdown/device-virtual-readme` are held out.

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
[python-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/python.sample
[requests-models]: https://github.com/psf/requests/blob/3c7baaa05ab4aa49cc819ce3a819d260de19c157/src/requests/models.py
[requests-licence]: https://github.com/psf/requests/blob/3c7baaa05ab4aa49cc819ce3a819d260de19c157/LICENSE
[black-strings]: https://github.com/psf/black/blob/945d902bef347d2e7e1d26897aa89a9c177fb535/src/black/strings.py
[black-licence]: https://github.com/psf/black/blob/945d902bef347d2e7e1d26897aa89a9c177fb535/LICENSE
[flask-helpers]: https://github.com/pallets/flask/blob/49943b5a94413a530761350f04e896f76e8d620e/src/flask/helpers.py
[flask-licence]: https://github.com/pallets/flask/blob/49943b5a94413a530761350f04e896f76e8d620e/LICENSE.txt
[rich-console]: https://github.com/Textualize/rich/blob/9d8f9a372cc5916fd4781fec207ced7ddac2f08f/rich/console.py
[rich-readme]: https://github.com/Textualize/rich/blob/9d8f9a372cc5916fd4781fec207ced7ddac2f08f/README.md
[rich-licence]: https://github.com/Textualize/rich/blob/9d8f9a372cc5916fd4781fec207ced7ddac2f08f/LICENSE
[fastapi-routing]: https://github.com/tiangolo/fastapi/blob/5cb6272ed7e3e07eaf6978e1e65b94aca76fa8e0/fastapi/routing.py
[fastapi-readme]: https://github.com/tiangolo/fastapi/blob/5cb6272ed7e3e07eaf6978e1e65b94aca76fa8e0/README.md
[fastapi-licence]: https://github.com/tiangolo/fastapi/blob/5cb6272ed7e3e07eaf6978e1e65b94aca76fa8e0/LICENSE
[pytest-python]: https://github.com/pytest-dev/pytest/blob/1211245aa72f1efe07bcfce63a7c543156cee976/src/_pytest/python.py
[pytest-licence]: https://github.com/pytest-dev/pytest/blob/1211245aa72f1efe07bcfce63a7c543156cee976/LICENSE
[yaml-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/yaml.sample
[checkout-test]: https://github.com/actions/checkout/blob/f548e57e544e1ff5a4c46bf1e1b8685f8e4a348a/.github/workflows/test.yml
[checkout-licence]: https://github.com/actions/checkout/blob/f548e57e544e1ff5a4c46bf1e1b8685f8e4a348a/LICENSE
[petstore-openapi]: https://github.com/swagger-api/swagger-petstore/blob/5cb909fc7780bc708151bce0014939acba8b212a/src/main/resources/openapi.yaml
[petstore-licence]: https://github.com/swagger-api/swagger-petstore/blob/5cb909fc7780bc708151bce0014939acba8b212a/LICENSE
[airflow-compose]: https://github.com/apache/airflow/blob/cfbd70b66d11f5d464eb9c0ba926dd34304e110d/airflow-core/docs/howto/docker-compose/docker-compose.yaml
[airflow-licence]: https://github.com/apache/airflow/blob/cfbd70b66d11f5d464eb9c0ba926dd34304e110d/LICENSE
[ripgrep-ci]: https://github.com/BurntSushi/ripgrep/blob/91f5d22ef48bde459d90a8c7dbeab416e97d0f2b/.github/workflows/ci.yml
[prometheus-config]: https://github.com/prometheus/prometheus/blob/b0c3a30ab2a09b88e64d876017a0134aa2f38f40/documentation/examples/prometheus.yml
[prometheus-licence]: https://github.com/prometheus/prometheus/blob/b0c3a30ab2a09b88e64d876017a0134aa2f38f40/LICENSE
[jekyll-front-matter]: https://github.com/jekyll/jekyll/blob/97c3bdc822df9d5bea4c269b29a28de5a5e29d70/docs/_docs/front-matter.md
[jekyll-licence]: https://github.com/jekyll/jekyll/blob/97c3bdc822df9d5bea4c269b29a28de5a5e29d70/LICENSE
[traefik-docker]: https://github.com/traefik/traefik/blob/38d53dd06c53c7e9bd08e57b217e72686d823610/docs/content/getting-started/docker.md
[traefik-licence]: https://github.com/traefik/traefik/blob/38d53dd06c53c7e9bd08e57b217e72686d823610/LICENSE.md
[toml-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/toml.sample
[cargo-manifest]: https://github.com/rust-lang/cargo/blob/6238f6fd61bffc158aabeb77b779fc16d6e7b5fc/Cargo.toml
[tokio-manifest]: https://github.com/tokio-rs/tokio/blob/b499257ef62bd631b83a988d371f4330f545670c/tokio/Cargo.toml
[burntsushi-example]: https://github.com/BurntSushi/toml/blob/d733fc535e4a9f3c454e421a87b90ade5d49bf31/_example/example.toml
[burntsushi-toml-licence]: https://github.com/BurntSushi/toml/blob/d733fc535e4a9f3c454e421a87b90ade5d49bf31/COPYING
[ripgrep-manifest]: https://github.com/BurntSushi/ripgrep/blob/91f5d22ef48bde459d90a8c7dbeab416e97d0f2b/Cargo.toml
[clap-manifest]: https://github.com/clap-rs/clap/blob/db30b39a55c188763666268447050a0dacba3bf0/Cargo.toml
[poetry-pyproject]: https://github.com/python-poetry/poetry/blob/6ecadf5b71309b1da4f00949cc349a79884cff8f/pyproject.toml
[poetry-licence]: https://github.com/python-poetry/poetry/blob/6ecadf5b71309b1da4f00949cc349a79884cff8f/LICENSE
[ruff-pyproject]: https://github.com/astral-sh/ruff/blob/f10755306e3e932fd127960acb2c892fafb87410/pyproject.toml
[ruff-licence]: https://github.com/astral-sh/ruff/blob/f10755306e3e932fd127960acb2c892fafb87410/LICENSE
[make-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/make.sample
[nvm-make]: https://github.com/nvm-sh/nvm/blob/16df5aa69aeda3814703e14f9d35a4f60220dd37/Makefile
[nvm-make-licence]: https://github.com/nvm-sh/nvm/blob/16df5aa69aeda3814703e14f9d35a4f60220dd37/LICENSE.md
[cjson-make]: https://github.com/DaveGamble/cJSON/blob/6d9f2443ab071f86e5d9b43025a40929ec41c46c/Makefile
[cjson-licence]: https://github.com/DaveGamble/cJSON/blob/6d9f2443ab071f86e5d9b43025a40929ec41c46c/LICENSE
[prometheus-make]: https://github.com/prometheus/prometheus/blob/b0c3a30ab2a09b88e64d876017a0134aa2f38f40/Makefile.common
[prometheus-make-licence]: https://github.com/prometheus/prometheus/blob/b0c3a30ab2a09b88e64d876017a0134aa2f38f40/LICENSE
[node-make]: https://github.com/nodejs/node/blob/3556d018a4863a42fba62a927e7e605c1e8cf128/Makefile
[node-licence]: https://github.com/nodejs/node/blob/3556d018a4863a42fba62a927e7e605c1e8cf128/LICENSE
[minikube-make]: https://github.com/kubernetes/minikube/blob/1f8cacf5b0bdb6ecdcf1abf16da4f3d596ba7996/Makefile
[minikube-licence]: https://github.com/kubernetes/minikube/blob/1f8cacf5b0bdb6ecdcf1abf16da4f3d596ba7996/LICENSE
[moby-make]: https://github.com/moby/moby/blob/aed2b781abc209a639b79d00f6962b629242aa1a/Makefile
[moby-licence]: https://github.com/moby/moby/blob/aed2b781abc209a639b79d00f6962b629242aa1a/LICENSE
[zlib-make]: https://github.com/madler/zlib/blob/aaa772ce5c6d0023204c6ee45c59cf70c8372c51/Makefile.in
[zlib-licence]: https://github.com/madler/zlib/blob/aaa772ce5c6d0023204c6ee45c59cf70c8372c51/LICENSE
[helm-make]: https://github.com/helm/helm/blob/bc232c5312e784e808ea95ae76f9e82d4fe142fc/Makefile
[helm-licence]: https://github.com/helm/helm/blob/bc232c5312e784e808ea95ae76f9e82d4fe142fc/LICENSE
[click-man-readme]: https://github.com/click-contrib/click-man/blob/be9eac7642e32a43a93ec2c1f5deea7047bb7288/README.md
[click-man-licence]: https://github.com/click-contrib/click-man/blob/be9eac7642e32a43a93ec2c1f5deea7047bb7288/LICENSE
[httparser-readme]: https://github.com/antlabs/httparser/blob/622df4409cddf119cf437633f620947235d2d1eb/README.md
[httparser-licence]: https://github.com/antlabs/httparser/blob/622df4409cddf119cf437633f620947235d2d1eb/LICENSE
[latexrun-readme]: https://github.com/aclements/latexrun/blob/38ff6ec2815654513c91f64bdf2a5760c85da26e/README.md
[latexrun-licence]: https://github.com/aclements/latexrun/blob/38ff6ec2815654513c91f64bdf2a5760c85da26e/LICENSE
[device-virtual-readme]: https://github.com/edgexfoundry/device-virtual-go/blob/51a4860f7a496206684d7bc404f1c64d674ea9c4/README.md
[device-virtual-licence]: https://github.com/edgexfoundry/device-virtual-go/blob/51a4860f7a496206684d7bc404f1c64d674ea9c4/LICENSE
[js-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/javascript.sample
[express-application]: https://github.com/expressjs/express/blob/bcc0551cb7a785b7772f77cb59ce55a2bc94975a/lib/application.js
[express-licence]: https://github.com/expressjs/express/blob/bcc0551cb7a785b7772f77cb59ce55a2bc94975a/LICENSE
[jsx-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/jsx.sample
[preact-keys]: https://github.com/preactjs/preact/blob/81dbe69d39e948b2305193d2622c225699311083/test/browser/keys.test.jsx
[preact-licence]: https://github.com/preactjs/preact/blob/81dbe69d39e948b2305193d2622c225699311083/LICENSE
[tsx-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/tsx.sample
[shadcn-sidebar]: https://github.com/shadcn-ui/ui/blob/4aaea19d9cff3e0908e12a8c1d504b8c5ec34610/apps/v4/registry/new-york-v4/ui/sidebar.tsx
[shadcn-licence]: https://github.com/shadcn-ui/ui/blob/4aaea19d9cff3e0908e12a8c1d504b8c5ec34610/LICENSE.md
[express-response]: https://github.com/expressjs/express/blob/bcc0551cb7a785b7772f77cb59ce55a2bc94975a/lib/response.js
[axios-xhr]: https://github.com/axios/axios/blob/7324e94d8a7b5156a4baf3c86c8b5daa242daee3/lib/adapters/xhr.js
[axios-licence]: https://github.com/axios/axios/blob/7324e94d8a7b5156a4baf3c86c8b5daa242daee3/LICENSE
[preact-render]: https://github.com/preactjs/preact/blob/81dbe69d39e948b2305193d2622c225699311083/compat/test/browser/render.test.jsx
[shadcn-chart]: https://github.com/shadcn-ui/ui/blob/4aaea19d9cff3e0908e12a8c1d504b8c5ec34610/apps/v4/registry/new-york-v4/ui/chart.tsx
[shadcn-combobox]: https://github.com/shadcn-ui/ui/blob/4aaea19d9cff3e0908e12a8c1d504b8c5ec34610/apps/v4/registry/new-york-v4/ui/combobox.tsx
[shadcn-data-table]: https://github.com/shadcn-ui/ui/blob/4aaea19d9cff3e0908e12a8c1d504b8c5ec34610/apps/v4/content/docs/components/radix/data-table.mdx
[shadcn-message-scroller]: https://github.com/shadcn-ui/ui/blob/4aaea19d9cff3e0908e12a8c1d504b8c5ec34610/apps/v4/content/docs/components/radix/message-scroller.mdx
[preact-readme]: https://github.com/preactjs/preact/blob/81dbe69d39e948b2305193d2622c225699311083/README.md
