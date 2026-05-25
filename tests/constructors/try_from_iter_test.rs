use g_string::{GString, GStringError, NoValidation, Validator};
use std::convert::Infallible;

// ---------------------------------------------------------------------------
// Helper validator: rejects strings containing digits
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NoDigits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NoDigitsError;

impl std::fmt::Display for NoDigitsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "string must not contain digits")
    }
}

impl std::error::Error for NoDigitsError {}

impl Validator for NoDigits {
    type Err = NoDigitsError;

    fn validate(s: impl AsRef<str>) -> Result<(), Self::Err> {
        if s.as_ref().chars().any(|c| c.is_ascii_digit()) {
            Err(NoDigitsError)
        } else {
            Ok(())
        }
    }
}

// ---------------------------------------------------------------------------
// Test case types
// ---------------------------------------------------------------------------

struct OkCase {
    label: &'static str,
    /// Items to feed into try_from_iter
    items: &'static [&'static str],
    /// Oracle: what the concatenated String gives us back
    expected_str: &'static str,
    expected_len: usize,
}

struct ErrCase<VE> {
    label: &'static str,
    items: &'static [&'static str],
    expected_err: GStringError<VE>,
}

// ---------------------------------------------------------------------------
// 1. Basic concatenation — no constraints (MIN=0, MAX=255, ASCII_ONLY=false)
// ---------------------------------------------------------------------------

#[test]
fn try_from_iter_ok_no_constraints() {
    type G = GString<NoValidation, 0, 255, false>;

    let cases: &[OkCase] = &[
        OkCase {
            label: "empty iterator",
            items: &[],
            expected_str: "",
            expected_len: 0,
        },
        OkCase {
            label: "empty strings only",
            items: &["", "", ""],
            expected_str: "",
            expected_len: 0,
        },
        OkCase {
            label: "single item",
            items: &["hello"],
            expected_str: "hello",
            expected_len: 5,
        },
        OkCase {
            label: "multiple ascii items",
            items: &["foo", "bar", "baz"],
            expected_str: "foobarbaz",
            expected_len: 9,
        },
        OkCase {
            label: "single char items",
            items: &["a", "b", "c", "d"],
            expected_str: "abcd",
            expected_len: 4,
        },
        OkCase {
            label: "unicode items — 2-byte chars",
            items: &["hé", "llo"],
            expected_str: "héllo",
            expected_len: 6, // 'é' is 2 bytes
        },
        OkCase {
            label: "unicode items — 3-byte chars",
            items: &["こん", "にちは"],
            expected_str: "こんにちは",
            expected_len: 15, // each hiragana is 3 bytes
        },
        OkCase {
            label: "unicode items — 4-byte emoji",
            items: &["hi ", "🦀"],
            expected_str: "hi 🦀",
            expected_len: 7, // "hi " = 3 bytes, 🦀 = 4 bytes
        },
        OkCase {
            label: "mixed empty and non-empty items",
            items: &["", "hello", "", " ", "world", ""],
            expected_str: "hello world",
            expected_len: 11,
        },
        OkCase {
            label: "whitespace items",
            items: &["  ", " "],
            expected_str: "   ",
            expected_len: 3,
        },
    ];

    for case in cases {
        let result = G::try_from_iter(case.items.iter().copied());
        assert!(
            result.is_ok(),
            "[{}] expected Ok, got {:?}",
            case.label,
            result
        );
        let g = result.unwrap();

        assert_eq!(
            g.as_str(),
            case.expected_str,
            "[{}] expected mismatch",
            case.label
        );

        // Oracle: compare against String built by manual concatenation
        let oracle: String = case.items.iter().copied().collect();
        assert_eq!(
            g.as_str(),
            oracle.as_str(),
            "[{}] content mismatch",
            case.label
        );
        assert_eq!(g.len(), case.expected_len, "[{}] len mismatch", case.label);
        assert_eq!(
            g.len(),
            oracle.len(),
            "[{}] len vs String oracle mismatch",
            case.label
        );
    }
}

