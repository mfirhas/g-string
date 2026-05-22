impl<V: Validator + Eq, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> Ord
    for GString<V, MIN, MAX, ASCII_ONLY>
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<
    LHSV: Validator + Eq,
    RHSV: Validator + Eq,
    const MIN: usize,
    const MAX: usize,
    const ASCII_ONLY: bool,
> PartialOrd<GString<RHSV, MIN, MAX, ASCII_ONLY>> for GString<LHSV, MIN, MAX, ASCII_ONLY>
{
    fn partial_cmp(
        &self,
        other: &GString<RHSV, MIN, MAX, ASCII_ONLY>,
    ) -> Option<core::cmp::Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}
