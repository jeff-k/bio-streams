use core::marker::{PhantomData, Unpin};

use std::io::BufRead;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::stream::Stream;

use bio_seq::prelude::*;

use crate::record::Phred;
use crate::{FastxError, Reader, Record};

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
}

impl<R: BufRead + Unpin, T: Unpin + for<'a> TryFrom<&'a [u8]>> Iterator for Fastq<R, T> {
    type Item = Result<Record<T>, FastxError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_record()
    }
}

impl<R: BufRead + Unpin, T: Unpin + for<'a> TryFrom<&'a [u8]>> Stream for Fastq<R, T> {
    type Item = Result<Record<T>, FastxError>;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<<Self as Stream>::Item>> {
        let record = unsafe { self.get_unchecked_mut().parse_record() };

        Poll::Ready(record)
    }
}

impl<R: BufRead + Unpin, T: Unpin + for<'a> TryFrom<&'a [u8]>> Reader<T> for Fastq<R, T> {
    type Error = FastxError;

    fn parse_record(&mut self) -> Option<Result<Record<T>, Self::Error>> {
        let mut quality = Vec::<Phred>::new();
        let reader = Pin::get_mut(self.reader.as_mut());

        self.id_buf.clear();
        self.seq_buf.clear();
        self.sep_buf.clear();
        self.qual_buf.clear();

        if reader.read_until(b'\n', &mut self.id_buf).is_err() {
            return Some(Err(FastxError::FileError));
        }
        if self.id_buf.is_empty() {
            // This is the only condition where an empty reader means
            // that the file has successfully finished reading
            return None;
        }
        // The id line must begin with '@'
        if self.id_buf[0] != b'@' {
            return Some(Err(FastxError::InvalidId(
                String::from_utf8_lossy(&self.id_buf).into_owned(),
            )));
        }

        if reader.read_until(b'\n', &mut self.seq_buf).is_err() {
            return Some(Err(FastxError::FileError));
        }
        if self.seq_buf.is_empty() {
            return Some(Err(FastxError::TruncatedRecord));
        }

        if reader.read_until(b'\n', &mut self.sep_buf).is_err() {
            return Some(Err(FastxError::FileError));
        }
        if self.sep_buf.is_empty() {
            return Some(Err(FastxError::TruncatedRecord));
        }

        // Detect whether the '+' separation line is valid
        if self.sep_buf.len() != 2 || self.sep_buf[0] != b'+' {
            return Some(Err(FastxError::InvalidSeparationLine));
        }
        if reader.read_until(b'\n', &mut self.qual_buf).is_err() {
            return Some(Err(FastxError::FileError));
        }
        if self.qual_buf.is_empty() {
            return Some(Err(FastxError::TruncatedRecord));
        }

        // Parse the contents of the sequence and quality lines
        if self.qual_buf.len() != self.seq_buf.len() {
            return Some(Err(FastxError::InvalidQuality));
        }

        let Ok(seq) = T::try_from(&self.seq_buf[..self.seq_buf.len() - 1]) else {
            return Some(Err(FastxError::InvalidSequence(
                String::from_utf8_lossy(&self.seq_buf).into_owned(),
            )));
        };

        quality.extend(
            self.qual_buf[..self.qual_buf.len() - 1]
                .iter()
                .map(|q| Phred::from(*q)),
        );

        Some(Ok(Record {
            fields: Vec::<u8>::from(&self.id_buf[1..self.id_buf.len() - 1]),
            seq,
            quality: Some(quality),
        }))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use futures_test::task::noop_waker;
    use std::io::Cursor;
    use std::task::{Context, Poll};

    #[test]
    fn test_fastq_iterator() {
        let data = b"@SEQ_ID_1
ACTCGATCGCGACG
+
FFFFFFFFFFFFFF
@SEQ_ID_2
CATCGACTACGGCG
+
GGGGGGGGGGGGGG\n";
        let reader = Cursor::new(data as &[u8]);
        let mut fastq: Fastq<Cursor<&[u8]>, Seq<Dna>> = Fastq::new(reader);

        let record1 = fastq.next().unwrap().unwrap();
        assert_eq!(record1.fields, b"SEQ_ID_1".to_vec());
        assert_eq!(record1.seq, dna!("ACTCGATCGCGACG"));
        //assert_eq!(record1.quality, b"FFFFFFFFFFFFFF");

        let record2 = fastq
            .next()
            .expect("Expected a record")
            .expect("Expected valid record");
        assert_eq!(record2.fields, b"SEQ_ID_2".to_vec());
        assert_eq!(record2.seq, dna!("CATCGACTACGGCG"));

        assert!(fastq.next().is_none(), "Expected no more records");
    }

    #[test]
    fn test_fastq_poll_next() {
        let data = b"@SEQ_ID_1
ACTCGATCGCGACG
+
FFFFFFFFFFFFFF
@SEQ_ID_2
CATCGACTACGGCG
+
GGGGGGGGGGGGGG\n";

        let reader = Cursor::new(data as &[u8]);
        let mut fastq: Pin<Box<Fastq<Cursor<&[u8]>, Seq<Dna>>>> =
            Pin::new(Box::new(Fastq::new(reader)));

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // Manual polling using poll_next
        match fastq.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.fields, b"SEQ_ID_1".to_vec());
                assert_eq!(record.seq, dna!("ACTCGATCGCGACG"));
                //assert_eq!(record.quality, "FFFFFFFFFFFFFF");
            }
            _ => panic!("Unexpected result"),
        }

        match fastq.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.fields, b"SEQ_ID_2".to_vec());
                assert_eq!(record.seq, dna!("CATCGACTACGGCG"));
            }
            _ => panic!("Unexpected result"),
        }

        assert_eq!(fastq.as_mut().poll_next(&mut cx), Poll::Ready(None));
    }
}