// ---------------------------------------------------------------------------
// 2. TooLong errors
// ---------------------------------------------------------------------------

#[test]
fn try_from_iter_err_too_long() {
    type G = GString<NoValidation, 0, 5, false>;

    let cases: &[ErrCase<Infallible>] = &[
        ErrCase {
            label: "single item one byte over MAX",
            items: &["abcdef"], // 6 bytes > MAX=5
            expected_err: GStringError::TooLong(5),
        },
        ErrCase {
            label: "accumulated across items — each fits, combined does not",
            items: &["abc", "def"], // 3+3=6 > MAX=5
            expected_err: GStringError::TooLong(5),
        },
        ErrCase {
            label: "accumulated in three small items",
            items: &["ab", "ab", "ab"], // 2+2+2=6 > MAX=5
            expected_err: GStringError::TooLong(5),
        },
        ErrCase {
            label: "way over MAX",
            items: &["this is", " way too", " long"],
            expected_err: GStringError::TooLong(5),
        },
    ];

    for case in cases {
        let result = G::try_from_iter(case.items.iter().copied());
        assert!(result.is_err(), "[{}] expected Err, got Ok", case.label);
        assert_eq!(
            result.unwrap_err(),
            case.expected_err,
            "[{}] wrong error variant",
            case.label
        );
    }
}

// ---------------------------------------------------------------------------
// 3. TooShort errors
// ---------------------------------------------------------------------------

#[test]
fn try_from_iter_err_too_short() {
    type G = GString<NoValidation, 4, 16, false>;

    let cases: &[ErrCase<Infallible>] = &[
        ErrCase {
            label: "empty iterator",
            items: &[],
            expected_err: GStringError::TooShort(4),
        },
        ErrCase {
            label: "empty strings only",
            items: &["", ""],
            expected_err: GStringError::TooShort(4),
        },
        ErrCase {
            label: "one byte below MIN",
            items: &["ab", "c"], // 3 < MIN=4
            expected_err: GStringError::TooShort(4),
        },
    ];

    for case in cases {
        let result = G::try_from_iter(case.items.iter().copied());
        assert!(result.is_err(), "[{}] expected Err, got Ok", case.label);
        assert_eq!(
            result.unwrap_err(),
            case.expected_err,
            "[{}] wrong error variant",
            case.label
        );
    }
}

// ---------------------------------------------------------------------------
// 4. Exact boundary values (MIN and MAX edges)
// ---------------------------------------------------------------------------

#[test]
fn try_from_iter_ok_boundary_values() {
    type G = GString<NoValidation, 3, 8, false>;

    let cases: &[OkCase] = &[
        OkCase {
            label: "exactly MIN split across two items",
            items: &["ab", "c"],
            expected_str: "abc",
            expected_len: 3,
        },
        OkCase {
            label: "exactly MAX split across two items",
            items: &["abcd", "efgh"],
            expected_str: "abcdefgh",
            expected_len: 8,
        },
        OkCase {
            label: "between MIN and MAX",
            items: &["ab", "cde"],
            expected_str: "abcde",
            expected_len: 5,
        },
        OkCase {
            label: "single item exactly at MIN",
            items: &["abc"],
            expected_str: "abc",
            expected_len: 3,
        },
        OkCase {
            label: "single item exactly at MAX",
            items: &["abcdefgh"],
            expected_str: "abcdefgh",
            expected_len: 8,
        },
    ];

    for case in cases {
        let result = G::try_from_iter(case.items.iter().copied());
        assert!(
            result.is_ok(),
            "[{}] expected Ok, got {:?}",
            case.label,
            result
        );
        let g = result.unwrap();
        let oracle: String = case.items.iter().copied().collect();
        assert_eq!(
            g.as_str(),
            oracle.as_str(),
            "[{}] content mismatch",
            case.label
        );
        assert_eq!(g.len(), oracle.len(), "[{}] len mismatch", case.label);
    }
}

