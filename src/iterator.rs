use crate::{GString, Validator};

/// Iterates over the characters of a borrowed [`GString`].
///
/// This behaves the same as iterating over `&str`.
///
/// # Examples
///
/// ```ignore
/// for ch in &gstring {
///     // ch: char
/// }
/// ```
impl<'a, V, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> IntoIterator
    for &'a GString<V, MIN, MAX, ASCII_ONLY>
where
    V: Validator,
{
    type Item = char;
    type IntoIter = core::str::Chars<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.chars()
    }
}

/// Iterates over the characters of a mutably borrowed [`GString`].
///
/// Even though the string is mutably borrowed, iteration yields immutable
/// [`char`] values.
///
/// # Examples
///
/// ```ignore
/// for ch in &mut gstring {
///     // ch: char
/// }
/// ```
impl<'a, V, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> IntoIterator
    for &'a mut GString<V, MIN, MAX, ASCII_ONLY>
where
    V: Validator,
{
    type Item = char;
    type IntoIter = core::str::Chars<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.chars()
    }
}

/// An owning iterator over the characters of a [`GString`].
///
/// This iterator is created by [`IntoIterator`] for owned [`GString`].
///
/// # Examples
///
/// ```ignore
/// let iter = gstring.into_iter();
/// ```
pub struct IntoChars<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> {
    inner: GString<V, MIN, MAX, ASCII_ONLY>,
    front: usize,
    back: usize,
}

impl<V, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Iterator
    for IntoChars<V, MIN, MAX, ASCII_ONLY>
where
    V: Validator,
{
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }

        if ASCII_ONLY {
            let byte = self.inner.buf[self.front];
            self.front += 1;
            return Some(byte as char);
        }

        let s = self.inner.as_str();
        let ch = s[self.front..self.back].chars().next()?;
        self.front += ch.len_utf8();
        Some(ch)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.back - self.front;

        if ASCII_ONLY {
            return (remaining, Some(remaining));
        }

        (0, Some(remaining))
    }
}

/// Iterates over the characters of an owned [`GString`].
///
/// This consumes the string.
///
/// # Examples
///
/// ```ignore
/// for ch in gstring {
///     // ch: char
/// }
/// ```
impl<V, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> IntoIterator
    for GString<V, MIN, MAX, ASCII_ONLY>
where
    V: Validator,
{
    type Item = char;
    type IntoIter = IntoChars<V, MIN, MAX, ASCII_ONLY>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len;

        IntoChars {
            inner: self,
            front: 0,
            back: len,
        }
    }
}

/// Enables reverse iteration
///
/// # Examples
/// ```ignored
/// gstring.into_iter().rev()
/// ```
impl<V, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> DoubleEndedIterator
    for IntoChars<V, MIN, MAX, ASCII_ONLY>
where
    V: Validator,
{
    #[inline]
    fn next_back(&mut self) -> Option<char> {
        if self.front >= self.back {
            return None;
        }

        // ASCII fast path
        if ASCII_ONLY {
            self.back -= 1;
            return Some(self.inner.buf[self.back] as char);
        }

        let s = self.inner.as_str();

        // Start from the final byte
        let mut idx = self.back - 1;

        // Walk backwards until UTF-8 char boundary
        while idx > self.front && !s.is_char_boundary(idx) {
            idx -= 1;
        }

        // Extract the char slice
        let ch = s[idx..self.back].chars().next()?;

        // Move back pointer
        self.back = idx;

        Some(ch)
    }
}

impl<V, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> core::iter::FusedIterator
    for IntoChars<V, MIN, MAX, ASCII_ONLY>
where
    V: Validator,
{
}
