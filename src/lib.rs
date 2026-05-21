#![doc = include_str!("../README.md")]

mod macros;

use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display},
    str::FromStr,
};

pub const DEFAULT_MIN: usize = 0;
pub const DEFAULT_MAX: usize = 255;
pub const DEFAULT_ASCII_ONLY: bool = false;

pub const ERR_LEN_MIN: &'static str = "string len is smaller than MIN";
pub const ERR_LEN_MAX: &'static str = "string len is bigger than MAX";
pub const ERR_NOT_ASCII: &'static str = "ASCII_ONLY is true, but not ascii";

pub type Err = Box<dyn Error + Send + Sync + 'static>;

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
    _validator: std::marker::PhantomData<V>,
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> FromStr
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    pub const fn as_str(&self) -> &str {
        // UNSAFE: this is to avoid dealing with fallible function, also the bytes are always created from UTF-8 string.
        unsafe {
            let slice = std::slice::from_raw_parts(self.buf.as_ptr(), self.len);
            std::str::from_utf8_unchecked(slice)
        }
    }

    #[inline]
    pub fn new(s: &str) -> Result<Self, Err> {
        Self::stack_allocate(s)?
            .check_bounds()?
            .check_ascii()?
            .validate()
    }

    const fn stack_allocate(s: &str) -> Result<Self, &'static str> {
        let bytes = s.as_bytes();
        let len = bytes.len();

        if len > MAX {
            return Err("string len is bigger than MAX");
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
            _validator: std::marker::PhantomData,
        })
    }

    #[inline]
    const fn check_bounds(&self) -> Result<Self, &'static str> {
        assert!(MIN <= MAX, "MIN cannot be bigger than MAX");
        if self.len < MIN {
            return Err(ERR_LEN_MIN);
        }
        if self.len > MAX {
            return Err(ERR_LEN_MAX);
        }

        Ok(*self)
    }

    #[inline]
    const fn check_ascii(&self) -> Result<Self, &'static str> {
        if ASCII_ONLY {
            let mut i = 0;
            while i < self.len {
                // If a byte is >= 128, it's a multi-byte UTF-8 character (not ASCII)
                if self.buf[i] >= 128 {
                    return Err(ERR_NOT_ASCII);
                }
                i += 1;
            }
        }

        Ok(*self)
    }

    #[inline(always)]
    pub fn validate(&self) -> Result<Self, Err> {
        V::validate(self.as_str())?;
        Ok(*self)
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
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> PartialOrd
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Debug
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    From<GString<V, MIN, MAX, ASCII_ONLY>> for String
{
    fn from(value: GString<V, MIN, MAX, ASCII_ONLY>) -> Self {
        value.as_str().to_owned()
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> std::ops::Deref
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    std::borrow::Borrow<str> for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> std::hash::Hash
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> TryFrom<&str>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Error = Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}
