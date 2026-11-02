// The oracle's inputs, each pinned to a commit. tests/oracle/SOURCES.md
// records each one's licence; keep the two lists in step.

/** One source file the oracle tokenizes. */
export type Input = {
  /** Shiki's language id. */
  lang: string;
  /** The fixture's file name, without extension. */
  name: string;
  url: string;
};

const samples =
  "https://raw.githubusercontent.com/shikijs/textmate-grammars-themes/8f1828ebccc57930cf0adde9b35ae929584fe191/samples";
const ripgrep =
  "https://raw.githubusercontent.com/BurntSushi/ripgrep/91f5d22ef48bde459d90a8c7dbeab416e97d0f2b";
const serde =
  "https://raw.githubusercontent.com/serde-rs/serde/0db9b3b504bcb0a7fa9cf001429869519afd3e9a";
const vue =
  "https://raw.githubusercontent.com/vuejs/core/68f25a4a1957710de71731702bd42e05535f494b";
const zod =
  "https://raw.githubusercontent.com/colinhacks/zod/2554b70c3d4a80273381e61d527d820720626440";
const ky =
  "https://raw.githubusercontent.com/sindresorhus/ky/3541888878c4d303a8bc2ad61815f4e048f55a5d";

const tokio =
  "https://raw.githubusercontent.com/tokio-rs/tokio/b499257ef62bd631b83a988d371f4330f545670c";
const book =
  "https://raw.githubusercontent.com/rust-lang/book/2771df165f092a4fbb9f46b5b2e7450f19010adb";
const clap =
  "https://raw.githubusercontent.com/clap-rs/clap/db30b39a55c188763666268447050a0dacba3bf0";

const cargo =
  "https://raw.githubusercontent.com/rust-lang/cargo/6238f6fd61bffc158aabeb77b779fc16d6e7b5fc";

const nvm =
  "https://raw.githubusercontent.com/nvm-sh/nvm/5e38bc0caed342356ffd26eb21e0e9b995ac228b";

const rubyBuild =
  "https://raw.githubusercontent.com/rbenv/ruby-build/4ef3159f94dd66c8325efb42db2faeb7d5732db0";

const pyenv =
  "https://raw.githubusercontent.com/pyenv/pyenv/e6669f0a84df60daba8672c3b3f0caebd0fdcef8";

const vscode =
  "https://raw.githubusercontent.com/microsoft/vscode/a4e689ae13ef7a1fe9b515656b51c9be7c274294";

const eslint =
  "https://raw.githubusercontent.com/eslint/eslint/11c66ba08381cb7cff7679d662c9f6654b6fe712";

const rag =
  "https://raw.githubusercontent.com/jxnl/n-levels-of-rag/2ce1106b0163d11fcf7959187b42f0d3a67fbe05";

const guava =
  "https://raw.githubusercontent.com/google/guava/e98ea2340f6acc94ed9e2cca08976ccae68c3412";
const gson =
  "https://raw.githubusercontent.com/google/gson/57731049da3e415be3d5d34db1ca3b80f3c73810";
const commonsLang =
  "https://raw.githubusercontent.com/apache/commons-lang/d8470d57fda340864acf5194bb56305a133fba0e";
const rewrite =
  "https://raw.githubusercontent.com/openrewrite/rewrite/03d32965a3cdbf06f2717eb3ca706a162039703c";
const javapoet =
  "https://raw.githubusercontent.com/square/javapoet/b9017a9503b76e11b4ad4c1a9f050e2d29112cb0";
const requests =
  "https://raw.githubusercontent.com/psf/requests/3c7baaa05ab4aa49cc819ce3a819d260de19c157";
const black =
  "https://raw.githubusercontent.com/psf/black/945d902bef347d2e7e1d26897aa89a9c177fb535";
const flask =
  "https://raw.githubusercontent.com/pallets/flask/49943b5a94413a530761350f04e896f76e8d620e";
