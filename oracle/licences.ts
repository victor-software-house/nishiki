// The licence of every grammar `engine-tables.ts` compiles into tables, keyed
// by Shiki's name, read at its source.
//
// Invariant 2 (AGENTS.md): every grammar is read as its language's reference,
// whatever its licence, but a grammar is bundled only when its licence permits
// it. Tables under `src/` ship in the crate, so they hold only `allowed`
// grammars whose origin THIRD-PARTY-NOTICES.md names; test tables may also
// hold `refused` ones. The generator checks both.

/** A grammar whose licence was read at its source. */
type Allowed = {
  /** Where Shiki 4.6.0 takes the grammar from, as `owner/repo@commit`. */
  bundled: string;
  /** The repository the grammar comes from, whose licence covers it. */
  origin: string;
  /** The SPDX id, or the licence's name when it has none. */
  licence: string;
  /** The file in `origin` that states the licence. */
  file: string;
};

export const allowed: Record<string, Allowed> = {
  javascript: {
    bundled: "microsoft/vscode@210541906e5a96ab39f9c753f921b1bd35f4138b",
    origin: "microsoft/TypeScript-TmLanguage",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  json: {
    bundled: "microsoft/vscode@d6af4893ed9a3545163a4cb748fa5548bd1e51a5",
    origin: "microsoft/vscode-JSON.tmLanguage",
    licence: "MIT",
    file: "LICENSE",
  },
  jsonc: {
    bundled: "microsoft/vscode@0adddd82816db666c52038e8277312dc55da2756",
    origin: "microsoft/vscode-JSON.tmLanguage",
    licence: "MIT",
    file: "LICENSE",
  },
  jsonl: {
    bundled: "microsoft/vscode@0adddd82816db666c52038e8277312dc55da2756",
    origin: "microsoft/vscode-JSON.tmLanguage",
    licence: "MIT",
    file: "LICENSE",
  },
  make: {
    bundled: "microsoft/vscode@91db8ea85fee30ebb3c7a5e7939aa2a8d79811c5",
    origin: "fadeevab/make.tmbundle",
    licence: "MIT",
    file: "LICENSE.md",
  },
  markdown: {
    bundled: "microsoft/vscode@6d8ab9737d58fc5eaf07e2ae6553b38183a5de47",
    origin: "microsoft/vscode-markdown-tm-grammar",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  powershell: {
    bundled: "microsoft/vscode@af600487b1e94374d9f48f57cbf2cad24656b07f",
    origin: "PowerShell/EditorSyntax",
    licence: "MIT",
    file: "LICENSE",
  },
  python: {
    bundled: "microsoft/vscode@cf4c9e469d521fa5f33353737e8157eb0789ad02",
    origin: "MagicStack/MagicPython",
    licence: "MIT",
    file: "LICENSE",
  },
  rust: {
    bundled: "microsoft/vscode@af600487b1e94374d9f48f57cbf2cad24656b07f",
    origin: "dustypomerleau/rust-syntax",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  shellscript: {
    bundled: "microsoft/vscode@9473445f7d3dcb5c579f42ece8b6c18c43c63ed3",
    origin: "jeff-hykin/better-shell-syntax",
    licence: "MIT",
    file: "LICENSE",
  },
  toml: {
    bundled: "textmate/toml.tmbundle@e82b64c1e86396220786846201e9aa3f0a2d9ca2",
    origin: "textmate/toml.tmbundle",
    licence: "TextMate bundle licence",
    file: "README.mdown",
  },
  tsx: {
    bundled: "microsoft/vscode@210541906e5a96ab39f9c753f921b1bd35f4138b",
    origin: "microsoft/TypeScript-TmLanguage",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  typescript: {
    bundled: "microsoft/vscode@210541906e5a96ab39f9c753f921b1bd35f4138b",
    origin: "microsoft/TypeScript-TmLanguage",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  yaml: {
    bundled: "textmate/yaml.tmbundle@e54ceae3b719506dba7e481a77cea4a8b576ae46",
    origin: "textmate/yaml.tmbundle",
    licence: "MIT",
    file: "Syntaxes/YAML-license.txt",
  },
};

/** Grammars that must never be bundled, with the reason. */
export const refused: Record<string, string> = {
  glsl: "polym0rph/GLSL.tmbundle states no licence, in a file or its README",
  r: "posit-dev/positron, where Shiki takes it from, is under the Elastic License 2.0",
};
