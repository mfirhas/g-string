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

// ---------------------------------------------------------------------------
// 10. count() — Unicode scalar values, oracle: String::chars().count()
// ---------------------------------------------------------------------------

struct CountCase {
    label: &'static str,
    input: &'static str,
    /// oracle: input.chars().count() via String
    expected_count: usize,
}

#[test]
fn count_matches_string_chars_count() {
    type G = GString<NoValidation, 0, 255, false>;

    let cases: &[CountCase] = &[
        CountCase {
            label: "empty",
            input: "",
            expected_count: 0,
        },
        CountCase {
            label: "pure ASCII — each byte is one char",
            input: "hello",
            expected_count: 5,
        },
        CountCase {
            label: "2-byte chars — é is one char, two bytes",
            input: "héllo",
            expected_count: 5, // same char count as "hello"
        },
        CountCase {
            label: "3-byte CJK — each hiragana is one char",
            input: "こんにちは",
            expected_count: 5,
        },
        CountCase {
            label: "4-byte emoji — one scalar value",
            input: "🦀",
            expected_count: 1,
        },
        CountCase {
            label: "mixed widths",
            input: "a🦀é",
            expected_count: 3,
        },
        CountCase {
            label: "count != len for multibyte",
            input: "hi 🌍",
            // "hi " = 3 bytes/chars, 🌍 = 4 bytes but 1 char → total 4 chars
            expected_count: 4,
        },
    ];

    for case in cases {
        let g = G::try_new(case.input)
            .unwrap_or_else(|e| panic!("[{}] try_new failed: {:?}", case.label, e));

        // Oracle
        let oracle_count = String::from(case.input).chars().count();

        assert_eq!(
            g.count(),
            oracle_count,
            "[{}] count() vs String oracle mismatch",
            case.label
        );
        assert_eq!(
            g.count(),
            case.expected_count,
            "[{}] count() mismatch",
            case.label
        );
    }
}

#[test]
fn count_differs_from_len_for_multibyte() {
    type G = GString<NoValidation, 0, 255, false>;

    // Verify the key property: for multibyte strings, count() != len()
    let cases: &[(&str, bool)] = &[
        ("hello", false),     // pure ASCII: count == len
        ("héllo", true),      // 'é' is 2 bytes: count < len
        ("こんにちは", true), // 3 bytes each: count < len
        ("🦀", true),         // 4 bytes: count < len
        ("", false),          // empty: both zero
    ];

    for (input, should_differ) in cases {
        let g = G::try_new(input).unwrap();
        let oracle = String::from(*input);

        let count_ne_len = g.count() != g.len();
        assert_eq!(
            count_ne_len, *should_differ,
            "count() != len() should be {} for {:?}",
            should_differ, input
        );

        // Always agree with String
        assert_eq!(
            g.count(),
            oracle.chars().count(),
            "oracle mismatch for {:?}",
            input
        );
    }
}

// ---------------------------------------------------------------------------
// 11. capacity() — always equals MAX const param
// ---------------------------------------------------------------------------

#[test]
fn capacity_equals_max_const() {
    // capacity() is purely a function of the MAX const — independent of content

    struct CapCase {
        label: &'static str,
        capacity: usize,
    }

    let cases: &[CapCase] = &[
        CapCase {
            label: "MAX=8",
            capacity: 8,
        },
        CapCase {
            label: "MAX=16",
            capacity: 16,
        },
        CapCase {
            label: "MAX=64",
            capacity: 64,
        },
        CapCase {
            label: "MAX=255",
            capacity: 255,
        },
    ];

    // We verify by constructing with different MAX values and asserting capacity()
    // Each branch uses a distinct type alias.
    {
        let _ = cases; // suppress unused warning; actual checks are per-type below
    }

    let g8 = GString::<NoValidation, 0, 8, false>::try_new("hi").unwrap();
    let g16 = GString::<NoValidation, 0, 16, false>::try_new("hi").unwrap();
    let g64 = GString::<NoValidation, 0, 64, false>::try_new("hi").unwrap();
    let g255 = GString::<NoValidation, 0, 255, false>::try_new("hi").unwrap();

    assert_eq!(g8.capacity(), 8, "MAX=8 capacity mismatch");
    assert_eq!(g16.capacity(), 16, "MAX=16 capacity mismatch");
    assert_eq!(g64.capacity(), 64, "MAX=64 capacity mismatch");
    assert_eq!(g255.capacity(), 255, "MAX=255 capacity mismatch");
}

#[test]
fn capacity_independent_of_content() {
    // Same MAX, different content lengths — capacity() must not change
    type G = GString<NoValidation, 0, 32, false>;

    let inputs = ["", "a", "hello", "🦀🦀🦀"];

    for input in inputs {
        let g = G::try_new(input).unwrap();
        assert_eq!(g.capacity(), 32, "capacity() changed for input {:?}", input);
    }
}

// ---------------------------------------------------------------------------
// 12. is_full() — true iff len == MAX
// ---------------------------------------------------------------------------

struct IsFullCase {
    label: &'static str,
    input: &'static str,
    expected: bool,
}

#[test]
fn is_full_when_len_equals_max() {
    type G = GString<NoValidation, 0, 5, false>;

    let cases: &[IsFullCase] = &[
        IsFullCase {
            label: "empty — not full",
            input: "",
            expected: false,
        },
        IsFullCase {
            label: "partial — not full",
            input: "ab",
            expected: false,
        },
        IsFullCase {
            label: "one below MAX — not full",
            input: "abcd",
            expected: false,
        },
        IsFullCase {
            label: "exactly MAX — full",
            input: "abcde",
            expected: true,
        },
    ];

    for case in cases {
        let g = G::try_new(case.input)
            .unwrap_or_else(|e| panic!("[{}] try_new failed: {:?}", case.label, e));

        assert_eq!(
            g.is_full(),
            case.expected,
            "[{}] is_full() mismatch (len={}, capacity={})",
            case.label,
            g.len(),
            g.capacity()
        );

        // Invariant: is_full() iff len() == capacity()
        assert_eq!(
            g.is_full(),
            g.len() == g.capacity(),
            "[{}] is_full() inconsistent with len() == capacity()",
            case.label
        );
    }
}

#[test]
fn is_full_with_multibyte_chars() {
    // MAX is in bytes — a string of multibyte chars can fill the buffer
    // "🦀" = 4 bytes, so MAX=4 is full with one emoji
    type G = GString<NoValidation, 0, 4, false>;

    let g = G::try_new("🦀").unwrap();
    assert!(
        g.is_full(),
        "single emoji filling MAX=4 bytes should be full"
    );
    assert_eq!(g.count(), 1, "one char");
    assert_eq!(g.len(), 4, "four bytes");
    assert_eq!(g.capacity(), 4);

    // Contrast: an ASCII string of 3 bytes in the same type is not full
    let g2 = G::try_new("abc").unwrap();
    assert!(!g2.is_full());
}
