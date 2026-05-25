use g_string::{GString, NoValidation};

#[test]
fn test_debug() {
    let expected_str_with_min = "GString(\"debug msg\", MIN=2, MAX=255, ASCII_ONLY=false)";

    let expected_str = "GString(\"debug msg\", MIN=0, MAX=255, ASCII_ONLY=false)";

    let s = GString::<NoValidation, 2>::try_new("debug msg").unwrap();
    let debug = format!("{:?}", s);
    println!("{debug}");
    assert_eq!(&debug, expected_str_with_min, "debug string not equals");

    let s = GString::try_default("debug msg").unwrap();
    let debug = format!("{:?}", s);
    println!("{debug}");
    assert_eq!(&debug, expected_str, "debug string not equals");

    let s: GString = GString::try_default("debug msg").unwrap();
    let debug = format!("{:?}", s);
    println!("{debug}");
    assert_eq!(&debug, expected_str, "debug string not equals");

    let s: GString = GString::try_new("debug msg").unwrap();
    let debug = format!("{:?}", s);
    println!("{debug}");
    assert_eq!(&debug, expected_str, "debug string not equals");
}
