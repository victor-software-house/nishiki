//! Where the characters that lookaheads search for are in a line, found in
//! one pass the first time a lookahead asks, so that `[^{]*;` and its kin
//! cost a binary search at each position and not a scan to the line end.

/// The positions of the bytes the CSS lookaheads search for.
#[derive(Debug, Default)]
pub(crate) struct Ahead {
    open: Vec<usize>,
    close: Vec<usize>,
    semicolon: Vec<usize>,
    quote: Vec<usize>,
    colon: Vec<usize>,
}

/// A byte a lookahead searches for.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Find {
    /// `{`
    Open,
    /// `}`
    Close,
    /// `;`
    Semicolon,
    /// `"`
    Quote,
    /// `:`
    Colon,
}

impl Ahead {
    /// Indexes the bytes of `s`.
    pub(crate) fn new(s: &[u8]) -> Self {
        let mut ahead = Ahead::default();
        for (i, &b) in s.iter().enumerate() {
            match b {
                b'{' => ahead.open.push(i),
                b'}' => ahead.close.push(i),
                b';' => ahead.semicolon.push(i),
                b'"' => ahead.quote.push(i),
                b':' => ahead.colon.push(i),
                _ => {}
            }
        }
        ahead
    }

    /// The first `what` at or after `from`.
    pub(crate) fn next(&self, what: Find, from: usize) -> Option<usize> {
        let list = match what {
            Find::Open => &self.open,
            Find::Close => &self.close,
            Find::Semicolon => &self.semicolon,
            Find::Quote => &self.quote,
            Find::Colon => &self.colon,
        };
        let at = list.partition_point(|&i| i < from);
        list.get(at).copied()
    }
}
