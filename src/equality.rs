use crate::{GString, Validator};

impl<LHSV: Validator, RHSV: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    PartialEq<GString<RHSV, MIN, MAX, ASCII_ONLY>> for GString<LHSV, MIN, MAX, ASCII_ONLY>
{
    fn eq(&self, other: &GString<RHSV, MIN, MAX, ASCII_ONLY>) -> bool {
        self.len == other.len && self.buf[..self.len] == other.buf[..other.len]
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> PartialEq<str>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> PartialEq<&str>
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    PartialEq<GString<V, MIN, MAX, ASCII_ONLY>> for str
{
    fn eq(&self, other: &GString<V, MIN, MAX, ASCII_ONLY>) -> bool {
        self == other.as_str()
    }
}

impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    PartialEq<GString<V, MIN, MAX, ASCII_ONLY>> for &str
{
    fn eq(&self, other: &GString<V, MIN, MAX, ASCII_ONLY>) -> bool {
        *self == other.as_str()
    }
}
