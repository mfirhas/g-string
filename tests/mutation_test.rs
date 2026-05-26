use g_string::{GString, GStringError, NoValidation};

// -------------------------------------------------------------------------
// push
// -------------------------------------------------------------------------

#[test]
fn push_appends_char() {
    let cases: &[(&str, char, &str)] = &[
        ("", 'a', "a"),
        ("hello", '!', "hello!"),
        ("café", '?', "café?"),
        ("", '🦀', "🦀"),
        ("a", '日', "a日"),
    ];

    for &(initial, ch, expected) in cases {
        let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new(initial).unwrap();
        gs.push(ch).unwrap();
        assert_eq!(gs.as_str(), expected, "push {:?} onto {:?}", ch, initial);
    }
}

#[test]
fn push_too_long_returns_err() {
    let mut gs: GString<NoValidation, 0, 3, false> = GString::try_new("abc").unwrap();
    assert!(matches!(gs.push('x'), Err(GStringError::TooLong(3))));
    assert_eq!(gs.as_str(), "abc");
}

// -------------------------------------------------------------------------
// push_str
// -------------------------------------------------------------------------

#[test]
fn push_str_appends() {
    let cases: &[(&str, &str, &str)] = &[
        ("", "hello", "hello"),
        ("hello", " world", "hello world"),
        ("café", " au lait", "café au lait"),
        ("", "🦀🎉", "🦀🎉"),
        ("a", "", "a"),
    ];

    for &(initial, append, expected) in cases {
        let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new(initial).unwrap();
        gs.push_str(append).unwrap();
        assert_eq!(
            gs.as_str(),
            expected,
            "push_str {:?} onto {:?}",
            append,
            initial
        );
    }
}

#[test]
fn push_str_too_long_returns_err() {
    let mut gs: GString<NoValidation, 0, 5, false> = GString::try_new("hello").unwrap();
    assert!(matches!(
        gs.push_str(" world"),
        Err(GStringError::TooLong(5))
    ));
    assert_eq!(gs.as_str(), "hello");
}

#[test]
fn push_str_empty_is_noop() {
    let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new("hello").unwrap();
    gs.push_str("").unwrap();
    assert_eq!(gs.as_str(), "hello");
}

// -------------------------------------------------------------------------
// insert / insert_str
// -------------------------------------------------------------------------

#[test]
fn insert_char_at_various_positions() {
    // (initial, byte_idx, char, expected)
    // All idx values are char boundaries, verified against String::insert oracle
    let cases: &[(&str, usize, char, &str)] = &[
        ("helo", 3, 'l', "hello"),
        ("hello", 0, '!', "!hello"),
        ("hello", 5, '!', "hello!"),
        ("hello", 2, '-', "he-llo"),
        // "café" bytes: c=0, a=1, f=2, é=3..5; byte 5 is end boundary
        ("café", 5, '!', "café!"),
    ];

    for &(initial, idx, ch, expected) in cases {
        // String oracle
        let mut oracle = initial.to_string();
        oracle.insert(idx, ch);

        let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new(initial).unwrap();
        gs.insert(idx, ch).unwrap();
        assert_eq!(
            gs.as_str(),
            oracle.as_str(),
            "insert {:?} at {} into {:?}",
            ch,
            idx,
            initial
        );
        assert_eq!(
            gs.as_str(),
            expected,
            "insert {:?} at {} into {:?} expected {:?}",
            ch,
            idx,
            initial,
            expected
        );
    }
}

#[test]
fn insert_str_at_various_positions() {
    // (initial, byte_idx, insert, expected)
    let cases: &[(&str, usize, &str, &str)] = &[
        ("hllo", 1, "e", "hello"),
        ("world", 0, "hello ", "hello world"),
        ("hello", 5, "!", "hello!"),
        ("ab", 1, "XYZ", "aXYZb"),
        // "aβb": a=0, β=1..3, b=3; insert at byte 1 (start of β)
        ("aβb", 1, "α", "aαβb"),
    ];

    for &(initial, idx, ins, expected) in cases {
        let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new(initial).unwrap();
        gs.insert_str(idx, ins).unwrap();
        assert_eq!(
            gs.as_str(),
            expected,
            "insert_str {:?} at {} into {:?}",
            ins,
            idx,
            initial
        );
    }
}

