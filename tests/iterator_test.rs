use g_string::{GString, NoValidation};

type G = GString<NoValidation, 0, 64, false>;
type GAscii = GString<NoValidation, 0, 64, true>;

fn g(s: &str) -> G {
    G::try_new(s).unwrap()
}

fn ga(s: &str) -> GAscii {
    GAscii::try_new(s).unwrap()
}

// -------------------------------------------------------------------------
// Borrowed iteration (&GString)
// -------------------------------------------------------------------------

#[test]
fn borrowed_iter_matches_str() {
    let cases: &[&str] = &[
        "",
        "a",
        "hello",
        "hello world",
        "café",
        "日本語",
        "emoji 🦀",
        "mixed: abc αβγ 中文",
    ];

    for &input in cases {
        let gs = g(input);
        let oracle: Vec<char> = input.chars().collect();
        let got: Vec<char> = (&gs).into_iter().collect();
        assert_eq!(got, oracle, "borrowed iter failed for {:?}", input);
    }
}

#[test]
fn borrowed_iter_empty() {
    let gs = g("");
    let got: Vec<char> = (&gs).into_iter().collect();
    assert!(got.is_empty());
}

// -------------------------------------------------------------------------
// Mut-borrowed iteration (&mut GString)
// -------------------------------------------------------------------------

#[test]
fn mut_borrowed_iter_matches_str() {
    let cases: &[&str] = &["", "hello", "café", "日本語"];

    for &input in cases {
        let mut gs = g(input);
        let oracle: Vec<char> = input.chars().collect();
        let got: Vec<char> = (&mut gs).into_iter().collect();
        assert_eq!(got, oracle, "mut borrowed iter failed for {:?}", input);
    }
}

// -------------------------------------------------------------------------
// Owned iteration (IntoChars) — UTF-8
// -------------------------------------------------------------------------

#[test]
fn owned_iter_utf8_matches_string_oracle() {
    let cases: &[&str] = &[
        "",
        "a",
        "hello",
        "café",
        "日本語",
        "emoji 🦀",
        "αβγδ",
        "mixed: abc αβγ 中文 🎉",
    ];

    for &input in cases {
        let gs = g(input);
        let oracle: Vec<char> = input.chars().collect();
        let got: Vec<char> = gs.into_iter().collect();
        assert_eq!(got, oracle, "owned utf8 iter failed for {:?}", input);
    }
}

#[test]
fn owned_iter_ascii_matches_string_oracle() {
    let cases: &[&str] = &["", "a", "hello", "ABCDEFGHIJ", "0123456789", "!@#$%"];

    for &input in cases {
        let gs = ga(input);
        let oracle: Vec<char> = input.chars().collect();
        let got: Vec<char> = gs.into_iter().collect();
        assert_eq!(got, oracle, "owned ascii iter failed for {:?}", input);
    }
}

// -------------------------------------------------------------------------
// Reverse iteration (DoubleEndedIterator)
// -------------------------------------------------------------------------

#[test]
fn rev_iter_utf8_matches_string_oracle() {
    let cases: &[&str] = &[
        "",
        "a",
        "hello",
        "café",
        "日本語",
        "🦀🎉🌍",
        "αβγδ",
        "mixed abc 中文",
    ];

    for &input in cases {
        let gs = g(input);
        let oracle: Vec<char> = input.chars().rev().collect();
        let got: Vec<char> = gs.into_iter().rev().collect();
        assert_eq!(got, oracle, "rev utf8 iter failed for {:?}", input);
    }
}

#[test]
fn rev_iter_ascii_matches_string_oracle() {
    let cases: &[&str] = &["", "a", "hello", "ABCDE", "12345"];

    for &input in cases {
        let gs = ga(input);
        let oracle: Vec<char> = input.chars().rev().collect();
        let got: Vec<char> = gs.into_iter().rev().collect();
        assert_eq!(got, oracle, "rev ascii iter failed for {:?}", input);
    }
}

// -------------------------------------------------------------------------
// Alternating next() / next_back()
// -------------------------------------------------------------------------

