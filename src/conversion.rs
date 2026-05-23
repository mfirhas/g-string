#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{borrow::ToOwned, string::String};

use core::str::FromStr;

use crate::{GString, Validator, error::GStringError};

/// &str -> GString
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> FromStr
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Err = GStringError<V::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_new(s)
    }
}

/// GString AS &str
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> AsRef<str>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// String -> GString
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> TryFrom<String>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Error = GStringError<V::Err>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

/// GString -> String
#[cfg(feature = "alloc")]
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    From<GString<V, MIN, MAX, ASCII_ONLY>> for String
{
    fn from(value: GString<V, MIN, MAX, ASCII_ONLY>) -> Self {
        value.as_str().to_owned()
    }
}

/// &str -> GString (try_into)
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> TryFrom<&str>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Error = GStringError<V::Err>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

/// GString AS &\[u8\]
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> AsRef<[u8]>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(feature = "alloc")]
impl<'a, V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    TryFrom<alloc::borrow::Cow<'a, str>> for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Error = GStringError<V::Err>;

    fn try_from(value: alloc::borrow::Cow<'a, str>) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}
