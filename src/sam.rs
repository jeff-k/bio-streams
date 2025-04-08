use futures;
use futures::Stream as AsyncIterator;
use std::io;
use std::io::BufRead;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Poll;

pub use crate::error::ParseError;
pub use crate::record::{Alignment, Phred, PseudoAlignment, Record};

pub struct SamRecord;
pub struct SamAlignment;
pub struct SamPseudoAlignment;

impl SamRecord {
    pub fn from_slice<'a, S>(
        data: &'a [u8],
    ) -> impl Iterator<Item = Result<Record<'a, S>, ParseError>> + 'a
    where
        S: for<'b> TryFrom<&'b [u8], Error = ParseError> + 'a,
    {
        todo!();
        std::iter::empty()
    }

    pub fn from_reader<R, S>(
        reader: R,
    ) -> impl Iterator<Item = Result<Record<'static, S>, ParseError>>
    where
        R: BufRead,
        S: for<'b> TryFrom<&'b [u8], Error = ParseError>,
    {
        todo!();
        std::iter::empty()
    }

    pub fn stream<'a, S>(
        data: &'a [u8],
    ) -> impl futures::Stream<Item = Result<Record<'a, S>, ParseError>> + 'a
    where
        S: for<'b> TryFrom<&'b [u8], Error = ParseError> + 'static,
    {
        todo!();
        futures::stream::empty()
    }

    pub fn stream_from_reader<R, S>(
        reader: R,
    ) -> impl futures::Stream<Item = Result<Record<'static, S>, ParseError>>
    where
        R: futures::io::AsyncBufRead + Unpin,
        S: for<'b> TryFrom<&'b [u8], Error = ParseError> + 'static,
    {
        todo!();
        futures::stream::empty()
    }
}

impl SamAlignment {
    pub fn from_slice<'a, S>(
        data: &'a [u8],
    ) -> impl Iterator<Item = Result<Alignment<'a, S>, ParseError>> + 'a
    where
        S: for<'b> TryFrom<&'b [u8], Error = ParseError> + 'a,
    {
        todo!();
        std::iter::empty()
    }

    pub fn from_reader<R, S>(
        reader: R,
    ) -> impl Iterator<Item = Result<Alignment<'static, S>, ParseError>>
    where
        R: BufRead,
        S: for<'b> TryFrom<&'b [u8], Error = ParseError>,
    {
        todo!();
        std::iter::empty()
    }

    pub fn stream<'a, S>(
        data: &'a [u8],
    ) -> impl futures::Stream<Item = Result<Alignment<'a, S>, ParseError>> + 'a
    where
        S: for<'b> TryFrom<&'b [u8], Error = ParseError> + 'static,
    {
        todo!();
        futures::stream::empty()
    }

    pub fn stream_from_reader<R, S>(
        reader: R,
    ) -> impl futures::Stream<Item = Result<Alignment<'static, S>, ParseError>>
    where
        R: futures::io::AsyncBufRead + Unpin,
        S: for<'b> TryFrom<&'b [u8], Error = ParseError> + 'static,
    {
        todo!();
        futures::stream::empty()
    }
}

impl SamPseudoAlignment {
    pub fn from_slice<'a, S>(
        data: &'a [u8],
    ) -> impl Iterator<Item = Result<PseudoAlignment<'a, S>, ParseError>> + 'a
    where
        S: for<'b> TryFrom<&'b [u8], Error = ParseError> + 'a,
    {
        todo!();
        std::iter::empty()
    }

    pub fn from_reader<R, S>(
        reader: R,
    ) -> impl Iterator<Item = Result<PseudoAlignment<'static, S>, ParseError>>
    where
        R: BufRead,
        S: for<'b> TryFrom<&'b [u8], Error = ParseError>,
    {
        todo!();
        std::iter::empty()
    }

    pub fn stream<'a, S>(
        data: &'a [u8],
    ) -> impl futures::Stream<Item = Result<PseudoAlignment<'a, S>, ParseError>> + 'a
    where
        S: for<'b> TryFrom<&'b [u8], Error = ParseError> + 'static,
    {
        todo!();
        futures::stream::empty()
    }

    pub fn stream_from_reader<R, S>(
        reader: R,
    ) -> impl futures::Stream<Item = Result<PseudoAlignment<'static, S>, ParseError>>
    where
        R: futures::io::AsyncBufRead + Unpin,
        S: for<'b> TryFrom<&'b [u8], Error = ParseError> + 'static,
    {
        todo!();
        futures::stream::empty()
    }
}

struct SamIterator<'a, S> {
    data: &'a [u8],
    position: usize,
    _p: PhantomData<S>,
}

impl<'a, S: for<'b> TryFrom<&'b [u8], Error = ParseError>> Iterator for SamIterator<'a, S> {
    type Item = Result<Record<'a, S>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
