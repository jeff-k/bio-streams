use core::marker::{PhantomData, Unpin};

use std::io::BufRead;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::stream::Stream;

use bio_seq::prelude::*;

use crate::Record;

#[derive(Debug, PartialEq)]
pub enum FastaError {
    InvalidId,
    TruncatedRecord,
    InvalidSequence,
    FileError,
}

pub struct Fasta<R: BufRead, T = Seq<Dna>>
where
    T: for<'a> TryFrom<&'a [u8]>,
{
    reader: Pin<Box<R>>,
    line_buf: Vec<u8>,
    field_buf: Option<Vec<u8>>,
    p: PhantomData<T>,
}

impl<R: BufRead + Unpin, T: Unpin + for<'a> TryFrom<&'a [u8]>> Unpin for Fasta<R, T> {}

impl<R: BufRead + Into<Box<R>> + Unpin, T: for<'a> TryFrom<&'a [u8]>> Fasta<R, T> {
    pub fn new(reader: R) -> Self {
        Fasta {
            reader: Box::pin(reader),
            line_buf: Vec::<u8>::with_capacity(256),
            field_buf: None,
            p: PhantomData,
        }
    }

    fn parse_record(&mut self) -> Option<Result<Record<T>, FastaError>> {
        let reader = Pin::get_mut(self.reader.as_mut());

        let mut seq_buf: Vec<u8> = Vec::new();

        // if field_buf is None, first line should start with '>'
        if self.field_buf.is_none() {
            if let Ok(size) = reader.read_until(b'\n', &mut self.line_buf) {
                if size == 0 {
                    // end of stream before a record (empty stream)
                    return None;
                }

                if self.line_buf[0] == b'>' {
                    self.field_buf = Some(Vec::from(&self.line_buf[1..self.line_buf.len() - 1]));
                } else {
                    return Some(Err(FastaError::InvalidId));
                }
                self.line_buf.clear();
            }
        }

        // Read the next non-'>' lines into the sequence buffer
        while let Ok(size) = reader.read_until(b'\n', &mut self.line_buf) {
            if size == 0 {
                // end of stream
                break;
            }
            if self.line_buf[0] == b'>' {
                // new record starts
                break;
            } else {
                // treat this line as sequence
                seq_buf.extend_from_slice(&self.line_buf[..self.line_buf.len() - 1]);
                self.line_buf.clear();
            }
        }

        // return working record
        if let Some(fields) = self.field_buf.take() {
            let seq = match T::try_from(&seq_buf) {
                Ok(s) => s,
                Err(_) => {
                    return Some(Err(FastaError::InvalidSequence));
                }
            };
            return Some(Ok(Record {
                fields,
                seq,
                quality: None,
            }));
        }

        None
    }
}

impl<R: BufRead + Unpin, A: Codec> Iterator for Fasta<R, Seq<A>> {
    type Item = Result<Record<Seq<A>>, FastaError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_record()
    }
}

impl<R: BufRead + Unpin, T: Unpin + for<'a> TryFrom<&'a [u8]>> Stream for Fasta<R, T> {
    type Item = Result<Record<T>, FastaError>;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<<Self as Stream>::Item>> {
        let record = unsafe { self.get_unchecked_mut().parse_record() };

        Poll::Ready(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_test::task::noop_waker;
    use std::io::Cursor;
    use std::task::{Context, Poll};

    #[test]
    fn test_fasta_iterator() {
        let data = b">SEQ_ID_1
ACTCGATCGCGACG
ACACGATCGCGCGC
CATCGACTACGGCG
>SEQ_ID_2
GGGGGGGGGGGGGG\n";
        let reader = Cursor::new(data as &[u8]);
        let mut fasta: Fasta<Cursor<&[u8]>, Seq<Dna>> = Fasta::new(reader);

        let record1 = fasta.next().unwrap().unwrap();
        assert_eq!(record1.fields, b"SEQ_ID_1".to_vec());
        assert_eq!(
            record1.seq,
            dna!("ACTCGATCGCGACGACACGATCGCGCGCCATCGACTACGGCG")
        );

        let record2 = fasta
            .next()
            .expect("Expected a record")
            .expect("Expected valid record");
        assert_eq!(record2.fields, b"SEQ_ID_2".to_vec());
        assert_eq!(record2.seq, dna!("GGGGGGGGGGGGGG"));

        assert!(fasta.next().is_none(), "Expected no more records");
    }

    #[test]
    fn test_fasta_poll_next() {
        let data = b">SEQ_ID_1
ACTCGATCGCGACG
ACACGATCGCGCGC
CATCGACTACGGCG
>SEQ_ID_2
GGGGGGGGGGGGGG\n";

        let reader = Cursor::new(data as &[u8]);
        let mut fasta: Pin<Box<Fasta<Cursor<&[u8]>, Seq<Dna>>>> =
            Pin::new(Box::new(Fasta::new(reader)));

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // Manual polling using poll_next
        match fasta.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.fields, b"SEQ_ID_1".to_vec());
                assert_eq!(
                    record.seq,
                    dna!("ACTCGATCGCGACGACACGATCGCGCGCCATCGACTACGGCG")
                );
            }
            _ => panic!("Unexpected result"),
        }

        match fasta.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.fields, b"SEQ_ID_2".to_vec());
                assert_eq!(record.seq, dna!("GGGGGGGGGGGGGG"));
            }
            _ => panic!("Unexpected result"),
        }

        assert_eq!(fasta.as_mut().poll_next(&mut cx), Poll::Ready(None));
    }
}