// ---------------------------------------------------------------------------
// 5. ASCII_ONLY=true — valid ASCII inputs
// ---------------------------------------------------------------------------

#[test]
fn try_from_iter_ok_ascii_only() {
    type G = GString<NoValidation, 0, 64, true>;

    let cases: &[OkCase] = &[
        OkCase {
            label: "empty iterator",
            items: &[],
            expected_str: "",
            expected_len: 0,
        },
        OkCase {
            label: "printable ascii items",
            items: &["Hello", ", ", "World!"],
            expected_str: "Hello, World!",
            expected_len: 13,
        },
        OkCase {
            label: "digits split across items",
            items: &["012", "345", "6789"],
            expected_str: "0123456789",
            expected_len: 10,
        },
        OkCase {
            label: "symbols",
            items: &["!@#", "$%^", "&*"],
            expected_str: "!@#$%^&*",
            expected_len: 8,
        },
    ];

    for case in cases {
        let result = G::try_from_iter(case.items.iter().copied());
        assert!(
            result.is_ok(),
            "[{}] expected Ok, got {:?}",
            case.label,
            result
        );
        let g = result.unwrap();
        let oracle: String = case.items.iter().copied().collect();
        assert_eq!(
            g.as_str(),
            oracle.as_str(),
            "[{}] content mismatch",
            case.label
        );
        assert_eq!(g.len(), oracle.len(), "[{}] len mismatch", case.label);
    }
}

// ---------------------------------------------------------------------------
// 6. ASCII_ONLY=true — non-ASCII inputs rejected
// ---------------------------------------------------------------------------

#[test]
fn try_from_iter_err_not_ascii() {
    type G = GString<NoValidation, 0, 64, true>;

    let cases: &[ErrCase<Infallible>] = &[
        ErrCase {
            label: "latin extended in single item",
            items: &["café"],
            expected_err: GStringError::NotAscii,
        },
        ErrCase {
            label: "non-ascii in second item",
            items: &["hello", " こんにちは"],
            expected_err: GStringError::NotAscii,
        },
        ErrCase {
            label: "emoji in single item",
            items: &["🦀"],
            expected_err: GStringError::NotAscii,
        },
        ErrCase {
            label: "ascii first item, non-ascii second",
            items: &["hello ", "世界"],
            expected_err: GStringError::NotAscii,
        },
    ];

    for case in cases {
        let result = G::try_from_iter(case.items.iter().copied());
        assert!(result.is_err(), "[{}] expected Err, got Ok", case.label);
        assert_eq!(
            result.unwrap_err(),
            case.expected_err,
            "[{}] wrong error variant",
            case.label
        );
    }
}

// ---------------------------------------------------------------------------
// 7. Custom validator — Validation error propagation
// ---------------------------------------------------------------------------

#[test]
fn try_from_iter_err_custom_validator() {
    type G = GString<NoDigits, 0, 64, false>;

    let cases: &[ErrCase<NoDigitsError>] = &[
        ErrCase {
            label: "digit in single item",
            items: &["hello1"],
            expected_err: GStringError::Validation(NoDigitsError),
        },
        ErrCase {
            label: "digit introduced in second item",
            items: &["hello", "world2"],
            expected_err: GStringError::Validation(NoDigitsError),
        },
        ErrCase {
            label: "digit only item",
            items: &["abc", "9", "def"],
            expected_err: GStringError::Validation(NoDigitsError),
        },
        ErrCase {
            label: "all-digit item",
            items: &["12345"],
            expected_err: GStringError::Validation(NoDigitsError),
        },
    ];

    for case in cases {
        let result = G::try_from_iter(case.items.iter().copied());
        assert!(result.is_err(), "[{}] expected Err, got Ok", case.label);
        assert_eq!(
            result.unwrap_err(),
            case.expected_err,
            "[{}] wrong error variant",
            case.label
        );
    }
}

