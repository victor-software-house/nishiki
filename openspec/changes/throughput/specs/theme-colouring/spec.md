## ADDED Requirements

### Requirement: A highlighter styles a token from its stack

A highlighter SHALL give a token's stack the style `Theme::style` gives that stack's scopes.
It SHALL keep the styles it computed for the two themes it used last, and reuse them.

#### Scenario: A stack seen before

- **WHEN** a highlighter styles a stack it has styled with the same theme before
- **THEN** it returns the cached style without walking the theme's rules

#### Scenario: Two themes in turn

- **WHEN** a highlighter styles each token with one theme and then another
- **THEN** both themes' styles stay cached, and each equals `Theme::style` on the stack's scopes

#### Scenario: A third theme

- **WHEN** a highlighter styles a stack with a theme other than the two it used last
- **THEN** it returns that theme's style for the stack, equal to `Theme::style` on the stack's scopes, in place of the older theme's styles
