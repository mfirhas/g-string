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
    /// Characters to feed into try_from_chars
    chars: &'static [char],
    /// Oracle: what String::from_iter(chars) gives us back
    expected_str: &'static str,
    /// Expected byte length (not char count)
    expected_len: usize,
}

struct ErrCase<VE> {
    label: &'static str,
    chars: &'static [char],
    expected_err: GStringError<VE>,
}

// ---------------------------------------------------------------------------
// 1. Basic construction — no constraints (MIN=0, MAX=255, ASCII_ONLY=false)
// ---------------------------------------------------------------------------

#[test]
fn try_from_chars_ok_no_constraints() {
    type G = GString<NoValidation, 0, 255, false>;

    let cases: &[OkCase] = &[
        OkCase {
            label: "empty iterator",
            chars: &[],
            expected_str: "",
            expected_len: 0,
        },
        OkCase {
            label: "ascii chars",
            chars: &['h', 'e', 'l', 'l', 'o'],
            expected_str: "hello",
            expected_len: 5,
        },
        OkCase {
            label: "single char",
            chars: &['x'],
            expected_str: "x",
            expected_len: 1,
        },
        OkCase {
            label: "2-byte unicode chars — é is 2 bytes",
            chars: &['h', 'é', 'l', 'l', 'o'],
            expected_str: "héllo",
            expected_len: 6,
        },
        OkCase {
            label: "3-byte unicode chars — each hiragana is 3 bytes",
            chars: &['こ', 'ん', 'に', 'ち', 'は'],
            expected_str: "こんにちは",
            expected_len: 15,
        },
        OkCase {
            label: "4-byte emoji — 🦀 is 4 bytes",
            chars: &['h', 'i', ' ', '🦀'],
            expected_str: "hi 🦀",
            expected_len: 7, // "hi " = 3 bytes, 🦀 = 4 bytes
        },
        OkCase {
            label: "whitespace chars",
            chars: &[' ', '\t', '\n'],
            expected_str: " \t\n",
            expected_len: 3,
        },
        OkCase {
            label: "digits and symbols",
            chars: &['a', 'b', 'c', '1', '2', '3', '!'],
            expected_str: "abc123!",
            expected_len: 7,
        },
        OkCase {
            label: "mixed widths",
            chars: &['a', '🦀', 'é'],
            expected_str: "a🦀é",
            expected_len: 7, // 'a'=1, '🦀'=4, 'é'=2
        },
    ];

    for case in cases {
        let result = G::try_from_chars(case.chars.iter().copied());
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

        // Oracle: compare against String built from same chars
        let oracle: String = case.chars.iter().collect();
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
// 2. TooLong errors — MAX is in bytes, not char count
// ---------------------------------------------------------------------------

#[test]
fn try_from_chars_err_too_long() {
    type G = GString<NoValidation, 0, 5, false>;

    let cases: &[ErrCase<Infallible>] = &[
        ErrCase {
            label: "one ascii char over MAX",
            chars: &['a', 'b', 'c', 'd', 'e', 'f'], // 6 bytes > MAX=5
            expected_err: GStringError::TooLong(5),
        },
        ErrCase {
            label: "single multibyte char exceeds MAX alone",
            // MAX=5; '🦀' = 4 bytes; adding 'a'(1) + 'b'(1) = 6 total
            chars: &['a', 'b', '🦀'],
            expected_err: GStringError::TooLong(5),
        },
        ErrCase {
            label: "multibyte char alone exceeds MAX",
            // MAX=5; 'こ'(3) + 'ん'(3) = 6 bytes > MAX=5
            chars: &['こ', 'ん'],
            expected_err: GStringError::TooLong(5),
        },
        ErrCase {
            label: "single 4-byte char in MAX=3 buffer",
            // 🦀 alone is 4 bytes > MAX=3
            chars: &['🦀'],
            expected_err: GStringError::TooLong(3), // uses G with MAX=3 below
        },
    ];

    // First three cases use MAX=5
    for case in &cases[..3] {
        let result = G::try_from_chars(case.chars.iter().copied());
        assert!(result.is_err(), "[{}] expected Err, got Ok", case.label);
        assert_eq!(
            result.unwrap_err(),
            case.expected_err,
            "[{}] wrong error variant",
            case.label
        );
    }

    // Last case needs MAX=3
    {
        type G3 = GString<NoValidation, 0, 3, false>;
        let case = &cases[3];
        let result = G3::try_from_chars(case.chars.iter().copied());
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
// 3. TooShort errors — MIN is in bytes
// ---------------------------------------------------------------------------

#[test]
fn try_from_chars_err_too_short() {
    type G = GString<NoValidation, 4, 16, false>;

    let cases: &[ErrCase<Infallible>] = &[
        ErrCase {
            label: "empty iterator",
            chars: &[],
            expected_err: GStringError::TooShort(4),
        },
        ErrCase {
            label: "one ascii char below MIN",
            chars: &['a', 'b', 'c'], // 3 bytes < MIN=4
            expected_err: GStringError::TooShort(4),
        },
        ErrCase {
            label: "single char below MIN",
            chars: &['x'], // 1 byte < MIN=4
            expected_err: GStringError::TooShort(4),
        },
    ];

    for case in cases {
        let result = G::try_from_chars(case.chars.iter().copied());
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
fn try_from_chars_ok_boundary_values() {
    type G = GString<NoValidation, 3, 8, false>;

    let cases: &[OkCase] = &[
        OkCase {
            label: "exactly MIN bytes — 3 ascii chars",
            chars: &['a', 'b', 'c'],
            expected_str: "abc",
            expected_len: 3,
        },
        OkCase {
            label: "exactly MAX bytes — 8 ascii chars",
            chars: &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
            expected_str: "abcdefgh",
            expected_len: 8,
        },
        OkCase {
            label: "between MIN and MAX",
            chars: &['a', 'b', 'c', 'd', 'e'],
            expected_str: "abcde",
            expected_len: 5,
        },
        OkCase {
            label: "exactly MAX bytes via multibyte — 🦀(4) + ab(2) + cd(2) = 8",
            chars: &['🦀', 'a', 'b', 'c', 'd'],
            expected_str: "🦀abcd",
            expected_len: 8,
        },
        OkCase {
            label: "exactly MIN bytes via multibyte — é(2) + a(1) = 3",
            chars: &['é', 'a'],
            expected_str: "éa",
            expected_len: 3,
        },
    ];

    for case in cases {
        let result = G::try_from_chars(case.chars.iter().copied());
        assert!(
            result.is_ok(),
            "[{}] expected Ok, got {:?}",
            case.label,
            result
        );
        let g = result.unwrap();
        let oracle: String = case.chars.iter().collect();
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
fn try_from_chars_ok_ascii_only() {
    type G = GString<NoValidation, 0, 64, true>;

    let cases: &[OkCase] = &[
        OkCase {
            label: "empty iterator",
            chars: &[],
            expected_str: "",
            expected_len: 0,
        },
        OkCase {
            label: "printable ascii chars",
            chars: &['H', 'e', 'l', 'l', 'o'],
            expected_str: "Hello",
            expected_len: 5,
        },
        OkCase {
            label: "digits",
            chars: &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
            expected_str: "0123456789",
            expected_len: 10,
        },
        OkCase {
            label: "symbols",
            chars: &['!', '@', '#', '$', '%'],
            expected_str: "!@#$%",
            expected_len: 5,
        },
        OkCase {
            label: "control chars that are still ASCII — tab and newline",
            chars: &['\t', '\n'],
            expected_str: "\t\n",
            expected_len: 2,
        },
    ];

    for case in cases {
        let result = G::try_from_chars(case.chars.iter().copied());
        assert!(
            result.is_ok(),
            "[{}] expected Ok, got {:?}",
            case.label,
            result
        );
        let g = result.unwrap();
        let oracle: String = case.chars.iter().collect();
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
// 6. ASCII_ONLY=true — non-ASCII chars rejected
// ---------------------------------------------------------------------------

#[test]
fn try_from_chars_err_not_ascii() {
    type G = GString<NoValidation, 0, 64, true>;

    let cases: &[ErrCase<Infallible>] = &[
        ErrCase {
            label: "latin extended — é",
            chars: &['c', 'a', 'f', 'é'],
            expected_err: GStringError::NotAscii,
        },
        ErrCase {
            label: "CJK character",
            chars: &['こ'],
            expected_err: GStringError::NotAscii,
        },
        ErrCase {
            label: "emoji — 🦀",
            chars: &['🦀'],
            expected_err: GStringError::NotAscii,
        },
        ErrCase {
            label: "non-ascii mixed in with ascii chars",
            chars: &['h', 'e', 'l', 'l', 'o', ' ', '世'],
            expected_err: GStringError::NotAscii,
        },
    ];

    for case in cases {
        let result = G::try_from_chars(case.chars.iter().copied());
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
fn try_from_chars_err_custom_validator() {
    type G = GString<NoDigits, 0, 64, false>;

    let cases: &[ErrCase<NoDigitsError>] = &[
        ErrCase {
            label: "single digit char",
            chars: &['a', 'b', '1'],
            expected_err: GStringError::Validation(NoDigitsError),
        },
        ErrCase {
            label: "digit at the start",
            chars: &['9', 'a', 'b'],
            expected_err: GStringError::Validation(NoDigitsError),
        },
        ErrCase {
            label: "all digit chars",
            chars: &['1', '2', '3', '4', '5'],
            expected_err: GStringError::Validation(NoDigitsError),
        },
        ErrCase {
            label: "digit lone in otherwise clean chars",
            chars: &['h', 'i', '0', 'u'],
            expected_err: GStringError::Validation(NoDigitsError),
        },
    ];

    for case in cases {
        let result = G::try_from_chars(case.chars.iter().copied());
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
fn try_from_chars_ok_custom_validator() {
    type G = GString<NoDigits, 0, 64, false>;

    let cases: &[OkCase] = &[
        OkCase {
            label: "no digits",
            chars: &['h', 'e', 'l', 'l', 'o'],
            expected_str: "hello",
            expected_len: 5,
        },
        OkCase {
            label: "empty iterator",
            chars: &[],
            expected_str: "",
            expected_len: 0,
        },
        OkCase {
            label: "unicode without digits",
            chars: &['こ', 'ん', 'に', 'ち', 'は'],
            expected_str: "こんにちは",
            expected_len: 15,
        },
    ];

    for case in cases {
        let result = G::try_from_chars(case.chars.iter().copied());
        assert!(
            result.is_ok(),
            "[{}] expected Ok, got {:?}",
            case.label,
            result
        );
        let g = result.unwrap();
        let oracle: String = case.chars.iter().collect();
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
fn try_from_chars_bounds_checked_before_validator() {
    type G = GString<NoDigits, 0, 3, false>;

    // "12" — has digits AND within bounds — should get Validation error
    let result = G::try_from_chars(['1', '2']);
    assert_eq!(result.unwrap_err(), GStringError::Validation(NoDigitsError));

    // "1234" — has digits AND too long — should get TooLong (bounds checked first)
    let result = G::try_from_chars(['1', '2', '3', '4']);
    assert_eq!(result.unwrap_err(), GStringError::TooLong(3));
}

// ---------------------------------------------------------------------------
// 9. as_str(), len(), count() agree with String oracle
// ---------------------------------------------------------------------------

#[test]
fn try_from_chars_str_and_len_match_string_oracle() {
    type G = GString<NoValidation, 0, 255, false>;

    // (chars, expected concatenation)
    let cases: &[(&[char], &str)] = &[
        (&[], ""),
        (&['a'], "a"),
        (&['h', 'e', 'l', 'l', 'o'], "hello"),
        (&['🦀'], "🦀"),
        (&['こ', 'ん', 'に', 'ち', 'は'], "こんにちは"),
        (&['a', '🦀', 'é'], "a🦀é"),
    ];

    for (chars, expected) in cases {
        let oracle = String::from(*expected);
        let g = G::try_from_chars(chars.iter().copied())
            .unwrap_or_else(|e| panic!("try_from_chars({:?}) failed: {:?}", chars, e));

        assert_eq!(
            g.as_str(),
            oracle.as_str(),
            "as_str mismatch for chars {:?}",
            chars
        );
        assert_eq!(g.len(), oracle.len(), "len mismatch for chars {:?}", chars);
        assert_eq!(
            g.count(),
            oracle.chars().count(),
            "count() mismatch for chars {:?}",
            chars
        );
        assert_eq!(
            g.is_empty(),
            oracle.is_empty(),
            "is_empty mismatch for chars {:?}",
            chars
        );
    }
}

// ---------------------------------------------------------------------------
// 10. count() reflects char count, not byte count
// ---------------------------------------------------------------------------

#[test]
fn try_from_chars_count_equals_input_char_count() {
    type G = GString<NoValidation, 0, 255, false>;

    struct CountCase {
        label: &'static str,
        chars: &'static [char],
        expected_count: usize,
        expected_len: usize,
    }

    let cases: &[CountCase] = &[
        CountCase {
            label: "empty",
            chars: &[],
            expected_count: 0,
            expected_len: 0,
        },
        CountCase {
            label: "pure ascii — count == len",
            chars: &['a', 'b', 'c'],
            expected_count: 3,
            expected_len: 3,
        },
        CountCase {
            label: "2-byte chars — count < len",
            chars: &['é', 'é', 'é'],
            expected_count: 3,
            expected_len: 6, // 3 × 2 bytes
        },
        CountCase {
            label: "3-byte chars — count < len",
            chars: &['こ', 'ん'],
            expected_count: 2,
            expected_len: 6, // 2 × 3 bytes
        },
        CountCase {
            label: "4-byte emoji — count much less than len",
            chars: &['🦀', '🦀'],
            expected_count: 2,
            expected_len: 8, // 2 × 4 bytes
        },
    ];

    for case in cases {
        let g = G::try_from_chars(case.chars.iter().copied())
            .unwrap_or_else(|e| panic!("[{}] try_from_chars failed: {:?}", case.label, e));

        // Oracle
        let oracle: String = case.chars.iter().collect();

        assert_eq!(
            g.count(),
            oracle.chars().count(),
            "[{}] count() vs String oracle mismatch",
            case.label
        );
        assert_eq!(
            g.count(),
            case.expected_count,
            "[{}] count() mismatch",
            case.label
        );
        assert_eq!(
            g.len(),
            case.expected_len,
            "[{}] len() mismatch",
            case.label
        );
    }
}

// ---------------------------------------------------------------------------
// 11. Iterator sources — try_from_chars accepts any IntoIterator<Item = char>
// ---------------------------------------------------------------------------

#[test]
fn try_from_chars_accepts_str_chars_iterator() {
    type G = GString<NoValidation, 0, 64, false>;

    let result = G::try_from_chars("hello world".chars());
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str(), "hello world");
}

#[test]
fn try_from_chars_accepts_vec_of_chars() {
    type G = GString<NoValidation, 0, 64, false>;

    let chars = vec!['r', 'u', 's', 't'];
    let oracle: String = chars.iter().collect();

    let result = G::try_from_chars(chars);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str(), oracle.as_str());
}

#[test]
fn try_from_chars_accepts_mapped_iterator() {
    type G = GString<NoValidation, 0, 64, false>;

    // Lowercased chars from a &str
    let result = G::try_from_chars("RUST".chars().map(|c| c.to_ascii_lowercase()));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str(), "rust");
}

#[test]
fn try_from_chars_accepts_filtered_iterator() {
    type G = GString<NoValidation, 0, 64, false>;

    // Strip spaces
    let result = G::try_from_chars("h e l l o".chars().filter(|c| !c.is_whitespace()));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str(), "hello");
}
