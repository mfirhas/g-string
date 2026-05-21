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
        Self::from_str(s)
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
    fn from_str(s: &str) -> Result<Self, Err> {
        Self::build(s).runtime_check()
    }

    const fn build(s: &str) -> Self {
        let bytes = s.as_bytes();
        let len = bytes.len();

        let mut buf = [0u8; MAX];
        let mut i = 0;
        while i < len {
            buf[i] = bytes[i];
            i += 1;
        }

        Self {
            buf,
            len,
            _validator: std::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub fn validate(&self) -> Result<Self, Err> {
        V::validate(self.as_str())?;
        Ok(*self)
    }

    const fn comptime_check(&self) -> Self {
        // check length
        assert!(MIN <= MAX, "MIN cannot be bigger than MAX");
        assert!(self.len >= MIN, "string len cannot be smaller than MIN");
        assert!(self.len <= MAX, "string len cannot be bigger than MAX");

        // check ascii only
        if ASCII_ONLY {
            let mut i = 0;
            while i < self.len {
                // If a byte is >= 128, it's a multi-byte UTF-8 character (not ASCII)
                assert!(self.buf[i] < 128, "ASCII_ONLY is true, but not ascii");
                i += 1;
            }
        }

        *self
    }

    fn runtime_check(&self) -> Result<Self, Err> {
        // check length
        const {
            assert!(MIN <= MAX, "MIN cannot be bigger than MAX");
        }
        if self.len < MIN {
            return Err(ERR_LEN_MIN.into());
        }
        if self.len > MAX {
            return Err(ERR_LEN_MAX.into());
        }

        // check ascii only
        if ASCII_ONLY {
            let mut i = 0;
            while i < self.len {
                // If a byte is >= 128, it's a multi-byte UTF-8 character (not ASCII)
                if self.buf[i] >= 128 {
                    return Err(ERR_NOT_ASCII.into());
                }
                i += 1;
            }
        }

        // check validation
        self.validate()?;

        Ok(*self)
    }
}

impl<LHSV: Validator, RHSV: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    PartialEq<GString<RHSV, MIN, MAX, ASCII_ONLY>> for GString<LHSV, MIN, MAX, ASCII_ONLY>
{
    fn eq(&self, other: &GString<RHSV, MIN, MAX, ASCII_ONLY>) -> bool {
        self.len == other.len && self.buf == other.buf
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
