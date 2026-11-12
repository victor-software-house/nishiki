//! Lexed rows cut to each token's innermost scope: what a theme colours.

use super::rows::Row;

/// One lexed line with each token cut to its innermost scope, neighbouring
/// tokens of the same scope merged.
pub(crate) type LeafRow = Vec<(String, String)>;

/// `rows` with each token cut to its innermost scope and neighbouring tokens
/// of the same innermost scope merged into one.
pub(crate) fn merge_by_leaf(rows: Vec<Row>) -> Vec<LeafRow> {
    rows.into_iter()
        .map(|row| {
            let mut merged = LeafRow::new();
            for (text, mut scopes) in row {
                let leaf = scopes.pop().unwrap_or_default();
                match merged.last_mut() {
                    Some((last, scope)) if *scope == leaf => last.push_str(&text),
                    _ => merged.push((text, leaf)),
                }
            }
            merged
        })
        .collect()
}

/// A [`LeafRow`] from `(text, innermost scope)` pairs, to compare a lexed one
/// with.
#[cfg(any(
    feature = "lang-dockerfile",
    feature = "lang-ini",
    feature = "lang-xml"
))]
pub(crate) fn leaf_row(parts: &[(&str, &str)]) -> LeafRow {
    parts
        .iter()
        .map(|&(text, scope)| (text.to_owned(), scope.to_owned()))
        .collect()
}

/// The innermost scope of the merged token in `row` whose text is `token`.
#[cfg(feature = "_test-leaves")]
pub(crate) fn leaf_in(row: &LeafRow, token: &str) -> String {
    row.iter()
        .find(|(text, _)| text == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {row:?}"))
        .1
        .clone()
}
