use g_string::{
    DEFAULT_ASCII_ONLY, DEFAULT_MAX, DEFAULT_MIN, GString, GStringError, NoValidation, Validator,
    gformat, gstring,
};

// -------------------------------------------------------------------------
// Custom validator used across tests
// -------------------------------------------------------------------------

/// Rejects any string containing a digit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NoDigits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NoDigitsError;

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

impl core::fmt::Display for NoDigitsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl core::error::Error for NoDigitsError {}

// -------------------------------------------------------------------------
// gstring! — default form (literal only)
// -------------------------------------------------------------------------

#[test]
fn gstring_default_form() {
    let g = gstring!("hello");

    // Value is correct
    assert_eq!(g.as_str(), "hello");

    // Inferred defaults
    assert_eq!(g.len(), 5);
    assert_eq!(g.capacity(), DEFAULT_MAX);

    // Type is GString<NoValidation, DEFAULT_MIN, DEFAULT_MAX, DEFAULT_ASCII_ONLY>
    let _: GString<NoValidation, DEFAULT_MIN, DEFAULT_MAX, DEFAULT_ASCII_ONLY> = g;
}

#[test]
fn gstring_default_form_empty() {
    let g = gstring!("");
    assert_eq!(g.as_str(), "");
    assert_eq!(g.len(), 0);
}

#[test]
fn gstring_default_form_unicode() {
    let g = gstring!("café");
    assert_eq!(g.as_str(), "café");
}

// -------------------------------------------------------------------------
// gstring! — validator only
// -------------------------------------------------------------------------

#[test]
fn gstring_with_validator() {
    let g = gstring!("hello", NoDigits);
    assert_eq!(g.as_str(), "hello");
    let _: GString<NoDigits, DEFAULT_MIN, DEFAULT_MAX, DEFAULT_ASCII_ONLY> = g;
}

// -------------------------------------------------------------------------
// gstring! — validator + min
// -------------------------------------------------------------------------

#[test]
fn gstring_with_validator_and_min() {
    let g = gstring!("hello", NoValidation, 1);
    assert_eq!(g.as_str(), "hello");
    let _: GString<NoValidation, 1, DEFAULT_MAX, DEFAULT_ASCII_ONLY> = g;
}

// -------------------------------------------------------------------------
// gstring! — validator + min + max
// -------------------------------------------------------------------------

#[test]
fn gstring_with_validator_min_max() {
    let g = gstring!("hi", NoValidation, 1, 32);
    assert_eq!(g.as_str(), "hi");
    assert_eq!(g.capacity(), 32);
    let _: GString<NoValidation, 1, 32, DEFAULT_ASCII_ONLY> = g;
}

// -------------------------------------------------------------------------
// gstring! — full form (all params)
// -------------------------------------------------------------------------

#[test]
fn gstring_full_form() {
    let g = gstring!("hello", NoValidation, 1, 32, false);
    assert_eq!(g.as_str(), "hello");
    assert_eq!(g.capacity(), 32);
    let _: GString<NoValidation, 1, 32, false> = g;
}

#[test]
fn gstring_full_form_ascii_only() {
    let g = gstring!("hello", NoValidation, 0, 32, true);
    assert_eq!(g.as_str(), "hello");
    let _: GString<NoValidation, 0, 32, true> = g;
}

#[test]
fn gstring_full_form_custom_validator() {
    let g = gstring!("hello", NoDigits, 1, 32, false);
    assert_eq!(g.as_str(), "hello");
    let _: GString<NoDigits, 1, 32, false> = g;
}

// -------------------------------------------------------------------------
// gstring! — is a const expression
// -------------------------------------------------------------------------

#[test]
fn gstring_is_const() {
    const G: GString<NoValidation, 0, 32, false> = gstring!("const", NoValidation, 0, 32, false);
    assert_eq!(G.as_str(), "const");
}

