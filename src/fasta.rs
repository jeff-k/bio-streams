use core::marker::PhantomData;
use std::io::BufRead;

use crate::Record;
use bio_seq::prelude::*;

#[derive(Debug)]
pub struct FastaError {}

pub struct Fasta<R: BufRead, T = String> {
    reader: R,
    buf: String,
    p: PhantomData<T>,
}

impl<R: BufRead, T> Fasta<R, T> {
    pub fn new(reader: R) -> Self {
        Fasta {
            reader,
            buf: String::with_capacity(128),
            p: PhantomData,
        }
    }
}

impl<R: BufRead, A: Codec> Iterator for Fasta<R, Seq<A>> {
    type Item = Result<Record<Seq<A>>, FastaError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut fields = String::new();
        let mut seq = Seq::<A>::new();

        loop {
            let read_res = self.reader.read_line(&mut self.buf);
            if let Err(e) = read_res {
                eprintln!("Error reading line: {:?}", e);
                return None;
            }
            if read_res.unwrap() == 0 {
                break;
            }
            if self.buf.starts_with('>') {
                if !fields.is_empty() {
                    break;
                }
                fields = self.buf[1..].trim().to_string();
            } else {
                match Seq::<A>::try_from(self.buf.trim()) {
                    Ok(parsed_seq) => seq.extend(&parsed_seq),
                    Err(e) => {
                        eprintln!("Error parsing sequence: {}", e);
                        return None;
                    }
                }
            }
            self.buf.clear();
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