#[test]
fn insert_str_non_boundary_returns_err() {
    // "café" bytes: c=0,a=1,f=2,é=3..5; byte 4 is inside 'é'
    let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new("café").unwrap();
    assert!(
        matches!(gs.insert_str(4, "x"), Err(GStringError::Mutation(_))),
        "expected Mutation error for non-char-boundary insertion"
    );
    assert_eq!(gs.as_str(), "café");
}

#[test]
fn insert_str_too_long_returns_err() {
    let mut gs: GString<NoValidation, 0, 5, false> = GString::try_new("hello").unwrap();
    assert!(matches!(
        gs.insert_str(0, "x"),
        Err(GStringError::TooLong(5))
    ));
    assert_eq!(gs.as_str(), "hello");
}

// -------------------------------------------------------------------------
// pop
// -------------------------------------------------------------------------

#[test]
fn pop_removes_last_char() {
    // (initial, expected_popped, expected_remainder)
    let cases: &[(&str, char, &str)] = &[
        ("hello", 'o', "hell"),
        ("café", 'é', "caf"),
        ("🦀", '🦀', ""),
        ("a🦀", '🦀', "a"),
        ("αβγ", 'γ', "αβ"),
    ];

    for &(initial, expected_ch, expected_str) in cases {
        // String oracle
        let mut oracle = initial.to_string();
        let oracle_ch = oracle.pop();

        let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new(initial).unwrap();
        let popped = gs.pop().unwrap();
        assert_eq!(popped, oracle_ch, "pop char from {:?}", initial);
        assert_eq!(popped, Some(expected_ch), "pop char from {:?}", initial);
        assert_eq!(
            gs.as_str(),
            oracle.as_str(),
            "pop remainder from {:?}",
            initial
        );
        assert_eq!(
            gs.as_str(),
            expected_str,
            "pop remainder from {:?}",
            initial
        );
    }
}

#[test]
fn pop_empty_returns_none() {
    let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new("").unwrap();
    assert_eq!(gs.pop().unwrap(), None);
    assert_eq!(gs.as_str(), "");
}

#[test]
fn pop_respects_min_constraint() {
    let mut gs: GString<NoValidation, 3, 64, false> = GString::try_new("abc").unwrap();
    assert!(matches!(gs.pop(), Err(GStringError::TooShort(3))));
    assert_eq!(gs.as_str(), "abc");
}

#[test]
fn pop_all_chars_one_by_one() {
    let input = "héllo";
    let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new(input).unwrap();
    let mut oracle = input.to_string();

    while !oracle.is_empty() {
        let oracle_ch = oracle.pop();
        let gs_ch = gs.pop().unwrap();
        assert_eq!(gs_ch, oracle_ch, "sequential pop mismatch");
    }
    assert_eq!(gs.pop().unwrap(), None);
}

// -------------------------------------------------------------------------
// remove
// -------------------------------------------------------------------------

#[test]
fn remove_char_at_index() {
    // (initial, byte_idx, expected_char, expected_remainder)
    let cases: &[(&str, usize, char, &str)] = &[
        ("hello", 0, 'h', "ello"),
        ("hello", 4, 'o', "hell"),
        ("hello", 2, 'l', "helo"),
        ("café", 3, 'é', "caf"),
        ("αβγ", 0, 'α', "βγ"),
        ("a🦀b", 1, '🦀', "ab"),
    ];

    for &(initial, idx, expected_ch, expected_rem) in cases {
        // String oracle
        let mut oracle = initial.to_string();
        let oracle_ch = oracle.remove(idx);

        let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new(initial).unwrap();
        let ch = gs.remove(idx).unwrap();

        assert_eq!(ch, oracle_ch, "removed char from {:?} at {}", initial, idx);
        assert_eq!(
            ch, expected_ch,
            "removed char from {:?} at {}",
            initial, idx
        );
        assert_eq!(
            gs.as_str(),
            oracle.as_str(),
            "remainder after remove {:?} at {}",
            initial,
            idx
        );
        assert_eq!(
            gs.as_str(),
            expected_rem,
            "remainder after remove {:?} at {}",
            initial,
            idx
        );
    }
}

