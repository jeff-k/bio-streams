use core::marker::PhantomData;
use std::io::BufRead;

use crate::Record;
use bio_seq::prelude::*;

#[derive(Debug)]
pub struct FastaError {}

pub struct Fasta<R: BufRead, T = String> {
    reader: R,
    buf: Vec<u8>,
    p: PhantomData<T>,
}

impl<R: BufRead, T> Fasta<R, T> {
    pub fn new(reader: R) -> Self {
        Fasta {
            reader,
            buf: Vec::<u8>::with_capacity(2048),
            p: PhantomData,
        }
    }
}

impl<R: BufRead, A: Codec> Iterator for Fasta<R, Seq<A>> {
    type Item = Result<Record<Seq<A>>, FastaError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut fields = Vec::<u8>::new();
        let mut seq = Seq::<A>::new();

        loop {
            self.buf.clear();
            let read_res = self.reader.read_until(b'\n', &mut self.buf);
            if let Err(e) = read_res {
                eprintln!("Error reading line: {:?}", e);
                return Some(Err(FastaError {}));
            }
            if read_res.unwrap() == 0 {
                break;
            }
            if self.buf[0] == b'>' {
                if !fields.is_empty() {
                    break;
                }
                fields = Vec::<u8>::from(&self.buf[1..self.buf.len() - 2]);
            } else {
                match Seq::<A>::try_from(&self.buf[..self.buf.len() - 2]) {
                    Ok(parsed_seq) => seq.extend(&parsed_seq),
                    Err(e) => {
                        eprintln!(
                            "Error parsing sequence: {} {}",
                            String::from_utf8_lossy(&self.buf).trim(),
                            e
                        );
                        return None;
                    }
                }
            }
        }

        if fields.is_empty() {
            None
        } else {
            Some(Ok(Record {
                fields,
                seq,
                quality: None,
            }))
        }
    }
}
