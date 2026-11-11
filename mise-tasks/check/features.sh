#!/usr/bin/env bash
#MISE description="Each advertised feature set compiles alone"
#MISE env={CARGO_TARGET_DIR = "target/features", RUSTFLAGS = "-D warnings -A dead_code"}
# verify depends on check:features. Each language and each tier builds with
# nothing else on, and warns of nothing but dead code: a language built
# without the ones that embed it leaves some of their helpers unused, which
# no set can avoid, while an unused import or an unreachable pattern is a
# bug. The empty set has no language, so `Language` has no variants.
# The code only languages use sits behind the private `_language` feature,
# so the empty set warns of nothing, dead code included.
# So does `json`, which is the theme files feature, not the JSON language.
# So does `textmate`, the engine alone: code only the hand-written lexers use
# carries an expectation under `not(feature = "_hand-lexers")`.
set -euo pipefail
RUSTFLAGS="-D warnings" cargo check --locked --tests --no-default-features
# Markdown alone: its own tests, which need no other language.
RUSTFLAGS="-D warnings" cargo check --locked --tests --no-default-features --features json
cargo check --locked --tests --no-default-features --features config
cargo check --locked --tests --no-default-features --features docs
# Two libraries warn of nothing, dead code included: the engine with C and
# the unstable module, as codegen builds them, and Vue with Markdown.
unstable="--cfg nishiki_unstable -D warnings"
RUSTFLAGS="$unstable" cargo check --locked --no-default-features --features textmate,lang-c
RUSTFLAGS="-D warnings" cargo check --locked --no-default-features --features lang-vue,lang-markdown
cargo check --locked --tests --no-default-features --features lang-asm
cargo check --locked --tests --no-default-features --features lang-astro
cargo check --locked --tests --no-default-features --features lang-bat
cargo check --locked --tests --no-default-features --features lang-blade
cargo check --locked --tests --no-default-features --features lang-c
cargo check --locked --tests --no-default-features --features lang-cmake
cargo check --locked --tests --no-default-features --features lang-coffee
cargo check --locked --tests --no-default-features --features lang-common-lisp
cargo check --locked --tests --no-default-features --features lang-cpp
cargo check --locked --tests --no-default-features --features lang-csharp
cargo check --locked --tests --no-default-features --features lang-css
cargo check --locked --tests --no-default-features --features lang-dart
cargo check --locked --tests --no-default-features --features lang-diff
cargo check --locked --tests --no-default-features --features lang-dockerfile
# Elixir alone, with HTML off: its `~H` and `~L` sigils embed nothing.
cargo check --locked --tests --no-default-features --features lang-elixir
cargo check --locked --tests --no-default-features --features lang-erb
cargo check --locked --tests --no-default-features --features lang-gdscript
cargo check --locked --tests --no-default-features --features lang-go
cargo check --locked --tests --no-default-features --features lang-graphql
cargo check --locked --tests --no-default-features --features lang-hack
cargo check --locked --tests --no-default-features --features lang-haml
cargo check --locked --tests --no-default-features --features lang-haskell
cargo check --locked --tests --no-default-features --features lang-hcl
cargo check --locked --tests --no-default-features --features lang-hlsl
cargo check --locked --tests --no-default-features --features lang-html
cargo check --locked --tests --no-default-features --features lang-html-derivative
cargo check --locked --tests --no-default-features --features lang-ini
cargo check --locked --tests --no-default-features --features lang-java
cargo check --locked --tests --no-default-features --features lang-javascript
cargo check --locked --tests --no-default-features --features lang-jinja
cargo check --locked --tests --no-default-features --features lang-json
cargo check --locked --tests --no-default-features --features lang-jsx
cargo check --locked --tests --no-default-features --features lang-julia
cargo check --locked --tests --no-default-features --features lang-kotlin
cargo check --locked --tests --no-default-features --features lang-latex
cargo check --locked --tests --no-default-features --features lang-less
cargo check --locked --tests --no-default-features --features lang-lua
cargo check --locked --tests --no-default-features --features lang-make
cargo check --locked --tests --no-default-features --features lang-markdown
cargo check --locked --tests --no-default-features --features lang-matlab
cargo check --locked --tests --no-default-features --features lang-mermaid
cargo check --locked --tests --no-default-features --features lang-nix
cargo check --locked --tests --no-default-features --features lang-objective-c
cargo check --locked --tests --no-default-features --features lang-objective-cpp
cargo check --locked --tests --no-default-features --features lang-pascal
cargo check --locked --tests --no-default-features --features lang-perl
cargo check --locked --tests --no-default-features --features lang-php
cargo check --locked --tests --no-default-features --features lang-postcss
cargo check --locked --tests --no-default-features --features lang-powershell
cargo check --locked --tests --no-default-features --features lang-proto
cargo check --locked --tests --no-default-features --features lang-python
cargo check --locked --tests --no-default-features --features lang-r
cargo check --locked --tests --no-default-features --features lang-ruby
cargo check --locked --tests --no-default-features --features lang-rust
cargo check --locked --tests --no-default-features --features lang-sass
cargo check --locked --tests --no-default-features --features lang-scala
cargo check --locked --tests --no-default-features --features lang-scss
cargo check --locked --tests --no-default-features --features lang-shaderlab
cargo check --locked --tests --no-default-features --features lang-shell
cargo check --locked --tests --no-default-features --features lang-sql
cargo check --locked --tests --no-default-features --features lang-svelte
cargo check --locked --tests --no-default-features --features lang-swift
cargo check --locked --tests --no-default-features --features lang-tex
cargo check --locked --tests --no-default-features --features lang-toml
cargo check --locked --tests --no-default-features --features lang-tsx
cargo check --locked --tests --no-default-features --features lang-typescript
cargo check --locked --tests --no-default-features --features lang-vb
cargo check --locked --tests --no-default-features --features lang-vue
cargo check --locked --tests --no-default-features --features lang-xml
cargo check --locked --tests --no-default-features --features lang-yaml
cargo check --locked --tests --no-default-features --features lang-zig
cargo check --locked --tests --no-default-features --features systems
RUSTFLAGS="-D warnings" cargo check --locked --no-default-features --features textmate
RUSTFLAGS="-D warnings" cargo check --locked --tests --no-default-features --features textmate
# The build-script features, each alone: tokens, then the formatted file.
RUSTFLAGS="-D warnings" cargo check --locked --tests --no-default-features --features codegen
RUSTFLAGS="-D warnings" cargo check --locked --tests --no-default-features --features format
cargo check --locked --tests --no-default-features --features web
# CoffeeScript with the engine and without Svelte's lexer: a loaded Svelte
# grammar still gives the CoffeeScript it hands off Svelte's `$` prefixes.
cargo test --locked --lib --no-default-features --features lang-coffee --features textmate \
  grammar::tests::coffeescript_in_a_loaded_svelte_grammar
# HTML with the engine for loaded grammars and without the Markdown and
# Svelte lexers: a loaded Markdown grammar still embeds HTML in Shiki's
# Markdown order, and a loaded Svelte grammar in Svelte's.
cargo test --locked --lib --no-default-features --features lang-html --features textmate \
  html::tests::html_in_a_loaded_
# JavaScript with the engine and without Svelte's lexer: a loaded Svelte
# grammar still gives the JavaScript it hands off Svelte's `$` prefixes.
cargo test --locked --lib --no-default-features --features lang-javascript --features textmate \
  grammar::tests::javascript_in_a_loaded_svelte_grammar
cargo test --locked --lib --no-default-features --features lang-markdown
