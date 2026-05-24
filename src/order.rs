use crate::{GString, Validator};

impl<V: Validator + Eq, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Ord
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> PartialOrd
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.as_str().cmp(other))
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> PartialOrd<str>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn partial_cmp(&self, other: &str) -> Option<core::cmp::Ordering> {
        Some(self.as_str().cmp(other))
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> PartialOrd<&str>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn partial_cmp(&self, other: &&str) -> Option<core::cmp::Ordering> {
        Some(self.as_str().cmp(*other))
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    PartialOrd<GString<V, MIN, MAX, ASCII_ONLY>> for str
{
    fn partial_cmp(&self, other: &GString<V, MIN, MAX, ASCII_ONLY>) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other.as_str()))
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    PartialOrd<GString<V, MIN, MAX, ASCII_ONLY>> for &str
{
    fn partial_cmp(&self, other: &GString<V, MIN, MAX, ASCII_ONLY>) -> Option<core::cmp::Ordering> {
        Some((*self).cmp(other.as_str()))
    }
}
