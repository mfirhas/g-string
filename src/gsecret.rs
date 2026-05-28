use core::{convert::Infallible, fmt::Debug, str::FromStr};

use crate::{GString, GStringError, NoValidation, Validator};

#[derive(Clone, PartialEq, Eq)]
pub struct GSecret<
    V: Validator = NoValidation,
    const MIN: usize = 0,
    const MAX: usize = 255,
    const ASCII_ONLY: bool = false,
> {
    inner: GString<V, MIN, MAX, ASCII_ONLY>,
}

impl GSecret {
    #[inline]
    pub fn try_default<S>(secret: S) -> Result<Self, GStringError<Infallible>>
    where
        S: AsRef<str>,
    {
        Ok(Self {
            inner: GString::try_default(secret)?,
        })
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GSecret<V, MIN, MAX, ASCII_ONLY>
{
    #[inline]
    pub fn try_new<S>(secret: S) -> Result<Self, GStringError<V::Err>>
    where
        S: AsRef<str>,
    {
        Ok(Self {
            inner: GString::<V, MIN, MAX, ASCII_ONLY>::try_new(secret)?,
        })
    }

    pub fn reveal<R>(&self, func: impl FnOnce(&str) -> R) -> R {
        func(self.inner.as_str())
    }

    #[inline]
    pub fn zeroize(&mut self) {
        self.inner.zeroize();
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Drop
    for GSecret<V, MIN, MAX, ASCII_ONLY>
{
    fn drop(&mut self) {
        self.inner.zeroize();
    }
}

impl<V, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> zeroize::Zeroize
    for GSecret<V, MIN, MAX, ASCII_ONLY>
where
    V: Validator,
{
    fn zeroize(&mut self) {
        self.inner.zeroize();
    }
}

impl<V, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> zeroize::ZeroizeOnDrop
    for GSecret<V, MIN, MAX, ASCII_ONLY>
where
    V: Validator,
{
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Debug
    for GSecret<V, MIN, MAX, ASCII_ONLY>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GSecret(<REDACTED>)")
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> FromStr
    for GSecret<V, MIN, MAX, ASCII_ONLY>
{
    type Err = GStringError<V::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let secret = GSecret::try_new(s)?;

        Ok(secret)
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> TryFrom<&str>
    for GSecret<V, MIN, MAX, ASCII_ONLY>
{
    type Error = GStringError<V::Err>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let secret = GSecret::try_new(value)?;

        Ok(secret)
    }
}

#[cfg(feature = "alloc")]
impl<V, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> TryFrom<String>
    for GSecret<V, MIN, MAX, ASCII_ONLY>
where
    V: Validator,
{
    type Error = GStringError<V::Err>;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        use zeroize::Zeroize;

        let secret = GSecret::try_new(&value);

        value.zeroize();

        secret
    }
}

impl<
    GSTRINGV: Validator,
    GSECRETV: Validator,
    const MIN: usize,
    const MAX: usize,
    const ASCII_ONLY: bool,
> TryFrom<GString<GSTRINGV, MIN, MAX, ASCII_ONLY>> for GSecret<GSECRETV, MIN, MAX, ASCII_ONLY>
{
    type Error = GStringError<GSECRETV::Err>;

    fn try_from(value: GString<GSTRINGV, MIN, MAX, ASCII_ONLY>) -> Result<Self, Self::Error> {
        Self::try_new(value.as_str())
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> core::hash::Hash
    for GSecret<V, MIN, MAX, ASCII_ONLY>
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}
