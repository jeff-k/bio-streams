use futures::Stream as AsyncIterator;
use std::io;
use std::io::BufRead;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Poll;

pub use crate::error::ParseError;
pub use crate::record::{Phred, Record};

pub struct FastqReader<'a, R: BufRead, S: TryFrom<&'a [u8]> = Vec<u8>> {
    reader: Pin<Box<R>>,
    buffer: Vec<u8>,
    _p: PhantomData<&'a ()>,
    _s: PhantomData<S>,
}

impl<R: BufRead + Into<Box<R>> + Unpin, S: for<'a> TryFrom<&'a [u8]>> FastqReader<'_, R, S> {
    pub fn new(reader: R) -> Self {
        FastqReader {
            reader: Box::pin(reader),
            buffer: Vec::<u8>::with_capacity(1024),
            _p: PhantomData,
            _s: PhantomData,
        }
    }
}

impl<'a, R: BufRead + Into<Box<R>> + Unpin, S: TryFrom<&'a [u8]>> FastqReader<'a, R, S> {
    fn parse(&mut self) -> Option<Result<Record<'a, S>, std::io::Error>> {
        self.buffer.clear();

        // total bytes read
        let mut t_bs = 0;

        // indices of carriage returns
        let mut crs = [0; 4];

        let reader = self.reader.as_mut().get_mut();

        for (i, crs_i) in crs.iter_mut().enumerate() {
            match reader.read_until(b'\n', &mut self.buffer) {
                // end of file
                Ok(0) => {
                    if i == 0 {
                        // proper file end
                        return None;
                    }
                    // truncated records
                    return Some(Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "Truncated FASTQ record",
                    )));
                }
                Ok(n_bs) => {
                    *crs_i = n_bs + t_bs;
                    t_bs += n_bs;
                }
                Err(e) => return Some(Err(e)),
            }
        }

        // test for valid header start
        if !self.buffer[0] == b'@' {
            return Some(Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid FASTQ header",
            )));
        }

        // test for valid separator line
        let sep_pos = crs[2] + 1;
        if !self.buffer[sep_pos] == b'+' {
            return Some(Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid FASTQ separator",
            )));
        }

        // test that quality and sequence strings are equal length
        if (crs[1] - crs[0]) != (crs[3] - crs[2]) {
            return Some(Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FASTQ quality and sequence lines differ",
            )));
        }

        unsafe {
            let buf = std::slice::from_raw_parts(self.buffer.as_ptr(), self.buffer.len());

            Some(Ok(Record {
                raw_fields: &buf[1..crs[0] - 1],
                raw_seq: &buf[crs[0]..crs[1] - 1],
                raw_quality: Some(&buf[crs[2]..crs[3] - 1]),
                _p: PhantomData,
            }))
        }
    }
}

impl<'a, R: BufRead + Unpin, S: TryFrom<&'a [u8]>> Iterator for FastqReader<'a, R, S> {
    type Item = Result<Record<'a, S>, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse()
    }
}

impl<'a, R: BufRead + Unpin, S: TryFrom<&'a [u8]>> AsyncIterator for FastqReader<'a, R, S> {
    type Item = Result<Record<'a, S>, std::io::Error>;

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
    use std::io::Cursor;
    use std::task::{Context, Poll};

    #[test]
    fn test_fastq_iterator() {
        let data = b"@SEQ_ID_1
ACTCGATCGCGACGAA
+
AFFFFFFFFFFFFEBA
@SEQ_ID_2
CATCGACTACGGCG
+
GGGGGGGGGGGGGG\n";
        let reader = Cursor::new(data as &[u8]);
        let mut fastq: FastqReader<Cursor<&[u8]>> = FastqReader::new(reader);

        let record1 = fastq.next().unwrap().unwrap();
        assert_eq!(record1.raw_fields, b"SEQ_ID_1".to_vec());
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
        let data = b"@SEQ_ID_1
ACTCGATCGCGACG
+
FFFFFFFFFFFFFF
@SEQ_ID_2
CATCGACTACGGCG
+
GGGGGGGGGGGGGG\n";

        let reader = Cursor::new(data as &[u8]);
        let mut fastq: Pin<Box<FastqReader<Cursor<&[u8]>>>> =
            Pin::new(Box::new(FastqReader::new(reader)));

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
}
