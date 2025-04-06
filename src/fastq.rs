use futures;
use futures::Stream as AsyncIterator;
use std::io;
use std::io::BufRead;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Poll;

pub use crate::error::ParseError;
pub use crate::record::{Phred, Record};

/*
fn build_record_borrowed<'a, S: TryFrom<&'a [u8], Error = ParseError>>(
    lines: &[&'a [u8]; 4],
) -> Result<Record<&'a [u8], S>, ParseError> {
    // test for valid header start

    if lines[0][0] != b'@' {
        return Err(ParseError::InvalidDelimiter);
    }

    // test for valid separator line
    let sep_pos = lines[2][0];
    if !sep_pos == b'+' {
        return Err(ParseError::InvalidDelimiter);
    }

    // test that quality and sequence strings are equal length
    if lines[1].len() != lines[3].len() {
        return Err(ParseError::LengthMismatch);
    }

/*
    Ok(Record {
        raw_fields: &lines[0][1..],
        raw_seq: lines[1],
        raw_quality: Some(lines[3]),
        _p: PhantomData,
    })
*/
    todo!()
}
*/

pub struct Fastq;

impl Fastq {
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

/*
fn parse_next(&mut self) -> Option<Result<Record<&'src [u8], S>, ParseError>> {
    if self.pos >= self.buffer.len() {
        return None;
    }

    let mut lines: [&[u8]; 4] = [&[]; 4];

    for crs_i in &mut lines {
        if let Some(n) = self.buffer[self.pos..].iter().position(|&b| b == b'\n') {
            *crs_i = &self.buffer[self.pos..self.pos + n];
            self.pos += n + 1;
        } else {
            // truncated records
            return Some(Err(ParseError::TruncatedRecord));
        }
    }

    Some(build_record_borrowed(&lines))
}
*/

/*
pub struct FastqReader<R: BufRead + Unpin, S: TryFrom<Vec<u8>> = Vec<u8>> {
    reader: Pin<Box<R>>,
    buffer: Vec<u8>,
    _s: PhantomData<S>,
}
*/

/*
impl<R: BufRead + Unpin, S: TryFrom<Vec<u8>>> FastqReader<R, S> {
    pub fn new(reader: R) -> Self {
        FastqReader {
            reader: Box::pin(reader),
            buffer: Vec::<u8>::with_capacity(1024),
            _s: PhantomData,
        }
    }

    fn parse(&mut self) -> Option<Result<Record<Vec<u8>, S>, std::io::Error>> {
        self.buffer.clear();

        // total bytes read
        let mut t_bs = 0;

        // indices of carriage returns
        let mut crs: [usize; 4] = [0; 4];

        let reader = self.reader.as_mut().get_mut();

        for i in 0..4 {
            match reader.read_until(b'\n', &mut self.buffer) {
                // end of file
                Ok(0) => {
                    return if i == 0 {
                        // proper file end
                        None
                    } else {
                        // truncated records
                        return Some(Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "Truncated FASTQ record",
                        )));
                    };
                }
                Ok(n) => {
                    crs[i] = n + t_bs;
                    t_bs += n;
                }
                Err(e) => return Some(Err(e)),
            }
        }

        let buf = unsafe { std::slice::from_raw_parts(self.buffer.as_ptr(), self.buffer.len()) };

        let lines: [&[u8]; 4] = [
            &buf[0..crs[0] - 1],
            &buf[crs[0]..crs[1] - 1],
            &buf[crs[1]..crs[2] - 1],
            &buf[crs[2]..crs[3] - 1],
        ];

        Some(build_record_owned(&lines))
    }
}
*/

/*
impl<R: BufRead + Unpin, S: TryFrom<Vec<u8>>> Iterator for &mut FastqReader<R, S> {
    type Item = Result<Record<Vec<u8>, S>, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse()
    }
}
*/

/*
impl<R: BufRead + Unpin, S: TryFrom<Vec<u8>>> AsyncIterator for FastqReader<R, S> {
    type Item = Result<Record<Vec<u8>, S>, std::io::Error>;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let record = unsafe { self.get_unchecked_mut().parse() };

        Poll::Ready(record)
    }
}
*/

struct FastqIterator<'a, S> {
    data: &'a [u8],
    position: usize,
    _p: PhantomData<S>,
}

impl<'a, S: for<'b> TryFrom<&'b [u8], Error = ParseError>> Iterator for FastqIterator<'a, S> {
    type Item = Result<Record<'a, S>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/*
impl<'a, S: for<'b> TryFrom<&'b [u8]>> AsyncIterator for Fastq<'a, S> {
    type Item = Result<Record<&'a [u8], S>, std::io::Error>;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let record = unsafe { self.get_unchecked_mut().parse() };

        Poll::Ready(record)
    }
}
    */

/*
#[cfg(test)]
mod tests {
    use super::*;
    use futures::task::noop_waker;
    //    use futures::Stream;
    use std::io::Cursor;
    use std::iter::Iterator;
    use std::task::Context;

    const FQ1: &'static [u8] = b"@SEQ_ID_1
ACTCGATCGCGACGAA
+
AFFFFFFFFFFFFEBA
@SEQ_ID_2
CATCGACTACGGCG
+
GGGGGGGGGGGGGG\n";

    #[test]
    fn test_fastq_iterator() {
        let mut fastq: Fastq<&[u8]> = Fastq::new(FQ1);

        let record1: Record<&[u8]> = fastq.next().unwrap().unwrap();
        assert_eq!(record1.raw_fields, b"SEQ_ID_1");
        assert_eq!(record1.raw_seq, b"ACTCGATCGCGACGAA");
        assert_eq!(record1.raw_quality.unwrap(), b"AFFFFFFFFFFFFEBA");

        let record2 = fastq
            .next()
            .expect("Expected a record")
            .expect("Expected valid record");
        assert_eq!(record2.raw_fields, b"SEQ_ID_2".to_vec());
        assert_eq!(record2.raw_seq, b"CATCGACTACGGCG");

        assert!(fastq.next().is_none(), "Expected no more records");
    }

    #[test]
    fn test_fastq_poll_next() {
        let reader = Cursor::new(FQ1);

        let mut fastq = Pin::new(Box::new(FastqReader::<Cursor<&[u8]>>::new(reader)));

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // manual polling using poll_next
        match fastq.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.raw_fields, b"SEQ_ID_1");
                assert_eq!(record.raw_seq, b"ACTCGATCGCGACGAA");
                assert_eq!(record.raw_quality.unwrap(), b"AFFFFFFFFFFFFEBA");
            }
            _ => panic!("Unexpected result"),
        }

        match fastq.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.raw_fields, b"SEQ_ID_2");
                assert_eq!(record.raw_seq, b"CATCGACTACGGCG");
            }
            _ => panic!("Unexpected result"),
        }

        //assert_eq!(fastq.as_mut().poll_next(&mut cx), Poll::Ready(None));
    }
}
*/
