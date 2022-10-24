extern crate bio_streams;
extern crate flate2;

use bio_streams::fastq::Fastq;
use std::io::BufReader;
use std::path::Path;
use std::str;

use flate2::read::MultiGzDecoder;
use std::fs::File;

fn main() {
    let fq1: Fastq<BufReader<MultiGzDecoder<File>>> = Fastq::new(BufReader::new(
        MultiGzDecoder::new(File::open(Path::new("file_1.fastq.gz")).unwrap()).unwrap(),
    ));

    let fq2: Fastq<BufReader<MultiGzDecoder<File>>> = Fastq::new(BufReader::new(
        MultiGzDecoder::new(File::open(Path::new("file_2.fastq.gz")).unwrap()).unwrap(),
    ));

    for (r1, r2) in fq1.zip(fq2) {
        // check that the last characters of the name strings are 1 and 2
        if r1.fields[r1.fields.len() - 1] != b'1' || r2.fields[r2.fields.len() - 1] != b'2' {
            println!("paired records do not end in 1/2");
        }

        // check that the description fields are equal up to the last character
        if r1.fields[..r1.fields.len() - 1] != r2.fields[..r2.fields.len() - 1] {
            println!("reads do not have the same names");
        }
    }
}
