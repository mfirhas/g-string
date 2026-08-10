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
// Range<usize>
// ---------------------------------------------------------------------------

#[test]
fn index_range() {
    let s = gstring("hello world");

    assert_eq!(&s[0..5], "hello");
    assert_eq!(&s[6..11], "world");
    assert_eq!(&s[1..4], "ell");
}

#[test]
fn index_range_at_start() {
    let s = gstring("hello");

    assert_eq!(&s[0..0], "");
    assert_eq!(&s[0..1], "h");
    assert_eq!(&s[0..5], "hello");
}

#[test]
fn index_range_at_end() {
    let s = gstring("hello");

    assert_eq!(&s[5..5], "");
    assert_eq!(&s[4..5], "o");
    assert_eq!(&s[1..5], "ello");
}

#[test]
fn index_range_full() {
    let s = gstring("hello");

    assert_eq!(&s[0..5], "hello");
}

// ---------------------------------------------------------------------------
// RangeFrom<usize>
// ---------------------------------------------------------------------------

#[test]
fn index_range_from() {
    let s = gstring("hello world");

    assert_eq!(&s[0..], "hello world");
    assert_eq!(&s[1..], "ello world");
    assert_eq!(&s[6..], "world");
    assert_eq!(&s[11..], "");
}

#[test]
fn index_range_from_start() {
    let s = gstring("hello");

    assert_eq!(&s[0..], "hello");
}

#[test]
fn index_range_from_end() {
    let s = gstring("hello");

    assert_eq!(&s[5..], "");
}

// ---------------------------------------------------------------------------
// RangeTo<usize>
// ---------------------------------------------------------------------------

#[test]
fn index_range_to() {
    let s = gstring("hello world");

    assert_eq!(&s[..0], "");
    assert_eq!(&s[..1], "h");
    assert_eq!(&s[..5], "hello");
    assert_eq!(&s[..11], "hello world");
}

// ---------------------------------------------------------------------------
// RangeFull
// ---------------------------------------------------------------------------

#[test]
fn index_range_full_operator() {
    let s = gstring("hello world");

    assert_eq!(&s[..], "hello world");
}

#[test]
fn index_range_full_empty() {
    let s = gstring("");

    assert_eq!(&s[..], "");
}

// ---------------------------------------------------------------------------
// RangeInclusive<usize>
// ---------------------------------------------------------------------------

#[test]
fn index_range_inclusive() {
    let s = gstring("hello world");

    assert_eq!(&s[0..=4], "hello");
    assert_eq!(&s[6..=10], "world");
    assert_eq!(&s[1..=3], "ell");
}

#[test]
fn index_range_inclusive_single_byte() {
    let s = gstring("hello");

    assert_eq!(&s[0..=0], "h");
    assert_eq!(&s[4..=4], "o");
}

#[test]
fn index_range_inclusive_full() {
    let s = gstring("hello");

    assert_eq!(&s[0..=4], "hello");
}

// ---------------------------------------------------------------------------
// RangeToInclusive<usize>
// ---------------------------------------------------------------------------

#[test]
fn index_range_to_inclusive() {
    let s = gstring("hello world");

    assert_eq!(&s[..=0], "h");
    assert_eq!(&s[..=4], "hello");
    assert_eq!(&s[..=10], "hello world");
}

// ---------------------------------------------------------------------------
// Empty strings
// ---------------------------------------------------------------------------

#[test]
fn index_empty_string() {
    let s = gstring("");

    assert_eq!(&s[..], "");
    assert_eq!(&s[0..0], "");
    assert_eq!(&s[0..], "");
    assert_eq!(&s[..0], "");
}

// ---------------------------------------------------------------------------
// Single-character strings
// ---------------------------------------------------------------------------

#[test]
fn index_single_character() {
    let s = gstring("a");

    assert_eq!(&s[..], "a");
    assert_eq!(&s[0..1], "a");
    assert_eq!(&s[0..=0], "a");

    assert_eq!(&s[0..0], "");
    assert_eq!(&s[1..1], "");
    assert_eq!(&s[1..], "");
    assert_eq!(&s[..0], "");
}

// ---------------------------------------------------------------------------
// Unicode
//
// Indexing str is byte-based, not character-based.
//
// "héllo 🌍"
//   h  = byte 0
//   é  = bytes 1..3
//   l  = byte 3
//   l  = byte 4
//   o  = byte 5
//   ' '= byte 6
//   🌍 = bytes 7..11
// ---------------------------------------------------------------------------