#[test]
fn remove_non_boundary_returns_err() {
    // byte 4 is inside 'é' in "café"
    let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new("café").unwrap();
    assert!(matches!(gs.remove(4), Err(GStringError::Mutation(_))));
    assert_eq!(gs.as_str(), "café");
}

#[test]
fn remove_respects_min_constraint() {
    let mut gs: GString<NoValidation, 1, 64, false> = GString::try_new("a").unwrap();
    assert!(matches!(gs.remove(0), Err(GStringError::TooShort(1))));
    assert_eq!(gs.as_str(), "a");
}

// -------------------------------------------------------------------------
// truncate
// -------------------------------------------------------------------------

#[test]
fn truncate_to_various_lengths() {
    // (initial, new_len, expected)
    let cases: &[(&str, usize, &str)] = &[
        ("hello", 3, "hel"),
        ("hello", 0, ""),
        ("hello", 5, "hello"), // no-op (equal)
        ("hello", 9, "hello"), // no-op (greater)
        ("café", 3, "caf"),
        ("αβγ", 2, "α"),   // α = 2 bytes
        ("🦀ab", 4, "🦀"), // 🦀 = 4 bytes
    ];

    for &(initial, new_len, expected) in cases {
        let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new(initial).unwrap();
        gs.truncate(new_len).unwrap();
        assert_eq!(
            gs.as_str(),
            expected,
            "truncate {:?} to {}",
            initial,
            new_len
        );
    }
}

#[test]
fn truncate_non_boundary_returns_err() {
    // byte 4 is inside 'é' in "café"
    let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new("café").unwrap();
    assert!(matches!(gs.truncate(4), Err(GStringError::Mutation(_))));
    assert_eq!(gs.as_str(), "café");
}

#[test]
fn truncate_respects_min_constraint() {
    let mut gs: GString<NoValidation, 3, 64, false> = GString::try_new("hello").unwrap();
    assert!(matches!(gs.truncate(2), Err(GStringError::TooShort(3))));
    assert_eq!(gs.as_str(), "hello");
}

// -------------------------------------------------------------------------
// clear
// -------------------------------------------------------------------------

#[test]
fn clear_empties_string() {
    let cases = ["hello", "café", "αβγ", "🦀🎉", "a"];

    for &input in &cases {
        let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new(input).unwrap();
        gs.clear().unwrap();
        assert_eq!(gs.as_str(), "", "clear {:?}", input);
        assert_eq!(gs.len(), 0);
    }
}

#[test]
fn clear_blocked_when_min_nonzero() {
    let mut gs: GString<NoValidation, 1, 64, false> = GString::try_new("hello").unwrap();
    assert!(matches!(gs.clear(), Err(GStringError::Mutation(_))));
    assert_eq!(gs.as_str(), "hello");
}

// -------------------------------------------------------------------------
// replace (alloc feature)
// -------------------------------------------------------------------------

#[test]
fn replace_substitutes_pattern() {
    // (initial, from, to, expected)
    let cases: &[(&str, &str, &str, &str)] = &[
        ("hello world", "world", "Rust", "hello Rust"),
        ("aabbcc", "bb", "XX", "aaXXcc"),
        ("hello", "xyz", "abc", "hello"), // no match
        ("hello", "l", "L", "heLLo"),     // multiple matches
        ("café", "é", "e", "cafe"),
        ("abc", "abc", "", ""), // replace with empty
    ];

    for &(initial, from, to, expected) in cases {
        // String oracle
        let oracle = initial.replace(from, to);

        let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new(initial).unwrap();
        gs.replace(from, to).unwrap();
        assert_eq!(
            gs.as_str(),
            oracle.as_str(),
            "replace {:?} with {:?} in {:?}",
            from,
            to,
            initial
        );
        assert_eq!(
            gs.as_str(),
            expected,
            "replace {:?} with {:?} in {:?}",
            from,
            to,
            initial
        );
    }
}

