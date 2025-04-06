use core::fmt::Debug;
use core::marker::PhantomData;

mod seqrecord;
pub use seqrecord::Record;

mod alignment;
pub use alignment::{Alignment, PseudoAlignment};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Cigar(u8);

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
