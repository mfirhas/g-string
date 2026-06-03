use g_string::GString;

const CASES: &[&str] = &[
    "",
    "hello",
    "a,b,c",
    "abcabc",
    "héllo",
    "🦀rust🦀",
    " line1 \n line2 ",
    "\t hello \n",
    "🥷🔥🌏 🌎 🌍 👨‍👩‍👧‍👦",
];

#[test]
fn basic_methods() {
    for input in CASES {
        let s: GString = GString::try_new(input).unwrap();

        assert_eq!(s.len(), input.len());
        assert_eq!(s.count(), input.chars().count());
        assert_eq!(s.is_empty(), input.is_empty());
        assert_eq!(s.as_str(), *input);
        assert_eq!(s.as_bytes(), input.as_bytes());

        for i in 0..=input.len() {
            assert_eq!(s.is_char_boundary(i), input.is_char_boundary(i));
        }

        assert_eq!(s.capacity(), 255);
        assert_eq!(s.is_full(), input.len() == s.capacity());
    }
}

#[test]
fn chars_char_indices_bytes() {
    for input in CASES {
        let s: GString = GString::try_new(input).unwrap();

        assert_eq!(
            s.chars().collect::<Vec<_>>(),
            input.chars().collect::<Vec<_>>()
        );

        assert_eq!(
            s.char_indices().collect::<Vec<_>>(),
            input.char_indices().collect::<Vec<_>>()
        );

        assert_eq!(
            s.bytes().collect::<Vec<_>>(),
            input.bytes().collect::<Vec<_>>()
        );
    }
}

#[test]
fn graphemes() {
    for input in CASES {
        let s = GString::try_default(input).unwrap();

        for g in s.graphemes() {
            let iter_s = GString::try_default(g).unwrap();
            assert_eq!(iter_s.grapheme_count(), 1);
        }
    }
}

#[test]
fn search_char() {
    let pats = ['a', 'b', ',', '🦀', 'é', 'x'];

    for input in CASES {
        let s: GString = GString::try_new(input).unwrap();

        for pat in pats {
            assert_eq!(s.contains(pat), input.contains(pat));
            assert_eq!(s.starts_with(pat), input.starts_with(pat));
            assert_eq!(s.ends_with(pat), input.ends_with(pat));
            assert_eq!(s.find(pat), input.find(pat));
            assert_eq!(s.rfind(pat), input.rfind(pat));
        }
    }
}

#[test]
fn search_str() {
    let pats = ["", "a", "ab", ",", "🦀", "é", "xyz"];

    for input in CASES {
        let s: GString = GString::try_new(input).unwrap();

        for pat in pats {
            assert_eq!(s.contains(pat), input.contains(pat));
            assert_eq!(s.starts_with(pat), input.starts_with(pat));
            assert_eq!(s.ends_with(pat), input.ends_with(pat));
            assert_eq!(s.find(pat), input.find(pat));
            assert_eq!(s.rfind(pat), input.rfind(pat));
        }
    }
}

#[test]
fn get_matches_str() {
    let cases = [("hello", 0..2), ("héllo", 0..2), ("🦀rust🦀", 0..4)];

    for (input, range) in cases {
        let s: GString = GString::try_new(input).unwrap();

        assert_eq!(s.get(range.clone()), input.get(range));
    }
}

#[test]
fn split_char() {
    let pats = [',', 'a', '🦀'];

    for input in CASES {
        let s: GString = GString::try_new(input).unwrap();

        for pat in pats {
            assert_eq!(
                s.split(pat).collect::<Vec<_>>(),
                input.split(pat).collect::<Vec<_>>()
            );

            assert_eq!(s.split_once(pat), input.split_once(pat));

            assert_eq!(s.rsplit_once(pat), input.rsplit_once(pat));
        }
    }
}

#[test]
fn split_str() {
    let pats = ["", ",", "ab", "🦀"];

    for input in CASES {
        let s: GString = GString::try_new(input).unwrap();

        for pat in pats {
            assert_eq!(
                s.split(pat).collect::<Vec<_>>(),
                input.split(pat).collect::<Vec<_>>()
            );

            assert_eq!(s.split_once(pat), input.split_once(pat));

            assert_eq!(s.rsplit_once(pat), input.rsplit_once(pat));
        }
    }
}

