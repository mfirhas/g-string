use core::str::{Bytes, CharIndices, Chars};

use crate::{GString, GStringError, Validator};

// basic
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    /// Get length of GString in bytes.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Tells maximum capacity returning MAX.
    #[inline]
    pub const fn capacity(&self) -> usize {
        MAX
    }

    /// Counts Unicode scalar values (`char`).
    #[inline]
    pub fn count(&self) -> usize {
        self.chars().count()
    }

    /// Check if empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Tells if maximum capacity met.
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.len == MAX
    }

    #[inline]
    pub const fn is_char_boundary(&self, index: usize) -> bool {
        self.as_str().is_char_boundary(index)
    }

    /// Show `GString` as `&str`.
    #[inline]
    pub const fn as_str(&self) -> &str {
        // SAFETY:
        // GString is always built from valid UTF-8 encoded string.
        unsafe {
            let slice = core::slice::from_raw_parts(self.buf.as_ptr(), self.len);
            core::str::from_utf8_unchecked(slice)
        }
    }

    /// Show `GString` as bytes.
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.buf.as_ptr(), self.len) }
    }
}

// graphemes
#[cfg(feature = "grapheme")]
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    /// Counts user-perceived characters (grapheme clusters).
    #[inline]
    pub fn grapheme_count(&self) -> usize {
        use crate::UnicodeSegmentation;
        self.as_str().graphemes(true).count()
    }

    /// Iterate over graphemes.
    #[inline]
    pub fn graphemes(&self) -> crate::Graphemes<'_> {
        use crate::UnicodeSegmentation;
        self.as_str().graphemes(true)
    }
}

// iterator
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    /// Returns an iterator over the chars of a string slice.
    #[inline]
    pub fn chars(&self) -> Chars<'_> {
        self.as_str().chars()
    }

    /// Returns an iterator over the chars of a string slice, and their positions.
    #[inline]
    pub fn char_indices(&self) -> CharIndices<'_> {
        self.as_str().char_indices()
    }

    /// Returns an iterator over the bytes of a string slice.
    #[inline]
    pub fn bytes(&self) -> Bytes<'_> {
        self.as_str().bytes()
    }
}

use core::str::FromStr;
use core::str::{EncodeUtf16, EscapeDebug, EscapeDefault, EscapeUnicode, Lines, SplitWhitespace};

pub trait Pattern {
    type SplitIter<'a>: Iterator<Item = &'a str>
    where
        Self: 'a;

    type MatchesIter<'a>: Iterator<Item = &'a str>
    where
        Self: 'a;

    type RMatchesIter<'a>: Iterator<Item = &'a str>
    where
        Self: 'a;

    type MatchIndicesIter<'a>: Iterator<Item = (usize, &'a str)>
    where
        Self: 'a;

    type RMatchIndicesIter<'a>: Iterator<Item = (usize, &'a str)>
    where
        Self: 'a;

    fn split<'a>(self, s: &'a str) -> Self::SplitIter<'a>
    where
        Self: 'a;

    fn split_once<'a>(&self, s: &'a str) -> Option<(&'a str, &'a str)>;

    fn rsplit_once<'a>(&self, s: &'a str) -> Option<(&'a str, &'a str)>;

    fn strip_prefix<'a>(&self, s: &'a str) -> Option<&'a str>;

    fn strip_suffix<'a>(&self, s: &'a str) -> Option<&'a str>;

    fn matches<'a>(self, s: &'a str) -> Self::MatchesIter<'a>
    where
        Self: 'a;

    fn rmatches<'a>(self, s: &'a str) -> Self::RMatchesIter<'a>
    where
        Self: 'a;

    fn match_indices<'a>(self, s: &'a str) -> Self::MatchIndicesIter<'a>
    where
        Self: 'a;

    fn rmatch_indices<'a>(self, s: &'a str) -> Self::RMatchIndicesIter<'a>
    where
        Self: 'a;

    // searching
    //

    fn contains(&self, s: &str) -> bool;

    fn starts_with(&self, s: &str) -> bool;

    fn ends_with(&self, s: &str) -> bool;

    fn find(&self, s: &str) -> Option<usize>;

    fn rfind(&self, s: &str) -> Option<usize>;
}

// search
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    #[inline]
    pub fn contains<P>(&self, pat: P) -> bool
    where
        P: Pattern,
    {
        pat.contains(self.as_str())
    }

    #[inline]
    pub fn starts_with<P>(&self, pat: P) -> bool
    where
        P: Pattern,
    {
        pat.starts_with(self.as_str())
    }

    #[inline]
    pub fn ends_with<P>(&self, pat: P) -> bool
    where
        P: Pattern,
    {
        pat.ends_with(self.as_str())
    }

    #[inline]
    pub fn find<P>(&self, pat: P) -> Option<usize>
    where
        P: Pattern,
    {
        pat.find(self.as_str())
    }

    #[inline]
    pub fn rfind<P>(&self, pat: P) -> Option<usize>
    where
        P: Pattern,
    {
        pat.rfind(self.as_str())
    }
}

