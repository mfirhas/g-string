use g_string::{GString, Validator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Uppercase;

impl Validator for Uppercase {
    type Error = ();

    #[inline]
    fn validate(s: impl AsRef<str>) -> Result<(), Self::Error> {
        if s.as_ref().bytes().all(|b| !b.is_ascii_lowercase()) {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[test]
fn test_map() {
    let s: GString = GString::try_new("hello").unwrap();

    let mapped = s
        .map::<(), 0, 32, false, _, _>(|s| s.to_uppercase())
        .unwrap();

    assert_eq!(mapped.as_str(), "HELLO");
}

#[test]
fn test_map_validation_error() {
    let s: GString = GString::try_new("hello").unwrap();

    let result = s.map::<Uppercase, 0, 32, false, _, _>(|s| s.to_string());

    assert!(result.is_err());
}

#[test]
fn test_and_then() {
    let s: GString = GString::try_new("  hello  ").unwrap();

    let mapped: GString<(), 0, 32> = s
        .and_then(|s| GString::<(), 0, 32, false>::try_new(s.trim()))
        .unwrap();

    assert_eq!(mapped.as_str(), "hello");
}

#[test]
fn test_and_then_validation_error() {
    let s: GString = GString::try_new("hello").unwrap();

    let result = s
        .and_then::<Uppercase, 0, 32, false, _>(|s| GString::<Uppercase, 0, 32, false>::try_new(s));

    assert!(result.is_err());
}

#[test]
fn test_inspect() {
    use std::cell::RefCell;

    let observed = RefCell::new(String::new());

    let s: GString = GString::try_new("hello").unwrap();

    let result = s.inspect(|v| {
        *observed.borrow_mut() = v.to_owned();
    });

    assert_eq!(&*observed.borrow(), "hello");
    assert_eq!(result.as_str(), "hello");
}
