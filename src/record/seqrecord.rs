use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::Range;
use std::borrow::Cow;

use crate::error::ParseError;

pub struct Record<'a, S: for<'b> TryFrom<&'b [u8], Error = ParseError>> {
    pub(crate) data: Cow<'a, [u8]>,
    pub(crate) raw_header: Range<usize>,
    pub(crate) raw_seq: Range<usize>,
    pub(crate) raw_qual: Option<Range<usize>>,
    pub(crate) _p: PhantomData<S>,
}

impl<S: for<'b> TryFrom<&'b [u8], Error = ParseError>> Record<'_, S> {
    pub fn header(&self) -> &[u8] {
        match &self.data {
            Cow::Borrowed(data) => &data[self.raw_header.clone()],
            Cow::Owned(data) => &data[self.raw_header.clone()],
        }
    }

    pub fn seq_bytes(&self) -> &[u8] {
        match &self.data {
            Cow::Borrowed(data) => &data[self.raw_seq.clone()],
            Cow::Owned(data) => &data[self.raw_seq.clone()],
        }
    }

    /// # Errors
    /// Parsing into the target sequence type may fail on bad characters
    pub fn seq(&self) -> Result<S, ParseError> {
        S::try_from(self.seq_bytes())
    }

    /// TODO Parsing phred qualities may be fallible
    pub fn quality(&self) -> Option<&[u8]> {
        self.raw_qual.as_ref().map(|range| match &self.data {
            Cow::Borrowed(data) => &data[range.clone()],
            Cow::Owned(data) => &data[range.clone()],
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn record_field() {
        assert!(false);
    }
}