#[test]
fn alternating_next_next_back_utf8() {
    let cases: &[&str] = &["", "a", "ab", "abc", "abcd", "abcde", "café", "αβγδε"];

    for &input in cases {
        let gs = g(input);
        let mut iter = gs.into_iter();

        let chars: Vec<char> = input.chars().collect();
        let mut front = 0usize;
        let mut back = chars.len();
        let mut result = Vec::new();
        let mut from_front = true;

        loop {
            if front >= back {
                break;
            }
            if from_front {
                let ch = iter.next();
                assert_eq!(
                    ch,
                    Some(chars[front]),
                    "next() mismatch at front={} for {:?}",
                    front,
                    input
                );
                result.push(ch.unwrap());
                front += 1;
            } else {
                back -= 1;
                let ch = iter.next_back();
                assert_eq!(
                    ch,
                    Some(chars[back]),
                    "next_back() mismatch at back={} for {:?}",
                    back,
                    input
                );
                result.push(ch.unwrap());
            }
            from_front = !from_front;
        }

        // Both ends exhausted: both should return None
        assert_eq!(
            iter.next(),
            None,
            "expected None from next() after exhaustion for {:?}",
            input
        );
        assert_eq!(
            iter.next_back(),
            None,
            "expected None from next_back() after exhaustion for {:?}",
            input
        );
    }
}

// -------------------------------------------------------------------------
// Fused semantics
// -------------------------------------------------------------------------

#[test]
fn fused_none_after_exhaustion_utf8() {
    let cases: &[&str] = &["", "a", "hello", "日本語"];

    for &input in cases {
        let gs = g(input);
        let mut iter = gs.into_iter();
        while iter.next().is_some() {}
        // must remain None forever
        for _ in 0..5 {
            assert_eq!(iter.next(), None, "fused violation for {:?}", input);
        }
    }
}

#[test]
fn fused_none_after_exhaustion_ascii() {
    let cases: &[&str] = &["", "a", "hello"];

    for &input in cases {
        let gs = ga(input);
        let mut iter = gs.into_iter();
        while iter.next().is_some() {}
        for _ in 0..5 {
            assert_eq!(iter.next(), None, "ascii fused violation for {:?}", input);
        }
    }
}

#[test]
fn fused_none_after_exhaustion_rev() {
    let cases: &[&str] = &["", "a", "hello", "café"];

    for &input in cases {
        let gs = g(input);
        let mut iter = gs.into_iter();
        while iter.next_back().is_some() {}
        for _ in 0..5 {
            assert_eq!(
                iter.next_back(),
                None,
                "rev fused violation for {:?}",
                input
            );
            assert_eq!(
                iter.next(),
                None,
                "fwd after rev fused violation for {:?}",
                input
            );
        }
    }
}

// -------------------------------------------------------------------------
// size_hint
// -------------------------------------------------------------------------

#[test]
fn size_hint_ascii_exact() {
    let cases: &[(&str, usize)] = &[("", 0), ("a", 1), ("hello", 5), ("0123456789", 10)];

    for &(input, expected_remaining) in cases {
        let gs = ga(input);
        let iter = gs.into_iter();
        let (lo, hi) = iter.size_hint();
        assert_eq!(
            lo, expected_remaining,
            "size_hint lo wrong for ascii {:?}",
            input
        );
        assert_eq!(
            hi,
            Some(expected_remaining),
            "size_hint hi wrong for ascii {:?}",
            input
        );
    }
}

#[test]
fn size_hint_ascii_decrements_correctly() {
    let input = "hello";
    let gs = ga(input);
    let mut iter = gs.into_iter();

    for remaining in (0..=5).rev() {
        let (lo, hi) = iter.size_hint();
        assert_eq!(lo, remaining);
        assert_eq!(hi, Some(remaining));
        iter.next();
    }
}

