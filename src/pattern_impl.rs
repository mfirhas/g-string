use crate::Pattern;

impl Pattern for char {
    type SplitIter<'a> = core::str::Split<'a, char>;

    type MatchesIter<'a> = core::str::Matches<'a, char>;

    type RMatchesIter<'a> = core::str::RMatches<'a, char>;

    type MatchIndicesIter<'a> = core::str::MatchIndices<'a, char>;

    type RMatchIndicesIter<'a> = core::str::RMatchIndices<'a, char>;

    #[inline]
    fn split<'a>(self, s: &'a str) -> Self::SplitIter<'a>
    where
        Self: 'a,
    {
        s.split(self)
    }

    #[inline]
    fn split_once<'a>(&self, s: &'a str) -> Option<(&'a str, &'a str)> {
        s.split_once(*self)
    }

    #[inline]
    fn rsplit_once<'a>(&self, s: &'a str) -> Option<(&'a str, &'a str)> {
        s.rsplit_once(*self)
    }

    #[inline]
    fn strip_prefix<'a>(&self, s: &'a str) -> Option<&'a str> {
        s.strip_prefix(*self)
    }

    #[inline]
    fn strip_suffix<'a>(&self, s: &'a str) -> Option<&'a str> {
        s.strip_suffix(*self)
    }

    #[inline]
    fn matches<'a>(self, s: &'a str) -> Self::MatchesIter<'a>
    where
        Self: 'a,
    {
        s.matches(self)
    }

    #[inline]
    fn rmatches<'a>(self, s: &'a str) -> Self::RMatchesIter<'a>
    where
        Self: 'a,
    {
        s.rmatches(self)
    }

    #[inline]
    fn match_indices<'a>(self, s: &'a str) -> Self::MatchIndicesIter<'a>
    where
        Self: 'a,
    {
        s.match_indices(self)
    }

    #[inline]
    fn rmatch_indices<'a>(self, s: &'a str) -> Self::RMatchIndicesIter<'a>
    where
        Self: 'a,
    {
        s.rmatch_indices(self)
    }

    #[inline]
    fn contains(&self, s: &str) -> bool {
        s.contains(*self)
    }

    #[inline]
    fn starts_with(&self, s: &str) -> bool {
        s.starts_with(*self)
    }

    #[inline]
    fn ends_with(&self, s: &str) -> bool {
        s.ends_with(*self)
    }

    #[inline]
    fn find(&self, s: &str) -> Option<usize> {
        s.find(*self)
    }

    #[inline]
    fn rfind(&self, s: &str) -> Option<usize> {
        s.rfind(*self)
    }
}

impl<'p> Pattern for &'p str {
    type SplitIter<'a>
        = core::str::Split<'a, &'p str>
    where
        Self: 'a;
    type MatchesIter<'a>
        = core::str::Matches<'a, &'p str>
    where
        Self: 'a;
    type RMatchesIter<'a>
        = core::str::RMatches<'a, &'p str>
    where
        Self: 'a;
    type MatchIndicesIter<'a>
        = core::str::MatchIndices<'a, &'p str>
    where
        Self: 'a;
    type RMatchIndicesIter<'a>
        = core::str::RMatchIndices<'a, &'p str>
    where
        Self: 'a;

    #[inline]
    fn split<'a>(self, s: &'a str) -> Self::SplitIter<'a>
    where
        Self: 'a,
    {
        s.split(self)
    }

    #[inline]
    fn matches<'a>(self, s: &'a str) -> Self::MatchesIter<'a>
    where
        Self: 'a,
    {
        s.matches(self)
    }

    #[inline]
    fn rmatches<'a>(self, s: &'a str) -> Self::RMatchesIter<'a>
    where
        Self: 'a,
    {
        s.rmatches(self)
    }

    #[inline]
    fn match_indices<'a>(self, s: &'a str) -> Self::MatchIndicesIter<'a>
    where
        Self: 'a,
    {
        s.match_indices(self)
    }

    #[inline]
    fn rmatch_indices<'a>(self, s: &'a str) -> Self::RMatchIndicesIter<'a>
    where
        Self: 'a,
    {
        s.rmatch_indices(self)
    }

    #[inline]
    fn split_once<'a>(&self, s: &'a str) -> Option<(&'a str, &'a str)> {
        s.split_once(*self)
    }

    #[inline]
    fn rsplit_once<'a>(&self, s: &'a str) -> Option<(&'a str, &'a str)> {
        s.rsplit_once(*self)
    }

    #[inline]
    fn strip_prefix<'a>(&self, s: &'a str) -> Option<&'a str> {
        s.strip_prefix(*self)
    }

    #[inline]
    fn strip_suffix<'a>(&self, s: &'a str) -> Option<&'a str> {
        s.strip_suffix(*self)
    }

    #[inline]
    fn contains(&self, s: &str) -> bool {
        s.contains(*self)
    }

    #[inline]
    fn starts_with(&self, s: &str) -> bool {
        s.starts_with(*self)
    }

    #[inline]
    fn ends_with(&self, s: &str) -> bool {
        s.ends_with(*self)
    }

    #[inline]
    fn find(&self, s: &str) -> Option<usize> {
        s.find(*self)
    }

    #[inline]
    fn rfind(&self, s: &str) -> Option<usize> {
        s.rfind(*self)
    }
}
