use crate::{GString, Validator};
use core::ops::{Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Index<Range<usize>>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Output = str;

    #[inline]
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.as_str()[index]
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    Index<RangeFrom<usize>> for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Output = str;

    #[inline]
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.as_str()[index]
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Index<RangeTo<usize>>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Output = str;

    #[inline]
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.as_str()[index]
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Index<RangeFull>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Output = str;

    #[inline]
    fn index(&self, _: RangeFull) -> &Self::Output {
        self.as_str()
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    Index<RangeInclusive<usize>> for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Output = str;

    #[inline]
    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.as_str()[index]
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    Index<RangeToInclusive<usize>> for GString<V, MIN, MAX, ASCII_ONLY>
{
    type Output = str;

    #[inline]
    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        &self.as_str()[index]
    }
}
