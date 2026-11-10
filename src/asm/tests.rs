use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Asm);
    let mut state = State::default();
    let mut rows = Vec::new();
    for line in text.split('\n') {
        let mut tokens = Vec::new();
        let report = highlighter.line(line, &mut state, &mut tokens);
        assert!(report.is_complete(), "{line:?} reached a limit");
        let mut start = 0;
        let mut row = Vec::new();
        for token in &tokens {
            let mut scopes: Vec<String> = highlighter
                .scopes(token.stack())
                .map(ToString::to_string)
                .collect();
            scopes.reverse();
            row.push((line[start..token.end()].to_owned(), scopes));
            start = token.end();
        }
        assert_eq!(start, line.len(), "the tokens cover the line {line:?}");
        rows.push(row);
    }
    rows
}

/// The innermost scope of the token that holds byte `at` of `text`'s first
/// line.
fn scope_at(text: &str, at: usize) -> String {
    let mut start = 0;
    for (token, mut scopes) in lex(text).swap_remove(0) {
        start += token.len();
        if at < start {
            return scopes.pop().unwrap_or_default();
        }
    }
    panic!("no byte {at} in {text:?}")
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("asm"), Some(Language::Asm));
}

#[test]
fn a_word_takes_the_first_list_that_holds_it() {
    assert_eq!(
        scope_at("mov eax, ebx", 0),
        "keyword.operator.word.mnemonic.general-purpose.data-transfer.mov"
    );
    assert_eq!(
        scope_at("mov eax, ebx", 4),
        "constant.language.register.general-purpose.asm.x86_64"
    );
    assert_eq!(
        scope_at("MOV EAX, 1", 4),
        "constant.language.register.general-purpose.asm.x86_64"
    );
    assert_eq!(scope_at("movx eax", 0), "source.asm.x86_64");
}

#[test]
fn a_hash_comment_starts_at_the_last_blank_before_it() {
    let rows = lex("  movl %eax, %ebx   # copy it");
    let comment = rows[0]
        .iter()
        .find(|(_, scopes)| scopes.last().is_some_and(|s| s == "comment.line"))
        .map(|(text, _)| text.as_str());
    assert_eq!(comment, Some(" # copy it"));
}

#[test]
fn a_label_may_start_inside_a_word() {
    assert_eq!(scope_at("start:", 0), "entity.name.function.asm.x86_64");
    assert_eq!(scope_at(".loop: ret", 1), "entity.name.function.asm.x86_64");
    assert_eq!(
        scope_at("1bad:", 0),
        "invalid.illegal.entity.name.function.asm.x86_64"
    );
}

#[test]
fn a_continued_directive_still_ends_with_its_line() {
    let rows = lex(indoc! {"
        %define X \\
          1
    "});
    assert_eq!(
        rows[0]
            .last()
            .and_then(|(_, scopes)| scopes.last())
            .map(String::as_str),
        Some("punctuation.separator.continuation.c")
    );
    assert_eq!(
        rows[1]
            .last()
            .and_then(|(_, scopes)| scopes.last())
            .map(String::as_str),
        Some("constant.numeric.decimal.asm.x86_64")
    );
}

/// Oniguruma folds `ß` or a ligature to its letters only inside one literal
/// string of the expression.
/// `\b(f(i?(ld|stp?)|…))\b` holds `st` in one piece but not `fi`.
#[test]
fn a_ligature_matches_only_where_the_expression_spells_its_letters() {
    let fpu = "keyword.operator.word.mnemonic.fpu.data-transfer.other";
    assert_eq!(scope_at("\u{fb01}ld st0", 0), "source.asm.x86_64");
    assert_eq!(scope_at("fi\u{fb06}p st0", 0), fpu);
    assert_eq!(scope_at("FI\u{fb05}P st0", 0), fpu);
    assert_eq!(scope_at("\u{fb01}stp st0", 0), "source.asm.x86_64");
    assert_eq!(scope_at("\u{fb06}rict", 0), "storage.modifier.asm.x86_64");
    assert_eq!(scope_at("\u{17f}trict", 0), "storage.modifier.asm.x86_64");
}

#[test]
fn a_quote_starts_no_string() {
    assert_eq!(scope_at("msg db \"a;b\", 0", 9), "comment.line");
}

#[test]
fn numbers_follow_the_grammar_s_order() {
    assert_eq!(scope_at("db 0x1F", 3), "constant.numeric.hex.asm.x86_64");
    assert_eq!(scope_at("db 1Fh", 3), "constant.numeric.hex.asm.x86_64");
    assert_eq!(scope_at("db 101b", 3), "constant.numeric.binary.asm.x86_64");
    assert_eq!(
        scope_at("dd 1.5e3", 3),
        "constant.numeric.decimal.floating-point.asm.x86_64"
    );
    assert_eq!(
        scope_at("dt 12p3", 3),
        "constant.numeric.decimal.packed-bcd.asm.x86_64"
    );
}

#[test]
fn nasm_macros_are_literal_names() {
    assert_eq!(scope_at("dw __?FILE?__", 3), "support.function.asm.x86_64");
    assert_eq!(
        scope_at("dw __USE_ALTREG__", 3),
        "support.function.asm.x86_64"
    );
}

#[cfg(feature = "json")]
mod probe {
    use crate::asm::engine::Asm;
    use crate::asm::grammar::{Answer, Rx};
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Assembly's matchers (see
    /// [`answer_cases`]).
    /// A pattern merged into `Pattern::Word` is answered by its own word list.
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Rx::ALL, |arena, rx, case| {
            lexer::<Asm, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| match rx.answer() {
                    Answer::Pattern(pattern) => lexer.matcher(pattern, p),
                    Answer::List(index) => lexer.listed_in(p, index),
                })
            })
        });
    }
}
