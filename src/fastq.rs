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
        let mut quality = Vec::new();

        loop {
            self.buf.clear();
            if self.reader.read_line(&mut self.buf).is_err() {
                return None;
            }
            if self.buf.is_empty() {
                break;
            }
            if self.buf.starts_with('@') {
                if !fields.is_empty() {
                    break;
                }
                fields = self.buf[1..].trim().to_string();
            } else if self.buf.starts_with('+') {
                continue;
            } else if fields.is_empty() {
                match Seq::<A>::try_from(self.buf.trim()) {
                    Ok(parsed_seq) => seq.extend(&parsed_seq),
                    Err(e) => {
                        eprintln!("Error parsing sequence: {}", e);
                        return None;
                    }
                }
            } else {
                quality.extend(self.buf.trim().as_bytes().iter().map(|q| Phred::from(*q)));
            }
        }

        if fields.is_empty() {
            None
        } else {
            Some(Ok(Record {
                fields,
                seq,
                quality: Some(quality),
            }))
        }
    }
}

impl<R: BufRead> Iterator for Fastq<R, String> {
    type Item = Result<Record<String>, FastqError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut fields = String::new();
        let mut seq = String::with_capacity(512);
        let mut quality = Vec::with_capacity(512);

        loop {
            self.buf.clear();
            if self.reader.read_line(&mut self.buf).is_err() {
                return None;
            }
            if self.buf.is_empty() {
                break;
            }
            if self.buf.starts_with('@') {
                if !fields.is_empty() {
                    break;
                }
                fields = self.buf[1..].trim().to_string();
            } else if self.buf.starts_with('+') {
                continue;
            } else if fields.is_empty() {
                seq = self.buf.trim().to_string();
            } else {
                quality.extend(self.buf.trim().as_bytes().iter().map(|q| Phred::from(*q)));
            }
        }

        if fields.is_empty() {
            None
        } else {
            Some(Ok(Record {
                fields,
                seq,
                quality: Some(quality),
            }))
        }
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
