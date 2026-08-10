use g_string::GStringNV;

type S = GStringNV<0, 256, false>;
type SAscii = GStringNV<0, 256, true>;

fn gstring(s: &str) -> S {
    S::try_new(s).unwrap()
}

fn gstring_ascii(s: &str) -> SAscii {
    SAscii::try_new(s).unwrap()
}

// ---------------------------------------------------------------------------
// Range
// ---------------------------------------------------------------------------

#[test]
fn index_range_oracle() {
    let cases = [
        ("hello", 0..0),
        ("hello", 0..1),
        ("hello", 0..5),
        ("hello", 1..4),
        ("hello", 4..5),
        ("hello", 5..5),
        ("hello world", 0..5),
        ("hello world", 6..11),
        ("hello world", 1..4),
        ("", 0..0),
        ("a", 0..1),
        ("a", 0..0),
        ("a", 1..1),
        ("héllo 🌍", 0..1),
        ("héllo 🌍", 1..3),
        ("héllo 🌍", 3..6),
        ("héllo 🌍", 7..11),
        ("héllo 🌍", 0..11),
    ];

    for (input, range) in cases {
        let g = gstring(input);
        let oracle = input.to_string();

        assert_eq!(&g[range.clone()], &oracle[range]);
    }
}

// ---------------------------------------------------------------------------
// RangeFrom
// ---------------------------------------------------------------------------

#[test]
fn index_range_from_oracle() {
    let cases = [
        ("hello", 0),
        ("hello", 1),
        ("hello", 4),
        ("hello", 5),
        ("hello world", 0),
        ("hello world", 6),
        ("hello world", 11),
        ("", 0),
        ("a", 0),
        ("a", 1),
        ("héllo 🌍", 0),
        ("héllo 🌍", 1),
        ("héllo 🌍", 3),
        ("héllo 🌍", 7),
        ("héllo 🌍", 11),
    ];

    for (input, start) in cases {
        let g = gstring(input);
        let oracle = input.to_string();

        assert_eq!(&g[start..], &oracle[start..]);
    }
}

// ---------------------------------------------------------------------------
// RangeTo
// ---------------------------------------------------------------------------

#[test]
fn index_range_to_oracle() {
    let cases = [
        ("hello", 0),
        ("hello", 1),
        ("hello", 5),
        ("hello world", 0),
        ("hello world", 5),
        ("hello world", 11),
        ("", 0),
        ("a", 0),
        ("a", 1),
        ("héllo 🌍", 0),
        ("héllo 🌍", 1),
        ("héllo 🌍", 3),
        ("héllo 🌍", 6),
        ("héllo 🌍", 11),
    ];

    for (input, end) in cases {
        let g = gstring(input);
        let oracle = input.to_string();

        assert_eq!(&g[..end], &oracle[..end]);
    }
}

// ---------------------------------------------------------------------------
// RangeFull
// ---------------------------------------------------------------------------

#[test]
fn index_range_full_oracle() {
    let cases = [
        "",
        "a",
        "hello",
        "hello world",
        "héllo",
        "héllo 🌍",
        "👨‍👩‍👧‍👦",
        "e\u{301}",
    ];

    for input in cases {
        let g = gstring(input);
        let oracle = input.to_string();

        assert_eq!(&g[..], &oracle[..]);
    }
}

// ---------------------------------------------------------------------------
// RangeInclusive
// ---------------------------------------------------------------------------

#[test]
fn index_range_inclusive_oracle() {
    let cases = [
        ("hello", 0..=0),
        ("hello", 0..=4),
        ("hello", 1..=3),
        ("hello", 4..=4),
        ("hello world", 0..=4),
        ("hello world", 6..=10),
        ("héllo 🌍", 0..=0),
        ("héllo 🌍", 1..=2),
        ("héllo 🌍", 3..=5),
        ("héllo 🌍", 7..=10),
    ];

    for (input, range) in cases {
        let g = gstring(input);
        let oracle = input.to_string();

        assert_eq!(&g[range.clone()], &oracle[range]);
    }
}

// ---------------------------------------------------------------------------
// RangeToInclusive
// ---------------------------------------------------------------------------

#[test]
fn index_range_to_inclusive_oracle() {
    let cases = [
        ("hello", 0),
        ("hello", 4),
        ("hello world", 0),
        ("hello world", 4),
        ("hello world", 10),
        ("héllo 🌍", 0),
        ("héllo 🌍", 2),
        ("héllo 🌍", 5),
        ("héllo 🌍", 10),
    ];

    for (input, end) in cases {
        let g = gstring(input);
        let oracle = input.to_string();

        assert_eq!(&g[..=end], &oracle[..=end]);
    }
}

// ---------------------------------------------------------------------------
// ASCII_ONLY
// ---------------------------------------------------------------------------

#[test]
fn index_ascii_only_oracle() {
    let cases = [
        ("", 0..0),
        ("a", 0..1),
        ("hello", 0..5),
        ("hello world", 0..5),
        ("hello world", 6..11),
        ("abcdefghijklmnopqrstuvwxyz", 5..20),
    ];

    for (input, range) in cases {
        let g = gstring_ascii(input);
        let oracle = input.to_string();

        assert_eq!(&g[range.clone()], &oracle[range]);
    }
}

// ---------------------------------------------------------------------------
// Invalid ranges
//
// The important property here is that GString panics for exactly the same
// kinds of ranges as str.
//
// We don't need to compare the panic itself; these ranges are deliberately
// invalid for both implementations.
// ---------------------------------------------------------------------------

#[test]
#[should_panic]
fn index_range_out_of_bounds() {
    let g = gstring("hello");
    let _ = &g[0..6];
}

#[test]
#[should_panic]
fn index_range_from_out_of_bounds() {
    let g = gstring("hello");
    let _ = &g[6..];
}

#[test]
#[should_panic]
fn index_range_to_out_of_bounds() {
    let g = gstring("hello");
    let _ = &g[..6];
}

#[test]
#[should_panic]
fn index_range_inclusive_out_of_bounds() {
    let g = gstring("hello");
    let _ = &g[0..=5];
}

#[test]
#[should_panic]
fn index_range_to_inclusive_out_of_bounds() {
    let g = gstring("hello");
    let _ = &g[..=5];
}

// ---------------------------------------------------------------------------
// Invalid UTF-8 boundaries
// ---------------------------------------------------------------------------

#[test]
#[should_panic]
fn index_range_starts_inside_unicode_character() {
    let g = gstring("héllo");
    let _ = &g[2..];
}

#[test]
#[should_panic]
fn index_range_ends_inside_unicode_character() {
    let g = gstring("héllo");
    let _ = &g[..2];
}

#[test]
#[should_panic]
fn index_range_splits_unicode_character() {
    let g = gstring("héllo");
    let _ = &g[1..2];
}

#[test]
#[should_panic]
fn index_range_inclusive_splits_unicode_character() {
    let g = gstring("héllo");
    let _ = &g[1..=1];
}

#[test]
#[should_panic]
fn index_range_to_inclusive_splits_unicode_character() {
    let g = gstring("héllo");
    let _ = &g[..=1];
}

// ---------------------------------------------------------------------------
// Invalid range ordering
// ---------------------------------------------------------------------------

#[test]
#[should_panic]
fn index_range_start_after_end() {
    let g = gstring("hello");
    let _ = &g[4..2];
}

#[test]
#[should_panic]
fn index_range_inclusive_start_after_end() {
    let g = gstring("hello");
    let _ = &g[4..=2];
}
