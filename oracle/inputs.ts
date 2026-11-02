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

export const inputs: Input[] = [
  { lang: "rust", name: "sample", url: `${samples}/rust.sample` },
  {
    lang: "rust",
    name: "ripgrep-searcher",
    url: `${ripgrep}/crates/searcher/src/searcher/mod.rs`,
  },
  { lang: "rust", name: "serde-de", url: `${serde}/serde_core/src/de/mod.rs` },
];