#[test]
fn split_whitespace_and_lines() {
    for input in CASES {
        let s: GString = GString::try_new(input).unwrap();

        assert_eq!(
            s.split_whitespace().collect::<Vec<_>>(),
            input.split_whitespace().collect::<Vec<_>>()
        );

        assert_eq!(
            s.lines().collect::<Vec<_>>(),
            input.lines().collect::<Vec<_>>()
        );
    }
}

#[test]
fn trimming() {
    for input in CASES {
        let s: GString = GString::try_new(input).unwrap();

        assert_eq!(s.try_trim().unwrap().as_str(), input.trim());

        assert_eq!(s.try_trim_start().unwrap().as_str(), input.trim_start());

        assert_eq!(s.try_trim_end().unwrap().as_str(), input.trim_end());
    }
}

#[test]
fn strip_char() {
    let pats = ['a', 'h', '🦀'];

    for input in CASES {
        let s: GString = GString::try_new(input).unwrap();

        for pat in pats {
            assert_eq!(s.strip_prefix(pat), input.strip_prefix(pat));

            assert_eq!(s.strip_suffix(pat), input.strip_suffix(pat));
        }
    }
}

#[test]
fn strip_str() {
    let pats = ["", "a", "he", "🦀"];

    for input in CASES {
        let s: GString = GString::try_new(input).unwrap();

        for pat in pats {
            assert_eq!(s.strip_prefix(pat), input.strip_prefix(pat));

            assert_eq!(s.strip_suffix(pat), input.strip_suffix(pat));
        }
    }
}

#[test]
fn matches_char() {
    let pats = ['a', ',', '🦀'];

    for input in CASES {
        let s: GString = GString::try_new(input).unwrap();

        for pat in pats {
            assert_eq!(
                s.matches(pat).collect::<Vec<_>>(),
                input.matches(pat).collect::<Vec<_>>()
            );

            assert_eq!(
                s.rmatches(pat).collect::<Vec<_>>(),
                input.rmatches(pat).collect::<Vec<_>>()
            );

            assert_eq!(
                s.match_indices(pat).collect::<Vec<_>>(),
                input.match_indices(pat).collect::<Vec<_>>()
            );

            assert_eq!(
                s.rmatch_indices(pat).collect::<Vec<_>>(),
                input.rmatch_indices(pat).collect::<Vec<_>>()
            );
        }
    }
}

#[test]
fn matches_str() {
    let pats = ["", "ab", ",", "🦀"];

    for input in CASES {
        let s: GString = GString::try_new(input).unwrap();

        for pat in pats {
            assert_eq!(
                s.matches(pat).collect::<Vec<_>>(),
                input.matches(pat).collect::<Vec<_>>()
            );

            assert_eq!(
                s.rmatches(pat).collect::<Vec<_>>(),
                input.rmatches(pat).collect::<Vec<_>>()
            );

            assert_eq!(
                s.match_indices(pat).collect::<Vec<_>>(),
                input.match_indices(pat).collect::<Vec<_>>()
            );

            assert_eq!(
                s.rmatch_indices(pat).collect::<Vec<_>>(),
                input.rmatch_indices(pat).collect::<Vec<_>>()
            );
        }
    }
}

#[test]
fn parse_is_ascii_case_insensitive() {
    let s: GString = GString::try_new("123").unwrap();
    assert_eq!(s.parse::<u32>(), "123".parse::<u32>());

    for input in CASES {
        let s: GString = GString::try_new(input).unwrap();

        assert_eq!(s.is_ascii(), input.is_ascii());

        assert_eq!(
            s.eq_ignore_ascii_case("HELLO"),
            input.eq_ignore_ascii_case("HELLO")
        );
    }
}

#[test]
fn escaping() {
    let cases = ["", "hello", "\n\t", "🦀", "\"quoted\""];

    for input in cases {
        let s: GString = GString::try_new(input).unwrap();

        assert_eq!(
            s.escape_debug().to_string(),
            input.escape_debug().to_string()
        );

        assert_eq!(
            s.escape_default().to_string(),
            input.escape_default().to_string()
        );

        assert_eq!(
            s.escape_unicode().to_string(),
            input.escape_unicode().to_string()
        );
    }
}

#[test]
fn encode_utf16() {
    for input in CASES {
        let s: GString = GString::try_new(input).unwrap();

        assert_eq!(
            s.encode_utf16().collect::<Vec<_>>(),
            input.encode_utf16().collect::<Vec<_>>()
        );
    }
}
