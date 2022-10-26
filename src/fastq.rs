use core::marker::PhantomData;
use std::io::BufRead;

//use futures::{AsyncRead, Stream, StreamExt};
//use futures::task::{Context, Poll};
//use futures::io::BufReader;

use crate::Record;

pub struct Fastq<R: BufRead, T = Vec<u8>> {
    buf: R,
    p: PhantomData<T>,
}

impl<R: BufRead, T> Fastq<R, T> {
    pub fn new(buf: R) -> Self {
        Fastq {
            buf,
            p: PhantomData,
        }
    }
}

impl<R: BufRead, T: From<Vec<u8>>> Iterator for Fastq<R, T> {
    type Item = Record<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut name: Vec<u8> = Vec::new();
        let mut seq: Vec<u8> = Vec::new();
        let mut delim: Vec<u8> = Vec::new();
        let mut quality: Vec<u8> = Vec::new();

        // this includes the newline
        let n_bs = self.buf.read_until(b'\n', &mut name).unwrap();
        if n_bs == 0 {
            // the stream has properly ended
            return None;
        }
        assert_eq!(name[0], b'@');

        let s_bs = self.buf.read_until(b'\n', &mut seq).unwrap();

        // one option is to just consume the +\n line:
        //self.buf.consume(2);

        let d_bs = self.buf.read_until(b'\n', &mut delim).unwrap();
        assert_eq!(d_bs, 2);

        let q_bs = self.buf.read_until(b'\n', &mut quality).unwrap();
        // read equal number of bytes for sequence and quality string
        assert_eq!(s_bs, q_bs);

        name.truncate(n_bs - 1);
        seq.truncate(s_bs - 1);
        quality.truncate(q_bs - 1);

        Some(Record {
            fields: name,
            seq: seq.into(),
            quality: Some(quality),
        })
    }
}

// TODO: interleaved fastqs parse 8 lines into tuple of (Record, Record)
pub struct InterleavedFastq<R: BufRead, T = Vec<u8>> {
    buf: R,
    p: PhantomData<T>,
}

impl<R: BufRead, T: From<Vec<u8>>> Iterator for InterleavedFastq<R, T> {
    type Item = (Record<T>, Record<T>);

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

/*
impl<R: BufRead, T: From<Vec<u8>>> Stream

impl<T> Fastq<T> {
    fn borrow_next<'a>(&'a mut self) -> Option<&'a Record<T>> {
        None
    }
}
*/
