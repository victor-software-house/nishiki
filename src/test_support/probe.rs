//! What the languages' probe tests share.
//! `oracle/onig.ts` writes cases that Oniguruma answers.
//! A language's matchers answer the same cases here.

#[cfg(feature = "json")]
use std::fmt::Debug;

use crate::arena::{Arena, ROOT};
use crate::language::Parts;
use crate::limits::{Limits, LineReport};
use crate::rules::{Grammar, Lexer, Scratch, Text};

/// Whether `\G` holds where a matcher is tried.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Anchor {
    G,
    Free,
}

/// What follows the text a matcher is tried on.
#[derive(Clone, Copy, Debug)]
pub(crate) enum After {
    LineFeed,
    Nothing,
}

/// One case: a pattern tried on a text at one byte.
pub(crate) struct Case<'a> {
    /// The text, which is the start of a document.
    pub(crate) text: &'a str,
    /// The byte the pattern is tried at.
    pub(crate) p: usize,
    pub(crate) anchor: Anchor,
    pub(crate) after: After,
}

/// What a matcher answers: where its match ends and the span of each capture
/// group. It is `None` when the matcher does not match.
#[cfg(feature = "json")]
pub(crate) type Answer = Option<(usize, Vec<Option<(usize, usize)>>)>;

/// Hands `probe` a lexer of grammar `G` over the text of `case`.
/// The lexer starts at the start of a document.
/// Its cursor stands at `p`, and `\G` and the line feed follow the case.
/// It returns what `probe` returns.
pub(crate) fn lexer<G: Grammar, R>(
    arena: &mut Arena,
    case: &Case<'_>,
    probe: impl FnOnce(&mut Lexer<'_, G>) -> R,
) -> R {
    let limits = Limits::default();
    let mut tokens = Vec::new();
    let mut report = LineReport::default();
    let mut scratch = Scratch::<G>::default();
    let parts = Parts {
        arena,
        limits: &limits,
        out: &mut tokens,
        report: &mut report,
    };
    let text = Text {
        bytes: case.text.as_bytes(),
        start: 0,
        anchored: false,
    };
    let mut lexer = Lexer::new(parts, text, ROOT, &mut scratch);
    lexer.nl = matches!(case.after, After::LineFeed);
    lexer.pos = case.p;
    lexer.g = matches!(case.anchor, Anchor::G);
    probe(&mut lexer)
}

/// Runs `answer` on the cases in the file that `PROBE_IN` names.
/// That file holds one JSON object per line, with these fields:
/// - `rx`, the name of one of `patterns`, as `Debug` prints it;
/// - `text`;
/// - `p`, the byte it is tried at;
/// - `g`, whether `\G` holds there, false when absent;
/// - `nl`, whether the line feed follows the text, true when absent.
///
/// It writes the answers to the file `PROBE_OUT` names, one per line.
/// An answer is `null` for no match, else the end and the capture groups.
/// `oracle/onig.ts` compares them with Oniguruma's.
#[cfg(feature = "json")]
pub(crate) fn answer_cases<P: Copy + Debug>(
    patterns: &[P],
    mut answer: impl FnMut(&mut Arena, P, &Case<'_>) -> Answer,
) {
    use serde_json::{Value, json};

    let input = std::env::var("PROBE_IN").expect("PROBE_IN");
    let output = std::env::var("PROBE_OUT").expect("PROBE_OUT");
    let cases = std::fs::read_to_string(input).expect("cases");
    let mut arena = Arena::new(None);
    let mut out = String::new();
    for line in cases.lines() {
        let fields: Value = serde_json::from_str(line).expect("a case");
        let name = fields["rx"].as_str().expect("rx");
        let pattern = patterns
            .iter()
            .copied()
            .find(|pattern| format!("{pattern:?}") == name)
            .expect("a known pattern");
        let p = usize::try_from(fields["p"].as_u64().expect("p")).expect("p");
        let case = Case {
            text: fields["text"].as_str().expect("text"),
            p,
            anchor: match fields["g"].as_bool() {
                Some(true) => Anchor::G,
                Some(false) | None => Anchor::Free,
            },
            after: match fields["nl"].as_bool() {
                Some(false) => After::Nothing,
                Some(true) | None => After::LineFeed,
            },
        };
        let value = answer(&mut arena, pattern, &case).map_or(Value::Null, |(end, caps)| {
            let caps: Vec<Option<[usize; 2]>> = caps
                .into_iter()
                .map(|span| span.map(|(a, b)| [a, b]))
                .collect();
            json!({ "end": end, "caps": caps })
        });
        out.push_str(&value.to_string());
        out.push('\n');
    }
    std::fs::write(output, out).expect("write");
}
