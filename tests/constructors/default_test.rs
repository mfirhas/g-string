use g_string::GString;

#[test]
fn test_default() {
    let d: GString = GString::default();
    let e: GString = GString::try_new("").unwrap();
    let s = String::default();
    assert_eq!(d, e, "should equal");
    assert_eq!(d, s, "should equal");
    assert_eq!(e, s, "should equal");
}
