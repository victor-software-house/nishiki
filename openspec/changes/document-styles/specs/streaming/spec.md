## ADDED Requirements

### Requirement: A document's tokens come styled

Each token of an exact or provisional line SHALL come with the style the document's theme gives its stack.
That style SHALL equal `Theme::style` of the stack's scopes.
After a theme change, no token SHALL carry a style from the earlier theme.

#### Scenario: Styles match the theme

- **WHEN** an oracle fixture is streamed into a document with an oracle theme, then lexed to its end
- **THEN** every token's style equals `Theme::style` of its scopes

#### Scenario: A theme switched mid-stream

- **WHEN** the theme changes after half of a fixture has arrived
- **THEN** every token's style equals the new theme's `Theme::style` of its scopes