#[test]
fn size_hint_utf8_upper_bound() {
    // Upper bound is byte count, lower bound is at least 1 (div_ceil(4))
    let cases: &[&str] = &["", "a", "café", "日本語", "🦀🎉"];

    for &input in cases {
        let gs = g(input);
        let iter = gs.into_iter();
        let (lo, hi) = iter.size_hint();
        let oracle_chars = input.chars().count();
        let byte_len = input.len();

        // hi must be >= actual char count
        assert!(
            hi.unwrap() >= oracle_chars,
            "size_hint hi too small for {:?}: hi={:?} but {} chars",
            input,
            hi,
            oracle_chars
        );
        // hi must equal byte_len (upper bound)
        assert_eq!(
            hi,
            Some(byte_len),
            "size_hint hi should be byte_len for {:?}",
            input
        );
        // lo must be >= 1 for non-empty (div_ceil(4))
        if !input.is_empty() {
            assert!(
                lo >= 1,
                "size_hint lo should be >= 1 for non-empty {:?}",
                input
            );
        }
    }
}

// -------------------------------------------------------------------------
// collect() round-trip
// -------------------------------------------------------------------------

#[test]
fn collect_round_trip() {
    let cases: &[&str] = &["hello", "café", "αβγ", "🦀", "mixed 日本語 abc"];

    for &input in cases {
        let gs = g(input);
        let collected: String = gs.into_iter().collect();
        assert_eq!(
            collected, input,
            "collect round-trip failed for {:?}",
            input
        );
    }
}

#[test]
fn collect_rev_round_trip() {
    let cases: &[&str] = &["hello", "café", "αβγ", "🦀🎉🌍"];

    for &input in cases {
        let gs = g(input);
        let oracle: String = input.chars().rev().collect();
        let got: String = gs.into_iter().rev().collect();
        assert_eq!(got, oracle, "rev collect failed for {:?}", input);
    }
}

// -------------------------------------------------------------------------
// Iterator adapters (map, filter, enumerate)
// -------------------------------------------------------------------------

#[test]
fn iter_map_uppercase_matches_oracle() {
    let cases: &[&str] = &["hello", "world", "café"];

    for &input in cases {
        let gs = g(input);
        let oracle: String = input
            .chars()
            .map(|c| c.to_uppercase().next().unwrap())
            .collect();
        let got: String = gs
            .into_iter()
            .map(|c| c.to_uppercase().next().unwrap())
            .collect();
        assert_eq!(got, oracle, "map uppercase failed for {:?}", input);
    }
}

#[test]
fn iter_filter_alpha_matches_oracle() {
    let cases: &[&str] = &["he110", "w0r1d", "c4f3", "abc123"];

    for &input in cases {
        let gs = g(input);
        let oracle: String = input.chars().filter(|c| c.is_alphabetic()).collect();
        let got: String = gs.into_iter().filter(|c| c.is_alphabetic()).collect();
        assert_eq!(got, oracle, "filter alpha failed for {:?}", input);
    }
}

#[test]
fn iter_enumerate_matches_oracle() {
    let cases: &[&str] = &["", "a", "hello", "αβγ"];

    for &input in cases {
        let gs = g(input);
        let oracle: Vec<(usize, char)> = input.chars().enumerate().collect();
        let got: Vec<(usize, char)> = gs.into_iter().enumerate().collect();
        assert_eq!(got, oracle, "enumerate failed for {:?}", input);
    }
}

// -------------------------------------------------------------------------
// Single-char edge cases
// -------------------------------------------------------------------------

#[test]
fn single_ascii_char() {
    let gs = g("x");
    let mut iter = gs.into_iter();
    assert_eq!(iter.next(), Some('x'));
    assert_eq!(iter.next(), None);
}

#[test]
fn single_multibyte_char() {
    // '日' is 3 bytes in UTF-8
    let gs = g("日");
    let mut iter = gs.into_iter();
    assert_eq!(iter.next(), Some('日'));
    assert_eq!(iter.next(), None);
}

#[test]
fn single_4byte_char() {
    // '🦀' is 4 bytes in UTF-8
    let gs = g("🦀");
    let mut iter = gs.into_iter();
    assert_eq!(iter.next(), Some('🦀'));
    assert_eq!(iter.next(), None);
}

#[test]
fn single_char_rev() {
    let gs = g("🦀");
    let mut iter = gs.into_iter();
    assert_eq!(iter.next_back(), Some('🦀'));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
}
