#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{borrow::ToOwned, string::String};

mod macros;

use core::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

pub const DEFAULT_MIN: usize = 0;
pub const DEFAULT_MAX: usize = 255;
pub const DEFAULT_ASCII_ONLY: bool = false;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Err {
    TooShort,
    TooLong,
    NotAscii,
}

impl Display for Err {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooShort => {
                write!(f, "string len is smaller than MIN")
            }
            Self::TooLong => {
                write!(f, "string len is bigger than MAX")
            }
            Self::NotAscii => {
                write!(f, "ASCII_ONLY is true, but not ascii")
            }
        }
    }
}

impl Error for Err {}

impl<VE> From<Err> for GStringError<VE> {
    fn from(value: Err) -> Self {
        match value {
            Err::TooShort => Self::TooShort,
            Err::TooLong => Self::TooLong,
            Err::NotAscii => Self::NotAscii,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GStringError<VE> {
    TooShort,
    TooLong,
    NotAscii,
    Validation(VE),
    Mutation(&'static str),
}

impl<VE: Display + Debug> Display for GStringError<VE> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooShort => {
                write!(f, "string len is smaller than MIN")
            }
            Self::TooLong => {
                write!(f, "string len is bigger than MAX")
            }
            Self::NotAscii => {
                write!(f, "ASCII_ONLY is true, but not ascii")
            }
            Self::Validation(err) => write!(f, "validation error: {}", err),
            Self::Mutation(err) => write!(f, "mutation error: {}", err),
        }
    }
}

impl<VE: Display + Debug> Error for GStringError<VE> {}

pub trait Validator: Copy {
    type Err: Error + Send + Sync + 'static;

    fn validate(s: impl AsRef<str>) -> Result<(), Self::Err>;
}

#[derive(Debug, Clone, Copy)]
pub struct NoValidation;

impl Validator for NoValidation {
    type Err = Infallible;

    fn validate(_: impl AsRef<str>) -> Result<(), Self::Err> {
        Ok(())
    }
}

#[derive(Copy, Clone, Eq)]
pub struct GString<
    V: Validator = NoValidation,
    const MIN: usize = DEFAULT_MIN,
    const MAX: usize = DEFAULT_MAX,
    const ASCII_ONLY: bool = DEFAULT_ASCII_ONLY,
> {
    buf: [u8; MAX],
    len: usize,
    _validator: PhantomData<V>,
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> FromStr
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Err = GStringError<V::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

macro_rules! errpanic {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(Err::TooShort) => {
                panic!("string len is smaller than MIN")
            }
            Err(Err::TooLong) => {
                panic!("string len is bigger than MAX")
            }
            Err(Err::NotAscii) => {
                panic!("ASCII_ONLY is true, but not ascii")
            }
        }
    };
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    #[doc(hidden)]
    pub const fn __new(s: &str) -> Self {
        let ret = errpanic!(Self::stack_allocate(s));
        errpanic!(ret.check_bounds());
        errpanic!(ret.check_ascii());
        ret
    }

    #[inline]
    pub fn new(s: &str) -> Result<Self, GStringError<V::Err>> {
        let gstring = Self::stack_allocate(s)?;

        gstring.check_bounds()?;
        gstring.check_ascii()?;

        gstring.validate()
    }

    const fn stack_allocate(s: &str) -> Result<Self, Err> {
        let bytes = s.as_bytes();
        let len = bytes.len();

        if len > MAX {
            return Err(Err::TooLong);
        }

        let mut buf = [0u8; MAX];
        let mut i = 0;
        while i < len {
            buf[i] = bytes[i];
            i += 1;
        }

        Ok(Self {
            buf,
            len,
            _validator: PhantomData,
        })
    }

    #[inline]
    const fn check_bounds(&self) -> Result<(), Err> {
        const {
            assert!(MIN <= MAX, "MIN cannot be bigger than MAX");
        }
        if self.len < MIN {
            return Err(Err::TooShort);
        }
        if self.len > MAX {
            return Err(Err::TooLong);
        }

        Ok(())
    }

    #[inline]
    const fn check_ascii(&self) -> Result<(), Err> {
        if ASCII_ONLY {
            let mut i = 0;
            while i < self.len {
                // If a byte is >= 128, it's a multi-byte UTF-8 character (not ASCII)
                if self.buf[i] >= 128 {
                    return Err(Err::NotAscii);
                }
                i += 1;
            }
        }

        Ok(())
    }

    #[inline(always)]
    pub fn validate(self) -> Result<Self, GStringError<V::Err>> {
        V::validate(self.as_str()).map_err(GStringError::Validation)?;
        Ok(self)
    }

    pub const fn as_str(&self) -> &str {
        // UNSAFE: this is to avoid dealing with fallible function, also the bytes are always created from UTF-8 string.
        unsafe {
            let slice = core::slice::from_raw_parts(self.buf.as_ptr(), self.len);
            core::str::from_utf8_unchecked(slice)
        }
    }

    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.buf
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        MAX
    }
}

impl<LHSV: Validator, RHSV: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    PartialEq<GString<RHSV, MIN, MAX, ASCII_ONLY>> for GString<LHSV, MIN, MAX, ASCII_ONLY>
{
    fn eq(&self, other: &GString<RHSV, MIN, MAX, ASCII_ONLY>) -> bool {
        self.len == other.len && self.buf[..self.len] == other.buf[..other.len]
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> PartialEq<str>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> PartialEq<&str>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<V: Validator + Eq, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Ord
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> PartialOrd
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> AsRef<str>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Display
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Debug
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "GString(\"{}\", MIN={}, MAX={}, ASCII_ONLY={})",
            self.as_str(),
            MIN,
            MAX,
            ASCII_ONLY
        )
    }
}