#[test]
fn replace_too_long_returns_err() {
    let mut gs: GString<NoValidation, 0, 5, false> = GString::try_new("hello").unwrap();
    assert!(matches!(
        gs.replace("hello", "toolong"),
        Err(GStringError::TooLong(5))
    ));
    assert_eq!(gs.as_str(), "hello");
}

// -------------------------------------------------------------------------
// replace_range
// -------------------------------------------------------------------------

#[test]
fn replace_range_various() {
    // (initial, start, end, replacement, expected)
    // all start/end are char boundaries
    let cases: &[(&str, usize, usize, &str, &str)] = &[
        ("hello world", 6, 11, "Rust", "hello Rust"),
        ("hello", 0, 5, "world", "world"),
        ("abcde", 1, 3, "XY", "aXYde"),
        ("hello", 2, 2, "XY", "heXYllo"), // zero-width insert
        ("hello", 0, 5, "", ""),          // delete all
        ("αβγ", 0, 2, "X", "Xβγ"),        // α = 2 bytes
    ];

    for &(initial, start, end, replacement, expected) in cases {
        // String oracle
        let mut oracle = initial.to_string();
        oracle.replace_range(start..end, replacement);

        let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new(initial).unwrap();
        gs.replace_range(start..end, replacement).unwrap();
        assert_eq!(
            gs.as_str(),
            oracle.as_str(),
            "replace_range [{}..{}] with {:?} in {:?}",
            start,
            end,
            replacement,
            initial
        );
        assert_eq!(
            gs.as_str(),
            expected,
            "replace_range [{}..{}] with {:?} in {:?}",
            start,
            end,
            replacement,
            initial
        );
    }
}

#[test]
fn replace_range_non_boundary_returns_err() {
    // byte 4 is inside 'é' in "café"
    let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new("café").unwrap();
    assert!(matches!(
        gs.replace_range(3..4, "x"),
        Err(GStringError::Mutation(_))
    ));
    assert_eq!(gs.as_str(), "café");
}

#[test]
fn replace_range_too_long_returns_err() {
    let mut gs: GString<NoValidation, 0, 5, false> = GString::try_new("hello").unwrap();
    assert!(matches!(
        gs.replace_range(0..5, "toolong!"),
        Err(GStringError::TooLong(5))
    ));
    assert_eq!(gs.as_str(), "hello");
}

#[test]
fn replace_range_unbounded() {
    let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new("hello").unwrap();
    gs.replace_range(.., "world").unwrap();
    assert_eq!(gs.as_str(), "world");
}

// -------------------------------------------------------------------------
// try_extend
// -------------------------------------------------------------------------

#[test]
fn try_extend_appends_str_slices() {
    // (initial, parts, expected)
    let cases: &[(&str, &[&str], &str)] = &[
        ("", &["hello", " ", "world"], "hello world"),
        ("start", &["-mid-", "end"], "start-mid-end"),
        ("abc", &[], "abc"),
        ("", &["🦀", "🎉"], "🦀🎉"),
    ];

    for &(initial, parts, expected) in cases {
        // String oracle
        let oracle: String = parts.iter().fold(initial.to_string(), |mut acc, s| {
            acc.push_str(s);
            acc
        });

        let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new(initial).unwrap();
        gs.try_extend(parts.iter().copied()).unwrap();
        assert_eq!(
            gs.as_str(),
            oracle.as_str(),
            "try_extend {:?} onto {:?}",
            parts,
            initial
        );
        assert_eq!(
            gs.as_str(),
            expected,
            "try_extend {:?} onto {:?}",
            parts,
            initial
        );
    }
}

