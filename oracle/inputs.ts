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

const nvm =
  "https://raw.githubusercontent.com/nvm-sh/nvm/5e38bc0caed342356ffd26eb21e0e9b995ac228b";

const rubyBuild =
  "https://raw.githubusercontent.com/rbenv/ruby-build/4ef3159f94dd66c8325efb42db2faeb7d5732db0";

const pyenv =
  "https://raw.githubusercontent.com/pyenv/pyenv/e6669f0a84df60daba8672c3b3f0caebd0fdcef8";

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
];
