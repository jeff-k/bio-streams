use core::marker::{PhantomData, Unpin};

use std::io::BufRead;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::stream::Stream;

use bio_seq::prelude::*;

use crate::{FastxError, Reader, Record};

pub struct Fasta<R: BufRead, T = Seq<Dna>>
where
    T: for<'a> TryFrom<&'a [u8]>,
{
    reader: Pin<Box<R>>,
    line_buf: Vec<u8>,
    field_buf: Option<Vec<u8>>,
    p: PhantomData<T>,
}

fn end_pos(line_buf: &[u8]) -> usize {
    if line_buf.ends_with(b"\r\n") {
        line_buf.len() - 2
    } else if line_buf.ends_with(b"\n") {
        line_buf.len() - 1
    } else {
        line_buf.len()
    }
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

    fn parse_record(&mut self) -> Option<Result<Record<T>, FastxError>> {
        let reader = Pin::get_mut(self.reader.as_mut());

        let mut seq_buf: Vec<u8> = Vec::new();
        // if field_buf is None, first line should start with '>'
        let fields: Vec<u8> = if self.field_buf.is_none() {
            self.line_buf.clear();
            if let Ok(size) = reader.read_until(b'\n', &mut self.line_buf) {
                if size == 0 {
                    // end of stream before a record (empty stream)
                    return None;
                }

                if self.line_buf[0] == b'>' {
                    //                    let end = end_pos(&self.line_buf);
                    Vec::from(&self.line_buf[1..end_pos(&self.line_buf)])
                } else {
                    return Some(Err(FastxError::InvalidId(
                        String::from_utf8_lossy(&self.line_buf).into_owned(),
                    )));
                }
            } else {
                // premature end of fasta?
                return Some(Err(FastxError::TruncatedRecord));
            }
        } else {
            self.field_buf.take().unwrap()
        };

        // Read the next non-'>' lines into the sequence buffer
        self.line_buf.clear();
        while let Ok(size) = reader.read_until(b'\n', &mut self.line_buf) {
            if size == 0 {
                // end of stream
                break;
            }
            if self.line_buf[0] == b'>' {
                // new record starts
                self.field_buf = Some(Vec::from(&self.line_buf[1..end_pos(&self.line_buf)]));
                break;
            }
            // treat this line as sequence
            seq_buf.extend_from_slice(&self.line_buf[..end_pos(&self.line_buf)]);
            self.line_buf.clear();
        }

        let Ok(seq) = T::try_from(&seq_buf) else {
            // TODO
            return Some(Err(FastxError::InvalidSequence("TODO".to_string())));
        };
        Some(Ok(Record {
            fields,
            seq,
            quality: None,
        }))
    }
}

impl<R: BufRead + Unpin, T: Unpin + for<'a> TryFrom<&'a [u8]>> Iterator for Fasta<R, T> {
    type Item = Result<Record<T>, FastxError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_record()
    }
}

impl<R: BufRead + Unpin, T: Unpin + for<'a> TryFrom<&'a [u8]>> Stream for Fasta<R, T> {
    type Item = Result<Record<T>, FastxError>;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<<Self as Stream>::Item>> {
        let record = unsafe { self.get_unchecked_mut().parse_record() };

        Poll::Ready(record)
    }
}

impl<R: BufRead + Unpin, T: Unpin + for<'a> TryFrom<&'a [u8]>> Reader<T> for Fasta<R, T> {}

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
AAAAAAAAAAAAAA
CCCCCCCCCCCCC
GGGGGGGGGGGG
>SEQ_ID_2
TTTTTTTTTTT\n";

        let reader = Cursor::new(data as &[u8]);
        let mut fasta: Pin<Box<Fasta<Cursor<&[u8]>, Seq<Dna>>>> =
            Pin::new(Box::new(Fasta::new(reader)));

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // Manual polling using poll_next
        match fasta.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.fields, b"SEQ_ID_1".to_vec());
                assert_eq!(record.seq, dna!("AAAAAAAAAAAAAACCCCCCCCCCCCCGGGGGGGGGGGG"));
            }
            _ => panic!("Unexpected result"),
        }

        match fasta.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.fields, b"SEQ_ID_2".to_vec());
                assert_eq!(record.seq, dna!("TTTTTTTTTTT"));
            }
            e => panic!("Unexpected result {:?}", e),
        }

        assert_eq!(fasta.as_mut().poll_next(&mut cx), Poll::Ready(None));
    }

    #[test]
    fn test_fasta_poll_next_with_crlf() {
        let data = b">SEQ_ID_1\r\nAAAAAAAAAAAAAA\r\nCCCCCCCCCCCCC\r\nGGGGGGGGGGGG\r\n>SEQ_ID_2\r\nTTTTTTTTTTT";

        let reader = Cursor::new(data as &[u8]);
        let mut fasta: Pin<Box<Fasta<Cursor<&[u8]>, Seq<Dna>>>> =
            Pin::new(Box::new(Fasta::new(reader)));

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // Manual polling using poll_next
        match fasta.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.fields, b"SEQ_ID_1".to_vec());
                assert_eq!(record.seq, dna!("AAAAAAAAAAAAAACCCCCCCCCCCCCGGGGGGGGGGGG"));
            }
            _ => panic!("Unexpected result"),
        }

        match fasta.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.fields, b"SEQ_ID_2".to_vec());
                assert_eq!(record.seq, dna!("TTTTTTTTTTT"));
            }
            e => panic!("Unexpected result {:?}", e),
        }

        assert_eq!(fasta.as_mut().poll_next(&mut cx), Poll::Ready(None));
    }

    #[test]
    fn test_fasta_poll_next_no_eol() {
        let data = b">SEQ_ID_X\nAAAAAAAAAAAAAACCCCCCCCCCCCCGGGGGGGGGGGGACGTAAA";

        let reader = Cursor::new(data as &[u8]);
        let mut fasta: Pin<Box<Fasta<Cursor<&[u8]>, Seq<Dna>>>> =
            Pin::new(Box::new(Fasta::new(reader)));

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // Manual polling using poll_next
        match fasta.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.fields, b"SEQ_ID_X".to_vec());
                assert_eq!(
                    record.seq,
                    dna!("AAAAAAAAAAAAAACCCCCCCCCCCCCGGGGGGGGGGGGACGTAAA")
                );
            }
            _ => panic!("Unexpected result"),
        }

        assert_eq!(fasta.as_mut().poll_next(&mut cx), Poll::Ready(None));
    }
}
