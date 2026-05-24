//! Iterator implementations for [`GString`].
//!
//! You implemented 3 major iterator categories for [`GString`].
//!
//! ---
//!
//! # 1. Borrowed Iteration (`&GString`)
//!
//! Uses std’s native [`core::str::Chars`] iterator.
//!
//! ```ignore
//! for ch in &gstring {
//!     println!("{ch}");
//! }
//! ```
//!
//! Iterator type:
//!
//! ```ignore
//! core::str::Chars<'a>
//! ```
//!
//! Characteristics:
//!
//! - non-owning
//! - zero allocation
//! - lightweight
//! - identical to `&str`
//! - read-only iteration
//!
//! Implementation:
//!
//! ```ignore
//! impl IntoIterator for &GString
//! ```
//!
//! ---
//!
//! # 2. Mutable Borrowed Iteration (`&mut GString`)
//!
//! Also uses [`core::str::Chars`].
//!
//! ```ignore
//! for ch in &mut gstring {
//!     println!("{ch}");
//! }
//! ```
//!
//! Characteristics:
//!
//! - mutable borrow of container
//! - iteration still yields immutable [`char`] values
//! - prevents simultaneous mutation during iteration
//!
//! Implementation:
//!
//! ```ignore
//! impl IntoIterator for &mut GString
//! ```
//!
//! ---
//!
//! # 3. Owned Iteration (`GString`)
//!
//! Consumes the string and iterates characters using the custom
//! [`IntoChars`] iterator.
//!
//! ```ignore
//! for ch in gstring {
//!     println!("{ch}");
//! }
//! ```
//!
//! Iterator type:
//!
//! ```ignore
//! IntoChars
//! ```
//!
//! Characteristics:
//!
//! - owning iterator
//! - consumes [`GString`]
//! - supports forward + reverse iteration
//! - ASCII fast path
//! - UTF-8 aware
//! - double-ended
//!
//! Implementation:
//!
//! ```ignore
//! impl IntoIterator for GString
//! ```
//!
//! ---
//!
//! # 4. Forward Iterator (`Iterator`)
//!
//! Supports:
//!
//! ```ignore
//! iter.next()
//! ```
//!
//! Example:
//!
//! ```ignore
//! let mut iter = gstring.into_iter();
//!
//! assert_eq!(iter.next(), Some('a'));
//! assert_eq!(iter.next(), Some('b'));
//! ```
//!
//! Features:
//!
//! - UTF-8 decoding
//! - ASCII optimization
//! - maintains `front/back` cursor state
//! - custom [`Iterator::size_hint`]
//!
//! Implementation:
//!
//! ```ignore
//! impl Iterator for IntoChars
//! ```
//!
//! ---
//!
//! # 5. Reverse Iterator (`DoubleEndedIterator`)
//!
//! Supports reverse iteration:
//!
//! ```ignore
//! gstring.into_iter().rev()
//! ```
//!
//! Example:
//!
//! ```ignore
//! let mut iter = gstring.into_iter();
//!
//! assert_eq!(iter.next_back(), Some('z'));
//! assert_eq!(iter.next_back(), Some('y'));
//! ```
//!
//! Features:
//!
//! - reverse UTF-8 traversal
//! - UTF-8 character-boundary walking
//! - amortized O(n)
//! - ASCII fast path
//!
//! Implementation:
//!
//! ```ignore
//! impl DoubleEndedIterator for IntoChars
//! ```
//!
//! ---
//!
//! # 6. Fused Iterator (`FusedIterator`)
//!
//! Guarantees exhaustion permanence.
//!
//! Example:
//!
//! ```ignore
//! let mut iter = gstring.into_iter();
//!
//! while iter.next().is_some() {}
//!
//! assert_eq!(iter.next(), None);
//! assert_eq!(iter.next(), None);
//! ```
//!
//! Meaning:
//!
//! ```text
//! once None, always None
//! ```
//!
//! Implementation:
//!
//! ```ignore
//! impl FusedIterator for IntoChars
//! ```
//!
//! ---
//!
//! # 7. Sized Iterator Hint (`size_hint`)
//!
//! Provides iteration bounds information.
//!
//! Example:
//!
//! ```ignore
//! let iter = gstring.into_iter();
//!
//! let (min, max) = iter.size_hint();
//! ```
//!
//! ASCII case:
//!
//! ```text
//! exact remaining chars known
//! ```
//!
//! UTF-8 case:
//!
//! ```text
//! upper bound only
//! ```
//!
//! ---
//!
//! # Overall Design
//!
//! Your iterator system now has:
//!
//! - borrowed iteration
//! - mutable borrowed iteration
//! - owned iteration
//! - forward iteration
//! - reverse iteration
//! - fused semantics
//! - UTF-8 correctness
//! - ASCII optimization
//! - allocation-aware sizing hints
//!
//! This design is comparable to a proper std-quality custom string iterator
//! implementation.

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

        let min = remaining.div_ceil(4);

        (min, Some(remaining))
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
