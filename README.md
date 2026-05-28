# `g-string`

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
[![Crates.io](https://img.shields.io/crates/v/g-string.svg)](https://crates.io/crates/g-string)
[![ci](https://github.com/mfirhas/g-string/actions/workflows/ci.yml/badge.svg)](https://github.com/mfirhas/g-string/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/g-string/badge.svg)](https://docs.rs/g-string)
[![codecov](https://codecov.io/gh/mfirhas/g-string/branch/master/graph/badge.svg)](https://codecov.io/gh/mfirhas/g-string)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/mfirhas/g-string/blob/master/LICENSE)

`g-string` — a stack-allocated, `Copy`, and generically configurable UTF-8 string type featuring:

* Stack allocation with no heap allocation in core operations.
* `Copy` semantics for lightweight fixed-capacity strings.
* Compile-time configurable invariants:
  * minimum length
  * maximum length
  * ASCII-only enforcement
  * custom validation logic
* Full `str` interoperability through `Deref<Target = str>`.
* Mutation APIs that preserve all invariants.
* Conversions to and from standard string-related types.
* `serde` support for serialization and deserialization.
* `GSecret` for secret storage with zeroization and redacted formatting.
* `no_std` compatibility with optional `alloc` and `std` features.

## Overview

`g-string` provides stack-allocated, bounded UTF-8 string types designed for predictable memory usage, validation, and zero-allocation operation.

Unlike `String`, which always allocates on the heap, `g-string` stores its contents inline using a fixed-capacity buffer determined at compile time. This makes it suitable for embedded systems, constrained environments, protocol types, validated domain strings, and performance-sensitive applications where heap allocation is undesirable.

The crate centers around two primary types:

* `GString` — a validated bounded string type.
* `GSecret` — a secret string type with zeroization and redacted formatting.

### **GString**
`GString` is generically configurable through const generics and validation traits:

* minimum length
* maximum length
* ASCII-only restriction
* custom validation logic

All construction and mutation APIs preserve these invariants automatically.

`g-string` maintains full interoperability with Rust’s string ecosystem through `Deref<Target = str>`, conversion traits, iterator support, formatting traits, and optional `serde` integration.

Core goals of the `GString` include:

* deterministic memory usage
* zero heap allocation in core operations
* UTF-8 correctness
* ergonomic `str` interoperability
* compile-time configurability
* invariant-preserving mutation APIs
* `no_std` compatibility
* validation embedded into type

### **GSecret**
`GSecret` is wrapper around `GString` providing type for secret information such as password, API keys, tokens, private keys, etc. Unlike `GString`, `GSecret` is not Copy for security reason,
to prevent it from having multiple copies in many places. It's an owned type instead, getting moved and having scope before getting dropped. Normally, when a string or buffer is dropped, the memory is simply marked as reusable. The bytes are usually not overwritten immediately. That means the old secret may still physically exist in RAM until something else reuses that memory region. It's even worse in garbage-collected languages,
where secrets might linger longer before getting collected. This becomes a problem in scenarios such as:
- memory dumps
- crash reports
- swap/pagefile leakage
- use-after-free bugs
- forensic analysis
- accidental memory exposure
- debugging tools
- cold boot attacks

To avoid this lingering secrets, we need *zeroization*. `GSecret` is equipped with this mechanism to zeroizes the secrets automatically on drop(or manually). So, even the memory is still there, the data already zeroized.
This zeroization also avoid compiler optimization like dead-code elimination and optimization removal. `GSecret` has no Display implementation and debug is redacted. To work with the secret, method `reveal` is provided
making sure secret reference stays within closure scope. This type prevents secrets from leaking easily.

## Example

```rust
use core::fmt;

use g_string::{GSecret, GString, Validator, gstring, gformat};

#[derive(Copy, Clone, PartialEq, Eq)]
struct UsernameValidator;

#[derive(Debug)]
struct UsernameError(&'static str);

impl fmt::Display for UsernameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl std::error::Error for UsernameError {}

impl Validator for UsernameValidator {
    type Err = UsernameError;

    fn validate(s: impl AsRef<str>) -> Result<(), Self::Err> {
        if s.as_ref().chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            Ok(())
        } else {
            Err(UsernameError("username contains invalid characters"))
        }
    }
}

#[derive(Debug, Clone)]
struct AlphaOnly;

impl Validator for AlphaOnly {
    type Err = UsernameError;

    fn validate(s: impl AsRef<str>) -> Result<(), Self::Err> {
        if s.as_ref().chars().all(|c| c.is_ascii_alphabetic() || c == '_') {
            Ok(())
        } else {
            Err(UsernameError("may only contain ASCII alphabets"))
        }
    }
}

type Username = GString<UsernameValidator, 3, 16, true>;
type Password = GSecret<UsernameValidator, 8, 64, false>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut username = Username::try_new("alice_01")?;

    username.push('x')?;

    assert_eq!(username.as_str(), "alice_01x");
    assert!(Username::try_new("ab").is_err());
    assert!(Username::try_new("alice-01").is_err());

    let mut password = Password::try_new("super_secret_password")?;

    println!("{username}");
    println!("{password:?}"); // GSecret(<REDACTED>)

    password.zeroize(); // now password is emptied, not only visually, but also got zeroized in memory.
    password.reveal(|p| assert_eq!(p, "", "should be emptied"));

    let ret = gstring!("im checked at compile time").validate().unwrap();
    // let ret = gstring!("im failed at compile time because I violate invariant", (), 2, 10); // won't even compile
    let ret = gformat!("format: {} {}", ret, "🔥"; (), 2, 100).unwrap();
    assert_eq!(ret, "format: im checked at compile time 🔥");
    let ret = gformat!("im failed at validation: {}", "😞"; AlphaOnly, 0, 100, true);
    assert!(ret.is_err());

    Ok(())
}

```

## Features
### `default`, `std`, `alloc`
Default feature enable `std`, which also enable `alloc`. If you want `no_std`, disable default feature.

### `secret`
Enable `GSecret` and all of its related attributes and methods, including secret serde.
GSecret has no serialization implementation, for security purpose. You can implement it if you need.

### `serde`
Enable serde for both `GString` and `GSecret`(if `secret` also enabled).

### `grapheme`
Enable counting number of grapheme clusters for `GString`.
