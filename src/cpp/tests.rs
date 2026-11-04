//! Tests of the C++ lexer.

mod raw_strings {
    use crate::Language;
    use crate::test_support::scopes_of;

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

    use serde_json::{Value, json};

    use crate::arena::Arena;
    use crate::cpp::engine::{Lexer, Scratch};
    use crate::cpp::grammar::Rx;
    use crate::language::Parts;
    use crate::limits::{Limits, LineReport};
    use crate::rules::Text;

    /// Runs the matchers on the cases in the file that `PROBE_IN` names, one
    /// JSON object per line (`rx`, `text`, `p`, `g`, `nl`), and writes their
    /// answers to the file `PROBE_OUT` names. `oracle/onig.ts` compares them
    /// with Oniguruma's.
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        let input = std::env::var("PROBE_IN").expect("PROBE_IN");
        let output = std::env::var("PROBE_OUT").expect("PROBE_OUT");
        let cases = std::fs::read_to_string(input).expect("cases");
        let mut out = String::new();
        let mut arena = Arena::new(None);
        let limits = Limits::default();
        for line in cases.lines() {
            let case: Value = serde_json::from_str(line).expect("a case");
            let name = case["rx"].as_str().expect("rx");
            let rx = Rx::ALL
                .iter()
                .copied()
                .find(|rx| format!("{rx:?}") == name)
                .expect("a known expression");
            let text = case["text"].as_str().expect("text");
            let p = usize::try_from(case["p"].as_u64().expect("p")).expect("p");
            let mut tokens = Vec::new();
            let mut report = LineReport::default();
            let mut scratch = Scratch::default();
            let parts = Parts {
                arena: &mut arena,
                limits: &limits,
                out: &mut tokens,
                report: &mut report,
            };
            let mut lexer = Lexer::new(
                parts,
                Text {
                    bytes: text.as_bytes(),
                    start: 0,
                    anchored: false,
                },
                crate::arena::ROOT,
                &mut scratch,
            );
            lexer.nl = case["nl"].as_bool().unwrap_or(true);
            let g = case["g"].as_bool().unwrap_or(false);
            let answer = lexer.probe_matcher(p, g, |lexer, p| lexer.matcher(rx, p));
            let value = answer.map_or(Value::Null, |(end, caps)| {
                json!({ "end": end, "caps": caps.iter().map(|c| c.map(|(a, b)| [a, b])).collect::<Vec<_>>() })
            });
            out.push_str(&value.to_string());
            out.push('\n');
        }
        std::fs::write(output, out).expect("write");
    }
}
