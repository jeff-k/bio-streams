use core::marker::PhantomData;
use std::io::BufRead;

use crate::Record;

pub struct Fasta<R: BufRead, T = Vec<u8>> {
    buf: R,
    bs: Vec<u8>,
    p: PhantomData<T>,
}

impl<R: BufRead, T> Fasta<R, T> {
    pub fn new(buf: R) -> Self {
        Fasta {
            buf,
            bs: vec![],
            p: PhantomData,
        }
    }
}

impl<R: BufRead, T: From<Vec<u8>>> Iterator for Fasta<R, T> {
    type Item = Record<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut seq: Vec<u8> = Vec::new();

        if self.bs.is_empty() {
            let n_bs = self.buf.read_until(b'\n', &mut self.bs).unwrap();
            if n_bs == 0 {
                return None;
            }
        }
        // new record
        let fields = if self.bs[self.bs.len() - 2] == b'\r' {
            self.bs[1..self.bs.len() - 2].to_vec()
        } else {
            self.bs[1..self.bs.len() - 1].to_vec()
        };

        loop {
            self.bs.clear();
            let n_bs = self.buf.read_until(b'\n', &mut self.bs).unwrap();

            if n_bs == 0 {
                break;
            }
            if self.bs[0] == b'>' {
                break;
            }
            // handle \r\n endings
            if self.bs[n_bs - 2] == b'\r' {
                self.bs.truncate(n_bs - 2);
            } else {
                self.bs.truncate(n_bs - 1);
            }

            seq.append(&mut self.bs.to_vec().clone());
        }

        Some(Record {
            fields,
            seq: seq.into(),
            quality: None,
        })
    }
}