#[test]
fn try_extend_too_long_is_atomic() {
    // Overflow mid-extend: original must be unchanged
    let mut gs: GString<NoValidation, 0, 8, false> = GString::try_new("hello").unwrap();
    assert!(matches!(
        gs.try_extend(["123", "456"].iter().copied()),
        Err(GStringError::TooLong(8))
    ));
    assert_eq!(gs.as_str(), "hello");
}

// -------------------------------------------------------------------------
// try_extend_chars
// -------------------------------------------------------------------------

#[test]
fn try_extend_chars_appends_chars() {
    // (initial, chars, expected)
    let cases: &[(&str, &[char], &str)] = &[
        ("", &['h', 'e', 'l', 'l', 'o'], "hello"),
        ("hi", &['!'], "hi!"),
        ("abc", &[], "abc"),
        ("", &['🦀', 'α'], "🦀α"),
    ];

    for &(initial, chars, expected) in cases {
        let oracle: String = initial.chars().chain(chars.iter().copied()).collect();

        let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new(initial).unwrap();
        gs.try_extend_chars(chars.iter().copied()).unwrap();
        assert_eq!(
            gs.as_str(),
            oracle.as_str(),
            "try_extend_chars {:?} onto {:?}",
            chars,
            initial
        );
        assert_eq!(
            gs.as_str(),
            expected,
            "try_extend_chars {:?} onto {:?}",
            chars,
            initial
        );
    }
}

#[test]
fn try_extend_chars_too_long_is_atomic() {
    let mut gs: GString<NoValidation, 0, 3, false> = GString::try_new("abc").unwrap();
    assert!(matches!(
        gs.try_extend_chars(['x', 'y']),
        Err(GStringError::TooLong(3))
    ));
    assert_eq!(gs.as_str(), "abc");
}

// -------------------------------------------------------------------------
// fmt::Write
// -------------------------------------------------------------------------

#[test]
fn write_str_via_fmt_write() {
    use core::fmt::Write;

    let cases: &[(&str, &str, &str)] = &[
        ("", "hello", "hello"),
        ("hello", " world", "hello world"),
        ("abc", "", "abc"),
    ];

    for &(initial, append, expected) in cases {
        let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new(initial).unwrap();
        write!(gs, "{}", append).unwrap();
        assert_eq!(
            gs.as_str(),
            expected,
            "write! {:?} onto {:?}",
            append,
            initial
        );
    }
}

#[test]
fn write_char_via_fmt_write() {
    use core::fmt::Write;

    let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new("hello").unwrap();
    write!(gs, "{}", '!').unwrap();
    assert_eq!(gs.as_str(), "hello!");
}

#[test]
fn write_format_args() {
    use core::fmt::Write;

    let mut gs: GString<NoValidation, 0, 64, false> = GString::try_new("").unwrap();
    write!(gs, "{}-{}", "hello", 42).unwrap();
    assert_eq!(gs.as_str(), "hello-42");
}

// -------------------------------------------------------------------------
// Idempotency: failed mutations must leave the string unchanged
// -------------------------------------------------------------------------

#[test]
fn failed_mutation_leaves_string_unchanged() {
    let original = "hello";

    let mut gs: GString<NoValidation, 0, 5, false> = GString::try_new(original).unwrap();
    let _ = gs.push('x');
    assert_eq!(gs.as_str(), original);

    let mut gs: GString<NoValidation, 0, 5, false> = GString::try_new(original).unwrap();
    let _ = gs.push_str(" world");
    assert_eq!(gs.as_str(), original);

    let mut gs: GString<NoValidation, 0, 5, false> = GString::try_new(original).unwrap();
    let _ = gs.insert_str(0, "x");
    assert_eq!(gs.as_str(), original);

    let mut gs: GString<NoValidation, 3, 64, false> = GString::try_new(original).unwrap();
    let _ = gs.truncate(2);
    assert_eq!(gs.as_str(), original);

    let mut gs: GString<NoValidation, 1, 64, false> = GString::try_new(original).unwrap();
    let _ = gs.clear();
    assert_eq!(gs.as_str(), original);
}
