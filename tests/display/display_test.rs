use g_string::{GString, NoValidation};

#[cfg(feature = "alloc")]
#[test]
fn test_display() {
    let expected_str = "display msg";

    let s = GString::<NoValidation, 2>::try_new("display msg").unwrap();
    let display = format!("{}", s);
    println!("{display}");
    assert_eq!(s, display);
    assert_eq!(&display, expected_str, "display string not equals");

    let s = GString::try_default("display msg").unwrap();
    let display = s.to_string();
    println!("{display}");
    assert_eq!(s, display);
    assert_eq!(&display, expected_str, "display string not equals");

    let s: GString = GString::try_default("display msg").unwrap();
    let display = format!("{}", s);
    println!("{display}");
    assert_eq!(s, display);
    assert_eq!(&display, expected_str, "display string not equals");

    let s: GString = GString::try_new("display msg").unwrap();
    let display = s.to_string();
    println!("{display}");
    assert_eq!(s, display);
    assert_eq!(&display, expected_str, "display string not equals");
}
