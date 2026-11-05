//! Byte comparisons written as loops, for word tables.
//!
//! A word table compares a few bytes at a time.
//! `Ord` and `==` on byte slices call libc's `memcmp`, which costs more there.
//! Its count of instructions also follows where the bytes lie.
//! A change to another language's tables then moved Pascal's and Common
//! Lisp's rows by up to 1%, at equal call counts.

use std::cmp::Ordering;

/// `a` against `b`, in the order `Ord` on byte slices gives.
pub(crate) fn order(a: &[u8], b: &[u8]) -> Ordering {
    for (x, y) in a.iter().zip(b) {
        if x != y {
            return x.cmp(y);
        }
    }
    a.len().cmp(&b.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_orders_as_slices_do() {
        let words: [&[u8]; 7] = [b"", b"a", b"an", b"and", b"ant", b"b", b"be"];
        for a in words {
            for b in words {
                assert_eq!(order(a, b), a.cmp(b), "{a:?} against {b:?}");
            }
        }
    }
}
