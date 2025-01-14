use std::marker::PhantomData;

pub use crate::error::ParseError;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct Phred(u8);

impl Phred {
    fn to_prob(self) -> f64 {
        let q = f64::from(self.0 - 33);
        10_f64.powf(-q / 10.0)
    }

    fn from_prob(p: f64) -> Self {
        unsafe { Phred((-10_f64 * p.log10()).to_int_unchecked::<u8>() + 33) }
    }
}

impl From<u8> for Phred {
    fn from(b: u8) -> Self {
        Phred(b)
    }
}

impl From<Phred> for u8 {
    fn from(phred: Phred) -> Self {
        phred.0
    }
}

impl From<f64> for Phred {
    fn from(p: f64) -> Self {
        Phred::from_prob(p)
    }
}

impl From<Phred> for f64 {
    fn from(phred: Phred) -> Self {
        phred.to_prob()
    }
}

#[derive(PartialEq, Debug)]
pub struct Record<'a, S: TryFrom<&'a [u8]> = &'a [u8]> {
    pub(crate) raw_fields: &'a [u8],
    pub(crate) raw_seq: &'a [u8],
    pub(crate) raw_quality: Option<&'a [u8]>,
    pub(crate) _p: PhantomData<S>,
}

impl<'a, S: TryFrom<&'a [u8], Error = ParseError>> Record<'a, S> {
    pub fn fields(&self) -> &'a [u8] {
        self.raw_fields
    }

    /// # Errors
    /// Parsing into the target sequence type may fail on bad characters
    pub fn seq(&self) -> Result<S, ParseError> {
        S::try_from(self.raw_seq)
    }

    /// # Errors
    /// Parsing phred qualities may be fallible
    pub fn quality(&self) -> Result<&[Phred], ParseError> {
        match self.raw_quality {
            None => Err(ParseError::InvalidQuality),
            Some(_) => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Phred;

    fn assert_eqf(x: f64, y: f64) {
        assert!((x - y).abs() < 0.0001)
    }

    #[test]
    fn phred_to_probs() {
        assert_eqf(Phred(b'!').to_prob(), 1.0);

        assert_eqf(Phred(b'"').to_prob(), 0.7943);
        assert_eqf(Phred(b'$').to_prob(), 0.5011);
        assert_eqf(Phred(b'#').to_prob(), 0.6309);
        assert_eqf(Phred(b'*').to_prob(), 0.1258);
        assert_eqf(Phred(b'@').to_prob(), 0.0008);
        assert_eqf(Phred(b'A').to_prob(), 0.0006);
        assert_eqf(Phred(b'G').to_prob(), 0.0002);
        assert_eqf(Phred(b'H').to_prob(), 0.0001);
        assert_eqf(Phred(b'I').to_prob(), 0.0001);

        assert_eqf(f64::from(Phred(b'!')), 1.0);
        assert_eqf(f64::from(Phred(b'*')), 0.1258);
        assert_eqf(f64::from(Phred(b'I')), 0.0001);
    }

    #[test]
    fn prob_to_phred() {
        assert_eq!(Phred(b'!'), Phred::from(1.0));
        assert_eq!(Phred(b'"'), Phred::from(0.7943));
        assert_eq!(Phred(b'$'), Phred::from(0.5011));
        assert_eq!(Phred(b'E'), Phred::from(0.0002));
        assert_eq!(Phred(b'I'), Phred::from(0.0001));
    }
}
