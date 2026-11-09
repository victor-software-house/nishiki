# theme-colouring Specification

## Purpose

A TextMate theme in Shiki's format gives each token the colour and font style Shiki gives it.
So nishiki's output looks as it does in Shiki and VS Code.

## Requirements

### Requirement: Colours match Shiki on every fixture

For every oracle fixture and every oracle theme, nishiki SHALL give each non-whitespace byte the foreground colour and font style Shiki gave it.

#### Scenario: A theme with descendant selectors

- **WHEN** the colour test styles a fixture with one-light, whose rules use parent selectors and the `>` combinator
- **THEN** every non-whitespace byte has Shiki's colour and font style

### Requirement: Theme files load as Shiki loads them

With the `json` feature, a theme file Shiki loads SHALL load, and a field of the wrong type SHALL be ignored rather than rejected.

#### Scenario: A rule's colour is not a string

- **WHEN** a rule's `foreground` is a number
- **THEN** the theme loads and that rule sets no foreground

### Requirement: The crate builds without JSON

Without the `json` feature, the crate SHALL build with `memchr` as its only dependency, and a theme SHALL be buildable from values.

#### Scenario: A build without features

- **WHEN** the crate is built with no features
- **THEN** it builds, and `Theme::new` takes a `ThemeSource`
