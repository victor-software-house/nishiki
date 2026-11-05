//! Byte comparisons written as loops, for word tables.
//!
//! A word table compares a few bytes at a time.
//! `Ord` and `==` on byte slices call libc's `memcmp`, which costs more there.
//! Its count of instructions also follows where the bytes lie.
//! A change to another language's tables then moved Pascal's and Common
//! Lisp's rows by up to 1%, at equal call counts.

use std::cmp::Ordering;

/// Whether `a` and `b` hold the same bytes, as `==` on byte slices says.
#[cfg(any(feature = "lang-cmake", feature = "lang-html"))]
pub(crate) fn same(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x == y)
}

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

    #[cfg(any(feature = "lang-cmake", feature = "lang-html"))]
    #[test]
    fn it_finds_slices_the_same_as_eq_does() {
        let words: [&[u8]; 7] = [b"", b"a", b"an", b"and", b"ant", b"b", b"be"];
        for a in words {
            for b in words {
                assert_eq!(same(a, b), a == b, "{a:?} against {b:?}");
            }
        }
    }
}