#[test]
fn index_unicode() {
    let s = gstring("héllo 🌍");

    assert_eq!(&s[0..1], "h");
    assert_eq!(&s[1..3], "é");
    assert_eq!(&s[3..6], "llo");
    assert_eq!(&s[0..6], "héllo");
    assert_eq!(&s[7..11], "🌍");
    assert_eq!(&s[0..11], "héllo 🌍");
}

#[test]
fn index_unicode_from() {
    let s = gstring("héllo 🌍");

    assert_eq!(&s[0..], "héllo 🌍");
    assert_eq!(&s[1..], "éllo 🌍");
    assert_eq!(&s[3..], "llo 🌍");
    assert_eq!(&s[7..], "🌍");
    assert_eq!(&s[11..], "");
}

#[test]
fn index_unicode_to() {
    let s = gstring("héllo 🌍");

    assert_eq!(&s[..0], "");
    assert_eq!(&s[..1], "h");
    assert_eq!(&s[..3], "hé");
    assert_eq!(&s[..6], "héllo");
    assert_eq!(&s[..11], "héllo 🌍");
}

#[test]
fn index_unicode_inclusive() {
    let s = gstring("héllo 🌍");

    assert_eq!(&s[0..=0], "h");
    assert_eq!(&s[1..=2], "é");
    assert_eq!(&s[3..=5], "llo");
    assert_eq!(&s[7..=10], "🌍");
}

#[test]
fn index_unicode_to_inclusive() {
    let s = gstring("héllo 🌍");

    assert_eq!(&s[..=0], "h");
    assert_eq!(&s[..=2], "hé");
    assert_eq!(&s[..=5], "héllo");
    assert_eq!(&s[..=10], "héllo 🌍");
}

// ---------------------------------------------------------------------------
// ASCII-only GString
//
// Make sure Index works independently of the ASCII_ONLY const parameter.
// ---------------------------------------------------------------------------

#[test]
fn index_ascii_only_gstring() {
    let s = gstring_ascii("hello world");

    assert_eq!(&s[..5], "hello");
    assert_eq!(&s[6..], "world");
    assert_eq!(&s[1..=4], "ello");
    assert_eq!(&s[..], "hello world");
}

// ---------------------------------------------------------------------------
// Boundary behavior
//
// These are delegated to str's indexing implementation and should panic.
// ---------------------------------------------------------------------------

#[test]
#[should_panic]
fn index_range_out_of_bounds() {
    let s = gstring("hello");

    let _ = &s[0..6];
}

#[test]
#[should_panic]
fn index_range_from_out_of_bounds() {
    let s = gstring("hello");

    let _ = &s[6..];
}

#[test]
#[should_panic]
fn index_range_to_out_of_bounds() {
    let s = gstring("hello");

    let _ = &s[..6];
}

#[test]
#[should_panic]
fn index_range_inclusive_out_of_bounds() {
    let s = gstring("hello");

    let _ = &s[0..=5];
}

#[test]
#[should_panic]
fn index_range_to_inclusive_out_of_bounds() {
    let s = gstring("hello");

    let _ = &s[..=5];
}

// ---------------------------------------------------------------------------
// Invalid range ordering
// ---------------------------------------------------------------------------

#[test]
#[should_panic]
fn index_range_start_after_end() {
    let s = gstring("hello");

    let _ = &s[4..2];
}

#[test]
#[should_panic]
fn index_range_inclusive_start_after_end() {
    let s = gstring("hello");

    let _ = &s[4..=2];
}

// ---------------------------------------------------------------------------
// Invalid UTF-8 boundaries
//
// These are valid byte positions in the string, but they split a UTF-8
// codepoint. str indexing must panic.
//
// "é" occupies bytes 0..2, so byte index 1 is NOT a character boundary.
// ---------------------------------------------------------------------------

#[test]
#[should_panic]
fn index_range_starts_inside_unicode_character() {
    let s = gstring("héllo");

    let _ = &s[2..];
}

#[test]
#[should_panic]
fn index_range_ends_inside_unicode_character() {
    let s = gstring("héllo");

    let _ = &s[..2];
}

#[test]
#[should_panic]
fn index_range_splits_unicode_character() {
    let s = gstring("héllo");

    let _ = &s[1..2];
}

#[test]
#[should_panic]
fn index_range_inclusive_splits_unicode_character() {
    let s = gstring("héllo");

    let _ = &s[1..=1];
}

#[test]
#[should_panic]
fn index_range_to_inclusive_splits_unicode_character() {
    let s = gstring("héllo");

    let _ = &s[..=1];
}