#[test]
fn gstring_default_form_is_const() {
    // The simple form also expands to a const block
    const G: GString = gstring!("hello");
    assert_eq!(G.as_str(), "hello");
}

// -------------------------------------------------------------------------
// gstring! — compile-time failure cases
//
// gstring! expands to a `const` block, so constraint violations are caught
// by the compiler (const eval panic), not at runtime. They cannot be
// expressed as `#[should_panic]` tests — they simply won't compile.
//
// Each case below is commented out with the exact compiler error it produces.
// Uncomment individually to verify the behaviour manually.
// -------------------------------------------------------------------------

// TOO LONG — input exceeds MAX
//
// error: evaluation of constant value failed
//   ... maximum length exceeds MAX
//
// let _g = gstring!("hello", NoValidation, 0, 4, false);

// TOO SHORT — input is below MIN
//
// error: evaluation of constant value failed
//   ... minimum length below MIN
//
// let _g = gstring!("hi", NoValidation, 3, 32, false);

// NON-ASCII — ASCII_ONLY = true but input contains multibyte character
//
// error: evaluation of constant value failed
//   ... only ASCII characters are allowed
//
// let _g = gstring!("café", NoValidation, 0, 32, true);

// MIN > MAX — violates the const assert inside check_bounds
//
// error: evaluation of constant value failed
//   ... MIN cannot be bigger than MAX
//
// let _g = gstring!("hello", NoValidation, 10, 5, false);

// VALIDATOR VIOLATION — gstring! does NOT invoke the Validator trait;
// validation is a runtime concern handled by try_new. The const path
// only checks bounds and ASCII. This means the following compiles fine
// even though NoDigits would reject it at runtime via try_new:
//
// let _g = gstring!("abc123", NoDigits, 0, 32, false); // compiles — no validator in const path

// -------------------------------------------------------------------------
// gstring! — __new panic cases
//
// __new is the const fn that gstring! delegates to. We drive it directly
// at runtime here so the panic messages are covered by the test suite.
// These mirror the compile-time failures above.
// -------------------------------------------------------------------------

#[test]
#[should_panic(expected = "maximum length exceeds MAX")]
fn gstring_new_panics_too_long() {
    // MAX = 4, input has 5 bytes
    GString::<NoValidation, 0, 4, false>::__new("hello");
}

#[test]
#[should_panic(expected = "minimum length below MIN")]
fn gstring_new_panics_too_short() {
    // MIN = 3, input has 2 bytes
    GString::<NoValidation, 3, 32, false>::__new("hi");
}

#[test]
#[should_panic(expected = "only ASCII characters are allowed")]
fn gstring_new_panics_non_ascii() {
    // ASCII_ONLY = true, input contains non-ASCII
    GString::<NoValidation, 0, 32, true>::__new("café");
}

// -------------------------------------------------------------------------
// gformat! — default form
// -------------------------------------------------------------------------

#[test]
fn gformat_default_no_args() {
    let g = gformat!("hello").expect("valid");
    assert_eq!(g.as_str(), "hello");
    let _: GString<NoValidation, DEFAULT_MIN, DEFAULT_MAX, DEFAULT_ASCII_ONLY> = g;
}

#[test]
fn gformat_default_with_args() {
    let name = "world";
    let g = gformat!("hello, {}!", name).expect("valid");
    assert_eq!(g.as_str(), "hello, world!");
}

#[test]
fn gformat_default_multiple_args() {
    let g = gformat!("{} + {} = {}", 1, 2, 3).expect("valid");
    assert_eq!(g.as_str(), "1 + 2 = 3");
}

// -------------------------------------------------------------------------
// gformat! — validator only
// -------------------------------------------------------------------------

#[test]
fn gformat_validator_only_ok() {
    let g = gformat!("hello"; NoDigits).expect("valid");
    assert_eq!(g.as_str(), "hello");
    let _: GString<NoDigits, DEFAULT_MIN, DEFAULT_MAX, DEFAULT_ASCII_ONLY> = g;
}

