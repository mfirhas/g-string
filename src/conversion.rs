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

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    pub fn try_from_iter<I, S>(iter: I) -> Result<Self, GStringError<V::Err>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut ret = if MIN > 0 {
            Self::try_new("0")?
        } else {
            Self::try_new("")?
        };

        let mut iter_peek = iter.into_iter().peekable();

        if iter_peek.peek().is_none() {
            if MIN == 0 {
                return Self::try_new("");
            } else {
                return Err(GStringError::TooShort);
            }
        }

        for (i, s) in iter_peek.into_iter().enumerate() {
            if i == 0 {
                ret = Self::try_new(s)?;
            } else {
                ret.push_str(s.as_ref())?;
            }
        }

        if MIN > 0 {
            ret = Self::try_new(&ret[1..])?;
        }

        Ok(ret)
    }

    pub fn try_from_chars<I>(iter: I) -> Result<Self, GStringError<V::Err>>
    where
        I: IntoIterator<Item = char>,
    {
        let mut ret = if MIN > 0 {
            Self::try_new("0")?
        } else {
            Self::try_new("")?
        };

        let mut iter_peek = iter.into_iter().peekable();

        if iter_peek.peek().is_none() {
            if MIN == 0 {
                return Self::try_new("");
            } else {
                return Err(GStringError::TooShort);
            }
        }

        for (i, s) in iter_peek.into_iter().enumerate() {
            if i == 0 {
                let mut buf = [0u8; 4];
                let s_from_char = s.encode_utf8(&mut buf);
                ret = Self::try_new(s_from_char)?;
            } else {
                ret.push(s)?;
            }
        }

        if MIN > 0 {
            ret = Self::try_new(&ret[1..])?;
        }

        Ok(ret)
    }
}
