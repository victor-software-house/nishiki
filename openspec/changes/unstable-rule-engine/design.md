# Design

This change makes the rule engine public under `--cfg nishiki_unstable`, so a caller can run a grammar of their own on it.
[The proposal][proposal] links the four changes split from `rust-codegen` with it.

## 1. `nishiki::unstable`

The rule engine becomes public under a cfg, as tokio does with `tokio_unstable`:

- Cargo features unify across a dependency graph, so a cfg is the opt-in.
  In [tokio's docs][tokio-unstable-docs], the flag is how a caller opts in to APIs that may break semver.
- `nishiki::unstable` compiles only with `--cfg nishiki_unstable`.
  It holds the rule engine's `Grammar` trait and every type its signatures name, such as `Act`, `CarriedState` and `Lexer`.
  `RuleSyntax<G>`, for `Highlighter<S: Syntax>`, follows in task 1.2.
  `unstable::codegen` is part of `rule-table-codegen` task 1.1, not of this change.
- Its module docs state that it is exempt from semver and may change in any patch release.
  `AGENTS.md` "Releases" gains the rule: items under `nishiki::unstable` never force a minor.
- `[workspace.lints.rust] unexpected_cfgs = { level = "deny", check-cfg = ["cfg(nishiki_unstable)"] }`, as in [tokio's workspace manifest][tokio-check-cfg].
  tokio warns; nishiki denies every lint it sets.
- `[package.metadata.docs.rs]` adds `rustc-args = ["--cfg", "nishiki_unstable"]` and the same cfg to `rustdoc-args`, as in [tokio's manifest][tokio-docs-rs].
  Unlike tokio's, the rustdoc args leave out `--cfg docsrs`, because docs.rs sets it.
  They keep `--cap-lints warn`, which `check:docs` requires.
- The module's own `#[cfg(nishiki_unstable)]` gives docs.rs the badge.
  The crate root turns on `doc_cfg` under `docsrs`, which shows each item's cfg.
  tokio writes `#[cfg_attr(docsrs, doc(cfg(tokio_unstable)))]` on each item, in its [`cfg_unstable_metrics!`][tokio-cfg-macro].
- The semver check runs without the cfg, as [tokio's semver job][tokio-semver] does.
- `check:unstable`, in `verify`, lints every target with the cfg and runs the module's doctests.
  It builds in `target/unstable`, so the cfg's builds do not replace those of `lint`.
  The full suite runs once, in `test`, without the cfg.

The rule engine's items stay in `src/rules.rs`, `src/arena.rs` and `src/scope.rs`, and `src/unstable.rs` re-exports them.
A `pub(crate)` item cannot be re-exported as `pub`.
So each item is `pub`, with an expectation for the lint that fires without the cfg:

```rust
#[cfg_attr(
    not(nishiki_unstable),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub trait Grammar: Sized + Copy + Debug + 'static {
```

A declarative macro could switch each item between `pub` and `pub(crate)` instead.
rustfmt does not format inside a macro's body, and the `Grammar` trait alone is about 300 lines.

Making `Lexer` public has two more effects under the cfg:

- An inherent `impl` block on `Lexer` bounded by a crate-private trait, such as CSS's `CssLike`, trips `private_bounds`.
  Each of the 18 such blocks carries `#[cfg_attr(nishiki_unstable, expect(private_bounds, reason = "…"))]`.
  The bound and the methods stay crate-private.
- A public item's docs may not link to a crate-private item (`rustdoc::private_intra_doc_links`).
  The docs of the exported items name such items in plain code spans instead.

[proposal]: proposal.md
[tokio-cfg-macro]: https://github.com/tokio-rs/tokio/blob/d6102c7e0a97b939c23d056122490faf777d2875/tokio/src/macros/cfg.rs#L278-L286
[tokio-check-cfg]: https://github.com/tokio-rs/tokio/blob/d6102c7e0a97b939c23d056122490faf777d2875/Cargo.toml#L28-L38
[tokio-docs-rs]: https://github.com/tokio-rs/tokio/blob/d6102c7e0a97b939c23d056122490faf777d2875/tokio/Cargo.toml#L179-L185
[tokio-semver]: https://github.com/tokio-rs/tokio/blob/d6102c7e0a97b939c23d056122490faf777d2875/.github/workflows/ci.yml#L504-L518
[tokio-unstable-docs]: https://github.com/tokio-rs/tokio/blob/d6102c7e0a97b939c23d056122490faf777d2875/tokio/src/lib.rs#L352-L374
