//! What Markdown does when the languages it embeds are not built.
//!
//! Each test runs only when its language is off, so `cargo test --lib
//! --no-default-features --features lang-markdown` exercises it.

use crate::{Highlighter, Language, State};

/// Every scope name of every token of `text`, line by line.
fn scopes_of(text: &str) -> Vec<String> {
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    let mut names = Vec::new();
    for line in text.split('\n') {
        let mut tokens = Vec::new();
        let report = highlighter.line(line, &mut state, &mut tokens);
        assert!(report.is_complete(), "{line:?} reached a limit");
        let mut start = 0;
        for token in &tokens {
            names.extend(highlighter.scopes(token.stack()).map(ToString::to_string));
            start = token.end();
        }
        assert_eq!(start, line.len(), "tokens must cover the line: {line:?}");
    }
    names
}

#[cfg(not(feature = "lang-rust"))]
#[test]
fn a_rust_fence_is_a_plain_fence_when_rust_is_off() {
    let names = scopes_of("```rust\nfn main() {}\n```");
    assert!(names.iter().all(|name| !name.contains("rust")), "{names:?}");
}

#[cfg(not(feature = "lang-yaml"))]
#[test]
fn front_matter_is_plain_text_when_yaml_is_off() {
    let names = scopes_of("---\na: 1\n---\ntext");
    assert!(names.iter().any(|name| name.contains("frontmatter")));
    assert!(names.iter().all(|name| !name.contains("yaml")), "{names:?}");
}
