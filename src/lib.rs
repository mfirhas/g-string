#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

mod conversion;
mod equality;
mod error;
mod iterator;
mod macros;
mod mutation;
mod order;
mod query;
pub use query::Pattern;

mod pattern_impl;

#[cfg(feature = "secret")]
mod gsecret;

#[cfg(feature = "secret")]
pub use gsecret::GSecret;

#[cfg(feature = "grapheme")]
pub use unicode_segmentation::{Graphemes, UnicodeSegmentation};

#[cfg(feature = "serde")]
mod serde;

pub use error::GStringError;

use error::Err;

use core::{
    convert::Infallible,
    fmt::{Debug, Display},
    marker::PhantomData,
};

/// Default value of lower bound.
pub const DEFAULT_MIN: usize = 0;

/// Default value of upper bound.
pub const DEFAULT_MAX: usize = 255;

/// Default value is ASCII only or not.
pub const DEFAULT_ASCII_ONLY: bool = false;

/// GString alias without validation.
pub type GStringNV<const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> =
    GString<NoValidation, MIN, MAX, ASCII_ONLY>;

/// Validation trait
///
/// Usually it's implemented by marker type.
pub trait Validator: Clone {
    type Error;

    /// Validate the string.
    fn validate(s: impl AsRef<str>) -> Result<(), Self::Error>;
}

/// Mark validation allowing empty string.
///
/// This will enable `Default` impl and `clear()` method provided that MIN == 0.
pub trait AllowEmpty {}

/// Validator implementation without validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoValidation;

impl Validator for NoValidation {
    type Error = Infallible;

