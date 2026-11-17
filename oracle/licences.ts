// The licence of every Shiki grammar the oracle reads, keyed by Shiki's name,
// read at its source.
//
// Invariant 2 (AGENTS.md): every grammar is read as its language's reference,
// whatever its licence. A grammar enters the repository only when its licence
// permits it. `engine-grammars.ts` commits the engine's test grammars. It
// refuses a grammar that `refused` lists, or that neither `allowed` nor
// `testData` lists.

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

/**
 * The grammars a table the crate ships may be compiled from.
 * Each origin has an entry in `THIRD-PARTY-NOTICES.md` (`tests/it/notices.rs`).
 */
export const allowed: Record<string, Allowed> = {
  asm: {
    bundled: "13xforever/x86_64-assembly-vscode@5c1958f4283d260f5dc88fa4f5f31c620e061a5a",
    origin: "13xforever/x86_64-assembly-vscode",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  bat: {
    bundled: "microsoft/vscode@df15c8e832a341633acd2e9bb26e1b275d09cfe0",
    origin: "mmims/language-batchfile",
    licence: "MIT",
    file: "LICENSE.md",
  },
  blade: {
    bundled: "laravel/vs-code-extension@80726f64975dc935d6b925d8230cdc1ea075802a",
    origin: "laravel/vs-code-extension",
    licence: "MIT",
    file: "LICENSE.md",
  },
  c: {
    bundled: "microsoft/vscode@bd1abe08c38bb5deccf65610f8f3c03d2338d20d",
    origin: "jeff-hykin/better-c-syntax",
    licence: "MIT",
    file: "LICENSE",
  },
  cmake: {
    bundled: "twxs/vs.language.cmake@19b1b8fe2d68d72cf294d0c3c22f0a3fc54f8f6f",
    origin: "twxs/vs.language.cmake",
    licence: "MIT",
    file: "LICENSE",
  },
  coffee: {
    bundled: "microsoft/vscode@ff58f5083f57b50e6a100ab60454dfb72fe21e80",
    origin: "atom/language-coffee-script",
    licence: "MIT",
    file: "LICENSE.md",
  },
  "common-lisp": {
    bundled: "qingpeng9802/vscode-common-lisp@79d04dc9ca80d98e65c4f28b3d67a24a7829b18f",
    origin: "qingpeng9802/vscode-common-lisp",
    licence: "MIT",
    file: "LICENSE",
  },
  cpp: {
    bundled: "microsoft/vscode@8623a79836fba2e7e058e3493345ee068d28ce4f",
    origin: "jeff-hykin/better-cpp-syntax",
    licence: "MIT",
    file: "LICENSE",
  },
  csharp: {
    bundled: "microsoft/vscode@af600487b1e94374d9f48f57cbf2cad24656b07f",
    origin: "dotnet/csharp-tmLanguage",
    licence: "MIT",
    file: "LICENSE",
  },
  css: {
    bundled: "microsoft/vscode@af600487b1e94374d9f48f57cbf2cad24656b07f",
    origin: "microsoft/vscode-css",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  dart: {
    bundled: "microsoft/vscode@af600487b1e94374d9f48f57cbf2cad24656b07f",
    origin: "dart-lang/dart-syntax-highlight",
    licence: "BSD-3-Clause",
    file: "LICENSE",
  },
  diff: {
    bundled: "microsoft/vscode@4549bd26c7b799284e0ebd8dc1e0310e6a8707a1",
    origin: "textmate/diff.tmbundle",
    licence: "TextMate bundle licence",
    file: "README.mdown",
  },
  docker: {
    bundled: "microsoft/vscode@8270a86019db7551da42b71d15c6080a414d8c81",
    origin: "moby/moby",
    licence: "Apache-2.0",
    file: "LICENSE",
  },
  elixir: {
    bundled: "elixir-editors/elixir-tmbundle@43c8cd957d5ac6e1abbd8730fc7a08c81a6e76c9",
    origin: "elixir-editors/elixir-tmbundle",
    licence: "Apache-2.0",
    file: "LICENSE",
  },
  gdscript: {
    bundled: "godotengine/godot-vscode-plugin@3f2860fd76c591a6010f0d35220490933e41d01d",
    origin: "godotengine/godot-vscode-plugin",
    licence: "MIT",
    file: "LICENSE",
  },
  go: {
    bundled: "microsoft/vscode@091ef378baaa141c8bc4bbe9775d4cb3bd655a80",
    origin: "worlpaker/go-syntax",
    licence: "MIT",
    file: "LICENSE",
  },
  graphql: {
    bundled: "prisma-labs/vscode-graphql@4569aef2364b42b6acd8188e4a6ca9b0ba3c6a1d",
    origin: "prisma-labs/vscode-graphql",
    licence: "MIT",
    file: "LICENSE",
  },
  // slackhq/vscode-hack converted the grammar from hhvm/atom-ide-hack, also
  // MIT (`LICENSE` at 35a7800fc5bc6ecd18913d17375e4d458eeb7c1f).
  hack: {
    bundled: "slackhq/vscode-hack@b518393296112ec3189d97a6051e06c4dd5b46d6",
    origin: "slackhq/vscode-hack",
    licence: "MIT",
    file: "LICENSE.md",
  },
  haml: {
    bundled: "karuna/haml-vscode@aa45415aefd691af0a501ea4f98e434490623e0d",
    origin: "karuna/haml-vscode",
    licence: "MIT",
    file: "LICENSE.md",
  },
  haskell: {
    bundled: "octref/language-haskell@ae87982aac2bf5cf6334c067e49c5ec8fc08b871",
    origin: "octref/language-haskell",
    licence: "BSD-3-Clause",
    file: "LICENSE",
  },
  hlsl: {
    bundled: "microsoft/vscode@efa12b2f49ec6ee84ed1e7bbd948808efec452f6",
    origin: "tgjones/shaders-tmLanguage",
    licence: "MIT",
    file: "LICENSE",
  },
  html: {
    bundled: "microsoft/vscode@45324363153075dab0482312ae24d8c068d81e4f",
    origin: "textmate/html.tmbundle",
    licence: "TextMate bundle licence",
    file: "README.mdown",
  },
  "html-derivative": {
    bundled: "microsoft/vscode@4064f172528461afc6944383d07521f6a30da89c",
    origin: "textmate/html.tmbundle",
    licence: "TextMate bundle licence",
    file: "README.mdown",
  },
  ini: {
    bundled: "microsoft/vscode@8fdf170a0850c1cc027382f31650aaf300d3ae2a",
    origin: "textmate/ini.tmbundle",
    licence: "TextMate bundle licence",
    file: "README.mdown",
  },
  // vscode-java is EPL-2.0, but its grammar states its own MIT licence, from atom/language-java.
  java: {
    bundled: "microsoft/vscode@3c86ede5f554f6e196c832394e126b291a1de606",
    origin: "atom/language-java",
    licence: "MIT",
    file: "LICENSE.md",
  },
  javascript: {
    bundled: "microsoft/vscode@210541906e5a96ab39f9c753f921b1bd35f4138b",
    origin: "microsoft/TypeScript-TmLanguage",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  jinja: {
    bundled: "samuelcolvin/jinjahtml-vscode@be729c88f723602e7b5df2e9dc5e8c016c31c38f",
    origin: "samuelcolvin/jinjahtml-vscode",
    licence: "MIT",
    file: "LICENSE",
  },
  "jinja-html": {
    bundled: "samuelcolvin/jinjahtml-vscode@be729c88f723602e7b5df2e9dc5e8c016c31c38f",
    origin: "samuelcolvin/jinjahtml-vscode",
    licence: "MIT",
    file: "LICENSE",
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
  jsx: {
    bundled: "microsoft/vscode@210541906e5a96ab39f9c753f921b1bd35f4138b",
    origin: "microsoft/TypeScript-TmLanguage",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  julia: {
    bundled: "microsoft/vscode@af600487b1e94374d9f48f57cbf2cad24656b07f",
    origin: "JuliaEditorSupport/atom-language-julia",
    licence: "MIT",
    file: "LICENSE.md",
  },
  latex: {
    bundled: "microsoft/vscode@af600487b1e94374d9f48f57cbf2cad24656b07f",
    origin: "jlelong/vscode-latex-basics",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  less: {
    bundled: "microsoft/vscode@03dc55030b3b2b5c4779d4ac1e2e46cd1f5a6b7c",
    origin: "radium-v/Better-Less",
    licence: "MIT",
    file: "LICENSE",
  },
  lua: {
    bundled: "microsoft/vscode@c8fdd1650c06321d687ddfd1eff06aae4b616256",
    origin: "sumneko/lua.tmbundle",
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
  "markdown-vue": {
    bundled: "vuejs/language-tools@784dd56026e07f1dadb9ef2498418eccbb3dee29",
    origin: "vuejs/language-tools",
    licence: "MIT",
    file: "LICENSE",
  },
  matlab: {
    bundled: "mathworks/MATLAB-Language-grammar@69dbf20c1eb19efd629c0438a2a493e60617e8de",
    origin: "mathworks/MATLAB-Language-grammar",
    licence: "BSD-2-Clause",
    file: "license.txt",
  },
  mermaid: {
    bundled:
      "bpruitt-goddard/vscode-mermaid-syntax-highlight@8b62f487cb7a89afcd152febfbf47f5d4787657f",
    origin: "bpruitt-goddard/vscode-mermaid-syntax-highlight",
    licence: "MIT",
    file: "LICENSE",
  },
  nix: {
    bundled: "nix-community/vscode-nix-ide@effbf3494a43250a537834805b305793994b9ca8",
    origin: "nix-community/vscode-nix-ide",
    licence: "MIT",
    file: "LICENSE",
  },
  "objective-c": {
    bundled: "microsoft/vscode@bd1abe08c38bb5deccf65610f8f3c03d2338d20d",
    origin: "jeff-hykin/better-objc-syntax",
    licence: "MIT",
    file: "LICENSE",
  },
  "objective-cpp": {
    bundled: "microsoft/vscode@bd1abe08c38bb5deccf65610f8f3c03d2338d20d",
    origin: "jeff-hykin/better-objcpp-syntax",
    licence: "MIT",
    file: "LICENSE",
  },
  pascal: {
    bundled: "alefragnani/vscode-language-pascal@f2d6345bd9fe83932f4d04fcacf98ce7d74b8034",
    origin: "textmate/pascal.tmbundle",
    licence: "TextMate bundle licence",
    file: "README.mdown",
  },
  perl: {
    bundled: "microsoft/vscode@e637ff1bf7f481134849625e982ec80df9b35bc8",
    origin: "textmate/perl.tmbundle",
    licence: "TextMate bundle licence",
    file: "README.mdown",
  },
  php: {
    bundled: "microsoft/vscode@af600487b1e94374d9f48f57cbf2cad24656b07f",
    origin: "KapitanOczywisty/language-php",
    licence: "MIT",
    file: "LICENSE.md",
  },
  powershell: {
    bundled: "microsoft/vscode@af600487b1e94374d9f48f57cbf2cad24656b07f",
    origin: "PowerShell/EditorSyntax",
    licence: "MIT",
    file: "LICENSE",
  },
  proto: {
    bundled: "zxh0/vscode-proto3@274be763ecae7055505354b44f4892a2669e1da8",
    origin: "zxh0/vscode-proto3",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  python: {
    bundled: "microsoft/vscode@cf4c9e469d521fa5f33353737e8157eb0789ad02",
    origin: "MagicStack/MagicPython",
    licence: "MIT",
    file: "LICENSE",
  },
  // The repository's root licence is the Elastic License 2.0, with no
  // exception for this folder. The grammar's own folder, at the commit Shiki
  // bundles, holds an MIT licence from the same holder, and its README states
  // that the code in the folder is under MIT. The folder's licence is read as the grammar's.
  r: {
    bundled: "posit-dev/positron@c4d6bab60dd914427a2d47a4b64e75e025d94059",
    origin: "posit-dev/positron",
    licence: "MIT",
    file: "extensions/positron-r/syntaxes/LICENSE.md",
  },
  ruby: {
    bundled: "microsoft/vscode@0ac234bbd0b6dd67efa40d1b593f9808f9dfdaf4",
    origin: "Shopify/ruby-lsp",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  rust: {
    bundled: "microsoft/vscode@af600487b1e94374d9f48f57cbf2cad24656b07f",
    origin: "dustypomerleau/rust-syntax",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  sass: {
    bundled: "TheRealSyler/vscode-sass-indented@f0e50cfaca4b64a1d49f8d938f188d6fd0bcfb6f",
    origin: "TheRealSyler/vscode-sass-indented",
    licence: "MIT",
    file: "LICENSE",
  },
  scala: {
    bundled: "scala/vscode-scala-syntax@672fa16c64e12b2cc8b98da011411463968de727",
    origin: "scala/vscode-scala-syntax",
    licence: "MIT",
    file: "LICENSE.md",
  },
  scss: {
    bundled: "microsoft/vscode@cf8d61ebd2f022f4ce8280171f0360d1fe0a206d",
    origin: "atom/language-sass",
    licence: "MIT",
    file: "LICENSE.md",
  },
  shaderlab: {
    bundled: "microsoft/vscode@8fdf170a0850c1cc027382f31650aaf300d3ae2a",
    origin: "tgjones/shaders-tmLanguage",
    licence: "MIT",
    file: "LICENSE",
  },
  shellscript: {
    bundled: "microsoft/vscode@9473445f7d3dcb5c579f42ece8b6c18c43c63ed3",
    origin: "jeff-hykin/better-shell-syntax",
    licence: "MIT",
    file: "LICENSE",
  },
  sql: {
    bundled: "microsoft/vscode@af600487b1e94374d9f48f57cbf2cad24656b07f",
    origin: "microsoft/vscode-mssql",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  svelte: {
    bundled: "sveltejs/language-tools@2cfcc15b4c44dfc1128432e20ca663fe20bdd12e",
    origin: "sveltejs/language-tools",
    licence: "MIT",
    file: "LICENSE",
  },
  swift: {
    bundled: "jtbandes/swift-tmlanguage@2be007b6bc88e54a0edfd3c313ff31276edc923d",
    origin: "jtbandes/swift-tmlanguage",
    licence: "MIT",
    file: "LICENSE.md",
  },
  tex: {
    bundled: "microsoft/vscode@609f17a76734f092792a33c891af40bbcaea0c64",
    origin: "jlelong/vscode-latex-basics",
    licence: "MIT",
    file: "LICENSE.txt",
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
  vb: {
    bundled: "microsoft/vscode@35d97bc7e439fce0f50f42074041ab2d8571b20a",
    origin: "textmate/asp.vb.net.tmbundle",
    licence: "TextMate bundle licence",
    file: "README.mdown",
  },
  vue: {
    bundled: "vuejs/language-tools@cd7a6e53d58111640f116d1e19f1c1cf06c7157d",
    origin: "vuejs/language-tools",
    licence: "MIT",
    file: "LICENSE",
  },
  "vue-directives": {
    bundled: "vuejs/language-tools@784dd56026e07f1dadb9ef2498418eccbb3dee29",
    origin: "vuejs/language-tools",
    licence: "MIT",
    file: "LICENSE",
  },
  "vue-interpolations": {
    bundled: "vuejs/language-tools@784dd56026e07f1dadb9ef2498418eccbb3dee29",
    origin: "vuejs/language-tools",
    licence: "MIT",
    file: "LICENSE",
  },
  "vue-sfc-style-variable-injection": {
    bundled: "vuejs/language-tools@e85556a2e4bf0d6c981bdbd95c8a0b1e85e7ae8a",
    origin: "vuejs/language-tools",
    licence: "MIT",
    file: "LICENSE",
  },
  xml: {
    bundled: "microsoft/vscode@10a1d2a50a2882f5ae85bdb51eb04d3064fb9de9",
    origin: "atom/language-xml",
    licence: "MIT",
    file: "LICENSE.md",
  },
  yaml: {
    bundled: "textmate/yaml.tmbundle@e54ceae3b719506dba7e481a77cea4a8b576ae46",
    origin: "textmate/yaml.tmbundle",
    licence: "MIT",
    file: "Syntaxes/YAML-license.txt",
  },
  zig: {
    bundled: "ziglang/vscode-zig@9c7bb7631f3af99331475b633aaf5b70345c6713",
    origin: "ziglang/vscode-zig",
    licence: "MIT",
    file: "LICENSE",
  },
};

/**
 * Grammars committed under `tests/engine/grammars/` only, for checks that
 * read them, such as `codegen/src/fence_stops.rs`.
 * No table the crate ships is compiled from them, so they need no entry in
 * `THIRD-PARTY-NOTICES.md`. `engine-grammars.ts` fails if a table or a word
 * list reads one.
 */
export const testData: Record<string, Allowed> = {
  abap: {
    bundled: "pvl/abap.tmbundle@92e17e62a9449d2a6f87bfb30b4f920986c24018",
    origin: "pvl/abap.tmbundle",
    licence: "TextMate bundle licence",
    file: "README.md",
  },
  bibtex: {
    bundled: "microsoft/vscode@9a2c94d213696a4453b6d9988b003d98e7646e75",
    origin: "jlelong/vscode-latex-basics",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  clojure: {
    bundled: "microsoft/vscode@a1a4c0a63b1ad59e84f54b874744fe35dea9abe1",
    origin: "atom/language-clojure",
    licence: "MIT",
    file: "LICENSE.md",
  },
  erlang: {
    bundled: "erlang-ls/grammar@fe898becd3004bcb4947ab50d6b6b830fbfb9228",
    origin: "erlang-ls/grammar",
    licence: "Apache-2.0",
    file: "LICENSE",
  },
  fsharp: {
    bundled: "microsoft/vscode@e6cffb21362b6d7addb977ff7db14f0681228780",
    origin: "ionide/ionide-fsgrammar",
    licence: "MIT",
    file: "LICENSE.md",
  },
  "git-commit": {
    bundled: "microsoft/vscode@f8c3f89468fea14103d8790d59ea8594d6e644eb",
    origin: "walles/git-commit-message-plus",
    licence: "MIT",
    file: "LICENSE",
  },
  "git-rebase": {
    bundled: "microsoft/vscode@e95c74c4c7af876e79ec58df262464467c06df28",
    origin: "textmate/git.tmbundle",
    licence: "MIT",
    file: "MIT-LICENSE",
  },
  handlebars: {
    bundled: "microsoft/vscode@af600487b1e94374d9f48f57cbf2cad24656b07f",
    origin: "daaain/Handlebars",
    licence: "MIT",
    file: "LICENSE",
  },
  log: {
    bundled: "emilast/vscode-logfile-highlighter@396c8552ff99d0e07142fdd940d75dae212dd28c",
    origin: "emilast/vscode-logfile-highlighter",
    licence: "MIT",
    file: "LICENSE",
  },
  pug: {
    bundled: "microsoft/vscode@6ac83c7c7dda9f27ca1fe9d98675253e010b75d1",
    origin: "davidrios/pug-tmbundle",
    licence: "MIT",
    file: "LICENSE",
  },
  raku: {
    bundled: "microsoft/vscode@8fdf170a0850c1cc027382f31650aaf300d3ae2a",
    origin: "textmate/perl.tmbundle",
    licence: "TextMate bundle licence",
    file: "README.mdown",
  },
  regexp: {
    bundled: "MagicStack/MagicPython@c9b3409deb69acec31bbf7913830e93a046b30cc",
    origin: "MagicStack/MagicPython",
    licence: "MIT",
    file: "LICENSE",
  },
  rst: {
    bundled: "microsoft/vscode@48e93f756b764991ede33ad09781a863aca6907a",
    origin: "trond-snekvik/vscode-rst",
    licence: "MIT",
    file: "LICENSE.txt",
  },
  xsl: {
    bundled: "microsoft/vscode@8fdf170a0850c1cc027382f31650aaf300d3ae2a",
    origin: "atom/language-xml",
    licence: "MIT",
    file: "LICENSE.md",
  },
};

/** Grammars that must never be bundled, with the reason. */
export const refused: Record<string, string> = {
  glsl: "polym0rph/GLSL.tmbundle states no licence, in a file or its README",
  groovy:
    "textmate/groovy.tmbundle, which VS Code converted it from, " +
    "states no licence in a file or its README; only VS Code's notices do",
};
