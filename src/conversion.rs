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
#[cfg(feature = "alloc")]
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> TryFrom<String>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Error = GStringError<V::Err>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(&value)
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

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    pub fn try_from_iter<I, S>(iter: I) -> Result<Self, GStringError<V::Err>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut buf = [0u8; MAX];
        let mut len = 0;

        for s in iter {
            let s = s.as_ref();
            let bytes = s.as_bytes();

            let end = len + bytes.len();

            if end > MAX {
                return Err(GStringError::TooLong(MAX));
            }

            buf[len..end].copy_from_slice(bytes);

            len = end;
        }

        // SAFETY:
        // concatenation of valid UTF-8 strings is valid UTF-8
        let candidate = unsafe { core::str::from_utf8_unchecked(&buf[..len]) };

        Self::try_new(candidate)
    }

    pub fn try_from_chars<I>(iter: I) -> Result<Self, GStringError<V::Err>>
    where
        I: IntoIterator<Item = char>,
    {
        let mut buf = [0u8; MAX];
        let mut len = 0;

        for ch in iter {
            let mut tmp = [0u8; 4];
            let encoded = ch.encode_utf8(&mut tmp);

            let bytes = encoded.as_bytes();
            let end = len + bytes.len();

            if end > MAX {
                return Err(GStringError::TooLong(MAX));
            }

            buf[len..end].copy_from_slice(bytes);

            len = end;
        }

        // SAFETY:
        // buf always comes from UTF-8 encoded char
        let candidate = unsafe { core::str::from_utf8_unchecked(&buf[..len]) };

        Self::try_new(candidate)
    }
}
