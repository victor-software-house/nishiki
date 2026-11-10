//! Byte comparisons written as loops, for word tables and a theme's trie.
//!
//! A word table or a scope part compares a few bytes at a time.
//! `Ord` and `==` on byte slices call libc's `memcmp`, which costs more there.
//! Its count of instructions also follows where the bytes lie.
//! A change to another language's tables then moved Pascal's and Common
//! Lisp's rows by up to 1%, at equal call counts.

use std::cmp::Ordering;

/// Whether `a` and `b` hold the same bytes, as `==` on byte slices says.
pub(crate) fn same(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x == y)
}

/// Whether `a` starts with `prefix`, as `starts_with` on byte slices says.
#[cfg(any(
    feature = "lang-javascript",
    feature = "lang-jsx",
    feature = "lang-tsx",
    feature = "lang-typescript"
))]
pub(crate) fn starts_with(a: &[u8], prefix: &[u8]) -> bool {
    a.get(..prefix.len()).is_some_and(|head| same(head, prefix))
}

/// Whether `a` ends with `suffix`, as `ends_with` on byte slices says.
#[cfg(any(
    feature = "lang-javascript",
    feature = "lang-jsx",
    feature = "lang-tsx",
    feature = "lang-typescript"
))]
pub(crate) fn ends_with(a: &[u8], suffix: &[u8]) -> bool {
    a.len()
        .checked_sub(suffix.len())
        .and_then(|start| a.get(start..))
        .is_some_and(|tail| same(tail, suffix))
}

/// Whether `list` holds `word`, as `contains` on a list of byte slices says.
#[cfg(any(feature = "lang-coffee", feature = "lang-objective-c"))]
pub(crate) fn contains(list: &[&[u8]], word: &[u8]) -> bool {
    list.iter().any(|entry| same(entry, word))
}

/// `a` against `b`, in the order `Ord` on byte slices gives.
#[cfg_attr(
    all(not(feature = "_hand-lexers"), not(test)),
    expect(
        dead_code,
        reason = "only hand-written lexers and the engine tests use it, and the TextMate engine alone turns no lexer on"
    )
)]
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

    #[test]
    fn it_finds_slices_the_same_as_eq_does() {
        let words: [&[u8]; 7] = [b"", b"a", b"an", b"and", b"ant", b"b", b"be"];
        for a in words {
            for b in words {
                assert_eq!(same(a, b), a == b, "{a:?} against {b:?}");
            }
        }
    }

    #[cfg(any(feature = "lang-coffee", feature = "lang-objective-c"))]
    #[test]
    fn it_finds_words_in_a_list_as_contains_does() {
        let words: [&[u8]; 7] = [b"", b"a", b"an", b"and", b"ant", b"b", b"be"];
        let list: [&[u8]; 3] = [b"an", b"b", b""];
        for word in words {
            assert_eq!(contains(&list, word), list.contains(&word), "{word:?}");
        }
    }

    #[cfg(any(
        feature = "lang-javascript",
        feature = "lang-jsx",
        feature = "lang-tsx",
        feature = "lang-typescript"
    ))]
    #[test]
    fn it_finds_heads_and_tails_as_slices_do() {
        let words: [&[u8]; 7] = [b"", b"a", b"an", b"and", b"ant", b"b", b"be"];
        for a in words {
            for b in words {
                assert_eq!(starts_with(a, b), a.starts_with(b), "{a:?} starting {b:?}");
                assert_eq!(ends_with(a, b), a.ends_with(b), "{a:?} ending {b:?}");
            }
        }
    }
}
