use bio_seq::prelude::*;

use core::marker::PhantomData;
use std::io::BufRead;

use crate::record::Phred;
use crate::Record;

#[derive(Debug)]
pub struct FastqError {}

pub struct Fastq<R: BufRead, T = Seq<Dna>> {
    reader: R,
    buf: Vec<u8>,
    p: PhantomData<T>,
}

impl<R: BufRead, T> Fastq<R, T> {
    pub fn new(reader: R) -> Self {
        Fastq {
            reader,
            buf: Vec::<u8>::with_capacity(512),
            p: PhantomData,
        }
    }
}

impl<R: BufRead, A: Codec> Iterator for Fastq<R, Seq<A>> {
    type Item = Result<Record<Seq<A>>, FastqError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut seq = Seq::<A>::new();
        let mut quality = Vec::with_capacity(512);

        self.buf.clear();
        if self.reader.read_until(b'\n', &mut self.buf).is_err() {
            println!("error reading");
            return None;
        }
        if self.buf.is_empty() {
            println!("buf empty");
            return None;
        }
        if !self.buf[0] == b'@' {
            println!("no @ {}", String::from_utf8_lossy(&self.buf));
            return None;
        }

        let fields = Vec::<u8>::from(&self.buf[1..self.buf.len() - 2]);

        self.buf.clear();
        match self.reader.read_until(b'\n', &mut self.buf) {
            Ok(_) => (),
            Err(_) => return Some(Err(FastqError {})),
        }

        match Seq::<A>::try_from(&self.buf[..self.buf.len() - 2]) {
            Ok(parsed_seq) => seq.extend(&parsed_seq),
            Err(_) => {
                match self.reader.read_until(b'\n', &mut self.buf) {
                    Ok(_) => (),
                    Err(_) => return Some(Err(FastqError {})),
                }

                match self.reader.read_until(b'\n', &mut self.buf) {
                    Ok(_) => (),
                    Err(_) => return Some(Err(FastqError {})),
                }

                return Some(Err(FastqError {}));
            }
        }

        self.buf.clear();
        match self.reader.read_until(b'\n', &mut self.buf) {
            Ok(_) => (),
            Err(_) => return Some(Err(FastqError {})),
        }

        if !self.buf[0] == b'+' {
            return None;
        }

        self.buf.clear();
        match self.reader.read_until(b'\n', &mut self.buf) {
            Ok(_) => (),
            Err(_) => return Some(Err(FastqError {})),
        }

        quality.extend(
            self.buf[..self.buf.len() - 2]
                .iter()
                .map(|q| Phred::from(*q)),
        );

        Some(Ok(Record {
            fields,
            seq,
            quality: Some(quality),
        }))
    }
}

// TODO: interleaved fastqs parse 8 lines into tuple of (Record, Record)
/*
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
*/

/*
impl<R: BufRead, T: From<Vec<u8>>> Stream

impl<T> Fastq<T> {
    fn borrow_next<'a>(&'a mut self) -> Option<&'a Record<T>> {
        None
    }
}
*/
