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

/// Whether every line builds its frames from the arena, as the test requires.
pub(crate) fn rebuilds() -> bool {
    REBUILDS.with(Cell::get)
}

/// The scopes of each token in `text`, grouped by line, innermost last, lexed
/// under `limits`.
fn scopes(
    language: crate::Language,
    text: &str,
    limits: crate::Limits,
    rebuild: bool,
) -> Vec<Vec<String>> {
    REBUILDS.with(|cell| cell.set(rebuild));
    let mut highlighter = crate::Highlighter::new_with_limits(language, limits);
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

/// Asserts that `text` gives the same tokens whether a line takes the last
/// line's rules over or builds them again.
fn assert_same_tokens(language: crate::Language, text: &str) {
    assert_same_tokens_under(language, text, crate::Limits::default());
}

/// Asserts the same as [`assert_same_tokens`], under `limits`.
fn assert_same_tokens_under(language: crate::Language, text: &str, limits: crate::Limits) {
    let resumed = scopes(language, text, limits, false);
    let rebuilt = scopes(language, text, limits, true);
    let first = resumed.iter().zip(&rebuilt).position(|(a, b)| a != b);
    assert!(
        first.is_none(),
        "{language:?} differs at line {}: {:?} against {:?}",
        first.unwrap_or(0) + 1,
        first.map(|line| &resumed[line]),
        first.map(|line| &rebuilt[line])
    );
}

/// A script can close on the line where the node limit keeps one of its
/// contexts out of the arena.
/// It still hands the line back to Svelte.
/// The line after it then holds no script scope, at every limit from 1 to
/// 400 (Kody, [pull request 618]).
///
/// [pull request 618]: https://github.com/victor-software-house/nishiki-archive/pull/618
#[cfg(feature = "lang-svelte")]
#[test]
fn a_script_closed_on_a_full_line_ends_there() {
    let document = indoc::indoc! {"
        <script>let a = [1, {c: 2}];</script><div/>
        <div class=\"x\">y</div>
    "};
    for nodes in 1..400 {
        let limits = crate::Limits {
            nodes: Some(nodes),
            ..crate::Limits::default()
        };
        let lines = scopes(crate::Language::Svelte, document, limits, false);
        let after = lines.get(1).map_or(&[][..], Vec::as_slice);
        assert!(
            after
                .iter()
                .all(|token| !token.contains("source.js") && !token.contains("source.ts")),
            "node limit {nodes}: {after:?}"
        );
    }
}

/// Every oracle fixture gives the same tokens whether a line takes the last
/// line's rules over or builds them again.
#[test]
fn taking_rules_over_gives_the_tokens_rebuilding_gives() {
    for (language, text) in &crate::test_support::oracle_documents() {
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

/// Svelte lines that end inside a style or a script hand their rules on
/// unchanged.
/// That holds even where the node limit keeps the embedding rule's new state
/// out of the arena.
/// Each limit from 1 to 400 stops the arena at another place in each
/// document.
#[cfg(feature = "lang-svelte")]
#[test]
fn embedded_lines_at_the_node_limit_take_their_rules_over() {
    let style = indoc::indoc! {"
        <style>
          a {
            color: red;
            margin: 0
              auto;
          }
          /* one
             two */
          @media (width > 1px) {
            d { e: f }
          }
        </style>
        <div class=\"b\">x</div>
    "};
    let script = indoc::indoc! {"
        <script>
          let a = 1;
          const c = {
            d: [a, 'two'],
            e: `f ${a}
              g`,
          };
          /* one
             two */
        </script>
        <div>{a}</div>
    "};
    for document in [style, script] {
        for nodes in 1..400 {
            let limits = crate::Limits {
                nodes: Some(nodes),
                ..crate::Limits::default()
            };
            assert_same_tokens_under(crate::Language::Svelte, document, limits);
        }
    }
}