    #[inline]
    fn validate(_: impl AsRef<str>) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Validator for () {
    type Error = Infallible;

    #[inline]
    fn validate(_: impl AsRef<str>) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl AllowEmpty for NoValidation {}

impl AllowEmpty for () {}

/// `GString` contains stack-allocated, Copy, bounded string along with ASCII toggle and embedded validation logic.
///
/// # Generic parameters:
/// - `V: Validator`: default to [`NoValidation`], it will validate the string upon creation and deserialization.
/// - `const MIN: usize`: default to [`DEFAULT_MIN`], in bytes, determine minimum length.
/// - `const MAX: usize`: default to [`DEFAULT_MAX`], in bytes, determine maximum length.
/// - `const ASCII_ONLY: bool`: default to [`DEFAULT_ASCII_ONLY`], determine whether GString may contains arbitrary or ASCII only UTF-8 encoded string.
///
/// # Usage
/// You can use this type in several ways:
/// - Defaulted to default generic params: `let a: GString = GString::try_new("anjay!!").unwrap()`.
/// - Defaulted to default generic params with `try_default(...)`: `let a = GString::try_default("anjay!!").unwrap()`.
/// - Using type aliases. You can declare some aliases matching the behavior you want: `type Username = GString<UsernameValidation, 3, 20, true>`.
/// - Declaring each generic params from left to right. Declaration must be from left to right with omission allowed on right-most params: `let a = GString::<(), 2>::try_new("anjay!!").unwrap()`.
///
/// # Examples
/// ```rust
/// use g_string::{GString, Validator, GStringError};
///
/// let a: GString = GString::try_new("anjay!!").unwrap();
/// assert_eq!(a, "anjay!!");
///
/// let a = GString::try_default("anjay!!").unwrap();
/// assert_eq!(a, "anjay!!");
///
/// #[derive(Debug, Clone)]
/// struct UsernameValidation;
///
/// impl Validator for UsernameValidation {
///     type Error = GStringError<&'static str>;
///
///     fn validate(_: impl AsRef<str>) -> Result<(), Self::Error> {
///         // some validation logics here...
///         Ok(())
///     }   
/// }
///
/// type Username = GString<UsernameValidation, 3, 20, true>;
/// let a = Username::try_new("wanjay!!🤣");
/// assert!(a.is_err()); // because '🤣' is not ASCII.
///
/// let a = GString::<(), 2, 4>::try_new("anjay!!");
/// assert!(a.is_err()); // MAX is 4.
/// ```
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

impl GString {
    /// Construct with default generic params.
    ///
    /// # Examples
    /// ```rust
    /// use g_string::GString;
    ///
    /// let a = GString::try_default("anjay!!").unwrap();
    /// assert_eq!(a, "anjay!!");
    /// ```
    #[inline]
    pub fn try_default<S>(s: S) -> Result<Self, GStringError<Infallible>>
    where
        S: AsRef<str>,
    {
        Self::try_new(s)
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    /// Construct new GString. All invariants from generic params will be imposed here.
    ///
    /// # Examples
    /// ```rust
    /// use g_string::GString;
    ///
    /// let a: GString = GString::try_new("anjay!!").unwrap();
    /// assert_eq!(a, "anjay!!");
    /// ```
    #[inline]
    pub fn try_new<S>(s: S) -> Result<Self, GStringError<V::Error>>
    where
        S: AsRef<str>,
    {
        let gstring = Self::stack_allocate(s.as_ref())?;

        gstring.check_bounds()?;
        gstring.check_ascii()?;

        gstring.validate()
    }

    // Allocate `s` in stack without validations(yet).
    const fn stack_allocate(s: &str) -> Result<Self, Err> {
        let bytes = s.as_bytes();
        let len = bytes.len();

        if len > MAX {
            return Err(Err::TooLong(MAX));
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

    // Check upper and lower bounds.
    #[inline]
    const fn check_bounds(&self) -> Result<(), Err> {
        const {
            assert!(MIN <= MAX, "MIN cannot be bigger than MAX");
        }
        if self.len < MIN {
            return Err(Err::TooShort(MIN));
        }
        if self.len > MAX {
            return Err(Err::TooLong(MAX));
        }

        Ok(())
    }

    // Check whether ASCII only met.
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

    // Execute validation logic.
    #[inline(always)]
    pub fn validate(self) -> Result<Self, GStringError<V::Error>> {
        V::validate(&self).map_err(GStringError::Validation)?;
        Ok(self)
    }

    /// Calls a closure with a reference to the contained string.
    ///
    /// Returns `self` unchanged.
    ///
    /// This behaves similarly to `Option::inspect` and
    /// `Result::inspect`.
    #[inline]
    pub fn inspect<F>(self, func: F) -> Self
    where
        F: FnOnce(&str),
    {
        func(self.as_str());
        self
    }

    /// Transforms this string into another validated string.
    ///
    /// The transformed value is validated using the destination
    /// validator and bounds before being returned.
    ///
    /// This behaves similarly to `Option::map` and `Result::map`.
    #[inline]
    pub fn map<UV, const UMIN: usize, const UMAX: usize, const UASCII_ONLY: bool, F, U>(
        self,
        func: F,
    ) -> Result<GString<UV, UMIN, UMAX, UASCII_ONLY>, GStringError<UV::Error>>
    where
        UV: Validator,
        F: FnOnce(&str) -> U,
        U: AsRef<str>,
    {
        GString::<UV, UMIN, UMAX, UASCII_ONLY>::try_new(func(self.as_str()).as_ref())
    }

    /// Chains another fallible validated transformation.
    ///
    /// This behaves similarly to `Option::and_then` and
    /// `Result::and_then`.
    #[inline]
    pub fn and_then<UV, const UMIN: usize, const UMAX: usize, const UASCII_ONLY: bool, F>(
        self,
        func: F,
    ) -> Result<GString<UV, UMIN, UMAX, UASCII_ONLY>, GStringError<UV::Error>>
    where
        UV: Validator,
        F: FnOnce(&str) -> Result<GString<UV, UMIN, UMAX, UASCII_ONLY>, GStringError<UV::Error>>,
    {
        func(self.as_str())
    }
}

/// GString from const generic constructor.
///
/// It must be validated before getting actual `GString`.
#[derive(Debug)]
pub struct InValidatedGString<
    V: Validator,
    const MIN: usize,
    const MAX: usize,
    const ASCII_ONLY: bool,
>(GString<V, MIN, MAX, ASCII_ONLY>);

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    InValidatedGString<V, MIN, MAX, ASCII_ONLY>
{
    /// Validate into GString.
    #[inline(always)]
    pub fn validate(self) -> Result<GString<V, MIN, MAX, ASCII_ONLY>, GStringError<V::Error>> {
        self.0.validate()
    }
}

macro_rules! errpanic {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(Err::TooShort(_)) => {
                panic!("minimum length below MIN")
            }
            Err(Err::TooLong(_)) => {
                panic!("maximum length exceeds MAX")
            }
            Err(Err::NotAscii) => {
                panic!("only ASCII characters are allowed")
            }
        }
    };
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    /// Construct GString in const context returning invalidated string.
    /// Validate the return to get GString.
    #[allow(clippy::new_ret_no_self)]
    pub const fn new(s: &str) -> InValidatedGString<V, MIN, MAX, ASCII_ONLY> {
        let ret = errpanic!(Self::stack_allocate(s));
        errpanic!(ret.check_bounds());
        errpanic!(ret.check_ascii());
        InValidatedGString(ret)
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

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    core::borrow::Borrow<str> for GString<V, MIN, MAX, ASCII_ONLY>
{
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> core::hash::Hash
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl<V: Validator + AllowEmpty, const MAX: usize, const ASCII_ONLY: bool> Default
    for GString<V, 0, MAX, ASCII_ONLY>
{
    #[inline]
    fn default() -> Self {
        Self {
            buf: [0u8; MAX],
            len: 0,
            _validator: PhantomData,
        }
    }
}

#[cfg(feature = "secret")]
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    pub fn zeroize(&mut self) {
        use zeroize::Zeroize;

        self.buf.zeroize();
        self.len.zeroize();
    }
}