const rich =
  "https://raw.githubusercontent.com/Textualize/rich/9d8f9a372cc5916fd4781fec207ced7ddac2f08f";
const fastapi =
  "https://raw.githubusercontent.com/tiangolo/fastapi/5cb6272ed7e3e07eaf6978e1e65b94aca76fa8e0";
const pytest =
  "https://raw.githubusercontent.com/pytest-dev/pytest/1211245aa72f1efe07bcfce63a7c543156cee976";

const checkout =
  "https://raw.githubusercontent.com/actions/checkout/f548e57e544e1ff5a4c46bf1e1b8685f8e4a348a";

const petstore =
  "https://raw.githubusercontent.com/swagger-api/swagger-petstore/5cb909fc7780bc708151bce0014939acba8b212a";

const airflow =
  "https://raw.githubusercontent.com/apache/airflow/cfbd70b66d11f5d464eb9c0ba926dd34304e110d";
const nvmMake =
  "https://raw.githubusercontent.com/nvm-sh/nvm/16df5aa69aeda3814703e14f9d35a4f60220dd37";

const cjson =
  "https://raw.githubusercontent.com/DaveGamble/cJSON/6d9f2443ab071f86e5d9b43025a40929ec41c46c";

const prometheus =
  "https://raw.githubusercontent.com/prometheus/prometheus/b0c3a30ab2a09b88e64d876017a0134aa2f38f40";

const traefik =
  "https://raw.githubusercontent.com/traefik/traefik/38d53dd06c53c7e9bd08e57b217e72686d823610";

const jekyll =
  "https://raw.githubusercontent.com/jekyll/jekyll/97c3bdc822df9d5bea4c269b29a28de5a5e29d70";
const nodejs =
  "https://raw.githubusercontent.com/nodejs/node/3556d018a4863a42fba62a927e7e605c1e8cf128";

const minikube =
  "https://raw.githubusercontent.com/kubernetes/minikube/1f8cacf5b0bdb6ecdcf1abf16da4f3d596ba7996";

const moby =
  "https://raw.githubusercontent.com/moby/moby/aed2b781abc209a639b79d00f6962b629242aa1a";

const zlib =
  "https://raw.githubusercontent.com/madler/zlib/aaa772ce5c6d0023204c6ee45c59cf70c8372c51";

const helm =
  "https://raw.githubusercontent.com/helm/helm/bc232c5312e784e808ea95ae76f9e82d4fe142fc";

const clickMan =
  "https://raw.githubusercontent.com/click-contrib/click-man/be9eac7642e32a43a93ec2c1f5deea7047bb7288";

const httparser =
  "https://raw.githubusercontent.com/antlabs/httparser/622df4409cddf119cf437633f620947235d2d1eb";

const latexrun =
  "https://raw.githubusercontent.com/aclements/latexrun/38ff6ec2815654513c91f64bdf2a5760c85da26e";

const deviceVirtual =
  "https://raw.githubusercontent.com/edgexfoundry/device-virtual-go/51a4860f7a496206684d7bc404f1c64d674ea9c4";

const express =
  "https://raw.githubusercontent.com/expressjs/express/bcc0551cb7a785b7772f77cb59ce55a2bc94975a";

const preact =
  "https://raw.githubusercontent.com/preactjs/preact/81dbe69d39e948b2305193d2622c225699311083";

const shadcn =
  "https://raw.githubusercontent.com/shadcn-ui/ui/4aaea19d9cff3e0908e12a8c1d504b8c5ec34610";

const axios =
  "https://raw.githubusercontent.com/axios/axios/7324e94d8a7b5156a4baf3c86c8b5daa242daee3";

const burntsushiToml =
  "https://raw.githubusercontent.com/BurntSushi/toml/d733fc535e4a9f3c454e421a87b90ade5d49bf31";
const poetry =
  "https://raw.githubusercontent.com/python-poetry/poetry/6ecadf5b71309b1da4f00949cc349a79884cff8f";
