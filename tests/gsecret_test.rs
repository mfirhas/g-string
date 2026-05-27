use g_string::{GSecret, NoValidation};
use std::convert::TryFrom;
use std::string::String;

type Secret<const MIN: usize, const MAX: usize, const ASCII: bool> =
    GSecret<NoValidation, MIN, MAX, ASCII>;

// ------------------------------------------------------------
// 1. Construction tests
// ------------------------------------------------------------

#[test]
fn try_new_success() {
    let s = Secret::<0, 64, false>::try_new("hello").unwrap();

    s.expose(|v| {
        assert_eq!(v, "hello");
    });
}

#[test]
fn try_new_via_from_str() {
    let s: Secret<0, 64, false> = "hello".parse().unwrap();

    s.expose(|v| {
        assert_eq!(v, "hello");
    });
}

#[test]
fn try_from_str_ref() {
    let s = Secret::<0, 64, false>::try_from("hello").unwrap();

    s.expose(|v| {
        assert_eq!(v, "hello");
    });
}

#[test]
fn try_from_string() {
    let s = Secret::<0, 64, false>::try_from(String::from("hello")).unwrap();

    s.expose(|v| {
        assert_eq!(v, "hello");
    });
}

// ------------------------------------------------------------
// 2. Clone behavior
// ------------------------------------------------------------

#[test]
fn clone_creates_equal_secret() {
    let a = Secret::<0, 64, false>::try_new("secret").unwrap();
    let b = a.clone();

    assert_eq!(a, b);

    a.expose(|va| {
        b.expose(|vb| {
            assert_eq!(va, vb);
        })
    });
}

// ------------------------------------------------------------
// 3. Equality / Hash behavior
// ------------------------------------------------------------

#[test]
fn equality_works() {
    let a = Secret::<0, 64, false>::try_new("abc").unwrap();
    let b = Secret::<0, 64, false>::try_new("abc").unwrap();

    assert_eq!(a, b);
}

#[test]
fn inequality_works() {
    let a = Secret::<0, 64, false>::try_new("abc").unwrap();
    let b = Secret::<0, 64, false>::try_new("def").unwrap();

    assert_ne!(a, b);
}

#[test]
fn hash_consistency() {
    use std::collections::HashSet;

    let a = Secret::<0, 64, false>::try_new("abc").unwrap();
    let b = a.clone();

    let mut set = HashSet::new();
    set.insert(a);

    assert!(set.contains(&b));
}

// ------------------------------------------------------------
// 4. expose API correctness
// ------------------------------------------------------------

#[test]
fn expose_does_not_escape() {
    let s = Secret::<0, 64, false>::try_new("secret").unwrap();

    let len = s.expose(|v| {
        assert_eq!(v, "secret");
        v.len()
    });

    assert_eq!(len, 6);
}

#[test]
fn expose_allows_transformation() {
    let s = Secret::<0, 64, false>::try_new("abc").unwrap();

    let upper = s.expose(|v| v.to_uppercase());

    assert_eq!(upper, "ABC");
}

// ------------------------------------------------------------
// 5. zeroize API
// ------------------------------------------------------------

#[test]
fn manual_inherent_zeroize_clears_data() {
    let mut s = Secret::<0, 64, false>::try_new("secret").unwrap();

    s.zeroize();

    s.expose(|v| {
        // behavior depends on your GString.zeroize implementation
        assert!(v.is_empty() || v == "");
    });
}

#[test]
fn manual_zeroize_clears_data() {
    let mut s = GSecret::try_default("secret").unwrap();

    <GSecret as zeroize::Zeroize>::zeroize(&mut s);

    s.expose(|v| {
        // behavior depends on your GString.zeroize implementation
        assert!(v.is_empty() || v == "");
    });
}

// ------------------------------------------------------------
// 6. Drop behavior (best-effort test)
// ------------------------------------------------------------

#[test]
fn drop_does_not_panic() {
    let s = Secret::<0, 64, false>::try_new("secret").unwrap();

    drop(s);
}

// ------------------------------------------------------------
// 7. Debug safety
// ------------------------------------------------------------

#[test]
fn debug_is_redacted() {
    let s = Secret::<0, 64, false>::try_new("secret").unwrap();

    let debug = format!("{:?}", s);

    assert_eq!(debug, "GSecret(<REDACTED>)");
}

// ------------------------------------------------------------
// 8. TryFrom<String> zeroization behavior (best-effort)
// ------------------------------------------------------------

#[test]
fn try_from_string_zeroizes_input() {
    let mut input = String::from("secret");

    let _ = Secret::<0, 64, false>::try_from(input.clone());

    // We cannot reliably assert memory zeroization,
    // but we can ensure API does not panic and consumes safely.
    input.clear();
}
