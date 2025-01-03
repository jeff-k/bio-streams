//! # bio-steams
//!
//! ### Types and datastructures for streaming genomics data
//!
//! #### This crate is in early development. Contributions are very welcome.
//!
//! Webassembly examples: [Remove non M. TB reads from streaming fastqs](https://jeff-k.github.io/fqdemo/), [amplicon based SARS-CoV-2 assembly](https://jeff-k.github.io/amplicon-tiling/)
//!
//! ## Features
//!
//! Shared `Record` type by `Fastq` and `Fasta` streams:
//!
//! ```
//! use bio_streams::record::Phred;
//!
//! pub struct Record<T: for<'a> TryFrom<&'a [u8]> = Vec<u8>> {
//!    pub fields: Vec<u8>,
//!    pub seq: T,
//!    pub quality: Option<Vec<Phred>>, // fasta records set quality to `None`//!
//! }
//! ```
//!
//! Records can be read into custom types: `pub struct Fastq<R: BufRead, T = Seq<Dna>>`
//!
//! ## Examples
//!
//! ### Stream a pair of fastqs and check some conditions on their name fields
//! ```text
//! // Open a pair of gzipped fastq files as streams of `Record`s with `Seq<Dna>` sequences
//!
//! let fq1: Fastq<BufReader<MultiGzDecoder<File>>> = Fastq::new(BufReader::new(
//!    MultiGzDecoder::new(File::open(&file1).unwrap()),
//! ));
//!
//! let fq2: Fastq<BufReader<MultiGzDecoder<File>>> = Fastq::new(BufReader::new(
//!    MultiGzDecoder::new(File::open(&file2).unwrap()),
//! ));
//!
//! for zipped in fq1.zip(fq2) {
//!     match zipped {
//!         (Ok(r1), Ok(r2)) => {
//!            // check that the last characters of the name strings are 1 and 2
//!            if r1.fields[r1.fields.len() - 1] != b'1' || r2.fields[r2.fields.len() - 1] != b'2'
//!            {
//!                eprintln!("paired records do not end in 1/2");
//!            }
//!
//!            // check that the description fields are equal up to the last character
//!            if r1.fields[..r1.fields.len() - 1] != r2.fields[..r2.fields.len() - 1] {
//!                eprintln!("reads do not have the same names");
//!            }
//!         }
//!         _ => {
//!             eprintln!("Parse error in fastq files");
//!         }
//!     }
//! }
//! ```
//!
//! ### Count amino acid k-mers
//!
//! ```text
//! // this opens a gzipped data stream and parses it into `Records` with `Seq<Amino>` sequence fields
//! let faa: Fasta<BufReader<File>, Seq<Amino>> =
//!     Fasta::new(BufReader::new(File::open(&faa_file).unwrap()));
//!
//! // we can convert amino acid k-mers directly into usizes and use them to index into a table
//! let mut histogram = Box::new([0u64; 1 << (K * Amino::BITS as usize)]);
//!
//! for contig in faa {
//!    // here "contig" is a fasta record
//!    for kmer in contig.unwrap().seq.kmers::<K>() {
//!        histogram[usize::from(kmer)] += 1;
//!    }
//! }
//! ```
//!

#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
mod error;
//pub mod fasta;
pub mod fastq;
pub mod record;
//pub mod sam;
//pub mod gfa;
//pub mod paf;
