use g_string::{GString, NoValidation, Validator};
use serde::{Deserialize, Serialize};
use serde_json;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

type Gs<const MIN: usize, const MAX: usize> = GString<NoValidation, MIN, MAX, false>;
type AsciiGs<const MIN: usize, const MAX: usize> = GString<NoValidation, MIN, MAX, true>;

/// A validator that only accepts lowercase ASCII letters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LowercaseOnly;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LowercaseError;

impl core::fmt::Display for LowercaseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "only lowercase ASCII letters are allowed")
    }
}

impl core::error::Error for LowercaseError {}

impl Validator for LowercaseOnly {
    type Err = LowercaseError;

    fn validate(s: impl AsRef<str>) -> Result<(), Self::Err> {
        if s.as_ref().chars().all(|c| c.is_ascii_lowercase()) {
            Ok(())
        } else {
            Err(LowercaseError)
        }
    }
}

type LcGs<const MIN: usize, const MAX: usize> = GString<LowercaseOnly, MIN, MAX, false>;

// ---------------------------------------------------------------------------
// Serialization
// ---------------------------------------------------------------------------

#[test]
fn serialize_to_json_string() {
    let gs: Gs<0, 32> = GString::try_new("hello").unwrap();
    let json = serde_json::to_string(&gs).unwrap();
    assert_eq!(json, r#""hello""#);
}

#[test]
fn serialize_empty_string() {
    let gs: Gs<0, 32> = GString::try_new("").unwrap();
    let json = serde_json::to_string(&gs).unwrap();
    assert_eq!(json, r#""""#);
}

#[test]
fn serialize_unicode_string() {
    let gs: Gs<0, 32> = GString::try_new("héllo").unwrap();
    let json = serde_json::to_string(&gs).unwrap();
    assert_eq!(json, r#""héllo""#);
}

#[test]
fn serialize_max_length_string() {
    let s = "a".repeat(16);
    let gs: Gs<0, 16> = GString::try_new(&s).unwrap();
    let json = serde_json::to_string(&gs).unwrap();
    assert_eq!(json, format!(r#""{s}""#));
}

#[test]
fn serialize_preserves_whitespace_and_special_chars() {
    let gs: Gs<0, 64> = GString::try_new("foo bar\tbaz").unwrap();
    let json = serde_json::to_string(&gs).unwrap();
    assert_eq!(json, r#""foo bar\tbaz""#);
}

// ---------------------------------------------------------------------------
// Deserialization — happy path
// ---------------------------------------------------------------------------

#[test]
fn deserialize_from_json_string() {
    let gs: Gs<0, 32> = serde_json::from_str(r#""hello""#).unwrap();
    assert_eq!(gs.as_str(), "hello");
}

#[test]
fn deserialize_empty_string() {
    let gs: Gs<0, 32> = serde_json::from_str(r#""""#).unwrap();
    assert_eq!(gs.as_str(), "");
}

#[test]
fn deserialize_unicode_string() {
    let gs: Gs<0, 32> = serde_json::from_str(r#""héllo""#).unwrap();
    assert_eq!(gs.as_str(), "héllo");
}

#[test]
fn deserialize_at_min_length() {
    let gs: Gs<3, 32> = serde_json::from_str(r#""abc""#).unwrap();
    assert_eq!(gs.as_str(), "abc");
}

#[test]
fn deserialize_at_max_length() {
    let s = format!(r#""{}""#, "x".repeat(8));
    let gs: Gs<0, 8> = serde_json::from_str(&s).unwrap();
    assert_eq!(gs.len(), 8);
}

// ---------------------------------------------------------------------------
// Deserialization — error cases
// ---------------------------------------------------------------------------

#[test]
fn deserialize_fails_too_short() {
    let result: Result<Gs<5, 32>, _> = serde_json::from_str(r#""hi""#);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("minimum length"), "unexpected error: {msg}");
}

#[test]
fn deserialize_fails_too_long() {
    let s = format!(r#""{}""#, "a".repeat(10));
    let result: Result<Gs<0, 5>, _> = serde_json::from_str(&s);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("maximum length"), "unexpected error: {msg}");
}

#[test]
fn deserialize_fails_non_ascii_when_ascii_only() {
    let result: Result<AsciiGs<0, 32>, _> = serde_json::from_str(r#""héllo""#);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("ASCII"), "unexpected error: {msg}");
}

#[test]
fn deserialize_ascii_only_passes_for_valid_ascii() {
    let gs: AsciiGs<0, 32> = serde_json::from_str(r#""hello""#).unwrap();
    assert_eq!(gs.as_str(), "hello");
}

#[test]
fn deserialize_fails_custom_validator() {
    let result: Result<LcGs<0, 32>, _> = serde_json::from_str(r#""Hello""#);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("lowercase"), "unexpected error: {msg}");
}

#[test]
fn deserialize_passes_custom_validator() {
    let gs: LcGs<0, 32> = serde_json::from_str(r#""hello""#).unwrap();
    assert_eq!(gs.as_str(), "hello");
}

// ---------------------------------------------------------------------------
// Round-trip
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_basic() {
    let original: Gs<0, 32> = GString::try_new("round-trip").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let restored: Gs<0, 32> = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}

#[test]
fn roundtrip_unicode() {
    let original: Gs<0, 64> = GString::try_new("日本語テスト").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let restored: Gs<0, 64> = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}

#[test]
fn roundtrip_with_validator() {
    let original: LcGs<3, 32> = GString::try_new("valid").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let restored: LcGs<3, 32> = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}

#[test]
fn roundtrip_ascii_only() {
    let original: AsciiGs<0, 32> = GString::try_new("ascii").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let restored: AsciiGs<0, 32> = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}

// ---------------------------------------------------------------------------
// Struct embedding
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct UserRecord {
    username: LcGs<3, 32>,
    display_name: Gs<1, 64>,
}

#[test]
fn serialize_embedded_in_struct() {
    let record = UserRecord {
        username: GString::try_new("alice").unwrap(),
        display_name: GString::try_new("Alice Smith").unwrap(),
    };
    let json = serde_json::to_string(&record).unwrap();
    assert!(json.contains(r#""username":"alice""#));
    assert!(json.contains(r#""display_name":"Alice Smith""#));
}

#[test]
fn deserialize_embedded_in_struct() {
    let json = r#"{"username":"bob","display_name":"Bob Jones"}"#;
    let record: UserRecord = serde_json::from_str(json).unwrap();
    assert_eq!(record.username.as_str(), "bob");
    assert_eq!(record.display_name.as_str(), "Bob Jones");
}

#[test]
fn deserialize_struct_fails_if_field_violates_constraint() {
    // username must be lowercase
    let json = r#"{"username":"Bob","display_name":"Bob Jones"}"#;
    let result: Result<UserRecord, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn roundtrip_embedded_in_struct() {
    let original = UserRecord {
        username: GString::try_new("charlie").unwrap(),
        display_name: GString::try_new("Charlie Brown").unwrap(),
    };
    let json = serde_json::to_string(&original).unwrap();
    let restored: UserRecord = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}

// deserializing invalid types
#[test]
fn test_numbers() {
    let json = r#"{"username":123, "display_name":"Bob Jones"}"#;
    let record = serde_json::from_str::<UserRecord>(json);
    assert!(record.is_err());
    assert!(
        record
            .unwrap_err()
            .to_string()
            .contains("a string with length between 3 and 32"),
    );
}

// deserializing invalid types
#[test]
fn test_numbers_ascii_only() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct UserRecord {
        username: GString<(), 3, 5, true>,
        display_name: Gs<1, 64>,
    }

    let json = r#"{"username":123, "display_name":"Bob Jones"}"#;
    let record = serde_json::from_str::<UserRecord>(json);
    assert!(record.is_err());
    assert!(
        record
            .unwrap_err()
            .to_string()
            .contains("a string with length between 3 and 5 (ASCII only)"),
    );
}

#[cfg(feature = "alloc")]
#[test]
fn visit_string_is_called() {
    use serde::de::IntoDeserializer;
    use serde::de::value::{Error as DeError, StringDeserializer};

    type TestGString = GString<NoValidation, 1, 64, false>;

    // StringDeserializer calls visit_string, not visit_str
    let deserializer: StringDeserializer<DeError> = "hello".to_owned().into_deserializer();

    let result = TestGString::deserialize(deserializer);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str(), "hello");
}
