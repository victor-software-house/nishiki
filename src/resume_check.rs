//! The tests that taking the last line's rules over changes no token.
//!
//! A line that continues the last starts from the frames that line left.
//! Otherwise it builds its frames again from the arena.
//! Both must give the same tokens.
//! A kept rule, one that closed without advancing, is handed on with its
//! content scopes put back, so the check covers it too.

use std::cell::Cell;

thread_local! {
    /// Whether every line builds its frames from the arena on this thread.
    static REBUILDS: Cell<bool> = const { Cell::new(false) };
}

/// Whether every line builds its frames from the arena, as the test asks.
pub(crate) fn rebuilds() -> bool {
    REBUILDS.with(Cell::get)
}

/// The scopes of each token of each line of `text`, innermost last.
fn scopes(language: crate::Language, text: &str, rebuild: bool) -> Vec<Vec<String>> {
    REBUILDS.with(|cell| cell.set(rebuild));
    let mut highlighter = crate::Highlighter::new(language);
    let mut state = crate::State::default();
    let mut tokens = Vec::new();
    let lines = text
        .lines()
        .map(|line| {
            tokens.clear();
            let _ = highlighter.line(line, &mut state, &mut tokens);
            tokens
                .iter()
                .map(|token| {
                    let mut names: Vec<&str> = highlighter.scopes(token.stack()).collect();
                    names.reverse();
                    format!("{} {}", token.end(), names.join(" "))
                })
                .collect()
        })
        .collect();
    REBUILDS.with(|cell| cell.set(false));
    lines
}

/// The text of every oracle fixture, with its language.
fn fixtures() -> Vec<(crate::Language, String)> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/oracle");
    let mut dirs: Vec<_> = std::fs::read_dir(&root)
        .expect("tests/oracle is readable")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();
    dirs.sort();
    let mut documents = Vec::new();
    for dir in dirs {
        let Some(language) = dir
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(crate::Language::from_name)
        else {
            continue;
        };
        let mut files: Vec<_> = std::fs::read_dir(&dir)
            .expect("a fixture directory is readable")
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "txt"))
            .collect();
        files.sort();
        for file in files {
            let text = std::fs::read_to_string(&file).expect("a fixture is readable");
            let lines: Vec<&str> = text
                .lines()
                .filter_map(|record| record.strip_prefix("L ").or((record == "L").then_some("")))
                .collect();
            documents.push((language, lines.join("\n")));
        }
    }
    documents
}

/// Asserts that `text` gives the same tokens whether a line takes the last
/// line's rules over or builds them again.
fn assert_same_tokens(language: crate::Language, text: &str) {
    let resumed = scopes(language, text, false);
    let rebuilt = scopes(language, text, true);
    let first = resumed.iter().zip(&rebuilt).position(|(a, b)| a != b);
    assert!(
        first.is_none(),
        "{language:?} differs at line {}: {:?} against {:?}",
        first.unwrap_or(0) + 1,
        first.map(|line| &resumed[line]),
        first.map(|line| &rebuilt[line])
    );
}

/// Every oracle fixture gives the same tokens whether a line takes the last
/// line's rules over or builds them again.
#[test]
fn taking_rules_over_gives_the_tokens_rebuilding_gives() {
    for (language, text) in &fixtures() {
        assert_same_tokens(*language, text);
    }
}

/// Shell lines that each open a rule hand their rules on unchanged.
#[cfg(feature = "lang-shell")]
#[test]
fn nested_shell_lines_take_their_rules_over() {
    let mut nested = "(\n".repeat(300);
    nested.push_str(&"echo x\n".repeat(300));
    assert_same_tokens(crate::Language::Shell, &nested);
}

/// Ruby calls whose paren a heredoc takes hand their rules on unchanged.
#[cfg(feature = "lang-ruby")]
#[test]
fn ruby_heredoc_calls_take_their_rules_over() {
    let call = indoc::indoc! {"
        expect(<<~RUBY)
          x
        RUBY
    "};
    assert_same_tokens(crate::Language::Ruby, &call.repeat(300));
}
