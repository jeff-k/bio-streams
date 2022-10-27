pub mod fasta;
pub mod fastq;
pub mod record;
//pub mod sam;

pub use record::{Alignment, Record};

#[cfg(test)]
mod tests {
    use super::fastq;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
