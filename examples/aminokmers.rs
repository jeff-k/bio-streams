extern crate bio_streams;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use clap::Parser;

use bio_streams::fasta::Fasta;

#[derive(Parser)]
struct Cli {
    faa: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let faa: Fasta<BufReader<File>> = Fasta::new(BufReader::new(File::open(&args.faa).unwrap()));

    let mut count: usize = 0;
    for contig in faa {
        println!("{}\t{}", count, contig);
        count += 1;
    }
}
