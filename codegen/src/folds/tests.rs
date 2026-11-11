use super::{Folds, Tables, grouped, tables, utf8_split};

/// A mask's hex digits go in groups of four from the right.
#[test]
fn a_mask_groups_its_digits_by_four() {
    assert_eq!(grouped(0x200), "0x200");
    assert_eq!(grouped(0x8000_0000), "0x8000_0000");
    assert_eq!(grouped(0x1_0000_0000_0000), "0x1_0000_0000_0000");
}

/// A two-byte form keeps its second byte, a three-byte form its second and
/// third, the second high.
#[test]
fn a_wide_character_splits_into_its_lead_and_tail() {
    assert!(matches!(utf8_split(0xdf), Ok((0xc3, 0x9f))));
    assert!(matches!(utf8_split(0x1e9e), Ok((0xe1, 0xba9e))));
    assert!(utf8_split(0x1_f600).is_err());
}

/// ß and ẞ fold to `ss`.
/// Each member of the orbit {S, s, ſ} names the next one.
/// The fold reads in canonical characters.
/// The wide characters of the orbits it involves sit under their lead bytes.
#[test]
fn the_tables_follow_the_orbits_and_folds() {
    let folds = Folds {
        orbits: vec![vec![0x53, 0x73, 0x17f], vec![0xdf, 0x1e9e]],
        folds: vec![(0x1e9e, vec![0x73, 0x73]), (0xdf, vec![0x73, 0x73])],
    };
    let wide_lead = (0..=255u8)
        .map(|byte| match byte {
            0xc3 => 1,
            0xc5 => 2,
            0xe1 => 3,
            _ => 0,
        })
        .collect();
    let expected = Tables {
        largest: 3,
        next: vec![
            (0x53, 0x73),
            (0x73, 0x17f),
            (0xdf, 0x1e9e),
            (0x17f, 0x53),
            (0x1e9e, 0xdf),
        ],
        multi: vec![(0xdf, vec![0x53, 0x53]), (0x1e9e, vec![0x53, 0x53])],
        unfold: vec![(vec![0x53, 0x53], vec![0xdf, 0x1e9e])],
        wide_lead,
        wide_second: vec![1 << (0x9f & 0x3f), 1 << (0xbf & 0x3f), 1 << (0xba & 0x3f)],
        wide_tails: vec![vec![0x9f], vec![0xbf], vec![0xba9e]],
    };
    assert_eq!(tables(&folds).ok(), Some(expected));
}
