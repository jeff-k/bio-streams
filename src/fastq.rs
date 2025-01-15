use futures::Stream as AsyncIterator;
use std::io;
use std::io::BufRead;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Poll;

pub use crate::error::ParseError;
pub use crate::record::{Phred, Record};

fn build_record<'a, S: TryFrom<&'a [u8]>>(
    lines: &[&'a [u8]; 4],
) -> Result<Record<&'a [u8], S>, io::Error> {
    // test for valid header start

    if lines[0][0] != b'@' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid FASTQ header",
        ));
    }

    // test for valid separator line
    let sep_pos = lines[2][0];
    if !sep_pos == b'+' {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid FASTQ separator",
        ));
    }

    // test that quality and sequence strings are equal length
    if lines[1].len() != lines[3].len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FASTQ quality and sequence lines differ",
        ));
    }

    Ok(Record {
        raw_fields: &lines[0][1..],
        raw_seq: lines[1],
        raw_quality: Some(lines[3]),
        _p: PhantomData,
    })
}

pub struct Fastq<'a, S: TryFrom<&'a [u8]> = Vec<u8>> {
    buffer: &'a [u8],
    pos: usize,
    _s: PhantomData<S>,
}

impl<'src, S: TryFrom<&'src [u8]>> Fastq<'src, S> {
    pub fn new(buf: &'src [u8]) -> Self {
        Fastq {
            buffer: buf,
            pos: 0,
            _s: PhantomData,
        }
    }
    fn parse(&mut self) -> Option<Result<Record<&'src [u8], S>, std::io::Error>> {
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
                return Some(Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Truncated FASTQ record",
                )));
            }
        }

        Some(build_record(&lines))
    }
}

pub struct FastqReader<R: BufRead + Unpin, S: for<'b> TryFrom<&'b [u8]> = Vec<u8>> {
    reader: Pin<Box<R>>,
    buffer: Vec<u8>,
    _s: PhantomData<S>,
}

/*
impl<R: BufRead + Unpin, S: for<'b> TryFrom<&'b [u8]>> FastqReader<R, S> {
    pub fn new(reader: R) -> Self {
        FastqReader {
            reader: Box::pin(reader),
            buffer: Vec::<u8>::with_capacity(1024),
            _s: PhantomData,
        }
    }

    fn parse(&mut self) -> Option<Result<Record<'_, S>, std::io::Error>> {
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
            &buf[0..crs[0]],
            &buf[crs[0] + 1..crs[1]],
            &buf[crs[1] + 1..crs[2]],
            &buf[crs[2] + 1..crs[3]],
        ];

        Some(build_record(&lines))
    }
}

impl<R: BufRead + Unpin, S: for<'b> TryFrom<&'b [u8]>> Iterator for &mut FastqReader<R, S> {
    type Item<'a> = Result<Record<'a, S>, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item<'a>> {
        self.parse()
    }
}

impl<'a, R: BufRead + Unpin, S: for<'b> TryFrom<&'b [u8]>> AsyncIterator for FastqReader<R, S> {
    type Item = Result<Record<'a, S>, std::io::Error>;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let record = unsafe { self.get_unchecked_mut().parse() };

        Poll::Ready(record)
    }
}
*/
impl<'a, S: TryFrom<&'a [u8]>> Iterator for Fastq<'a, S> {
    type Item = Result<Record<&'a [u8], S>, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse()
    }
}

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
        //        let reader = Cursor::new(FQ1);

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
    /*
    #[test]
    fn test_fastq_poll_next() {
        let reader = Cursor::new(FQ1);

        let mut fastq = FastqReader::new(reader);

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // manual polling using poll_next
        match fastq.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.raw_fields, b"SEQ_ID_1");
                assert_eq!(record.raw_seq, b"ACTCGATCGCGACG");
                assert_eq!(record.raw_quality.unwrap(), b"FFFFFFFFFFFFFF");
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
    */
}
