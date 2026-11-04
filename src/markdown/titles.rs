//! Where the image rule's title scans stop, kept per line.
//!
//! An image's title runs from its opening byte to the first closing byte
//! that the final parenthesis follows.
//! The scan for that byte also stops at a newline or at the end of the text.
//! Scans from different titles that share a closing byte reach the same
//! stops.
//! The first stop at or after a position is also the first stop at or after
//! every later position up to it.
//! The memo keeps, for each closing byte, the span the last scan crossed.
//! A later scan that reaches that span jumps to its stop.
//! A line of `![a](` repeated then reads each byte once, not once per image.

/// The byte that closes an image's title.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Shut {
    /// `)`, after a title opened by `(`.
    Paren,
    /// `"`.
    Double,
    /// `'`.
    Single,
}

impl Shut {
    const fn index(self) -> usize {
        match self {
            Shut::Paren => 0,
            Shut::Double => 1,
            Shut::Single => 2,
        }
    }
}

/// The memo of where title scans stop.
///
/// It lives in the box of [`Runs`](super::runs::Runs).
/// The runs' per-line reset forgets it too.
#[derive(Debug, Default)]
pub(crate) struct Titles {
    /// The line's address and length, and the end of its text.
    text: Option<(usize, usize, usize)>,
    /// For each closing byte, the last span a scan crossed: its start and its
    /// stop.
    /// Every position in the span has that stop as its first stop.
    spans: [Option<(usize, usize)>; 3],
}

impl Titles {
    /// Forgets the spans of the last line.
    pub(crate) fn forget(&mut self) {
        self.text = None;
    }

    /// The first position at or after `from` where `is_stop` holds, and how
    /// many positions this call stepped past.
    /// `text` is the line's address and length, and the end of its text.
    /// `is_stop` must hold at the end of the text.
    pub(crate) fn stop(
        &mut self,
        text: (usize, usize, usize),
        shut: Shut,
        from: usize,
        mut is_stop: impl FnMut(usize) -> bool,
    ) -> (usize, usize) {
        if self.text != Some(text) {
            self.text = Some(text);
            self.spans = [None; 3];
        }
        let slot = &mut self.spans[shut.index()];
        let mut i = from;
        let (start, stop) = loop {
            if let Some((start, stop)) = *slot
                && start <= i
                && i <= stop
            {
                break (start.min(from), stop);
            }
            if is_stop(i) {
                break (from, i);
            }
            i += 1;
        };
        *slot = Some((start, stop));
        (stop, i - from)
    }
}

#[cfg(test)]
mod tests {
    use super::{Shut, Titles};

    /// The first stop at or after `from` in `line`, found by a plain scan.
    fn plain(line: &[u8], from: usize) -> usize {
        (from..line.len())
            .find(|&i| matches!(line[i], b')' | b'\n'))
            .unwrap_or(line.len())
    }

    /// The memo gives the stop the plain scan gives.
    /// It does so for starts taken in rising, falling and mixed order.
    #[test]
    fn the_memo_stops_where_the_plain_scan_does() {
        let line = b"ab)cd  ef)g\nhi)jk";
        let text = (line.as_ptr().addr(), line.len(), line.len());
        let rising: Vec<usize> = (0..=line.len()).collect();
        let falling: Vec<usize> = rising.iter().rev().copied().collect();
        let mixed = [5, 1, 9, 0, 14, 3, 12, 7, 17, 2];
        for order in [&rising[..], &falling[..], &mixed[..]] {
            let mut titles = Titles::default();
            for &from in order {
                let (stop, _) = titles.stop(text, Shut::Paren, from, |i| {
                    line.get(i).is_none_or(|&b| matches!(b, b')' | b'\n'))
                });
                assert_eq!(stop, plain(line, from), "from {from} in {order:?}");
            }
        }
    }

    /// Rising starts on a line with no stop before its end step past each
    /// position once in all.
    #[test]
    fn rising_starts_without_a_stop_step_past_each_position_once() {
        let line = [b'x'; 64];
        let text = (line.as_ptr().addr(), line.len(), line.len());
        let mut titles = Titles::default();
        let tested: usize = (0..line.len())
            .map(|from| titles.stop(text, Shut::Paren, from, |i| i == line.len()).1)
            .sum();
        assert_eq!(tested, line.len());
    }
}
