use crate::{GString, GStringError, Validator};

// ------------------------------------------------------------------------------------
// MUTATION APIs
// ------------------------------------------------------------------------------------
impl<V: Validator, const MIN: usize, const MAX: usize, const ASCII_ONLY: bool>
    GString<V, MIN, MAX, ASCII_ONLY>
{
    pub fn push(&mut self, ch: char) -> Result<(), GStringError<V::Err>> {
        // char takes up to 4 bytes
        let mut buf = [0u8; 4];
        let encoded = ch.encode_utf8(&mut buf);

        self.push_str(encoded)
    }

    pub fn push_str(&mut self, s: &str) -> Result<(), GStringError<V::Err>> {
        let mut buf = [0u8; MAX];

        // copy current content
        buf[..self.len].copy_from_slice(&self.buf[..self.len]);

        // append new content
        let bytes = s.as_bytes();
        let end = self.len + bytes.len();

        if end > MAX {
            return Err(GStringError::TooLong);
        }

        buf[self.len..end].copy_from_slice(bytes);

        // SAFETY:
        // existing bytes are valid UTF-8
        // appended bytes come from &str
        // concatenation of valid UTF-8 is valid UTF-8
        let candidate = unsafe { core::str::from_utf8_unchecked(&buf[..end]) };

        // fully revalidate through centralized constructor
        *self = Self::try_new(candidate)?;

        Ok(())
    }

    pub fn insert(&mut self, idx: usize, ch: char) -> Result<(), GStringError<V::Err>> {
        let mut buf = [0u8; 4];
        let encoded = ch.encode_utf8(&mut buf);

        self.insert_str(idx, encoded)
    }

    pub fn insert_str(&mut self, idx: usize, string: &str) -> Result<(), GStringError<V::Err>> {
        if !self.as_str().is_char_boundary(idx) {
            return Err(GStringError::Mutation("idx is not a char boundary"));
        }

        let insert_bytes = string.as_bytes();
        let insert_len = insert_bytes.len();

        let new_len = self.len + insert_len;

        // early capacity check to avoid panic
        if new_len > MAX {
            return Err(GStringError::TooLong);
        }

        let mut buf = [0u8; MAX];

        // before insertion point
        buf[..idx].copy_from_slice(&self.buf[..idx]);

        // inserted bytes
        buf[idx..idx + insert_len].copy_from_slice(insert_bytes);

        let tail_len = self.len - idx;

        // after insertion point
        buf[idx + insert_len..idx + insert_len + tail_len]
            .copy_from_slice(&self.buf[idx..self.len]);

        // SAFETY:
        // - original bytes are valid UTF-8
        // - inserted string is valid UTF-8
        // - insertion only happens at char boundary
        // - concatenation of valid UTF-8 is valid UTF-8
        let candidate = unsafe { core::str::from_utf8_unchecked(&buf[..new_len]) };

        *self = Self::try_new(candidate)?;

        Ok(())
    }

    pub fn pop(&mut self) -> Result<Option<char>, GStringError<V::Err>> {
        if self.is_empty() {
            return Ok(None);
        }

        let s = self.as_str();
        let ch = s.chars().next_back();
        let ch = if let Some(ch) = ch {
            ch
        } else {
            return Ok(None);
        };

        let new_len = self.len - ch.len_utf8();

        // SAFETY:
        // truncating at char boundary preserves UTF-8 validity
        let candidate = unsafe { core::str::from_utf8_unchecked(&self.buf[..new_len]) };

        match Self::try_new(candidate) {
            Ok(new) => {
                *self = new;
                Ok(Some(ch))
            }
            Err(err) => Err(err),
        }
    }

    pub fn remove(&mut self, idx: usize) -> Result<char, GStringError<V::Err>> {
        if !self.as_str().is_char_boundary(idx) {
            return Err(GStringError::Mutation("idx is not a char boundary"));
        }

        let s = self.as_str();

        let ch = s[idx..].chars().next().ok_or(GStringError::Mutation(
            "cannot remove char from empty index",
        ))?;

        let ch_len = ch.len_utf8();

        let new_len = self.len - ch_len;

        let mut buf = [0u8; MAX];

        // before removed char
        buf[..idx].copy_from_slice(&self.buf[..idx]);

        // after removed char
        buf[idx..new_len].copy_from_slice(&self.buf[idx + ch_len..self.len]);

        // SAFETY:
        // removal at char boundary preserves UTF-8 validity
        let candidate = unsafe { core::str::from_utf8_unchecked(&buf[..new_len]) };

        *self = Self::try_new(candidate)?;

        Ok(ch)
    }

    pub fn truncate(&mut self, new_len: usize) -> Result<(), GStringError<V::Err>> {
        if new_len >= self.len {
            return Ok(());
        }

        if !self.as_str().is_char_boundary(new_len) {
            return Err(GStringError::Mutation("new_len is not a char boundary"));
        }

        // SAFETY:
        // truncating at char boundary preserves UTF-8 validity
        let candidate = unsafe { core::str::from_utf8_unchecked(&self.buf[..new_len]) };

        *self = Self::try_new(candidate)?;

        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), GStringError<V::Err>> {
        if MIN != 0 {
            return Err(GStringError::Mutation(
                "cannot clear GString if MIN is not zero",
            ));
        }

        *self = Self::try_new("")?;

        Ok(())
    }

    #[cfg(feature = "alloc")]
    pub fn replace(&mut self, from: &str, to: &str) -> Result<(), GStringError<V::Err>> {
        // .replace requires allocation
        let replaced = self.as_str().replace(from, to);

        *self = Self::try_new(&replaced)?;

        Ok(())
    }

    pub fn replace_range<R>(
        &mut self,
        range: R,
        replace_with: &str,
    ) -> Result<(), GStringError<V::Err>>
    where
        R: core::ops::RangeBounds<usize>,
    {
        use core::ops::Bound;

        let start = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n
                .checked_add(1)
                .ok_or(GStringError::Mutation("range overflow"))?,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&n) => n
                .checked_add(1)
                .ok_or(GStringError::Mutation("range overflow"))?,
            Bound::Excluded(&n) => n,
            Bound::Unbounded => self.len,
        };

        let s = self.as_str();

        if !s.is_char_boundary(start) {
            return Err(GStringError::Mutation("start range is not within boundary"));
        }
        if !s.is_char_boundary(end) {
            return Err(GStringError::Mutation("end range is not within boundary"));
        }
        if start > end {
            return Err(GStringError::Mutation(
                "start range cannot be bigger than end",
            ));
        }

        let mut buf = [0u8; MAX];

        let before = &self.buf[..start];
        let middle = replace_with.as_bytes();
        let after = &self.buf[end..self.len];

        let new_len = before
            .len()
            .checked_add(middle.len())
            .and_then(|n| n.checked_add(after.len()))
            .ok_or(GStringError::TooLong)?;

        if new_len > MAX {
            return Err(GStringError::TooLong);
        }

        // before
        buf[..before.len()].copy_from_slice(before);

        // replacement
        buf[before.len()..before.len() + middle.len()].copy_from_slice(middle);

        // after
        buf[before.len() + middle.len()..new_len].copy_from_slice(after);

        // SAFETY:
        // - original string valid UTF-8
        // - replace_with valid UTF-8
        // - slicing only at char boundaries
        let candidate = unsafe { core::str::from_utf8_unchecked(&buf[..new_len]) };

        *self = Self::try_new(candidate)?;

        Ok(())
    }
}
