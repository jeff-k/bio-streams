/*use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::exit;

use flate2::read::MultiGzDecoder;

use clap::Parser;

use bio_streams::fastq::Fastq;

#[derive(Parser)]
struct Cli {
    r1: PathBuf,
    r2: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let fq1: Fastq<BufReader<MultiGzDecoder<File>>> = Fastq::new(BufReader::new(
        MultiGzDecoder::new(File::open(&args.r1).unwrap()),
    ));

    let fq2: Fastq<BufReader<MultiGzDecoder<File>>> = Fastq::new(BufReader::new(
        MultiGzDecoder::new(File::open(&args.r2).unwrap()),
    ));

    let mut count: usize = 0;
    for zipped in fq1.zip(fq2) {
        match zipped {
            (Ok(r1), Ok(r2)) => {
                // check that the last characters of the name strings are 1 and 2
                if r1.fields[r1.fields.len() - 1] != b'1' || r2.fields[r2.fields.len() - 1] != b'2'
                {
                    eprintln!("paired records do not end in 1/2");
                    exit(1);
                }

                // check that the description fields are equal up to the last character
                if r1.fields[..r1.fields.len() - 1] != r2.fields[..r2.fields.len() - 1] {
                    eprintln!("reads do not have the same names");
                    exit(1);
                }

                count += 1;
            }
            _ => {
                eprintln!("Parse error in fastq files");
                exit(1);
            }
        }
    }
    eprintln!("read files have {} matching name records.", count);
    exit(0);
}
*/

fn main() {}
