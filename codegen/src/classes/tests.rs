use super::{gate, within};

/// A gate names one feature alone, two on one line, and more one to a line.
#[test]
fn a_gate_lists_more_than_two_features_one_to_a_line() {
    assert_eq!(gate(&[]), Vec::<String>::new());
    assert_eq!(gate(&["lang-r"]), [r#"#[cfg(feature = "lang-r")]"#]);
    assert_eq!(
        gate(&["lang-r", "lang-swift"]),
        [r#"#[cfg(any(feature = "lang-r", feature = "lang-swift"))]"#]
    );
    assert_eq!(
        gate(&["lang-r", "lang-swift", "lang-tex"]),
        [
            "#[cfg(any(",
            r#"    feature = "lang-r","#,
            r#"    feature = "lang-swift","#,
            r#"    feature = "lang-tex""#,
            "))]",
        ]
    );
}

/// The ranges are clipped to the points, sorted, and joined where they touch
/// or overlap, but not across a gap between points.
#[test]
fn ranges_are_clipped_to_the_points_and_joined() {
    let points = [(0x80, 0xd7ff), (0xe000, 0x33fff)];
    let members = vec![
        (0x100, 0x1ff),
        (0x41, 0x90),
        (0x200, 0x210),
        (0x150, 0x160),
        (0xd000, 0xe010),
    ];
    assert_eq!(
        within(members, &points),
        [
            (0x80, 0x90),
            (0x100, 0x210),
            (0xd000, 0xd7ff),
            (0xe000, 0xe010)
        ]
    );
}