const ruff =
  "https://raw.githubusercontent.com/astral-sh/ruff/f10755306e3e932fd127960acb2c892fafb87410";
const vitest =
  "https://raw.githubusercontent.com/vitest-dev/vitest/7a3844954d8753fc0d5bed76a96ffd3a21716c22";

const vite =
  "https://raw.githubusercontent.com/vitejs/vite/b3cb0575aff63004a71301a1ec798aeea2aa05d7";

const prettier =
  "https://raw.githubusercontent.com/prettier/prettier/a02e92be26ec8963a07aa9500a6c20e24599e95b";

const rustc =
  "https://raw.githubusercontent.com/rust-lang/rust/ae36fe22c7e590df40ffd53ac25fea35597c3f3d";

const dockerPython =
  "https://raw.githubusercontent.com/docker-library/python/f4c67ece208dead594e43628d22e3c6027dd97ab";
const dockerPostgres =
  "https://raw.githubusercontent.com/docker-library/postgres/dfc76bb299b057785026eb2eea39556b6fe52983";
const nextjs =
  "https://raw.githubusercontent.com/vercel/next.js/5a5a4cdc5263ee3ca5a14378de78d2b97dbc4378";
const buildx =
  "https://raw.githubusercontent.com/docker/buildx/6997c2578c3fb0ac66e883ceaa535aae90c1ee3d";
const linenoise =
  "https://raw.githubusercontent.com/antirez/linenoise/a473823d74b93eab2ba83480df16ed37617493f2";
const libuv =
  "https://raw.githubusercontent.com/libuv/libuv/7228e05f3e407ed3fa59edba920b8cd825b5d83d";
const jq =
  "https://raw.githubusercontent.com/jqlang/jq/5c001b18d7237cce1fdba63608732163f978dd23";
const lua =
  "https://raw.githubusercontent.com/lua/lua/8cb93bada561977eb370819e39d7d644e5fd0a3f";
const unity =
  "https://raw.githubusercontent.com/ThrowTheSwitch/Unity/54f864cf6027a50f4cc5424616e9e01e1bb729da";

const postgres =
  "https://raw.githubusercontent.com/postgres/postgres/5681f0cfa5acfdc5146e2b74ff5b7e940df039f3";

const sqlServerSamples =
  "https://raw.githubusercontent.com/microsoft/sql-server-samples/3c71696a3942879f0481f3bd65fe112a40023f18";

const pgvector =
  "https://raw.githubusercontent.com/pgvector/pgvector/1d5ef0a160cadec08f754528669ce17e20eab38c";

const typeorm =
  "https://raw.githubusercontent.com/typeorm/typeorm/5bdc67882dee5a02d975d5ed140c9c3f91661116";

const knex =
  "https://raw.githubusercontent.com/knex/knex/e25d54bcb707714a17f5a5744eba5c4246bb4d1d";
const django =
  "https://raw.githubusercontent.com/django/django/b501b8493f31cf8cba31211d59b3b78642b8cb02";
const sqlalchemy =
  "https://raw.githubusercontent.com/sqlalchemy/sqlalchemy/679372836fe7343683248467d276036a6d6f1673";
const kafka =
  "https://raw.githubusercontent.com/apache/kafka/da5316228ee604724dbcede405ebbf7f18f1dbf3";
const tomcat =
  "https://raw.githubusercontent.com/apache/tomcat/1b2785c63ca0e0f9a242745b16f26924b5488ddd";

const graphqlJs =
  "https://raw.githubusercontent.com/graphql/graphql-js/eb0bd0801ea5f9f7680e91ae575ed74397f051cb";
const graphiql =
  "https://raw.githubusercontent.com/graphql/graphiql/e7a5e07b6c2cb9b524831544c689e742cdc1b904";
