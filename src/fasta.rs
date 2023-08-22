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
    buf: Vec<u8>,
    p: PhantomData<T>,
}

impl<R: BufRead + Unpin, T: Unpin + for<'a> TryFrom<&'a [u8]>> Unpin for Fasta<R, T> {}

impl<R: BufRead + Into<Box<R>> + Unpin, T: for<'a> TryFrom<&'a [u8]>> Fasta<R, T> {
    pub fn new(reader: R) -> Self {
        Fasta {
            reader: Box::pin(reader),
            buf: Vec::<u8>::with_capacity(256),
            p: PhantomData,
        }
    }

    fn parse_record(&mut self) -> Option<Result<Record<T>, FastaError>> {
        let mut fields: Vec<u8> = Vec::new();
        let reader = Pin::get_mut(self.reader.as_mut());

        self.buf.clear();

        // unlike with the fastq parser, this will read an arbitrary number of lines
        // to construct the sequence of a record. So this will be in a loop:
        if reader.read_until(b'\n', &mut self.buf).is_err() {
            // ...

            // if this is the first id line for a record, starting with '>':
            fields = Vec::<u8>::from(&self.buf[1..self.buf.len() - 1]);
        }

        // construct the seq
        let seq: T = unimplemented!();

        Some(Ok(Record {
            fields,
            seq,
            quality: None,
        }))
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
    fn test_fastq_iterator() {
        let data = b">SEQ_ID_1
ACTCGATCGCGACG
ACACGATCGCGCGC
CATCGACTACGGCG
>SEQ_ID_2
GGGGGGGGGGGGGG\n";
        let reader = Cursor::new(data as &[u8]);
        let mut fastq: Fasta<Cursor<&[u8]>, Seq<Dna>> = Fasta::new(reader);

        let record1 = fastq.next().unwrap().unwrap();
        assert_eq!(record1.fields, b"SEQ_ID_1".to_vec());
        assert_eq!(
            record1.seq,
            dna!("ACTCGATCGCGACGACACGATCGCGCGCCATCGACTACGGCG")
        );

        let record2 = fastq
            .next()
            .expect("Expected a record")
            .expect("Expected valid record");
        assert_eq!(record2.fields, b"SEQ_ID_2".to_vec());
        assert_eq!(record2.seq, dna!("GGGGGGGGGGGGGG"));

        assert!(fastq.next().is_none(), "Expected no more records");
    }

    #[test]
    fn test_fastq_poll_next() {
        let data = b">SEQ_ID_1
ACTCGATCGCGACG
ACACGATCGCGCGC
CATCGACTACGGCG
>SEQ_ID_2
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
                assert_eq!(
                    record.seq,
                    dna!("ACTCGATCGCGACGACACGATCGCGCGCCATCGACTACGGCG")
                );
            }
            _ => panic!("Unexpected result"),
        }

        match fastq.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.fields, b"SEQ_ID_2".to_vec());
                assert_eq!(record.seq, dna!("GGGGGGGGGGGGGG"));
            }
            _ => panic!("Unexpected result"),
        }

        assert_eq!(fastq.as_mut().poll_next(&mut cx), Poll::Ready(None));
    }
}
