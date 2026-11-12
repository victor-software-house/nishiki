//! Tests of the C++ lexer.

mod raw_strings {
    use crate::Language;
    use crate::test_support::scopes::scopes_of;

    const LONG: &str =
        "auto s = R\"abcdefghijklmnopqrstuvwxyz(text)abcdefghijklmnopqrstuvwxyz\";\nint n;";

    #[test]
    fn a_delimiter_longer_than_16_bytes_closes_the_string() {
        assert_eq!(
            scopes_of(Language::Cpp, LONG, "int"),
            [
                "source.cpp",
                "storage.type.primitive.cpp",
                "storage.type.built-in.primitive.cpp"
            ],
        );
    }

    #[test]
    fn a_long_delimiter_is_left_out_of_the_name() {
        assert_eq!(
            scopes_of(Language::Cpp, LONG, "text"),
            ["source.cpp", "string.quoted.double.raw."],
        );
    }
}

#[cfg(feature = "json")]
mod probe {
    use crate::cpp::engine::Cpp;
    use crate::cpp::grammar::Rx;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with C++'s matchers (see
    /// [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Rx::ALL, |arena, rx, case| {
            lexer::<Cpp, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(rx, p))
            })
        });
    }
}
