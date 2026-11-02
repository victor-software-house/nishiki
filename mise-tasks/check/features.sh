#!/usr/bin/env bash
#MISE description="Each advertised feature set compiles alone"
#MISE env={CARGO_TARGET_DIR = "target/features"}
# verify depends on check:features. Each language, each tier and the empty set
# build with nothing else on. A set that leaves another language off leaves its
# helpers unused, so these are `cargo check`, which warns of that and does not
# fail on it; the clippy run in `lint` holds `--all-features` to no warnings.
# The empty set has no language: `Language` has no variants. `json` is the
# theme files feature, not the JSON language.
set -euo pipefail
cargo check --locked --tests --no-default-features
cargo check --locked --tests --no-default-features --features lang-rust
cargo check --locked --tests --no-default-features --features lang-markdown
cargo check --locked --tests --no-default-features --features lang-typescript
cargo check --locked --tests --no-default-features --features lang-javascript
cargo check --locked --tests --no-default-features --features lang-jsx
cargo check --locked --tests --no-default-features --features lang-tsx
cargo check --locked --tests --no-default-features --features lang-shell
cargo check --locked --tests --no-default-features --features lang-json
cargo check --locked --tests --no-default-features --features lang-python
cargo check --locked --tests --no-default-features --features lang-yaml
cargo check --locked --tests --no-default-features --features lang-toml
cargo check --locked --tests --no-default-features --features lang-make
cargo check --locked --tests --no-default-features --features lang-diff
cargo check --locked --tests --no-default-features --features lang-dockerfile
cargo check --locked --tests --no-default-features --features lang-java
cargo check --locked --tests --no-default-features --features lang-c
cargo check --locked --tests --no-default-features --features lang-sql
cargo check --locked --tests --no-default-features --features lang-ini
cargo check --locked --tests --no-default-features --features lang-graphql
cargo check --locked --tests --no-default-features --features lang-css
cargo check --locked --tests --no-default-features --features config
cargo check --locked --tests --no-default-features --features web
cargo check --locked --tests --no-default-features --features systems
cargo check --locked --tests --no-default-features --features docs
cargo check --locked --tests --no-default-features --features json
# Markdown alone: its own tests, which need no other language.
cargo test --locked --lib --no-default-features --features lang-markdown
