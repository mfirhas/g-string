// tests/try_new.rs

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
    input: &'static str,
    /// The oracle: what `String` gives us back
    expected_str: &'static str,
    expected_len: usize,
}

struct ErrCase<VE> {
    label: &'static str,
    input: &'static str,
    expected_err: GStringError<VE>,
}

// ---------------------------------------------------------------------------
// 1. Basic construction — no constraints (MIN=0, MAX=255, ASCII_ONLY=false)
// ---------------------------------------------------------------------------

#[test]
fn try_new_ok_no_constraints() {
    type G = GString<NoValidation, 0, 255, false>;

    let cases: &[OkCase] = &[
        OkCase {
            label: "empty string",
            input: "",
            expected_str: "",
            expected_len: 0,
        },
        OkCase {
            label: "ascii only",
            input: "hello",
            expected_str: "hello",
            expected_len: 5,
        },
        OkCase {
            label: "unicode — 2-byte chars",
            input: "héllo",
            expected_str: "héllo",
            expected_len: 6, // 'é' is 2 bytes
        },
        OkCase {
            label: "unicode — 3-byte chars",
            input: "こんにちは",
            expected_str: "こんにちは",
            expected_len: 15, // each hiragana is 3 bytes
        },
        OkCase {
            label: "unicode — 4-byte emoji",
            input: "hi 🦀",
            expected_str: "hi 🦀",
            expected_len: 7, // "hi " = 3 bytes, 🦀 = 4 bytes
        },
        OkCase {
            label: "whitespace",
            input: "   ",
            expected_str: "   ",
            expected_len: 3,
        },
        OkCase {
            label: "digits and symbols",
            input: "abc123!@#",
            expected_str: "abc123!@#",
            expected_len: 9,
        },
        OkCase {
            label: "exactly MAX bytes (255 'a's)",
            input: {
                // Build a &'static str of 255 'a's via a const trick
                const S: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
                S
            },
            expected_str: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            expected_len: 255,
        },
    ];

    for case in cases {
        let result = G::try_new(case.input);
        assert!(
            result.is_ok(),
            "[{}] expected Ok, got {:?}",
            case.label,
            result
        );
        let g = result.unwrap();

        // Oracle: compare against String
        let oracle = String::from(case.expected_str);
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
fn try_new_err_too_long() {
    type G = GString<NoValidation, 0, 5, false>;

    let cases: &[ErrCase<Infallible>] = &[
        ErrCase {
            label: "one byte over MAX",
            input: "abcdef", // 6 bytes > MAX=5
            expected_err: GStringError::TooLong(5),
        },
        ErrCase {
            label: "way over MAX",
            input: "this is way too long",
            expected_err: GStringError::TooLong(5),
        },
    ];

    for case in cases {
        let result = G::try_new(case.input);
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
fn try_new_err_too_short() {
    type G = GString<NoValidation, 3, 10, false>;

    let cases: &[ErrCase<Infallible>] = &[
        ErrCase {
            label: "empty string below MIN",
            input: "",
            expected_err: GStringError::TooShort(3),
        },
        ErrCase {
            label: "one byte below MIN",
            input: "ab", // 2 < MIN=3
            expected_err: GStringError::TooShort(3),
        },
    ];

    for case in cases {
        let result = G::try_new(case.input);
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
fn try_new_ok_boundary_values() {
    type G = GString<NoValidation, 3, 8, false>;

    let cases: &[OkCase] = &[
        OkCase {
            label: "exactly MIN (3 bytes)",
            input: "abc",
            expected_str: "abc",
            expected_len: 3,
        },
        OkCase {
            label: "exactly MAX (8 bytes)",
            input: "abcdefgh",
            expected_str: "abcdefgh",
            expected_len: 8,
        },
        OkCase {
            label: "between MIN and MAX",
            input: "abcde",
            expected_str: "abcde",
            expected_len: 5,
        },
    ];

    for case in cases {
        let result = G::try_new(case.input);
        assert!(
            result.is_ok(),
            "[{}] expected Ok, got {:?}",
            case.label,
            result
        );
        let g = result.unwrap();
        let oracle = String::from(case.expected_str);
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
fn try_new_ok_ascii_only() {
    type G = GString<NoValidation, 0, 64, true>;

    let cases: &[OkCase] = &[
        OkCase {
            label: "empty",
            input: "",
            expected_str: "",
            expected_len: 0,
        },
        OkCase {
            label: "printable ascii",
            input: "Hello, World!",
            expected_str: "Hello, World!",
            expected_len: 13,
        },
        OkCase {
            label: "digits",
            input: "0123456789",
            expected_str: "0123456789",
            expected_len: 10,
        },
        OkCase {
            label: "all printable ASCII symbols",
            input: "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
            expected_str: "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
            expected_len: 32,
        },
    ];

    for case in cases {
        let result = G::try_new(case.input);
        assert!(
            result.is_ok(),
            "[{}] expected Ok, got {:?}",
            case.label,
            result
        );
        let g = result.unwrap();
        let oracle = String::from(case.expected_str);
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
fn try_new_err_not_ascii() {
    type G = GString<NoValidation, 0, 64, true>;

    let cases: &[ErrCase<Infallible>] = &[
        ErrCase {
            label: "latin extended (é)",
            input: "café",
            expected_err: GStringError::NotAscii,
        },
        ErrCase {
            label: "CJK characters",
            input: "こんにちは",
            expected_err: GStringError::NotAscii,
        },
        ErrCase {
            label: "emoji",
            input: "🦀",
            expected_err: GStringError::NotAscii,
        },
        ErrCase {
            label: "mixed ascii and non-ascii",
            input: "hello 世界",
            expected_err: GStringError::NotAscii,
        },
    ];

    for case in cases {
        let result = G::try_new(case.input);
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
fn try_new_err_custom_validator() {
    type G = GString<NoDigits, 0, 64, false>;

    let cases: &[ErrCase<NoDigitsError>] = &[
        ErrCase {
            label: "string with digit",
            input: "hello1",
            expected_err: GStringError::Validation(NoDigitsError),
        },
        ErrCase {
            label: "all digits",
            input: "12345",
            expected_err: GStringError::Validation(NoDigitsError),
        },
        ErrCase {
            label: "digit at the end",
            input: "abc9",
            expected_err: GStringError::Validation(NoDigitsError),
        },
    ];

    for case in cases {
        let result = G::try_new(case.input);
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
fn try_new_ok_custom_validator() {
    type G = GString<NoDigits, 0, 64, false>;

    let cases: &[OkCase] = &[
        OkCase {
            label: "no digits",
            input: "hello world",
            expected_str: "hello world",
            expected_len: 11,
        },
        OkCase {
            label: "empty string",
            input: "",
            expected_str: "",
            expected_len: 0,
        },
        OkCase {
            label: "unicode without digits",
            input: "こんにちは",
            expected_str: "こんにちは",
            expected_len: 15,
        },
    ];

    for case in cases {
        let result = G::try_new(case.input);
        assert!(
            result.is_ok(),
            "[{}] expected Ok, got {:?}",
            case.label,
            result
        );
        let g = result.unwrap();
        let oracle = String::from(case.expected_str);
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
// 8. Validator errors take priority after bounds/ascii checks pass
// ---------------------------------------------------------------------------

#[test]
fn try_new_bounds_checked_before_validator() {
    // If the string is too long, we should get TooLong, not Validation
    type G = GString<NoDigits, 0, 3, false>;

    // "12" — has digits AND within bounds — should get Validation error
    let result = G::try_new("12");
    assert_eq!(result.unwrap_err(), GStringError::Validation(NoDigitsError));

    // "1234" — has digits AND too long — should get TooLong (bounds checked first)
    let result = G::try_new("1234");
    assert_eq!(result.unwrap_err(), GStringError::TooLong(3));
}

// ---------------------------------------------------------------------------
// 9. len() and as_str() agree with String oracle across types
// ---------------------------------------------------------------------------

#[test]
fn try_new_str_and_len_match_string_oracle() {
    type G = GString<NoValidation, 0, 255, false>;

    let inputs = ["", "a", "hello", "🦀", "こんにちは", "mixed: héllo 🌍"];

    for input in inputs {
        let oracle = String::from(input);
        let g =
            G::try_new(input).unwrap_or_else(|e| panic!("try_new({:?}) failed: {:?}", input, e));

        assert_eq!(
            g.as_str(),
            oracle.as_str(),
            "as_str mismatch for {:?}",
            input
        );
        assert_eq!(g.len(), oracle.len(), "len mismatch for {:?}", input);
        assert_eq!(
            g.is_empty(),
            oracle.is_empty(),
            "is_empty mismatch for {:?}",
            input
        );
    }
}
