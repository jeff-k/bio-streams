#![allow(clippy::must_use_candidate)]

use core::error;
use core::fmt;
use core::str;
use futures::Stream;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Phred(u8);

impl Phred {
    pub fn to_float(self) -> f32 {
        unimplemented!()
    }
}

impl From<u8> for Phred {
    fn from(b: u8) -> Phred {
        Phred(b)
    }
}

impl From<Phred> for u8 {
    fn from(phred: Phred) -> Self {
        phred.0
    }
}

//pub struct Cigar;

#[derive(Debug, PartialEq)]
pub struct Record<T: for<'a> TryFrom<&'a [u8]> = Vec<u8>> {
    pub fields: Vec<u8>,
    pub seq: T,
    pub quality: Option<Vec<Phred>>,
}

impl<T: fmt::Display + for<'b> TryFrom<&'b [u8]>> fmt::Display for Record<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\t{}",
            String::from_utf8_lossy(&self.fields),
            &self.seq,
        )
    }
}

impl<'a, T: for<'b> TryFrom<&'b [u8]>> Record<T> {
    pub fn name(self) -> &'a str {
        unimplemented!()
    }

    pub fn description(self) -> Option<&'a str> {
        unimplemented!()
    }
}

pub trait Reader<T>:
    Iterator<Item = Result<Record<T>, Self::Error>> + Stream<Item = Result<Record<T>, Self::Error>>
where
    T: for<'a> TryFrom<&'a [u8]>,
{
    type Error: error::Error;

    fn parse_record(&mut self) -> Option<Result<Record<T>, Self::Error>>;
}
