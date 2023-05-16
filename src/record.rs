use core::fmt;
use core::str;
//use core::String;

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

//pub struct Cigar;

pub struct Record<T = Vec<u8>> {
    pub fields: Vec<u8>,
    pub seq: T,
    pub quality: Option<Vec<Phred>>,
}

/*
pub struct Alignment<R = Record<Seq<A: Codec>>> {
    pub record: R,
    pub quality: Phred,
    pub cigar: Option<Cigar>,
}
*/

impl<T: fmt::Display> fmt::Display for Record<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\t{}",
            String::from_utf8_lossy(&self.fields),
            &self.seq,
        )
    }
}

impl<'a, T> Record<T> {
    pub fn name(self) -> &'a str {
        unimplemented!()
    }

    pub fn description(self) -> Option<&'a str> {
        unimplemented!()
    }
}
