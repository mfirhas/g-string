use g_string::GStringError;

#[test]
fn test_g_string_err() {
    let fmt_err = |err: GStringError<&str>| -> String { err.to_string() };

    assert_eq!(
        fmt_err(GStringError::TooShort(0)),
        "minimum length allowed is 0"
    );
    assert_eq!(
        fmt_err(GStringError::TooLong(100)),
        "maximum length allowed is 100"
    );
    assert_eq!(
        fmt_err(GStringError::NotAscii),
        "only ASCII characters are allowed"
    );
    assert_eq!(
        fmt_err(GStringError::Validation("ERROR")),
        "validation error: ERROR"
    );
    assert_eq!(
        fmt_err(GStringError::Mutation("qwe)(*())")),
        "mutation error: qwe)(*())"
    );
}