#[test]
fn try_from_iter_ok_custom_validator() {
    type G = GString<NoDigits, 0, 64, false>;

    let cases: &[OkCase] = &[
        OkCase {
            label: "no digits across items",
            items: &["hello", " ", "world"],
            expected_str: "hello world",
            expected_len: 11,
        },
        OkCase {
            label: "empty iterator",
            items: &[],
            expected_str: "",
            expected_len: 0,
        },
        OkCase {
            label: "unicode without digits",
            items: &["こん", "にちは"],
            expected_str: "こんにちは",
            expected_len: 15,
        },
    ];

    for case in cases {
        let result = G::try_from_iter(case.items.iter().copied());
        assert!(
            result.is_ok(),
            "[{}] expected Ok, got {:?}",
            case.label,
            result
        );
        let g = result.unwrap();
        let oracle: String = case.items.iter().copied().collect();
        assert_eq!(
            g.as_str(),
            oracle.as_str(),
            "[{}] content mismatch",
            case.label
        );
        assert_eq!(g.len(), oracle.len(), "[{}] len mismatch", case.label);
    }
}

// ---------------------------------------------------------------------------
// 8. Bounds checked before validator
// ---------------------------------------------------------------------------

#[test]
fn try_from_iter_bounds_checked_before_validator() {
    type G = GString<NoDigits, 0, 3, false>;

    // "12" — has digits AND within bounds — should get Validation error
    let result = G::try_from_iter(["12"]);
    assert_eq!(result.unwrap_err(), GStringError::Validation(NoDigitsError));

    // "1234" — has digits AND too long — should get TooLong (bounds checked first)
    let result = G::try_from_iter(["1234"]);
    assert_eq!(result.unwrap_err(), GStringError::TooLong(3));

    // Split across items: "12" + "34" = "1234" — still too long
    let result = G::try_from_iter(["12", "34"]);
    assert_eq!(result.unwrap_err(), GStringError::TooLong(3));
}

// ---------------------------------------------------------------------------
// 9. as_str() and len() agree with String oracle across item counts
// ---------------------------------------------------------------------------

#[test]
fn try_from_iter_str_and_len_match_string_oracle() {
    type G = GString<NoValidation, 0, 255, false>;

    // (items, expected concatenation)
    let cases: &[(&[&str], &str)] = &[
        (&[], ""),
        (&["a"], "a"),
        (&["hello", " ", "world"], "hello world"),
        (&["🦀", "🦀", "🦀"], "🦀🦀🦀"),
        (&["こん", "にちは"], "こんにちは"),
        (&["mixed: ", "héllo ", "🌍"], "mixed: héllo 🌍"),
    ];

    for (items, expected) in cases {
        let oracle = String::from(*expected);
        let g = G::try_from_iter(items.iter().copied())
            .unwrap_or_else(|e| panic!("try_from_iter({:?}) failed: {:?}", items, e));

        assert_eq!(
            g.as_str(),
            oracle.as_str(),
            "as_str mismatch for items {:?}",
            items
        );
        assert_eq!(g.len(), oracle.len(), "len mismatch for items {:?}", items);
        assert_eq!(
            g.is_empty(),
            oracle.is_empty(),
            "is_empty mismatch for items {:?}",
            items
        );
    }
}

// ---------------------------------------------------------------------------
// 10. Owned String items — accepts any S: AsRef<str>
// ---------------------------------------------------------------------------

#[test]
fn try_from_iter_accepts_owned_strings() {
    type G = GString<NoValidation, 0, 64, false>;

    let items = vec![
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
    ];
    let oracle: String = items.iter().map(String::as_str).collect();

    let result = G::try_from_iter(items);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str(), oracle.as_str());
}

#[test]
fn try_from_iter_accepts_cow_str() {
    use std::borrow::Cow;
    type G = GString<NoValidation, 0, 64, false>;

    let items: Vec<Cow<str>> = vec![
        Cow::Borrowed("foo"),
        Cow::Owned(String::from("bar")),
        Cow::Borrowed("baz"),
    ];
    let oracle = String::from("foobarbaz");

    let result = G::try_from_iter(items);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str(), oracle.as_str());
}
