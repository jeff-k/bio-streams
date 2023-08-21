use core::marker::{PhantomData, Unpin};

use std::io::BufRead;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::stream::Stream;

use bio_seq::prelude::*;

use crate::record::Phred;
use crate::Record;

#[derive(Debug)]
pub enum FastqError {
    InvalidSeparationLine,
    InvalidId,
    TruncatedRecord,
    InvalidSequence,
    InvalidQuality,
    FileError,
}

pub struct Fastq<R: BufRead, T = Seq<Dna>>
where
    T: for<'a> TryFrom<&'a [u8]>,
{
    reader: Pin<Box<R>>,
    id_buf: Vec<u8>,
    seq_buf: Vec<u8>,
    sep_buf: Vec<u8>,
    qual_buf: Vec<u8>,
    p: PhantomData<T>,
}

impl<R: BufRead + Unpin, T: Unpin + for<'a> TryFrom<&'a [u8]>> Unpin for Fastq<R, T> {}

impl<R: BufRead + Into<Box<R>> + Unpin, T: for<'a> TryFrom<&'a [u8]>> Fastq<R, T> {
    pub fn new(reader: R) -> Self {
        Fastq {
            reader: Box::pin(reader),
            id_buf: Vec::<u8>::with_capacity(256),
            seq_buf: Vec::<u8>::with_capacity(512),
            sep_buf: Vec::<u8>::with_capacity(4),
            qual_buf: Vec::<u8>::with_capacity(512),
            p: PhantomData,
        }
    }

    fn parse_record(&mut self) -> Option<Result<Record<T>, FastqError>> {
        let mut quality = Vec::<Phred>::new();
        let reader = Pin::get_mut(self.reader.as_mut());

        self.id_buf.clear();
        self.seq_buf.clear();
        self.sep_buf.clear();
        self.qual_buf.clear();

        if reader.read_until(b'\n', &mut self.id_buf).is_err() {
            return Some(Err(FastqError::FileError));
        }
        if self.id_buf.is_empty() {
            // This is the only condition where an empty reader means
            // that the file has successfully finished reading
            return None;
        }
        // The id line must begin with '@'
        if self.id_buf[0] != b'@' {
            return Some(Err(FastqError::InvalidId));
        }

        if reader.read_until(b'\n', &mut self.seq_buf).is_err() {
            return Some(Err(FastqError::FileError));
        }
        if self.seq_buf.is_empty() {
            return Some(Err(FastqError::TruncatedRecord));
        }

        if reader.read_until(b'\n', &mut self.sep_buf).is_err() {
            return Some(Err(FastqError::FileError));
        }
        if self.sep_buf.is_empty() {
            return Some(Err(FastqError::TruncatedRecord));
        }

        // Detect whether the '+' separation line is valid
        if self.sep_buf.len() != 2 || self.sep_buf[0] != b'+' {
            return Some(Err(FastqError::InvalidSeparationLine));
        }
        if reader.read_until(b'\n', &mut self.qual_buf).is_err() {
            return Some(Err(FastqError::FileError));
        }
        if self.qual_buf.is_empty() {
            return Some(Err(FastqError::TruncatedRecord));
        }

        // Parse the contents of the sequence and quality lines
        if self.qual_buf.len() != self.seq_buf.len() {
            return Some(Err(FastqError::InvalidQuality));
        }

        let seq = match T::try_from(&self.seq_buf[..self.seq_buf.len() - 2]) {
            Ok(parsed_seq) => parsed_seq,
            Err(_) => return Some(Err(FastqError::InvalidSequence)),
        };

        quality.extend(
            self.qual_buf[..self.qual_buf.len() - 2]
                .iter()
                .map(|q| Phred::from(*q)),
        );

        Some(Ok(Record {
            fields: Vec::<u8>::from(&self.id_buf[1..self.id_buf.len() - 2]),
            seq,
            quality: Some(quality),
        }))
    }
}

impl<R: BufRead + Unpin, A: Codec> Iterator for Fastq<R, Seq<A>> {
    type Item = Result<Record<Seq<A>>, FastqError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_record()
    }
}

impl<R: BufRead + Unpin, T: Unpin + for<'a> TryFrom<&'a [u8]>> Stream for Fastq<R, T> {
    type Item = Result<Record<T>, FastqError>;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<<Self as Stream>::Item>> {
        let record = unsafe { self.get_unchecked_mut().parse_record() };

        Poll::Ready(record)
    }
}
