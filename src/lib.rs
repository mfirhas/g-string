#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

mod conversion;
mod equality;
mod error;
mod macros;
mod mutation;
mod order;
mod query;

use error::{Err, GStringError};

use core::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display},
    marker::PhantomData,
};

pub const DEFAULT_MIN: usize = 0;
pub const DEFAULT_MAX: usize = 255;
pub const DEFAULT_ASCII_ONLY: bool = false;

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

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    #[inline]
    pub fn try_new(s: &str) -> Result<Self, GStringError<V::Err>> {
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
        // SAFETY:
        // self.buf is valid for self.len bytes
        // self.len is always maintained within bounds
        unsafe { core::slice::from_raw_parts(self.buf.as_ptr(), self.len) }
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
