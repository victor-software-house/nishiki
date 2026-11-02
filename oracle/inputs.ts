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

const tokio =
  "https://raw.githubusercontent.com/tokio-rs/tokio/b499257ef62bd631b83a988d371f4330f545670c";
const book =
  "https://raw.githubusercontent.com/rust-lang/book/2771df165f092a4fbb9f46b5b2e7450f19010adb";
const clap =
  "https://raw.githubusercontent.com/clap-rs/clap/db30b39a55c188763666268447050a0dacba3bf0";

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
];
