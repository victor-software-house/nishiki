## ADDED Requirements

### Requirement: A highlighter styles a token from its stack

A highlighter SHALL give a token's stack the style `Theme::style` gives that stack's scopes, computing each stack's style once per theme and reusing it.

#### Scenario: A stack seen before

- **WHEN** a highlighter styles a stack it has styled with the same theme before
- **THEN** it returns the cached style without walking the theme's rules

#### Scenario: Another theme

- **WHEN** a highlighter styles a stack with a theme other than the one it last used
- **THEN** it returns that theme's style for the stack, equal to `Theme::style` on the stack's scopes
