#!/usr/bin/env bash
#MISE description="Each advertised feature set compiles alone"
#MISE env={CARGO_TARGET_DIR = "target/features", RUSTFLAGS = "-D warnings -A dead_code"}
# verify depends on check:features. Clippy runs on each language and each
# tier with nothing else on, and denies every warning but dead code.
# Clippy does the work of cargo check, and it also catches a lint that holds
# only in a reduced set, such as unused_self on a stub.
# Dead code stays allowed: a language built without the ones that embed it
# leaves some of their helpers unused, which no set can avoid.
# An unused import or an unreachable pattern is a bug.
# The empty set has no language, so `Language` has no variants.
# The code only languages use sits behind the private `_language` feature,
# so the empty set warns of nothing, dead code included.
# So does `json`, which is the theme files feature, not the JSON language.
# So does `textmate`, the engine alone: code only the hand-written lexers use
# carries an expectation under `not(feature = "_hand-lexers")`.
set -euo pipefail
RUSTFLAGS="-D warnings" cargo clippy --locked --tests --no-default-features
# Markdown alone: its own tests, which need no other language.
RUSTFLAGS="-D warnings" cargo clippy --locked --tests --no-default-features --features json
cargo clippy --locked --tests --no-default-features --features config
cargo clippy --locked --tests --no-default-features --features docs
# Two libraries warn of nothing, dead code included: the engine with C and
# the unstable module, as codegen builds them, and Vue with Markdown.
unstable="--cfg nishiki_unstable -D warnings"
RUSTFLAGS="$unstable" cargo clippy --locked --no-default-features --features textmate,lang-c
RUSTFLAGS="-D warnings" cargo clippy --locked --no-default-features \
  --features lang-vue,lang-markdown
cargo clippy --locked --tests --no-default-features --features lang-asm
cargo clippy --locked --tests --no-default-features --features lang-astro
cargo clippy --locked --tests --no-default-features --features lang-bat
cargo clippy --locked --tests --no-default-features --features lang-blade
cargo clippy --locked --tests --no-default-features --features lang-c
cargo clippy --locked --tests --no-default-features --features lang-cmake
cargo clippy --locked --tests --no-default-features --features lang-coffee
cargo clippy --locked --tests --no-default-features --features lang-common-lisp
cargo clippy --locked --tests --no-default-features --features lang-cpp
cargo clippy --locked --tests --no-default-features --features lang-csharp
cargo clippy --locked --tests --no-default-features --features lang-css
cargo clippy --locked --tests --no-default-features --features lang-dart
cargo clippy --locked --tests --no-default-features --features lang-diff
cargo clippy --locked --tests --no-default-features --features lang-dockerfile
# Elixir alone, with HTML off: its `~H` and `~L` sigils embed nothing.
cargo clippy --locked --tests --no-default-features --features lang-elixir
cargo clippy --locked --tests --no-default-features --features lang-erb
cargo clippy --locked --tests --no-default-features --features lang-gdscript
cargo clippy --locked --tests --no-default-features --features lang-go
cargo clippy --locked --tests --no-default-features --features lang-graphql
cargo clippy --locked --tests --no-default-features --features lang-hack
cargo clippy --locked --tests --no-default-features --features lang-haml
cargo clippy --locked --tests --no-default-features --features lang-haskell
cargo clippy --locked --tests --no-default-features --features lang-hcl
cargo clippy --locked --tests --no-default-features --features lang-hlsl
cargo clippy --locked --tests --no-default-features --features lang-html
cargo clippy --locked --tests --no-default-features --features lang-html-derivative
cargo clippy --locked --tests --no-default-features --features lang-ini
cargo clippy --locked --tests --no-default-features --features lang-java
cargo clippy --locked --tests --no-default-features --features lang-javascript
cargo clippy --locked --tests --no-default-features --features lang-jinja
cargo clippy --locked --tests --no-default-features --features lang-json
cargo clippy --locked --tests --no-default-features --features lang-jsx
cargo clippy --locked --tests --no-default-features --features lang-julia
cargo clippy --locked --tests --no-default-features --features lang-kotlin
cargo clippy --locked --tests --no-default-features --features lang-latex
cargo clippy --locked --tests --no-default-features --features lang-less
cargo clippy --locked --tests --no-default-features --features lang-lua
cargo clippy --locked --tests --no-default-features --features lang-make
cargo clippy --locked --tests --no-default-features --features lang-markdown
cargo clippy --locked --tests --no-default-features --features lang-matlab
cargo clippy --locked --tests --no-default-features --features lang-mermaid
cargo clippy --locked --tests --no-default-features --features lang-nix
cargo clippy --locked --tests --no-default-features --features lang-objective-c
cargo clippy --locked --tests --no-default-features --features lang-objective-cpp
cargo clippy --locked --tests --no-default-features --features lang-pascal
cargo clippy --locked --tests --no-default-features --features lang-perl
cargo clippy --locked --tests --no-default-features --features lang-php
cargo clippy --locked --tests --no-default-features --features lang-postcss
cargo clippy --locked --tests --no-default-features --features lang-powershell
cargo clippy --locked --tests --no-default-features --features lang-proto
cargo clippy --locked --tests --no-default-features --features lang-python
cargo clippy --locked --tests --no-default-features --features lang-r
cargo clippy --locked --tests --no-default-features --features lang-ruby
cargo clippy --locked --tests --no-default-features --features lang-rust
cargo clippy --locked --tests --no-default-features --features lang-sass
cargo clippy --locked --tests --no-default-features --features lang-scala
cargo clippy --locked --tests --no-default-features --features lang-scss
cargo clippy --locked --tests --no-default-features --features lang-shaderlab
cargo clippy --locked --tests --no-default-features --features lang-shell
cargo clippy --locked --tests --no-default-features --features lang-sql
cargo clippy --locked --tests --no-default-features --features lang-svelte
cargo clippy --locked --tests --no-default-features --features lang-swift
cargo clippy --locked --tests --no-default-features --features lang-tex
cargo clippy --locked --tests --no-default-features --features lang-toml
cargo clippy --locked --tests --no-default-features --features lang-tsx
cargo clippy --locked --tests --no-default-features --features lang-typescript
cargo clippy --locked --tests --no-default-features --features lang-vb
cargo clippy --locked --tests --no-default-features --features lang-vue
cargo clippy --locked --tests --no-default-features --features lang-xml
cargo clippy --locked --tests --no-default-features --features lang-yaml
cargo clippy --locked --tests --no-default-features --features lang-zig
cargo clippy --locked --tests --no-default-features --features systems
RUSTFLAGS="-D warnings" cargo clippy --locked --no-default-features --features textmate
RUSTFLAGS="-D warnings" cargo clippy --locked --tests --no-default-features --features textmate
# The build-script features, each alone: tokens, then the formatted file.
RUSTFLAGS="-D warnings" cargo clippy --locked --tests --no-default-features --features codegen
RUSTFLAGS="-D warnings" cargo clippy --locked --tests --no-default-features --features format
cargo clippy --locked --tests --no-default-features --features web
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