const codegen =
  "https://raw.githubusercontent.com/dotansimha/graphql-code-generator/11ac951a30b79369d3b0aa78f83a79d309ffcc77";
const normalize =
  "https://raw.githubusercontent.com/necolas/normalize.css/fc091cce1534909334c1911709a39c22d406977b";
const tailwind =
  "https://raw.githubusercontent.com/tailwindlabs/tailwindcss/2626ae6ceddf36069ab6d55c53a5857c1e2e360a";
const highlightjs =
  "https://raw.githubusercontent.com/highlightjs/highlight.js/60e092057673595fa542db0bc2b13a99df5d0551";
const githubMarkdownCss =
  "https://raw.githubusercontent.com/sindresorhus/github-markdown-css/7f38dcb9054bc5d860fac65291f5c98fef47932c";
const milligram =
  "https://raw.githubusercontent.com/milligram/milligram/5e56af462aea08baf5011a045bf1bfaaef72552c";

export const inputs: Input[] = [
  { lang: "rust", name: "sample", url: `${samples}/rust.sample` },
  {
    lang: "rust",
    name: "ripgrep-searcher",
    url: `${ripgrep}/crates/searcher/src/searcher/mod.rs`,
  },
  { lang: "rust", name: "serde-de", url: `${serde}/serde_core/src/de/mod.rs` },
  { lang: "markdown", name: "sample", url: `${samples}/markdown.sample` },
  { lang: "markdown", name: "tokio-readme", url: `${tokio}/README.md` },
  { lang: "markdown", name: "serde-readme", url: `${serde}/README.md` },
  {
    lang: "markdown",
    name: "book-ownership",
    url: `${book}/src/ch04-01-what-is-ownership.md`,
  },
  {
    lang: "markdown",
    name: "book-strings",
    url: `${book}/src/ch08-02-strings.md`,
  },
  { lang: "markdown", name: "clap-readme", url: `${clap}/README.md` },
  {
    lang: "markdown",
    name: "zod-readme",
    url: `${zod}/packages/zod/README.md`,
  },
  { lang: "markdown", name: "ky-readme", url: `${ky}/readme.md` },
  {
    lang: "markdown",
    name: "zod-api",
    url: `${zod}/packages/docs/content/api.mdx`,
  },
  { lang: "markdown", name: "nvm-readme", url: `${nvm}/README.md` },
  { lang: "markdown", name: "pyenv-readme", url: `${pyenv}/README.md` },
  {
    lang: "markdown",
    name: "cargo-release",
    url: `${cargo}/doc/contrib/src/process/release.md`,
  },
  {
    lang: "markdown",
    name: "eslint-no-restricted-imports",
    url: `${eslint}/docs/src/rules/no-restricted-imports.md`,
  },
  { lang: "markdown", name: "eslint-mcp", url: `${eslint}/docs/src/use/mcp.md` },
  {
    lang: "markdown",
    name: "traefik-docker",
    url: `${traefik}/docs/content/getting-started/docker.md`,
  },
  {
    lang: "markdown",
    name: "jekyll-front-matter",
    url: `${jekyll}/docs/_docs/front-matter.md`,
  },
  { lang: "toml", name: "sample", url: `${samples}/toml.sample` },
  { lang: "toml", name: "cargo-manifest", url: `${cargo}/Cargo.toml` },
  { lang: "toml", name: "tokio-manifest", url: `${tokio}/tokio/Cargo.toml` },
  { lang: "toml", name: "burntsushi-example", url: `${burntsushiToml}/_example/example.toml` },
  { lang: "toml", name: "ripgrep-manifest", url: `${ripgrep}/Cargo.toml` },
  { lang: "toml", name: "clap-manifest", url: `${clap}/Cargo.toml` },
  { lang: "toml", name: "poetry-pyproject", url: `${poetry}/pyproject.toml` },
  { lang: "toml", name: "ruff-pyproject", url: `${ruff}/pyproject.toml` },
  { lang: "java", name: "sample", url: `${samples}/java.sample` },
  {
    lang: "java",
    name: "guava-preconditions",
    url: `${guava}/guava/src/com/google/common/base/Preconditions.java`,
  },
  {
    lang: "java",
    name: "gson-gson",
    url: `${gson}/gson/src/main/java/com/google/gson/Gson.java`,
  },
  {
    lang: "java",
    name: "commons-lang-validate",
    url: `${commonsLang}/src/main/java/org/apache/commons/lang3/Validate.java`,
  },
  {
    lang: "java",
    name: "guava-immutable-list",
    url: `${guava}/guava/src/com/google/common/collect/ImmutableList.java`,
  },
  {
    lang: "java",
    name: "gson-json-reader",
    url: `${gson}/gson/src/main/java/com/google/gson/stream/JsonReader.java`,
  },
  {
    lang: "java",
    name: "commons-lang-range",
    url: `${commonsLang}/src/main/java/org/apache/commons/lang3/Range.java`,
  },
  {
    lang: "java",
    name: "rewrite-switch-patterns",
    url: `${rewrite}/rewrite-java-tck/src/main/java/org/openrewrite/java/tree/SwitchPatternMatchingTest.java`,
  },
  { lang: "markdown", name: "gson-user-guide", url: `${gson}/UserGuide.md` },
  { lang: "markdown", name: "javapoet-readme", url: `${javapoet}/README.md` },
  { lang: "typescript", name: "sample", url: `${samples}/typescript.sample` },
  {
    lang: "typescript",
    name: "vue-effect",
    url: `${vue}/packages/reactivity/src/effect.ts`,
  },
  {
    lang: "typescript",
    name: "vue-api-create-app",
    url: `${vue}/packages/runtime-core/src/apiCreateApp.ts`,
  },
  {
    lang: "typescript",
    name: "zod-classic-schemas",
    url: `${zod}/packages/zod/src/v4/classic/schemas.ts`,
  },
  { lang: "typescript", name: "ky-core", url: `${ky}/source/core/Ky.ts` },
  { lang: "shellscript", name: "sample", url: `${samples}/shellscript.sample` },
  { lang: "shellscript", name: "nvm", url: `${nvm}/nvm.sh` },
  { lang: "shellscript", name: "ruby-build", url: `${rubyBuild}/bin/ruby-build` },
  { lang: "shellscript", name: "pyenv-init", url: `${pyenv}/libexec/pyenv-init` },
  { lang: "json", name: "sample", url: `${samples}/json.sample` },
  { lang: "json", name: "vue-package", url: `${vue}/package.json` },
  { lang: "json", name: "zod-package", url: `${zod}/packages/zod/package.json` },
  {
    lang: "json",
    name: "vscode-typescript-grammar",
    url: `${vscode}/extensions/typescript-basics/syntaxes/TypeScript.tmLanguage.json`,
  },
  { lang: "jsonc", name: "sample", url: `${samples}/jsonc.sample` },
  { lang: "jsonc", name: "vscode-launch", url: `${vscode}/.vscode/launch.json` },
  { lang: "jsonl", name: "sample", url: `${samples}/jsonl.sample` },
  { lang: "jsonl", name: "rag-output", url: `${rag}/output.jsonl` },
  { lang: "jsonl", name: "rag-output-50", url: `${rag}/output-50.jsonl` },
  { lang: "python", name: "sample", url: `${samples}/python.sample` },
  { lang: "python", name: "requests-models", url: `${requests}/src/requests/models.py` },
  { lang: "python", name: "black-strings", url: `${black}/src/black/strings.py` },
  { lang: "python", name: "flask-helpers", url: `${flask}/src/flask/helpers.py` },
  { lang: "python", name: "rich-console", url: `${rich}/rich/console.py` },
  { lang: "python", name: "fastapi-routing", url: `${fastapi}/fastapi/routing.py` },
  { lang: "python", name: "pytest-python", url: `${pytest}/src/_pytest/python.py` },
  { lang: "markdown", name: "fastapi-readme", url: `${fastapi}/README.md` },
  { lang: "markdown", name: "rich-readme", url: `${rich}/README.md` },
  { lang: "yaml", name: "sample", url: `${samples}/yaml.sample` },
  {
    lang: "yaml",
    name: "checkout-test",
    url: `${checkout}/.github/workflows/test.yml`,
  },
  {
    lang: "yaml",
    name: "petstore-openapi",
    url: `${petstore}/src/main/resources/openapi.yaml`,
  },
  {
    lang: "yaml",
    name: "airflow-compose",
    url: `${airflow}/airflow-core/docs/howto/docker-compose/docker-compose.yaml`,
  },
  {
    lang: "yaml",
    name: "ripgrep-ci",
    url: `${ripgrep}/.github/workflows/ci.yml`,
  },
  {
    lang: "yaml",
    name: "prometheus-config",
    url: `${prometheus}/documentation/examples/prometheus.yml`,
  },
  { lang: "make", name: "sample", url: `${samples}/make.sample` },
  { lang: "make", name: "nvm", url: `${nvmMake}/Makefile` },
  { lang: "make", name: "cjson", url: `${cjson}/Makefile` },
  { lang: "make", name: "prometheus-common", url: `${prometheus}/Makefile.common` },
  { lang: "make", name: "node", url: `${nodejs}/Makefile` },
  { lang: "make", name: "minikube", url: `${minikube}/Makefile` },
  { lang: "make", name: "moby", url: `${moby}/Makefile` },
  { lang: "make", name: "zlib", url: `${zlib}/Makefile.in` },
  { lang: "make", name: "helm", url: `${helm}/Makefile` },
  { lang: "markdown", name: "click-man-readme", url: `${clickMan}/README.md` },
  { lang: "markdown", name: "httparser-readme", url: `${httparser}/README.md` },
  { lang: "markdown", name: "latexrun-readme", url: `${latexrun}/README.md` },
  { lang: "markdown", name: "device-virtual-readme", url: `${deviceVirtual}/README.md` },
  { lang: "javascript", name: "sample", url: `${samples}/javascript.sample` },
  { lang: "javascript", name: "express-application", url: `${express}/lib/application.js` },
  { lang: "jsx", name: "sample", url: `${samples}/jsx.sample` },
  { lang: "jsx", name: "preact-keys", url: `${preact}/test/browser/keys.test.jsx` },
  { lang: "tsx", name: "sample", url: `${samples}/tsx.sample` },
  { lang: "tsx", name: "shadcn-sidebar", url: `${shadcn}/apps/v4/registry/new-york-v4/ui/sidebar.tsx` },
  { lang: "javascript", name: "express-response", url: `${express}/lib/response.js` },
  { lang: "javascript", name: "axios-xhr", url: `${axios}/lib/adapters/xhr.js` },
  { lang: "jsx", name: "preact-render", url: `${preact}/compat/test/browser/render.test.jsx` },
  { lang: "tsx", name: "shadcn-chart", url: `${shadcn}/apps/v4/registry/new-york-v4/ui/chart.tsx` },
  { lang: "tsx", name: "shadcn-combobox", url: `${shadcn}/apps/v4/registry/new-york-v4/ui/combobox.tsx` },
  { lang: "markdown", name: "shadcn-data-table", url: `${shadcn}/apps/v4/content/docs/components/radix/data-table.mdx` },
  { lang: "markdown", name: "shadcn-message-scroller", url: `${shadcn}/apps/v4/content/docs/components/radix/message-scroller.mdx` },
  { lang: "markdown", name: "preact-readme", url: `${preact}/README.md` },
  { lang: "diff", name: "sample", url: `${samples}/diff.sample` },
  {
    lang: "diff",
    name: "cranelift-stdarch-sve",
    url: `${rustc}/compiler/rustc_codegen_cranelift/patches/0001-stdarch-Disable-SVE.patch`,
  },
  {
    lang: "diff",
    name: "cranelift-stdlib-atomics",
    url: `${rustc}/compiler/rustc_codegen_cranelift/patches/0027-stdlib-128bit-atomic-operations.patch`,
  },
  {
    lang: "diff",
    name: "mir-opt-gvn-cast",
    url: `${rustc}/tests/mir-opt/const_prop/cast.main.GVN.diff`,
  },
  {
    lang: "diff",
    name: "cranelift-sysroot-tests",
    url: `${rustc}/compiler/rustc_codegen_cranelift/patches/0028-sysroot_tests-Disable-long-running-tests.patch`,
  },
  {
    lang: "diff",
    name: "mir-opt-gvn-slices",
    url: `${rustc}/tests/mir-opt/gvn.slices.GVN.panic-abort.diff`,
  },
  {
    lang: "diff",
    name: "vscode-foundry-local-sdk",
    url: `${vscode}/build/npm/patches/foundry-local-sdk.patch`,
  },
  {
    lang: "markdown",
    name: "vitest-migration-jest",
    url: `${vitest}/docs/guide/migration/jest.md`,
  },
  { lang: "markdown", name: "vite-ssr", url: `${vite}/docs/guide/ssr.md` },
  {
    lang: "markdown",
    name: "vite-features",
    url: `${vite}/docs/guide/features.md`,
  },
  {
    lang: "markdown",
    name: "vite-api-plugin",
    url: `${vite}/docs/guide/api-plugin.md`,
  },
  { lang: "markdown", name: "prettier-plugins", url: `${prettier}/docs/plugins.md` },
  { lang: "docker", name: "sample", url: `${samples}/docker.sample` },
  { lang: "docker", name: "python-slim-bookworm", url: `${dockerPython}/3.13/slim-bookworm/Dockerfile` },
  { lang: "docker", name: "next-with-docker", url: `${nextjs}/examples/with-docker/Dockerfile` },
  { lang: "docker", name: "prometheus", url: `${prometheus}/Dockerfile` },
  { lang: "docker", name: "moby", url: `${moby}/Dockerfile` },
  { lang: "docker", name: "postgres-bookworm", url: `${dockerPostgres}/18/bookworm/Dockerfile` },
  { lang: "docker", name: "buildx", url: `${buildx}/Dockerfile` },
  { lang: "markdown", name: "fastapi-docker", url: `${fastapi}/docs/en/docs/deployment/docker.md` },
  { lang: "markdown", name: "next-with-docker-readme", url: `${nextjs}/examples/with-docker/README.md` },
  { lang: "markdown", name: "next-with-docker-export-readme", url: `${nextjs}/examples/with-docker-export-output/README.md` },
  { lang: "c", name: "sample", url: `${samples}/c.sample` },
  { lang: "c", name: "cjson", url: `${cjson}/cJSON.c` },
  { lang: "c", name: "linenoise", url: `${linenoise}/linenoise.c` },
  { lang: "c", name: "libuv-core", url: `${libuv}/src/unix/core.c` },
  { lang: "c", name: "jq-print", url: `${jq}/src/jv_print.c` },
  { lang: "c", name: "lua-vm", url: `${lua}/lvm.c` },
  { lang: "c", name: "zlib-inflate", url: `${zlib}/inflate.c` },
  { lang: "markdown", name: "cjson-readme", url: `${cjson}/README.md` },
  {
    lang: "markdown",
    name: "unity-assertions",
    url: `${unity}/docs/UnityAssertionsReference.md`,
  },
  {
    lang: "markdown",
    name: "unity-getting-started",
    url: `${unity}/docs/UnityGettingStartedGuide.md`,
  },
  { lang: "sql", name: "sample", url: `${samples}/sql.sample` },
  { lang: "sql", name: "postgres-select", url: `${postgres}/src/test/regress/sql/select.sql` },
  {
    lang: "sql",
    name: "postgres-create-table",
    url: `${postgres}/src/test/regress/sql/create_table.sql`,
  },
  {
    lang: "sql",
    name: "ssms-create-table",
    url: `${sqlServerSamples}/samples/features/ssms-templates/Sql/Table/Create%20Table.sql`,
  },
  { lang: "sql", name: "postgres-with", url: `${postgres}/src/test/regress/sql/with.sql` },
  {
    lang: "sql",
    name: "northwind-pubs",
    url: `${sqlServerSamples}/samples/databases/northwind-pubs/instpubs.sql`,
  },
  { lang: "markdown", name: "pgvector-readme", url: `${pgvector}/README.md` },
  { lang: "markdown", name: "typeorm-indexes", url: `${typeorm}/docs/docs/indexes.md` },
  { lang: "markdown", name: "knex-schema-builder", url: `${knex}/docs/src/guide/schema-builder.md` },
  { lang: "ini", name: "sample", url: `${samples}/ini.sample` },
  { lang: "ini", name: "pytest-tox", url: `${pytest}/tox.ini` },
  { lang: "ini", name: "kafka-server", url: `${kafka}/config/server.properties` },
  { lang: "ini", name: "black-tox", url: `${black}/tox.ini` },
  { lang: "ini", name: "django-tox", url: `${django}/tox.ini` },
  { lang: "ini", name: "sqlalchemy-tox", url: `${sqlalchemy}/tox.ini` },
  { lang: "ini", name: "tomcat-catalina", url: `${tomcat}/conf/catalina.properties` },
  { lang: "ini", name: "kafka-consumer", url: `${kafka}/config/consumer.properties` },
  { lang: "markdown", name: "black-other-tools", url: `${black}/docs/guides/using_black_with_other_tools.md` },
  { lang: "markdown", name: "poetry-faq", url: `${poetry}/docs/faq.md` },
  { lang: "graphql", name: "sample", url: `${samples}/graphql.sample` },
  { lang: "graphql", name: "graphql-js-kitchen-sink", url: `${graphqlJs}/benchmark/kitchen-sink.graphql` },
  { lang: "graphql", name: "graphiql-schema-kitchen-sink", url: `${graphiql}/packages/codemirror-graphql/src/__tests__/schema-kitchen-sink.graphql` },
  { lang: "graphql", name: "codegen-apollo-tooling", url: `${codegen}/dev-test/apollo-tooling/schema.graphql` },
  { lang: "graphql", name: "graphiql-kitchen-sink", url: `${graphiql}/packages/codemirror-graphql/src/__tests__/kitchen-sink.graphql` },
  { lang: "graphql", name: "graphiql-star-wars", url: `${graphiql}/packages/graphql-language-service-server/src/__tests__/__schema__/StarWarsSchema.graphql` },
  { lang: "graphql", name: "graphiql-merged-query", url: `${graphiql}/packages/graphiql-toolkit/src/graphql-helpers/__tests__/__queries__/mergedQueryWithSchema.graphql` },
  { lang: "css", name: "sample", url: `${samples}/css.sample` },
  { lang: "css", name: "normalize", url: `${normalize}/normalize.css` },
  { lang: "css", name: "tailwind-preflight", url: `${tailwind}/packages/tailwindcss/preflight.css` },
  { lang: "css", name: "highlightjs-github", url: `${highlightjs}/src/styles/github.css` },
  { lang: "css", name: "github-markdown", url: `${githubMarkdownCss}/github-markdown.css` },
  { lang: "css", name: "milligram", url: `${milligram}/dist/milligram.css` },
  { lang: "css", name: "highlightjs-atom-one-dark", url: `${highlightjs}/src/styles/atom-one-dark.css` },
  { lang: "markdown", name: "highlightjs-readme", url: `${highlightjs}/README.md` },
];
