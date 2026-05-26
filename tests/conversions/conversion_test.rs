use std::borrow::Cow;
use std::str::FromStr;

use g_string::{GString, GStringError, NoValidation};

// -------------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------------

type G<const MIN: usize, const MAX: usize> = GString<NoValidation, MIN, MAX, false>;

struct Case {
    input: &'static str,
    oracle: Option<&'static str>, // None => expect error
}

impl Case {
    fn ok(input: &'static str) -> Self {
        Self {
            input,
            oracle: Some(input),
        }
    }
    fn err(input: &'static str) -> Self {
        Self {
            input,
            oracle: None,
        }
    }
}

// -------------------------------------------------------------------------
// FromStr  (&str -> GString via str::parse / GString::from_str)
// -------------------------------------------------------------------------

#[test]
fn from_str_ok_and_err() {
    let cases: &[Case] = &[
        Case::ok("hello"),
        Case::ok(""),
        Case::ok("rust"),
        Case::ok("a"),
        Case::err("this string is way too long for a max-4 gstring"),
    ];

    for case in cases {
        let result = G::<0, 5>::from_str(case.input);

        match case.oracle {
            Some(expected) => {
                let g = result.expect("expected Ok");
                assert_eq!(g.as_str(), expected);
                // oracle: same as std String round-trip
                assert_eq!(g.as_str(), case.input.to_string());
            }
            None => {
                assert!(result.is_err(), "expected Err for {:?}", case.input);
            }
        }
    }
}

#[test]
fn from_str_min_bound() {
    let cases: &[(&str, bool /* ok? */)] =
        &[("", false), ("a", false), ("ab", true), ("abc", true)];

    for (input, should_ok) in cases {
        let result = G::<2, 8>::from_str(input);
        assert_eq!(result.is_ok(), *should_ok, "input={:?}", input);
        if let Ok(g) = result {
            assert_eq!(g.as_str(), input.to_string().as_str());
        }
    }
}

// -------------------------------------------------------------------------
// AsRef<str>  (GString -> &str)
// -------------------------------------------------------------------------

#[test]
fn as_ref_str() {
    let cases: &[&str] = &["", "hello", "world", "rust 🦀"];

    for &input in cases {
        let g: GString = GString::try_new(input).unwrap();
        let r: &str = g.as_ref();
        // oracle: AsRef<str> must yield the same content as the source str
        assert_eq!(r, input.to_string().as_str());
    }
}

// -------------------------------------------------------------------------
// TryFrom<String>  (String -> GString)
// -------------------------------------------------------------------------

#[test]
fn try_from_string() {
    let cases: &[Case] = &[
        Case::ok(""),
        Case::ok("hello"),
        Case::ok("γεια"),
        Case::err("overflow this max-8 buffer with a long string!!"),
    ];

    for case in cases {
        let owned = case.input.to_string();
        let result = G::<0, 8>::try_from(owned);

        match case.oracle {
            Some(expected) => {
                let g = result.expect("expected Ok");
                assert_eq!(g.as_str(), expected);
                assert_eq!(g.as_str(), case.input.to_string());
            }
            None => {
                assert!(result.is_err(), "expected Err for {:?}", case.input);
            }
        }
    }
}

// -------------------------------------------------------------------------
// From<GString> for String  (GString -> String)
// -------------------------------------------------------------------------

#[test]
fn into_string() {
    let cases: &[&str] = &["", "hello", "world", "γεια 🦀"];

    for &input in cases {
        let g: GString = GString::try_new(input).unwrap();
        let s = String::from(g);
        // oracle: converting to String then back must be identity
        assert_eq!(s, input.to_string());
    }
}

// -------------------------------------------------------------------------
// TryFrom<&str>  (&str -> GString)
// -------------------------------------------------------------------------

#[test]
fn try_from_str_ref() {
    let cases: &[Case] = &[
        Case::ok(""),
        Case::ok("hi"),
        Case::ok("exactly8"),
        Case::err("nine_char"),
    ];

    for case in cases {
        let result = G::<0, 8>::try_from(case.input);

        match case.oracle {
            Some(expected) => {
                let g = result.expect("expected Ok");
                assert_eq!(g.as_str(), expected);
                assert_eq!(g.as_str(), case.input.to_string());
            }
            None => {
                assert!(result.is_err(), "expected Err for {:?}", case.input);
            }
        }
    }
}

// -------------------------------------------------------------------------
// AsRef<[u8]>  (GString -> &[u8])
// -------------------------------------------------------------------------

#[test]
fn as_ref_bytes() {
    let cases: &[&str] = &["", "hello", "rust", "γεια"];

    for &input in cases {
        let g: GString = GString::try_new(input).unwrap();
        let bytes: &[u8] = g.as_ref();
        // oracle: AsRef<[u8]> must match the raw bytes of the source str
        assert_eq!(bytes, input.to_string().as_bytes());
    }
}

// -------------------------------------------------------------------------
// TryFrom<Cow<str>>  (Cow<str> -> GString)
// -------------------------------------------------------------------------

#[test]
fn try_from_cow_borrowed() {
    let cases: &[Case] = &[
        Case::ok(""),
        Case::ok("hello"),
        Case::ok("exactly"),
        Case::err("way_too_long_for_max8"),
    ];

    for case in cases {
        let cow: Cow<str> = Cow::Borrowed(case.input);
        let result = G::<0, 8>::try_from(cow);

        match case.oracle {
            Some(expected) => {
                let g = result.expect("expected Ok");
                assert_eq!(g.as_str(), expected);
                assert_eq!(g.as_str(), case.input.to_string());
            }
            None => {
                assert!(result.is_err(), "expected Err for {:?}", case.input);
            }
        }
    }
}

#[test]
fn try_from_cow_owned() {
    let cases: &[Case] = &[
        Case::ok(""),
        Case::ok("hello"),
        Case::err("way_too_long_for_max8"),
    ];

    for case in cases {
        let cow: Cow<str> = Cow::Owned(case.input.to_string());
        let result = G::<0, 8>::try_from(cow);

        match case.oracle {
            Some(expected) => {
                let g = result.expect("expected Ok");
                assert_eq!(g.as_str(), expected);
                assert_eq!(g.as_str(), case.input.to_string());
            }
            None => {
                assert!(result.is_err(), "expected Err for {:?}", case.input);
            }
        }
    }
}

// -------------------------------------------------------------------------
// Error variant correctness
// -------------------------------------------------------------------------

#[test]
fn error_variants() {
    assert!(matches!(
        G::<0, 4>::from_str("toolong"),
        Err(GStringError::TooLong(4))
    ));
    assert!(matches!(
        G::<3, 8>::from_str("ab"),
        Err(GStringError::TooShort(3))
    ));
    assert!(matches!(
        GString::<NoValidation, 0, 8, true>::from_str("héllo"),
        Err(GStringError::NotAscii)
    ));
}
