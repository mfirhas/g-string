use crate::{GString, Validator};

// ------------------------------------------------------------------------------------
// QUERY APIs
// ------------------------------------------------------------------------------------
impl<V, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool> GString<V, MIN, MAX, ASCII_ONLY>
where
    V: Validator,
{
    #[inline]
    pub fn contains(&self, pat: impl AsRef<str>) -> bool {
        self.as_str().contains(pat.as_ref())
    }

    #[inline]
    pub fn find(&self, pat: impl AsRef<str>) -> Option<usize> {
        self.as_str().find(pat.as_ref())
    }

    #[inline]
    pub fn rfind(&self, pat: impl AsRef<str>) -> Option<usize> {
        self.as_str().rfind(pat.as_ref())
    }

    #[inline]
    pub fn starts_with(&self, pat: impl AsRef<str>) -> bool {
        self.as_str().starts_with(pat.as_ref())
    }

    #[inline]
    pub fn ends_with(&self, pat: impl AsRef<str>) -> bool {
        self.as_str().ends_with(pat.as_ref())
    }
}
