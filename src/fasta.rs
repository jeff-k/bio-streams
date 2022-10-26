use core::marker::PhantomData;
use std::io::BufRead;

//use futures::{AsyncRead, Stream, StreamExt};
//use futures::task::{Context, Poll};
//use futures::io::BufReader;

use crate::Record;

pub struct Fasta<R: BufRead, T = Vec<u8>> {
    buf: R,
    p: PhantomData<T>,
}

impl<R: BufRead, T> Fasta<R, T> {
    pub fn new(buf: R) -> Self {
        Fasta {
            buf,
            p: PhantomData,
        }
    }
}

impl<R: BufRead, T: From<Vec<u8>>> Iterator for Fasta<R, T> {
    type Item = Record<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut name: Vec<u8> = Vec::new();
        let mut seq: Vec<u8> = Vec::new();

        // this includes the newline
        let n_bs = self.buf.read_until(b'\n', &mut name).unwrap();
        if n_bs == 0 {
            // the stream has properly ended
            return None;
        }
        assert_eq!(name[0], b'>');

        // TODO: this only reads the first line
        let s_bs = self.buf.read_until(b'\n', &mut seq).unwrap();

        name.truncate(n_bs - 1);
        seq.truncate(s_bs - 1);

        Some(Record {
            fields: name,
            seq: seq.into(),
            quality: None,
        })
    }
}
