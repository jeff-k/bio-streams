use std::fmt;
use std::str;

type Phred = u8;

pub struct Cigar;

pub struct Record<T = Vec<u8>> {
    pub fields: Vec<u8>,
    pub seq: T,
    pub quality: Option<Vec<u8>>,
}

pub struct Alignment<R = Record<Vec<u8>>> {
    pub record: R,
    pub quality: u8,
    pub cigar: Option<Cigar>,
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

impl<'a> Record {
    pub fn name(self) -> &'a [u8] {
        unimplemented!()
    }

    pub fn description(self) -> Option<&'a [u8]> {
        unimplemented!()
    }
}
