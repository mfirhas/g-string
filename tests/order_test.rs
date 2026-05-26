use core::cmp::Ordering;
use g_string::{GString, NoValidation};

// -------------------------------------------------------------------------
// Oracle
// -------------------------------------------------------------------------

struct OrdOracle {
    lhs: &'static str,
    rhs: &'static str,
    want: Ordering,
}

impl OrdOracle {
    const fn new(lhs: &'static str, rhs: &'static str, want: Ordering) -> Self {
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

// Shared oracle cases reused across multiple test groups
const BASIC_CASES: &[OrdOracle] = &[
    OrdOracle::new("a", "b", Ordering::Less),
    OrdOracle::new("b", "a", Ordering::Greater),
    OrdOracle::new("a", "a", Ordering::Equal),
    OrdOracle::new("", "", Ordering::Equal),
    OrdOracle::new("", "a", Ordering::Less),
    OrdOracle::new("a", "", Ordering::Greater),
    OrdOracle::new("abc", "abd", Ordering::Less),
    OrdOracle::new("abd", "abc", Ordering::Greater),
    OrdOracle::new("abc", "abcd", Ordering::Less),
    OrdOracle::new("abcd", "abc", Ordering::Greater),
    OrdOracle::new("hello", "hello", Ordering::Equal),
    OrdOracle::new("hello", "world", Ordering::Less),
    OrdOracle::new("world", "hello", Ordering::Greater),
    // Lexicographic: uppercase < lowercase in ASCII
    OrdOracle::new("A", "a", Ordering::Less),
    OrdOracle::new("Z", "a", Ordering::Less),
    // Unicode: codepoint order
    OrdOracle::new("café", "cafó", Ordering::Less),
    OrdOracle::new("cafó", "café", Ordering::Greater),
    OrdOracle::new("café", "café", Ordering::Equal),
    // Length tie-break
    OrdOracle::new("ab", "abc", Ordering::Less),
    OrdOracle::new("abc", "ab", Ordering::Greater),
];

// -------------------------------------------------------------------------
// Ord::cmp — GString<V,MIN,MAX,ASCII> (same params)
// -------------------------------------------------------------------------

#[test]
fn ord_cmp_same_params() {
    for tc in BASIC_CASES {
        let lhs = make::<0, 32>(tc.lhs);
        let rhs = make::<0, 32>(tc.rhs);
        assert_eq!(
            lhs.cmp(&rhs),
            tc.want,
            "cmp({:?}, {:?}): expected {:?}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// PartialOrd::partial_cmp — GString == GString (same params)
// -------------------------------------------------------------------------

#[test]
fn partial_cmp_gstring_gstring_same_params() {
    for tc in BASIC_CASES {
        let lhs = make::<0, 32>(tc.lhs);
        let rhs = make::<0, 32>(tc.rhs);
        assert_eq!(
            lhs.partial_cmp(&rhs),
            Some(tc.want),
            "partial_cmp({:?}, {:?}): expected {:?}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// PartialOrd — GString vs GString with different const params
// -------------------------------------------------------------------------

#[test]
fn partial_cmp_gstring_gstring_different_params() {
    for tc in BASIC_CASES {
        let lhs = make::<0, 64>(tc.lhs);
        let rhs = make::<0, 128>(tc.rhs);
        assert_eq!(
            lhs.partial_cmp(&rhs),
            Some(tc.want),
            "partial_cmp GString<0,64>({:?}) vs GString<0,128>({:?}): expected {:?}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// PartialOrd — GString vs &str
// -------------------------------------------------------------------------

#[test]
fn partial_cmp_gstring_str_ref() {
    for tc in BASIC_CASES {
        let lhs = make::<0, 32>(tc.lhs);
        assert_eq!(
            PartialOrd::<&str>::partial_cmp(&lhs, &tc.rhs),
            Some(tc.want),
            "partial_cmp GString({:?}) vs &str({:?}): expected {:?}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// PartialOrd — GString vs str (unsized)
// -------------------------------------------------------------------------

#[test]
fn partial_cmp_gstring_str_unsized() {
    for tc in BASIC_CASES {
        let lhs = make::<0, 32>(tc.lhs);
        assert_eq!(
            PartialOrd::<str>::partial_cmp(&lhs, tc.rhs),
            Some(tc.want),
            "partial_cmp GString({:?}) vs str({:?}): expected {:?}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// PartialOrd — &str vs GString  (reversed)
// -------------------------------------------------------------------------

#[test]
fn partial_cmp_str_ref_gstring() {
    for tc in BASIC_CASES {
        let rhs = make::<0, 32>(tc.rhs);
        assert_eq!(
            tc.lhs.partial_cmp(&rhs),
            Some(tc.want),
            "partial_cmp &str({:?}) vs GString({:?}): expected {:?}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// PartialOrd — str (unsized) vs GString  (reversed)
// -------------------------------------------------------------------------

#[test]
fn partial_cmp_str_unsized_gstring() {
    for tc in BASIC_CASES {
        let rhs = make::<0, 32>(tc.rhs);
        let lhs: &str = tc.lhs;
        assert_eq!(
            (*lhs).partial_cmp(&rhs),
            Some(tc.want),
            "partial_cmp str({:?}) vs GString({:?}): expected {:?}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// PartialOrd — GString vs String
// -------------------------------------------------------------------------

#[test]
fn partial_cmp_gstring_string() {
    for tc in BASIC_CASES {
        let lhs = make::<0, 32>(tc.lhs);
        let rhs = String::from(tc.rhs);
        assert_eq!(
            lhs.partial_cmp(&rhs),
            Some(tc.want),
            "partial_cmp GString({:?}) vs String({:?}): expected {:?}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// PartialOrd — String vs GString  (reversed)
// -------------------------------------------------------------------------

#[test]
fn partial_cmp_string_gstring() {
    for tc in BASIC_CASES {
        let lhs = String::from(tc.lhs);
        let rhs = make::<0, 32>(tc.rhs);
        assert_eq!(
            lhs.partial_cmp(&rhs),
            Some(tc.want),
            "partial_cmp String({:?}) vs GString({:?}): expected {:?}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// Comparison operators: <, <=, >, >=
// -------------------------------------------------------------------------

#[test]
fn comparison_operators() {
    struct OpOracle {
        lhs: &'static str,
        rhs: &'static str,
        lt: bool,
        le: bool,
        gt: bool,
        ge: bool,
    }

    const CASES: &[OpOracle] = &[
        OpOracle {
            lhs: "a",
            rhs: "b",
            lt: true,
            le: true,
            gt: false,
            ge: false,
        },
        OpOracle {
            lhs: "b",
            rhs: "a",
            lt: false,
            le: false,
            gt: true,
            ge: true,
        },
        OpOracle {
            lhs: "a",
            rhs: "a",
            lt: false,
            le: true,
            gt: false,
            ge: true,
        },
        OpOracle {
            lhs: "",
            rhs: "",
            lt: false,
            le: true,
            gt: false,
            ge: true,
        },
        OpOracle {
            lhs: "",
            rhs: "a",
            lt: true,
            le: true,
            gt: false,
            ge: false,
        },
        OpOracle {
            lhs: "a",
            rhs: "",
            lt: false,
            le: false,
            gt: true,
            ge: true,
        },
    ];

    for tc in CASES {
        let lhs = make::<0, 32>(tc.lhs);
        let rhs = make::<0, 32>(tc.rhs);
        assert_eq!(
            lhs < rhs,
            tc.lt,
            "({:?} <  {:?}): expected {}",
            tc.lhs,
            tc.rhs,
            tc.lt
        );
        assert_eq!(
            lhs <= rhs,
            tc.le,
            "({:?} <= {:?}): expected {}",
            tc.lhs,
            tc.rhs,
            tc.le
        );
        assert_eq!(
            lhs > rhs,
            tc.gt,
            "({:?} >  {:?}): expected {}",
            tc.lhs,
            tc.rhs,
            tc.gt
        );
        assert_eq!(
            lhs >= rhs,
            tc.ge,
            "({:?} >= {:?}): expected {}",
            tc.lhs,
            tc.rhs,
            tc.ge
        );
    }
}

// -------------------------------------------------------------------------
// Antisymmetry: cmp(a, b) == Reverse(cmp(b, a))
// -------------------------------------------------------------------------

#[test]
fn antisymmetry() {
    for tc in BASIC_CASES {
        let a = make::<0, 32>(tc.lhs);
        let b = make::<0, 32>(tc.rhs);
        assert_eq!(
            a.cmp(&b),
            b.cmp(&a).reverse(),
            "antisymmetry violated for ({:?}, {:?})",
            tc.lhs,
            tc.rhs
        );
    }
}

// -------------------------------------------------------------------------
// Reflexivity: cmp(a, a) == Equal
// -------------------------------------------------------------------------

#[test]
fn reflexivity() {
    const CASES: &[&str] = &["", "a", "hello", "café", "日本語", "  spaces  "];

    for &s in CASES {
        let g = make::<0, 32>(s);
        assert_eq!(
            g.cmp(&g),
            Ordering::Equal,
            "reflexivity violated for {:?}",
            s
        );
    }
}

// -------------------------------------------------------------------------
// Transitivity: a <= b, b <= c  =>  a <= c
// -------------------------------------------------------------------------

#[test]
fn transitivity() {
    // "apple" < "banana" < "cherry"
    let a = make::<0, 32>("apple");
    let b = make::<0, 32>("banana");
    let c = make::<0, 32>("cherry");

    assert!(a < b, "apple < banana");
    assert!(b < c, "banana < cherry");
    assert!(a < c, "transitivity: apple < cherry");
}

// -------------------------------------------------------------------------
// Consistency with Ord: partial_cmp always returns Some and matches cmp
// -------------------------------------------------------------------------

#[test]
fn partial_cmp_consistent_with_cmp() {
    for tc in BASIC_CASES {
        let lhs = make::<0, 32>(tc.lhs);
        let rhs = make::<0, 32>(tc.rhs);
        assert_eq!(
            lhs.partial_cmp(&rhs),
            Some(lhs.cmp(&rhs)),
            "partial_cmp inconsistent with cmp for ({:?}, {:?})",
            tc.lhs,
            tc.rhs
        );
    }
}

// -------------------------------------------------------------------------
// ASCII_ONLY flag does not affect ordering of ASCII content
// -------------------------------------------------------------------------

#[test]
fn ascii_flag_does_not_affect_ascii_ordering() {
    const CASES: &[OrdOracle] = &[
        OrdOracle::new("a", "b", Ordering::Less),
        OrdOracle::new("b", "a", Ordering::Greater),
        OrdOracle::new("abc", "abc", Ordering::Equal),
        OrdOracle::new("hello", "world", Ordering::Less),
    ];

    for tc in CASES {
        let ascii = make_ascii::<0, 32>(tc.lhs);
        let utf8 = make::<0, 32>(tc.rhs);
        assert_eq!(
            ascii.partial_cmp(&utf8),
            Some(tc.want),
            "ASCII GString({:?}) vs UTF-8 GString({:?}): expected {:?}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}

// -------------------------------------------------------------------------
// Sort order: collect GStrings, sort, check ascending
// -------------------------------------------------------------------------

#[test]
fn sort_order() {
    let mut words: Vec<G<0, 32>> = vec![
        make("banana"),
        make("apple"),
        make("cherry"),
        make("date"),
        make(""),
    ];

    words.sort();

    let sorted: Vec<&str> = words.iter().map(|g| g.as_str()).collect();
    assert_eq!(sorted, vec!["", "apple", "banana", "cherry", "date"]);
}

// -------------------------------------------------------------------------
// min / max via Ord
// -------------------------------------------------------------------------

#[test]
fn min_max() {
    let a = make::<0, 32>("apple");
    let b = make::<0, 32>("banana");

    assert_eq!(a.clone().min(b.clone()).as_str(), "apple");
    assert_eq!(a.clone().max(b.clone()).as_str(), "banana");
}

// -------------------------------------------------------------------------
// Cross-type ordering matches pure &str ordering
// -------------------------------------------------------------------------

#[test]
fn cross_type_matches_str_ordering() {
    for tc in BASIC_CASES {
        let gstring = make::<0, 32>(tc.lhs);
        let expected = tc.lhs.cmp(tc.rhs);

        // GString vs &str
        assert_eq!(
            PartialOrd::<&str>::partial_cmp(&gstring, &tc.rhs),
            Some(expected),
            "GString vs &str: ({:?}, {:?})",
            tc.lhs,
            tc.rhs
        );

        // &str vs GString
        assert_eq!(
            tc.lhs.partial_cmp(&make::<0, 32>(tc.rhs)),
            Some(expected),
            "&str vs GString: ({:?}, {:?})",
            tc.lhs,
            tc.rhs
        );

        // GString vs String
        let owned = String::from(tc.rhs);
        assert_eq!(
            gstring.partial_cmp(&owned),
            Some(expected),
            "GString vs String: ({:?}, {:?})",
            tc.lhs,
            tc.rhs
        );

        // String vs GString
        let lhs_owned = String::from(tc.lhs);
        assert_eq!(
            lhs_owned.partial_cmp(&make::<0, 32>(tc.rhs)),
            Some(expected),
            "String vs GString: ({:?}, {:?})",
            tc.lhs,
            tc.rhs
        );
    }
}

// PartialOrd — &str vs GString  (reversed, explicit dispatch)
#[test]
fn partial_cmp_ref_str_ref_gstring() {
    for tc in BASIC_CASES {
        let rhs = make::<0, 32>(tc.rhs);
        let lhs: &str = tc.lhs;
        // Must call via a &&str receiver to hit the
        // `PartialOrd<GString> for &str` impl, not &str's own PartialOrd.
        assert_eq!(
            PartialOrd::partial_cmp(&lhs, &rhs),
            Some(tc.want),
            "partial_cmp &str({:?}) vs GString({:?}): expected {:?}",
            tc.lhs,
            tc.rhs,
            tc.want
        );
    }
}
