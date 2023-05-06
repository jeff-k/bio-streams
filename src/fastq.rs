use bio_seq::prelude::*;

use core::marker::PhantomData;
use std::io::BufRead;

use crate::record::Phred;
use crate::Record;

#[derive(Debug)]
pub struct FastqError {}

pub struct Fastq<R: BufRead, T = String> {
    reader: R,
    buf: String,
    p: PhantomData<T>,
}

impl<R: BufRead, T> Fastq<R, T> {
    pub fn new(reader: R) -> Self {
        Fastq {
            reader,
            buf: String::with_capacity(512),
            p: PhantomData,
        }
    }
}

impl<R: BufRead, A: Codec> Iterator for Fastq<R, Seq<A>> {
    type Item = Result<Record<Seq<A>>, FastqError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut fields = String::new();
        let mut seq = Seq::<A>::new();
        let mut quality = Vec::with_capacity(512);

        self.buf.clear();
        if self.reader.read_line(&mut self.buf).is_err() {
            println!("error reading");
            return None;
        }
        if self.buf.is_empty() {
            println!("buf empty");
            return None;
        }
        if !self.buf.starts_with('@') {
            println!("no @ {}", self.buf);
            return None;
        }

        fields = self.buf[1..].trim().to_string();

        self.buf.clear();
        self.reader.read_line(&mut self.buf);

        match Seq::<A>::try_from(self.buf.trim()) {
            Ok(parsed_seq) => seq.extend(&parsed_seq),
            Err(e) => {
                self.reader.read_line(&mut self.buf);
                self.reader.read_line(&mut self.buf);
                return Some(Err(FastqError {}));
            }
        }

        self.buf.clear();
        self.reader.read_line(&mut self.buf);

        if !self.buf.starts_with('+') {
            return None;
        }

        self.buf.clear();
        self.reader.read_line(&mut self.buf);
        quality.extend(self.buf.trim().as_bytes().iter().map(|q| Phred::from(*q)));

        Some(Ok(Record {
            fields,
            seq,
            quality: Some(quality),
        }))
    }
}

impl<R: BufRead> Iterator for Fastq<R, String> {
    type Item = Result<Record<String>, FastqError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut fields = String::new();
        let mut seq = String::with_capacity(512);
        let mut quality = Vec::with_capacity(512);

        self.buf.clear();
        if self.reader.read_line(&mut self.buf).is_err() {
            return None;
        }
        if self.buf.is_empty() {
            return None;
        }

        if !self.buf.starts_with('@') {
            return None;
        }

        fields = self.buf[1..].trim().to_string();

        self.buf.clear();
        self.reader.read_line(&mut self.buf);

        seq = self.buf.trim().to_string();

        self.buf.clear();
        self.reader.read_line(&mut self.buf);

        if !self.buf.starts_with('+') {
            return None;
        }

        self.buf.clear();
        self.reader.read_line(&mut self.buf);
        quality.extend(self.buf.trim().as_bytes().iter().map(|q| Phred::from(*q)));

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