// slicing
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    #[inline]
    pub fn get<I>(&self, i: I) -> Option<&<I as core::slice::SliceIndex<str>>::Output>
    where
        I: core::slice::SliceIndex<str>,
    {
        self.as_str().get(i)
    }
}

// splitting

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    /// Returns an iterator over substrings of this string slice, separated by
    /// characters matched by a pattern.
    #[inline]
    pub fn split<P: Pattern>(&self, pat: P) -> P::SplitIter<'_> {
        pat.split(self.as_str())
    }

    /// Returns an iterator over the non-whitespace substrings of this string
    /// slice, separated by any amount of whitespace.
    #[inline]
    pub fn split_whitespace(&self) -> SplitWhitespace<'_> {
        self.as_str().split_whitespace()
    }

    /// Returns an iterator over the lines of this string slice.
    #[inline]
    pub fn lines(&self) -> Lines<'_> {
        self.as_str().lines()
    }

    /// Divides this string slice into two at the first occurrence of a pattern.
    #[inline]
    pub fn split_once<P: Pattern>(&self, delimiter: P) -> Option<(&str, &str)> {
        delimiter.split_once(self.as_str())
    }

    /// Divides this string slice into two at the last occurrence of a pattern.
    #[inline]
    pub fn rsplit_once<P: Pattern>(&self, delimiter: P) -> Option<(&str, &str)> {
        delimiter.rsplit_once(self.as_str())
    }
}

// trimming

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    /// Returns a trimmed copy of this string.
    #[inline]
    pub fn try_trim(&self) -> Result<Self, GStringError<V::Err>> {
        Self::try_new(self.as_str().trim())
    }

    /// Returns a copy of this string with leading whitespace removed.
    #[inline]
    pub fn try_trim_start(&self) -> Result<Self, GStringError<V::Err>> {
        Self::try_new(self.as_str().trim_start())
    }

    /// Returns a copy of this string with trailing whitespace removed.
    #[inline]
    pub fn try_trim_end(&self) -> Result<Self, GStringError<V::Err>> {
        Self::try_new(self.as_str().trim_end())
    }

    /// Returns a string slice with the given prefix removed.
    #[inline]
    pub fn strip_prefix<P: Pattern>(&self, prefix: P) -> Option<&str> {
        prefix.strip_prefix(self.as_str())
    }

    /// Returns a string slice with the given suffix removed.
    #[inline]
    pub fn strip_suffix<P: Pattern>(&self, suffix: P) -> Option<&str> {
        suffix.strip_suffix(self.as_str())
    }
}

// matching

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    /// Returns an iterator over matches of a pattern within this string slice.
    #[inline]
    pub fn matches<P: Pattern>(&self, pat: P) -> P::MatchesIter<'_> {
        pat.matches(self.as_str())
    }

    /// Returns an iterator over matches of a pattern within this string slice,
    /// yielded in reverse order.
    #[inline]
    pub fn rmatches<P: Pattern>(&self, pat: P) -> P::RMatchesIter<'_> {
        pat.rmatches(self.as_str())
    }

    /// Returns an iterator over the disjoint matches of a pattern and their
    /// positions within this string slice.
    #[inline]
    pub fn match_indices<P: Pattern>(&self, pat: P) -> P::MatchIndicesIter<'_> {
        pat.match_indices(self.as_str())
    }

    /// Returns an iterator over the disjoint matches of a pattern and their
    /// positions within this string slice, yielded in reverse order.
    #[inline]
    pub fn rmatch_indices<P: Pattern>(&self, pat: P) -> P::RMatchIndicesIter<'_> {
        pat.rmatch_indices(self.as_str())
    }
}

// misc

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    /// Parses this string slice into another type.
    #[inline]
    pub fn parse<F>(&self) -> Result<F, F::Err>
    where
        F: FromStr,
    {
        self.as_str().parse()
    }

    /// Checks if all characters in this string are within the ASCII range.
    #[inline]
    pub fn is_ascii(&self) -> bool {
        self.as_str().is_ascii()
    }

    /// Compares two strings in a case-insensitive ASCII manner.
    #[inline]
    pub fn eq_ignore_ascii_case(&self, other: &str) -> bool {
        self.as_str().eq_ignore_ascii_case(other)
    }
}

// escaping

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    /// Returns an iterator that escapes this string using `char::escape_debug`.
    #[inline]
    pub fn escape_debug(&self) -> EscapeDebug<'_> {
        self.as_str().escape_debug()
    }

    /// Returns an iterator that escapes this string using
    /// `char::escape_default`.
    #[inline]
    pub fn escape_default(&self) -> EscapeDefault<'_> {
        self.as_str().escape_default()
    }

    /// Returns an iterator that escapes this string using
    /// `char::escape_unicode`.
    #[inline]
    pub fn escape_unicode(&self) -> EscapeUnicode<'_> {
        self.as_str().escape_unicode()
    }
}

// encoding

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    /// Returns an iterator of UTF-16 code units for this string slice.
    #[inline]
    pub fn encode_utf16(&self) -> EncodeUtf16<'_> {
        self.as_str().encode_utf16()
    }
}
