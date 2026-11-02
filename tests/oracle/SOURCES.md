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
| `java/sample` | [`samples/java.sample`][java-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `java/guava-preconditions` | [`Preconditions.java`][guava-preconditions] in google/guava | [Apache-2.0][guava-licence] |
| `java/gson-gson` | [`Gson.java`][gson-gson] in google/gson | [Apache-2.0][gson-licence] |
| `java/commons-lang-validate` | [`Validate.java`][commons-lang-validate] in apache/commons-lang | [Apache-2.0][commons-lang-licence] |
| `java/guava-immutable-list` | [`ImmutableList.java`][guava-immutable-list] in google/guava | [Apache-2.0][guava-licence] |
| `java/gson-json-reader` | [`JsonReader.java`][gson-json-reader] in google/gson | [Apache-2.0][gson-licence] |
| `java/commons-lang-range` | [`Range.java`][commons-lang-range] in apache/commons-lang | [Apache-2.0][commons-lang-licence] |
| `java/rewrite-switch-patterns` | [`SwitchPatternMatchingTest.java`][rewrite-switch-patterns] in openrewrite/rewrite | [Apache-2.0][rewrite-licence] |
| `markdown/gson-user-guide` | [`UserGuide.md`][gson-user-guide] in google/gson | [Apache-2.0][gson-licence] |
| `markdown/javapoet-readme` | [`README.md`][javapoet-readme] in square/javapoet | [Apache-2.0][javapoet-licence] |
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
| `c/sample` | [`samples/c.sample`][c-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `c/cjson` | [`cJSON.c`][cjson] in DaveGamble/cJSON | [MIT][cjson-licence] |
| `c/linenoise` | [`linenoise.c`][linenoise] in antirez/linenoise | [BSD-2-Clause][linenoise-licence] |
| `c/libuv-core` | [`src/unix/core.c`][libuv-core] in libuv/libuv | [MIT][libuv-licence] |
| `c/jq-print` | [`src/jv_print.c`][jq-print] in jqlang/jq | [MIT][jq-licence] |
| `c/lua-vm` | [`lvm.c`][lua-vm] in lua/lua | [MIT][lua-licence] |
| `c/zlib-inflate` | [`inflate.c`][zlib-inflate] in madler/zlib | [zlib][zlib-licence] |
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
| `diff/sample` | [`samples/diff.sample`][diff-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `diff/cranelift-stdarch-sve` | [`compiler/rustc_codegen_cranelift/patches/0001-stdarch-Disable-SVE.patch`][cranelift-sve] in rust-lang/rust | [MIT or Apache-2.0][rust-licence] |
| `diff/cranelift-stdlib-atomics` | [`compiler/rustc_codegen_cranelift/patches/0027-stdlib-128bit-atomic-operations.patch`][cranelift-atomics] in rust-lang/rust | [MIT or Apache-2.0][rust-licence] |
| `diff/mir-opt-gvn-cast` | [`tests/mir-opt/const_prop/cast.main.GVN.diff`][mir-cast] in rust-lang/rust | [MIT or Apache-2.0][rust-licence] |
| `diff/cranelift-sysroot-tests` | [`compiler/rustc_codegen_cranelift/patches/0028-sysroot_tests-Disable-long-running-tests.patch`][cranelift-tests] in rust-lang/rust | [MIT or Apache-2.0][rust-licence] |
| `diff/mir-opt-gvn-slices` | [`tests/mir-opt/gvn.slices.GVN.panic-abort.diff`][mir-slices] in rust-lang/rust | [MIT or Apache-2.0][rust-licence] |
| `diff/vscode-foundry-local-sdk` | [`build/npm/patches/foundry-local-sdk.patch`][vscode-foundry] in microsoft/vscode | [MIT][vscode-licence] |
| `markdown/vitest-migration-jest` | [`docs/guide/migration/jest.md`][vitest-jest] in vitest-dev/vitest | [MIT][vitest-licence] |
| `markdown/vite-ssr` | [`docs/guide/ssr.md`][vite-ssr] in vitejs/vite | [MIT][vite-licence] |
| `markdown/prettier-plugins` | [`docs/plugins.md`][prettier-plugins] in prettier/prettier | [MIT][prettier-licence] |
| `markdown/cjson-readme` | [`README.md`][cjson-readme] in DaveGamble/cJSON | [MIT][cjson-licence] |
| `markdown/unity-assertions` | [`docs/UnityAssertionsReference.md`][unity-assertions] in ThrowTheSwitch/Unity | [MIT][unity-licence] |
| `markdown/unity-getting-started` | [`docs/UnityGettingStartedGuide.md`][unity-getting-started] in ThrowTheSwitch/Unity | [MIT][unity-licence] |
| `sql/sample` | [`samples/sql.sample`][sql-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `sql/postgres-select` | [`src/test/regress/sql/select.sql`][pg-select] in postgres/postgres | [PostgreSQL][postgres-licence] |
| `sql/postgres-create-table` | [`src/test/regress/sql/create_table.sql`][pg-create] in postgres/postgres | [PostgreSQL][postgres-licence] |
| `sql/ssms-create-table` | [`samples/features/ssms-templates/Sql/Table/Create Table.sql`][ssms-create] in microsoft/sql-server-samples | [MIT][sqlserver-licence] |
| `sql/postgres-with` | [`src/test/regress/sql/with.sql`][pg-with] in postgres/postgres | [PostgreSQL][postgres-licence] |
| `sql/northwind-pubs` | [`samples/databases/northwind-pubs/instpubs.sql`][pubs] in microsoft/sql-server-samples | [MIT][sqlserver-licence] |
| `markdown/pgvector-readme` | [`README.md`][pgvector-readme] in pgvector/pgvector | [PostgreSQL][pgvector-licence] |
| `markdown/typeorm-indexes` | [`docs/docs/indexes.md`][typeorm-indexes] in typeorm/typeorm | [MIT][typeorm-licence] |
| `markdown/knex-schema-builder` | [`docs/src/guide/schema-builder.md`][knex-schema] in knex/knex | [MIT][knex-licence] |

Each fixture also has one `.colors` file per theme below: Shiki's colour and font style for each token.

| Theme | Source | Licence |
|:--|:--|:--|
| `themes/github-dark` | [primer/github-vscode-theme][github-theme], through `@shikijs/themes` 4.6.0 | [MIT][github-theme-licence] |
| `themes/one-light` | [akamud/vscode-theme-onelight][one-light], through `@shikijs/themes` 4.6.0 | [MIT][one-light-licence] |
| `themes/catppuccin-mocha` | [catppuccin/vscode][catppuccin], through `@shikijs/themes` 4.6.0 | [MIT][catppuccin-licence] |
| `docker/sample` | [`samples/docker.sample`][docker-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `docker/python-slim-bookworm` | [`3.13/slim-bookworm/Dockerfile`][python-docker] in docker-library/python | [MIT][python-docker-licence] |
| `docker/next-with-docker` | [`examples/with-docker/Dockerfile`][next-docker] in vercel/next.js | [MIT][next-licence] |
| `docker/prometheus` | [`Dockerfile`][prometheus-docker] in prometheus/prometheus | [Apache-2.0][prometheus-make-licence] |
| `docker/moby` | [`Dockerfile`][moby-docker] in moby/moby | [Apache-2.0][moby-licence] |
| `docker/postgres-bookworm` | [`18/bookworm/Dockerfile`][postgres-docker] in docker-library/postgres | [MIT][postgres-docker-licence] |
| `docker/buildx` | [`Dockerfile`][buildx-docker] in docker/buildx | [Apache-2.0][buildx-licence] |
| `markdown/fastapi-docker` | [`docs/en/docs/deployment/docker.md`][fastapi-docker] in tiangolo/fastapi | [MIT][fastapi-licence] |
| `markdown/next-with-docker-readme` | [`examples/with-docker/README.md`][next-docker-readme] in vercel/next.js | [MIT][next-licence] |
| `markdown/next-with-docker-export-readme` | [`examples/with-docker-export-output/README.md`][next-docker-export-readme] in vercel/next.js | [MIT][next-licence] |

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
`sql/sample`, `sql/postgres-select`, `sql/postgres-create-table` and `sql/ssms-create-table` are the development inputs for SQL.
`sql/postgres-with` and `sql/northwind-pubs` are held out.
`markdown/pgvector-readme` is the development input for `sql` fences, and `markdown/typeorm-indexes` and `markdown/knex-schema-builder` are held out.
The YAML lexer was written from the YAML 1.2 specification and from Shiki's output alone, as invariant 2 allows for a grammar with no licence, and it was checked against a local set of YAML files and fuzzed against Shiki before these inputs were added.
`yaml/sample`, `yaml/checkout-test` and `yaml/petstore-openapi` are its development inputs, and `markdown/jekyll-front-matter` is the development input for front matter and YAML fences.
`yaml/airflow-compose`, `yaml/ripgrep-ci`, `yaml/prometheus-config` and `markdown/traefik-docker` are held out.
Their first scores, before any fix, were 100% for the three YAML files and 64 of 3,517 bytes different for `traefik-docker`, all of them on the line after a `\` in a shell fence, which the fix for shell fences (nishiki-archive#54) makes match.
`make/sample`, `make/nvm`, `make/cjson` and `make/prometheus-common` are the development inputs for Makefiles.
`make/node`, `make/minikube`, `make/moby`, `make/zlib` and `make/helm` are held out.
`markdown/click-man-readme` and `markdown/httparser-readme` are the development inputs for `makefile` fences, and `markdown/latexrun-readme` and `markdown/device-virtual-readme` are held out.
`diff/sample`, `diff/cranelift-stdarch-sve`, `diff/cranelift-stdlib-atomics` and `diff/mir-opt-gvn-cast` are the development inputs for diff.
`diff/cranelift-sysroot-tests`, `diff/mir-opt-gvn-slices` and `diff/vscode-foundry-local-sdk` are held out.
`markdown/vitest-migration-jest` is the development input for diff fences, and `markdown/vite-ssr` and `markdown/prettier-plugins` are held out.
`docker/sample`, `docker/python-slim-bookworm`, `docker/next-with-docker` and `docker/prometheus` are the development inputs for Dockerfiles.
`docker/moby`, `docker/postgres-bookworm` and `docker/buildx` are held out.
`markdown/fastapi-docker` is the development input for `dockerfile` fences, and `markdown/next-with-docker-readme` and `markdown/next-with-docker-export-readme` are held out.
The lexer matched every input on its first run, held-out ones included.

`c/sample`, `c/cjson` and `c/linenoise` are the development inputs for C, and `markdown/cjson-readme` is the development input for C fences.
`c/libuv-core`, `c/jq-print`, `c/lua-vm`, `c/zlib-inflate`, `markdown/unity-assertions` and `markdown/unity-getting-started` are held out.
Their first scores, before any fix, were 115 of 39,696 bytes different for `libuv-core` and 48 of 9,496 for `jq-print`, from one bug in where a macro's line ends and one in how often the lexer scanned a word, and none for the other two C files or the two Markdown files.

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
[diff-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/diff.sample
[cranelift-sve]: https://github.com/rust-lang/rust/blob/ae36fe22c7e590df40ffd53ac25fea35597c3f3d/compiler/rustc_codegen_cranelift/patches/0001-stdarch-Disable-SVE.patch
[cranelift-atomics]: https://github.com/rust-lang/rust/blob/ae36fe22c7e590df40ffd53ac25fea35597c3f3d/compiler/rustc_codegen_cranelift/patches/0027-stdlib-128bit-atomic-operations.patch
[mir-cast]: https://github.com/rust-lang/rust/blob/ae36fe22c7e590df40ffd53ac25fea35597c3f3d/tests/mir-opt/const_prop/cast.main.GVN.diff
[cranelift-tests]: https://github.com/rust-lang/rust/blob/ae36fe22c7e590df40ffd53ac25fea35597c3f3d/compiler/rustc_codegen_cranelift/patches/0028-sysroot_tests-Disable-long-running-tests.patch
[mir-slices]: https://github.com/rust-lang/rust/blob/ae36fe22c7e590df40ffd53ac25fea35597c3f3d/tests/mir-opt/gvn.slices.GVN.panic-abort.diff
[rust-licence]: https://github.com/rust-lang/rust/blob/ae36fe22c7e590df40ffd53ac25fea35597c3f3d/LICENSE-MIT
[vscode-foundry]: https://github.com/microsoft/vscode/blob/a4e689ae13ef7a1fe9b515656b51c9be7c274294/build/npm/patches/foundry-local-sdk.patch
[vitest-jest]: https://github.com/vitest-dev/vitest/blob/7a3844954d8753fc0d5bed76a96ffd3a21716c22/docs/guide/migration/jest.md
[vitest-licence]: https://github.com/vitest-dev/vitest/blob/7a3844954d8753fc0d5bed76a96ffd3a21716c22/LICENSE
[vite-ssr]: https://github.com/vitejs/vite/blob/b3cb0575aff63004a71301a1ec798aeea2aa05d7/docs/guide/ssr.md
[vite-licence]: https://github.com/vitejs/vite/blob/b3cb0575aff63004a71301a1ec798aeea2aa05d7/LICENSE
[prettier-plugins]: https://github.com/prettier/prettier/blob/a02e92be26ec8963a07aa9500a6c20e24599e95b/docs/plugins.md
[prettier-licence]: https://github.com/prettier/prettier/blob/a02e92be26ec8963a07aa9500a6c20e24599e95b/LICENSE
[docker-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/docker.sample
[python-docker]: https://github.com/docker-library/python/blob/f4c67ece208dead594e43628d22e3c6027dd97ab/3.13/slim-bookworm/Dockerfile
[python-docker-licence]: https://github.com/docker-library/python/blob/f4c67ece208dead594e43628d22e3c6027dd97ab/LICENSE
[next-docker]: https://github.com/vercel/next.js/blob/5a5a4cdc5263ee3ca5a14378de78d2b97dbc4378/examples/with-docker/Dockerfile
[next-licence]: https://github.com/vercel/next.js/blob/5a5a4cdc5263ee3ca5a14378de78d2b97dbc4378/license.md
[prometheus-docker]: https://github.com/prometheus/prometheus/blob/b0c3a30ab2a09b88e64d876017a0134aa2f38f40/Dockerfile
[moby-docker]: https://github.com/moby/moby/blob/aed2b781abc209a639b79d00f6962b629242aa1a/Dockerfile
[postgres-docker]: https://github.com/docker-library/postgres/blob/dfc76bb299b057785026eb2eea39556b6fe52983/18/bookworm/Dockerfile
[postgres-docker-licence]: https://github.com/docker-library/postgres/blob/dfc76bb299b057785026eb2eea39556b6fe52983/LICENSE
[buildx-docker]: https://github.com/docker/buildx/blob/6997c2578c3fb0ac66e883ceaa535aae90c1ee3d/Dockerfile
[buildx-licence]: https://github.com/docker/buildx/blob/6997c2578c3fb0ac66e883ceaa535aae90c1ee3d/LICENSE
[fastapi-docker]: https://github.com/tiangolo/fastapi/blob/5cb6272ed7e3e07eaf6978e1e65b94aca76fa8e0/docs/en/docs/deployment/docker.md
[next-docker-readme]: https://github.com/vercel/next.js/blob/5a5a4cdc5263ee3ca5a14378de78d2b97dbc4378/examples/with-docker/README.md
[next-docker-export-readme]: https://github.com/vercel/next.js/blob/5a5a4cdc5263ee3ca5a14378de78d2b97dbc4378/examples/with-docker-export-output/README.md
[java-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/java.sample
[guava-preconditions]: https://github.com/google/guava/blob/e98ea2340f6acc94ed9e2cca08976ccae68c3412/guava/src/com/google/common/base/Preconditions.java
[guava-immutable-list]: https://github.com/google/guava/blob/e98ea2340f6acc94ed9e2cca08976ccae68c3412/guava/src/com/google/common/collect/ImmutableList.java
[guava-licence]: https://github.com/google/guava/blob/e98ea2340f6acc94ed9e2cca08976ccae68c3412/LICENSE
[gson-gson]: https://github.com/google/gson/blob/57731049da3e415be3d5d34db1ca3b80f3c73810/gson/src/main/java/com/google/gson/Gson.java
[gson-json-reader]: https://github.com/google/gson/blob/57731049da3e415be3d5d34db1ca3b80f3c73810/gson/src/main/java/com/google/gson/stream/JsonReader.java
[gson-user-guide]: https://github.com/google/gson/blob/57731049da3e415be3d5d34db1ca3b80f3c73810/UserGuide.md
[gson-licence]: https://github.com/google/gson/blob/57731049da3e415be3d5d34db1ca3b80f3c73810/LICENSE
[commons-lang-validate]: https://github.com/apache/commons-lang/blob/d8470d57fda340864acf5194bb56305a133fba0e/src/main/java/org/apache/commons/lang3/Validate.java
[commons-lang-range]: https://github.com/apache/commons-lang/blob/d8470d57fda340864acf5194bb56305a133fba0e/src/main/java/org/apache/commons/lang3/Range.java
[commons-lang-licence]: https://github.com/apache/commons-lang/blob/d8470d57fda340864acf5194bb56305a133fba0e/LICENSE.txt
[rewrite-switch-patterns]: https://github.com/openrewrite/rewrite/blob/03d32965a3cdbf06f2717eb3ca706a162039703c/rewrite-java-tck/src/main/java/org/openrewrite/java/tree/SwitchPatternMatchingTest.java
[rewrite-licence]: https://github.com/openrewrite/rewrite/blob/03d32965a3cdbf06f2717eb3ca706a162039703c/LICENSE
[javapoet-readme]: https://github.com/square/javapoet/blob/b9017a9503b76e11b4ad4c1a9f050e2d29112cb0/README.md
[javapoet-licence]: https://github.com/square/javapoet/blob/b9017a9503b76e11b4ad4c1a9f050e2d29112cb0/LICENSE.txt
[c-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/c.sample
[cjson]: https://github.com/DaveGamble/cJSON/blob/6d9f2443ab071f86e5d9b43025a40929ec41c46c/cJSON.c
[cjson-readme]: https://github.com/DaveGamble/cJSON/blob/6d9f2443ab071f86e5d9b43025a40929ec41c46c/README.md
[linenoise]: https://github.com/antirez/linenoise/blob/a473823d74b93eab2ba83480df16ed37617493f2/linenoise.c
[linenoise-licence]: https://github.com/antirez/linenoise/blob/a473823d74b93eab2ba83480df16ed37617493f2/LICENSE
[libuv-core]: https://github.com/libuv/libuv/blob/7228e05f3e407ed3fa59edba920b8cd825b5d83d/src/unix/core.c
[libuv-licence]: https://github.com/libuv/libuv/blob/7228e05f3e407ed3fa59edba920b8cd825b5d83d/LICENSE
[jq-print]: https://github.com/jqlang/jq/blob/5c001b18d7237cce1fdba63608732163f978dd23/src/jv_print.c
[jq-licence]: https://github.com/jqlang/jq/blob/5c001b18d7237cce1fdba63608732163f978dd23/COPYING
[lua-vm]: https://github.com/lua/lua/blob/8cb93bada561977eb370819e39d7d644e5fd0a3f/lvm.c
[lua-licence]: https://github.com/lua/lua/blob/8cb93bada561977eb370819e39d7d644e5fd0a3f/lua.h
[zlib-inflate]: https://github.com/madler/zlib/blob/aaa772ce5c6d0023204c6ee45c59cf70c8372c51/inflate.c
[unity-assertions]: https://github.com/ThrowTheSwitch/Unity/blob/54f864cf6027a50f4cc5424616e9e01e1bb729da/docs/UnityAssertionsReference.md
[unity-getting-started]: https://github.com/ThrowTheSwitch/Unity/blob/54f864cf6027a50f4cc5424616e9e01e1bb729da/docs/UnityGettingStartedGuide.md
[unity-licence]: https://github.com/ThrowTheSwitch/Unity/blob/54f864cf6027a50f4cc5424616e9e01e1bb729da/LICENSE.txt
[sql-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/sql.sample
[pg-select]: https://github.com/postgres/postgres/blob/5681f0cfa5acfdc5146e2b74ff5b7e940df039f3/src/test/regress/sql/select.sql
[pg-create]: https://github.com/postgres/postgres/blob/5681f0cfa5acfdc5146e2b74ff5b7e940df039f3/src/test/regress/sql/create_table.sql
[pg-with]: https://github.com/postgres/postgres/blob/5681f0cfa5acfdc5146e2b74ff5b7e940df039f3/src/test/regress/sql/with.sql
[postgres-licence]: https://github.com/postgres/postgres/blob/5681f0cfa5acfdc5146e2b74ff5b7e940df039f3/COPYRIGHT
[ssms-create]: https://github.com/microsoft/sql-server-samples/blob/3c71696a3942879f0481f3bd65fe112a40023f18/samples/features/ssms-templates/Sql/Table/Create%20Table.sql
[pubs]: https://github.com/microsoft/sql-server-samples/blob/3c71696a3942879f0481f3bd65fe112a40023f18/samples/databases/northwind-pubs/instpubs.sql
[sqlserver-licence]: https://github.com/microsoft/sql-server-samples/blob/3c71696a3942879f0481f3bd65fe112a40023f18/license.txt
[pgvector-readme]: https://github.com/pgvector/pgvector/blob/1d5ef0a160cadec08f754528669ce17e20eab38c/README.md
[pgvector-licence]: https://github.com/pgvector/pgvector/blob/1d5ef0a160cadec08f754528669ce17e20eab38c/LICENSE
[typeorm-indexes]: https://github.com/typeorm/typeorm/blob/5bdc67882dee5a02d975d5ed140c9c3f91661116/docs/docs/indexes.md
[typeorm-licence]: https://github.com/typeorm/typeorm/blob/5bdc67882dee5a02d975d5ed140c9c3f91661116/LICENSE
[knex-schema]: https://github.com/knex/knex/blob/e25d54bcb707714a17f5a5744eba5c4246bb4d1d/docs/src/guide/schema-builder.md
[knex-licence]: https://github.com/knex/knex/blob/e25d54bcb707714a17f5a5744eba5c4246bb4d1d/LICENSE