#[cfg(feature = "alloc")]
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    From<GString<V, MIN, MAX, ASCII_ONLY>> for String
{
    fn from(value: GString<V, MIN, MAX, ASCII_ONLY>) -> Self {
        value.as_str().to_owned()
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> core::ops::Deref
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    core::borrow::Borrow<str> for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> core::hash::Hash
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> TryFrom<&str>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Error = GStringError<V::Err>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl<V: Validator, const MAX: usize, const ASCII_ONLY: bool> Default
    for GString<V, 0, MAX, ASCII_ONLY>
{
    fn default() -> Self {
        Self {
            buf: [0u8; MAX],
            len: 0,
            _validator: PhantomData,
        }
    }
}

// ------------------------------------------------------------------------------------
// MUTATION APIs
// ------------------------------------------------------------------------------------
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    pub fn push(&mut self, ch: char) -> Result<(), GStringError<V::Err>> {
        // char takes up to 4 bytes
        let mut buf = [0u8; 4];
        let encoded = ch.encode_utf8(&mut buf);

        self.push_str(encoded)
    }

    pub fn push_str(&mut self, s: &str) -> Result<(), GStringError<V::Err>> {
        let mut buf = [0u8; MAX];

        // copy current content
        buf[..self.len].copy_from_slice(&self.buf[..self.len]);

        // append new content
        let bytes = s.as_bytes();
        let end = self.len + bytes.len();

        buf[self.len..end].copy_from_slice(bytes);

        // SAFETY:
        // existing bytes are valid UTF-8
        // appended bytes come from &str
        // concatenation of valid UTF-8 is valid UTF-8
        let candidate = unsafe { core::str::from_utf8_unchecked(&buf[..end]) };

        // fully revalidate through centralized constructor
        *self = Self::new(candidate)?;

        Ok(())
    }

    pub fn insert(&mut self, idx: usize, ch: char) -> Result<(), GStringError<V::Err>> {
        let mut buf = [0u8; 4];
        let encoded = ch.encode_utf8(&mut buf);

        self.insert_str(idx, encoded)
    }

    pub fn insert_str(&mut self, idx: usize, string: &str) -> Result<(), GStringError<V::Err>> {
        if !self.as_str().is_char_boundary(idx) {
            return Err(GStringError::Mutation("idx is not a char boundary"));
        }

        let insert_bytes = string.as_bytes();
        let insert_len = insert_bytes.len();

        let new_len = self.len + insert_len;

        // early capacity check to avoid panic
        if new_len > MAX {
            return Err(GStringError::TooLong);
        }

        let mut buf = [0u8; MAX];

        // before insertion point
        buf[..idx].copy_from_slice(&self.buf[..idx]);

        // inserted bytes
        buf[idx..idx + insert_len].copy_from_slice(insert_bytes);

        // after insertion point
        buf[idx + insert_len..new_len].copy_from_slice(&self.buf[idx..self.len]);

        // SAFETY:
        // - original bytes are valid UTF-8
        // - inserted string is valid UTF-8
        // - insertion only happens at char boundary
        // - concatenation of valid UTF-8 is valid UTF-8
        let candidate = unsafe { core::str::from_utf8_unchecked(&buf[..new_len]) };

        *self = Self::new(candidate)?;

        Ok(())
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.is_empty() {
            return None;
        }

        let s = self.as_str();
        let ch = s.chars().next_back()?;

        let new_len = self.len - ch.len_utf8();

        // SAFETY:
        // truncating at char boundary preserves UTF-8 validity
        let candidate = unsafe { core::str::from_utf8_unchecked(&self.buf[..new_len]) };

        match Self::new(candidate) {
            Ok(new) => {
                *self = new;
                Some(ch)
            }
            Err(_) => {
                // cannot happen:
                // existing instance is already valid,
                // removing chars cannot violate MAX/ASCII,
                // only possible issue is validator/MIN.
                //
                // std-like APIs should not fail here,
                // so we preserve old state.
                None
            }
        }
    }

    pub fn remove(&mut self, idx: usize) -> Result<char, GStringError<V::Err>> {
        if !self.as_str().is_char_boundary(idx) {
            return Err(GStringError::Mutation("idx is not a char boundary"));
        }

        let s = self.as_str();

        let ch = s[idx..].chars().next().ok_or(GStringError::Mutation(
            "cannot remove char from empty index",
        ))?;

        let ch_len = ch.len_utf8();

        let new_len = self.len - ch_len;

        let mut buf = [0u8; MAX];

        // before removed char
        buf[..idx].copy_from_slice(&self.buf[..idx]);

        // after removed char
        buf[idx..new_len].copy_from_slice(&self.buf[idx + ch_len..self.len]);

        // SAFETY:
        // removal at char boundary preserves UTF-8 validity
        let candidate = unsafe { core::str::from_utf8_unchecked(&buf[..new_len]) };

        *self = Self::new(candidate)?;

        Ok(ch)
    }

    pub fn truncate(&mut self, new_len: usize) -> Result<(), GStringError<V::Err>> {
        if new_len >= self.len {
            return Err(GStringError::Mutation("new_len is not a char boundary"));
        }

        if !self.as_str().is_char_boundary(new_len) {
            return Err(GStringError::Mutation("new_len is not a char boundary"));
        }

        // SAFETY:
        // truncating at char boundary preserves UTF-8 validity
        let candidate = unsafe { core::str::from_utf8_unchecked(&self.buf[..new_len]) };

        *self = Self::new(candidate)?;

        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), GStringError<V::Err>> {
        if MIN != 0 {
            return Err(GStringError::Mutation(
                "cannot clear GString if MIN is not zero",
            ));
        }

        *self = Self::new("")?;

        Ok(())
    }
}
