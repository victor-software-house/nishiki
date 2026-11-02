# Proposal

## Why

nishiki returned scopes only, so a caller had to map scopes to colours itself, as a-tui does today with its own table of eleven groups.
The theme decision makes TextMate themes in Shiki's format the theme format, so a theme written for Shiki or VS Code colours nishiki's output unchanged.

## What Changes

- `Theme`, built from a `ThemeSource` of rules, resolves rules the way VS Code's TextMate engine does: a trie on the dot-separated parts of each selector, parent selectors with the `>` combinator, VS Code's specificity order, and attributes inherited from outer scopes.
- `Theme::style` gives a scope stack its `Style`: foreground, background and font style.
- An optional `json` feature adds `Theme::from_json` for theme files, depending on `serde_json` only (operator, 2026-11-02).
- The oracle writes three themes and Shiki's colours for every fixture; a test compares nishiki's colours with them.

## Capabilities

### New Capabilities

- `theme-colouring`: a theme in Shiki's format gives each token the colour and font style Shiki gives it.

### Modified Capabilities

None.

## Impact

- New public types: `Theme`, `ThemeSource`, `ThemeRule`, `RuleScope`, `ThemeKind`, `Style`, `Color`, `FontStyle`, and `ThemeError` with `json`.
- Optional dependency `serde_json` behind `json`; without it the crate depends on `memchr` alone.
