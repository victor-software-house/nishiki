# Design

This change makes the rule engine public under `--cfg nishiki_unstable`, so a caller can run a grammar of their own on it.
[The proposal][proposal] links the four changes split from `rust-codegen` with it.

## 1. `nishiki::unstable`

The rule engine becomes public under a cfg, as tokio does with `tokio_unstable`:

- Cargo features unify across a dependency graph, so a cfg is the opt-in.
  In [tokio's docs][tokio-unstable-docs], the flag is how a caller opts in to APIs that may break semver.
- `nishiki::unstable` compiles only with `--cfg nishiki_unstable`.
  It holds the rule engine's `Grammar` trait, `Act`, the pattern definitions, the matcher signature, `RuleSyntax<G>` for `Highlighter<S: Syntax>`, and `unstable::codegen`.
- Its module docs state that it is exempt from semver and may change in any patch release.
  `AGENTS.md` "Releases" gains the rule: items under `nishiki::unstable` never force a minor.
- `[lints.rust] unexpected_cfgs = { level = "warn", check-cfg = ['cfg(nishiki_unstable)'] }`, as in [tokio's workspace manifest][tokio-check-cfg].
- `[package.metadata.docs.rs]` adds `rustdoc-args = ["--cfg", "docsrs", "--cfg", "nishiki_unstable"]` and `rustc-args = ["--cfg", "nishiki_unstable"]`, as in [tokio's manifest][tokio-docs-rs].
  Each item carries `#[cfg_attr(docsrs, doc(cfg(nishiki_unstable)))]`, as tokio's [`cfg_unstable_metrics!`][tokio-cfg-macro] does, so docs.rs shows the badge.
- The semver check runs without the cfg, as [tokio's semver job][tokio-semver] does.
- CI builds and tests once more with the cfg, so the unstable module stays compiled.

The rule engine's items stay in `src/rules.rs`, and `src/unstable.rs` re-exports them.
A `pub(crate)` item cannot be re-exported as `pub`, and `unreachable_pub` is denied.
So a declarative macro gives each item `pub` with the cfg and `pub(crate)` without it.

[proposal]: proposal.md
[tokio-cfg-macro]: https://github.com/tokio-rs/tokio/blob/d6102c7e0a97b939c23d056122490faf777d2875/tokio/src/macros/cfg.rs#L278-L286
[tokio-check-cfg]: https://github.com/tokio-rs/tokio/blob/d6102c7e0a97b939c23d056122490faf777d2875/Cargo.toml#L28-L38
[tokio-docs-rs]: https://github.com/tokio-rs/tokio/blob/d6102c7e0a97b939c23d056122490faf777d2875/tokio/Cargo.toml#L179-L185
[tokio-semver]: https://github.com/tokio-rs/tokio/blob/d6102c7e0a97b939c23d056122490faf777d2875/.github/workflows/ci.yml#L504-L518
[tokio-unstable-docs]: https://github.com/tokio-rs/tokio/blob/d6102c7e0a97b939c23d056122490faf777d2875/tokio/src/lib.rs#L352-L374
