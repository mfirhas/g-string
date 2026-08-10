use arrayvec::ArrayVec;
use core::fmt::{Debug, Display};
use core::marker::PhantomData;

use crate::{
    DEFAULT_ASCII_ONLY, DEFAULT_MAX, DEFAULT_MIN, Err, GStringError, NoValidation, Validator,
};

#[derive(Clone, Eq)]
pub struct GStringUninit<
    V: Validator = NoValidation,
    const MIN: usize = DEFAULT_MIN,
    const MAX: usize = DEFAULT_MAX,
    const ASCII_ONLY: bool = DEFAULT_ASCII_ONLY,
> {
    buf: ArrayVec<u8, MAX>,
    _validator: PhantomData<V>,
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GStringUninit<V, MIN, MAX, ASCII_ONLY>
{
    /// Construct new GStringUninit.
    ///
    /// All invariants from generic params will be imposed here.
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

    /// Allocate `s` on the stack without validations.
    ///
    /// `ArrayVec` uses uninitialized storage internally, so only
    /// the bytes actually contained in `s` are initialized.
    #[inline]
    fn stack_allocate(s: &str) -> Result<Self, Err> {
        let bytes = s.as_bytes();

        if bytes.len() > MAX {
            return Err(Err::TooLong(MAX));
        }

        let mut buf = ArrayVec::<u8, MAX>::new();

        // `bytes.len() <= MAX` was checked above.
        buf.try_extend_from_slice(bytes)
            .expect("length was checked against MAX");

        Ok(Self {
            buf,
            _validator: PhantomData,
        })
    }

    /// Check upper and lower bounds.
    #[inline]
    const fn check_bounds(&self) -> Result<(), Err> {
        const {
            assert!(MIN <= MAX, "MIN cannot be bigger than MAX");
        }

        if self.buf.len() < MIN {
            return Err(Err::TooShort(MIN));
        }

        if self.buf.len() > MAX {
            return Err(Err::TooLong(MAX));
        }

        Ok(())
    }

    /// Check whether ASCII-only constraint is met.
    #[inline]
    fn check_ascii(&self) -> Result<(), Err> {
        if ASCII_ONLY {
            let mut i = 0;

            while i < self.buf.len() {
                // If a byte is >= 128, it's a multi-byte UTF-8 character.
                if self.buf[i] >= 128 {
                    return Err(Err::NotAscii);
                }

                i += 1;
            }
        }

        Ok(())
    }

    /// Execute validation logic.
    #[inline(always)]
    fn validate(self) -> Result<Self, GStringError<V::Error>> {
        V::validate(self.as_str()).map_err(GStringError::Validation)?;
        Ok(self)
    }

    #[inline]
    fn as_str(&self) -> &str {
        // `buf` originated from `&str`, so it is guaranteed to be
        // valid UTF-8.
        unsafe { core::str::from_utf8_unchecked(self.buf.as_slice()) }
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Display
    for GStringUninit<V, MIN, MAX, ASCII_ONLY>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Debug
    for GStringUninit<V, MIN, MAX, ASCII_ONLY>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "GStringUninit(\"{}\", MIN={}, MAX={}, ASCII_ONLY={})",
            self.as_str(),
            MIN,
            MAX,
            ASCII_ONLY
        )
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    core::borrow::Borrow<str> for GStringUninit<V, MIN, MAX, ASCII_ONLY>
{
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> core::hash::Hash
    for GStringUninit<V, MIN, MAX, ASCII_ONLY>
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> PartialEq
    for GStringUninit<V, MIN, MAX, ASCII_ONLY>
{
    fn eq(&self, other: &Self) -> bool {
        self.buf == other.buf
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> PartialEq<str>
    for GStringUninit<V, MIN, MAX, ASCII_ONLY>
{
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
