use core::marker::PhantomData;
use std::fmt;
use std::io::BufRead;
use std::str;

//use futures::{StreamExt, Stream, AsyncRead};
//use futures::task::{Context, Poll};
//use futures::io::BufReader;

pub struct Record<T = Vec<u8>> {
    pub fields: Vec<u8>,
    pub seq: T,
    pub quality: Vec<u8>,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\t{}",
            str::from_utf8(&self.fields).unwrap(),
            str::from_utf8(&self.seq).unwrap()
        )
    }
}

impl Record {
    pub fn data_fields(self) -> Vec<Vec<u8>> {
        //        self.fields.split(' ')
        unimplemented!()
    }

    pub fn name(self) -> Vec<u8> {
        unimplemented!()
    }

    pub fn description(self) -> Option<Vec<u8>> {
        unimplemented!()
    }
}

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

//impl<R: BufRead, T> Fastq<R, T> {
//    pub fn from_fq(path: &Path) -> Fastq<BufReader<File>, T> {
//        Fastq { buf: BufReader::new(File::open(path).unwrap()), p: PhantomData }
//    }
//}

impl<R: BufRead, T: From<Vec<u8>>> Iterator for Fastq<R, T> {
    type Item = Record<Vec<u8>>;

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

        Some(Record {
            fields: name[..(n_bs - 1)].to_vec(),
            seq: seq[..(s_bs - 1)].into(),
            quality: quality[..(q_bs - 1)].to_vec(),
        })
    }
}
