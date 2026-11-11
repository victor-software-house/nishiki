//! The line filler for tables of plain values, such as word lists.
//!
//! It packs a table's items into lines of up to 100 columns
//! (openspec rust-codegen, design section 6.1).
//! A file it fills carries `#[rustfmt::skip]` on its `mod` line, so rustfmt
//! keeps the lines as written.

/// The widest line the filler writes, the bound `check:line-width` holds.
pub(crate) const WIDTH: usize = 100;

/// The indent of each line of items.
pub(crate) const INDENT: &str = "    ";

/// Appends `items` to `text`, each followed by a comma, in lines indented by
/// [`INDENT`].
///
/// Each line holds as many items as fit in [`WIDTH`] columns.
/// An item wider than that takes a line of its own.
pub(crate) fn fill(text: &mut String, items: impl IntoIterator<Item = String>) {
    fill_at(text, items, INDENT);
}

/// Appends `items` to `text` as [`fill`] does, in lines indented by `indent`.
pub(crate) fn fill_at(text: &mut String, items: impl IntoIterator<Item = String>, indent: &str) {
    let mut line = String::new();
    let mut columns = 0;
    for item in items {
        let width = item.chars().count() + 1;
        if columns > 0 && columns + 1 + width > WIDTH {
            text.push_str(&line);
            text.push('\n');
            line.clear();
            columns = 0;
        }
        if columns == 0 {
            line.push_str(indent);
            columns = indent.len();
        } else {
            line.push(' ');
            columns += 1;
        }
        line.push_str(&item);
        line.push(',');
        columns += width;
    }
    if columns > 0 {
        text.push_str(&line);
        text.push('\n');
    }
}
