use crate::{GString, Validator};

impl<
    LHSV: Validator,
    RHSV: Validator,
    const LMIN: usize,
    const LMAX: usize,
    const LASCII: bool,
    const RMIN: usize,
    const RMAX: usize,
    const RASCII: bool,
> PartialEq<GString<RHSV, RMIN, RMAX, RASCII>> for GString<LHSV, LMIN, LMAX, LASCII>
{
    fn eq(&self, other: &GString<RHSV, RMIN, RMAX, RASCII>) -> bool {
        self.as_str() == other.as_str()
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
