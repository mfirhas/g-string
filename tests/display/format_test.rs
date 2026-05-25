use g_string::{GString, GStringError, GStringNV, NoValidation};

#[cfg(feature = "alloc")]
#[test]
fn test_format() {
    let expected_str = "I am stack-allocated, copy and generic string type";
    let stack = GStringNV::try_default("stack-allocated").unwrap();
    let copy = GStringNV::<4, 4, true>::try_new("copy").unwrap();
    let generic = GString::<NoValidation, 2, 10>::try_new("generic").unwrap();
    let string = GStringNV::<4, 6, true>::try_new("string").unwrap();

    let ret = format!("I am {stack}, {} and {generic} {} type", copy, string);
    assert_eq!(ret.as_str(), expected_str);
    let g: GString = GString::try_from(ret).unwrap();
    assert_eq!(g, expected_str, "expect to be the same");

    let string = GStringNV::<4, 5, true>::try_new("string");
    assert!(string.is_err());
    assert_eq!(string.unwrap_err(), GStringError::TooLong(5));
}
