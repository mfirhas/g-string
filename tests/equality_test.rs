use g_string::{GString, NoValidation};

// -------------------------------------------------------------------------
// Oracle
// -------------------------------------------------------------------------

struct EqOracle {
    lhs: &'static str,
    rhs: &'static str,
    want: bool,
}

impl EqOracle {
    const fn new(lhs: &'static str, rhs: &'static str, want: bool) -> Self {
        Self { lhs, rhs, want }
    }
}

// -------------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------------

type G<const MIN: usize, const MAX: usize> = GString<NoValidation, MIN, MAX, false>;
type GA<const MIN: usize, const MAX: usize> = GString<NoValidation, MIN, MAX, true>;

fn make<const MIN: usize, const MAX: usize>(s: &str) -> G<MIN, MAX> {
    GString::try_new(s).expect("valid GString")
}

fn make_ascii<const MIN: usize, const MAX: usize>(s: &str) -> GA<MIN, MAX> {
    GString::try_new(s).expect("valid ASCII GString")
}

// -------------------------------------------------------------------------
// GString == GString (same params)
// -------------------------------------------------------------------------

#[test]
fn gstring_eq_gstring_same_params() {
    const CASES: &[EqOracle] = &[
        EqOracle::new("hello", "hello", true),
        EqOracle::new("hello", "world", false),
        EqOracle::new("", "", true),
        EqOracle::new("abc", "abcd", false),
        EqOracle::new("abcd", "abc", false),
        EqOracle::new("café", "café", true),
        EqOracle::new("café", "cafe", false),
        EqOracle::new("日本語", "日本語", true),
        EqOracle::new("日本語", "中国語", false),
        EqOracle::new("  ", "  ", true),
        EqOracle::new(" ", "  ", false),
    ];

    for tc in CASES {
        let lhs = make::<0, 32>(tc.lhs);
        let rhs = make::<0, 32>(tc.rhs);
        assert_eq!(
            lhs == rhs,
            tc.want,
            "GString({:?}) == GString({:?}): expected {}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// GString == GString (different const params)
// -------------------------------------------------------------------------

#[test]
fn gstring_eq_gstring_different_params() {
    const CASES: &[EqOracle] = &[
        EqOracle::new("hello", "hello", true),
        EqOracle::new("hello", "world", false),
        EqOracle::new("abc", "abc", true),
        EqOracle::new("abc", "xyz", false),
    ];

    for tc in CASES {
        // Different MIN/MAX
        let lhs = make::<0, 64>(tc.lhs);
        let rhs = make::<0, 128>(tc.rhs);
        assert_eq!(
            lhs == rhs,
            tc.want,
            "GString<0,64>({:?}) == GString<0,128>({:?}): expected {}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// GString == &str
// -------------------------------------------------------------------------

#[test]
fn gstring_eq_str_ref() {
    const CASES: &[EqOracle] = &[
        EqOracle::new("hello", "hello", true),
        EqOracle::new("hello", "world", false),
        EqOracle::new("", "", true),
        EqOracle::new("abc", "abcd", false),
        EqOracle::new("café", "café", true),
        EqOracle::new("café", "cafe", false),
        EqOracle::new("日本語", "日本語", true),
        EqOracle::new("日本語", "中国語", false),
    ];

    for tc in CASES {
        let lhs = make::<0, 32>(tc.lhs);
        assert_eq!(
            lhs == tc.rhs,
            tc.want,
            "GString({:?}) == &str({:?}): expected {}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// GString == str (unsized)
// -------------------------------------------------------------------------

#[test]
fn gstring_eq_str_unsized() {
    const CASES: &[EqOracle] = &[
        EqOracle::new("hello", "hello", true),
        EqOracle::new("hello", "world", false),
        EqOracle::new("", "", true),
        EqOracle::new("café", "café", true),
    ];

    for tc in CASES {
        let lhs = make::<0, 32>(tc.lhs);
        assert_eq!(
            lhs == *tc.rhs,
            tc.want,
            "GString({:?}) == str({:?}): expected {}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// &str == GString  (reversed)
// -------------------------------------------------------------------------

#[test]
fn str_ref_eq_gstring() {
    const CASES: &[EqOracle] = &[
        EqOracle::new("hello", "hello", true),
        EqOracle::new("hello", "world", false),
        EqOracle::new("", "", true),
        EqOracle::new("abc", "abcd", false),
        EqOracle::new("café", "café", true),
    ];

    for tc in CASES {
        let rhs = make::<0, 32>(tc.rhs);
        assert_eq!(
            tc.lhs == rhs,
            tc.want,
            "&str({:?}) == GString({:?}): expected {}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// str (unsized) == GString  (reversed)
// -------------------------------------------------------------------------

#[test]
fn str_unsized_eq_gstring() {
    const CASES: &[EqOracle] = &[
        EqOracle::new("hello", "hello", true),
        EqOracle::new("hello", "world", false),
        EqOracle::new("", "", true),
        EqOracle::new("café", "café", true),
    ];

    for tc in CASES {
        let rhs = make::<0, 32>(tc.rhs);
        let lhs: &str = tc.lhs;
        assert_eq!(
            *lhs == rhs,
            tc.want,
            "str({:?}) == GString({:?}): expected {}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// GString == String
// -------------------------------------------------------------------------

#[test]
fn gstring_eq_string() {
    const CASES: &[EqOracle] = &[
        EqOracle::new("hello", "hello", true),
        EqOracle::new("hello", "world", false),
        EqOracle::new("", "", true),
        EqOracle::new("abc", "abcd", false),
        EqOracle::new("café", "café", true),
        EqOracle::new("café", "cafe", false),
    ];

    for tc in CASES {
        let lhs = make::<0, 32>(tc.lhs);
        let rhs = String::from(tc.rhs);
        assert_eq!(
            lhs == rhs,
            tc.want,
            "GString({:?}) == String({:?}): expected {}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// String == GString  (reversed)
// -------------------------------------------------------------------------

#[test]
fn string_eq_gstring() {
    const CASES: &[EqOracle] = &[
        EqOracle::new("hello", "hello", true),
        EqOracle::new("hello", "world", false),
        EqOracle::new("", "", true),
        EqOracle::new("abc", "abcd", false),
        EqOracle::new("café", "café", true),
    ];

    for tc in CASES {
        let lhs = String::from(tc.lhs);
        let rhs = make::<0, 32>(tc.rhs);
        assert_eq!(
            lhs == rhs,
            tc.want,
            "String({:?}) == GString({:?}): expected {}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// Symmetry: a == b  ↔  b == a
// -------------------------------------------------------------------------

#[test]
fn symmetry_gstring_gstring() {
    const CASES: &[EqOracle] = &[
        EqOracle::new("hello", "hello", true),
        EqOracle::new("hello", "world", false),
        EqOracle::new("", "", true),
        EqOracle::new("café", "café", true),
    ];

    for tc in CASES {
        let a = make::<0, 32>(tc.lhs);
        let b = make::<0, 32>(tc.rhs);
        assert_eq!(
            a == b,
            b == a,
            "symmetry violated for ({:?}, {:?})",
            tc.lhs,
            tc.rhs
        );
    }
}

// -------------------------------------------------------------------------
// Reflexivity: a == a
// -------------------------------------------------------------------------

#[test]
fn reflexivity() {
    const CASES: &[&str] = &["", "a", "hello", "café", "日本語", "  spaces  "];

    for &s in CASES {
        let g = make::<0, 32>(s);
        assert!(g == g, "reflexivity violated for {:?}", s);
    }
}

// -------------------------------------------------------------------------
// Transitivity: a == b, b == c  =>  a == c
// -------------------------------------------------------------------------

#[test]
fn transitivity() {
    let a = make::<0, 32>("hello");
    let b = make::<0, 64>("hello");
    let c = make::<0, 128>("hello");

    assert!(a == b, "a == b");
    assert!(b == c, "b == c");
    assert!(a == c, "a == c (transitivity)");
}

// -------------------------------------------------------------------------
// ASCII_ONLY vs non-ASCII_ONLY with identical ASCII content
// -------------------------------------------------------------------------

#[test]
fn ascii_vs_non_ascii_flag_same_content() {
    const CASES: &[EqOracle] = &[
        EqOracle::new("hello", "hello", true),
        EqOracle::new("abc", "abc", true),
        EqOracle::new("hello", "world", false),
    ];

    for tc in CASES {
        let ascii = make_ascii::<0, 32>(tc.lhs);
        let utf8 = make::<0, 32>(tc.rhs);
        assert_eq!(
            ascii == utf8,
            tc.want,
            "ASCII GString({:?}) == UTF-8 GString({:?}): expected {}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// Ne: not equal  (!= is derived from PartialEq)
// -------------------------------------------------------------------------

#[test]
fn ne_gstring_gstring() {
    const CASES: &[EqOracle] = &[
        EqOracle::new("hello", "world", true),  // want ne == true
        EqOracle::new("hello", "hello", false), // want ne == false
        EqOracle::new("", "a", true),
        EqOracle::new("a", "", true),
    ];

    for tc in CASES {
        let lhs = make::<0, 32>(tc.lhs);
        let rhs = make::<0, 32>(tc.rhs);
        assert_eq!(
            lhs != rhs,
            tc.want,
            "GString({:?}) != GString({:?}): expected {}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// Hash consistency: a == b  =>  hash(a) == hash(b)
// -------------------------------------------------------------------------

#[test]
fn hash_consistent_with_eq() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn hash_of<T: Hash>(val: &T) -> u64 {
        let mut h = DefaultHasher::new();
        val.hash(&mut h);
        h.finish()
    }

    const CASES: &[&str] = &["", "hello", "world", "café", "日本語"];

    for &s in CASES {
        let g1 = make::<0, 32>(s);
        let g2 = make::<0, 64>(s);
        assert_eq!(
            hash_of(&g1),
            hash_of(&g2),
            "hash mismatch for equal GStrings ({:?})",
            s
        );
    }
}
