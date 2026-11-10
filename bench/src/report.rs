//! The Markdown tables the benchmark programs print.

/// How a column's cells align: text to the left, numbers to the right.
#[derive(Clone, Copy, Debug)]
pub enum Align {
    /// Text, such as a file's name.
    Left,
    /// A number.
    Right,
}

/// The heading row and the alignment row of a Markdown table with `columns`.
///
/// Each column is its heading and its alignment.
/// The two rows are one string, on two lines.
#[must_use]
pub fn table_head(columns: &[(&str, Align)]) -> String {
    let headings: Vec<&str> = columns.iter().map(|&(heading, _)| heading).collect();
    let alignments: Vec<&str> = columns
        .iter()
        .map(|&(_, align)| match align {
            Align::Left => ":--",
            Align::Right => "--:",
        })
        .collect();
    format!("| {} |\n|{}|", headings.join(" | "), alignments.join("|"))
}