#[test]
fn gformat_validator_only_err() {
    let result = gformat!("hello{}", 42; NoDigits);
    assert!(
        matches!(result, Err(GStringError::Validation(_))),
        "expected Validation error, got {:?}",
        result
    );
}

// -------------------------------------------------------------------------
// gformat! — validator + min
// -------------------------------------------------------------------------

#[test]
fn gformat_validator_min_ok() {
    let g = gformat!("hello"; NoValidation, 3).expect("valid");
    assert_eq!(g.as_str(), "hello");
    let _: GString<NoValidation, 3, DEFAULT_MAX, DEFAULT_ASCII_ONLY> = g;
}

#[test]
fn gformat_validator_min_err_too_short() {
    let result = gformat!("hi"; NoValidation, 5);
    assert!(
        matches!(result, Err(GStringError::TooShort(5))),
        "expected TooShort(5), got {:?}",
        result
    );
}

// -------------------------------------------------------------------------
// gformat! — validator + min + max
// -------------------------------------------------------------------------

#[test]
fn gformat_validator_min_max_ok() {
    let g = gformat!("hello"; NoValidation, 1, 32).expect("valid");
    assert_eq!(g.as_str(), "hello");
    assert_eq!(g.capacity(), 32);
    let _: GString<NoValidation, 1, 32, DEFAULT_ASCII_ONLY> = g;
}

#[test]
fn gformat_validator_min_max_err_too_long() {
    let result = gformat!("hello"; NoValidation, 0, 3);
    assert!(
        matches!(result, Err(GStringError::TooLong(3))),
        "expected TooLong(3), got {:?}",
        result
    );
}

// -------------------------------------------------------------------------
// gformat! — full form
// -------------------------------------------------------------------------

#[test]
fn gformat_full_form_ok() {
    let g = gformat!("hello"; NoValidation, 1, 32, false).expect("valid");
    assert_eq!(g.as_str(), "hello");
    let _: GString<NoValidation, 1, 32, false> = g;
}

#[test]
fn gformat_full_form_ascii_only_ok() {
    let g = gformat!("hello"; NoValidation, 0, 32, true).expect("valid");
    assert_eq!(g.as_str(), "hello");
    let _: GString<NoValidation, 0, 32, true> = g;
}

#[test]
fn gformat_full_form_ascii_only_err() {
    let result = gformat!("café"; NoValidation, 0, 32, true);
    assert!(
        matches!(result, Err(GStringError::NotAscii)),
        "expected NotAscii, got {:?}",
        result
    );
}

#[test]
fn gformat_full_form_custom_validator_ok() {
    let g = gformat!("hello"; NoDigits, 1, 32, false).expect("valid");
    assert_eq!(g.as_str(), "hello");
}

#[test]
fn gformat_full_form_custom_validator_err() {
    let result = gformat!("hello42"; NoDigits, 0, 32, false);
    assert!(
        matches!(result, Err(GStringError::Validation(_))),
        "expected Validation error, got {:?}",
        result
    );
}

// -------------------------------------------------------------------------
// gformat! — runtime formatting is reflected in the result
// -------------------------------------------------------------------------

#[test]
fn gformat_runtime_value_reflected() {
    for i in 0..5u32 {
        let g = gformat!("item-{}", i; NoValidation, 0, 32).expect("valid");
        let expected = format!("item-{}", i);
        assert_eq!(g.as_str(), expected.as_str());
    }
}

// -------------------------------------------------------------------------
// gformat! returns Err, not panics, on constraint violation
// -------------------------------------------------------------------------

#[test]
fn gformat_returns_err_not_panic_on_too_long() {
    // gformat! must return Err instead of panicking (unlike gstring!)
    let result = gformat!("toolong"; NoValidation, 0, 3);
    assert!(result.is_err());
}

#[test]
fn gformat_returns_err_not_panic_on_too_short() {
    let result = gformat!("hi"; NoValidation, 10, 32);
    assert!(result.is_err());
}
