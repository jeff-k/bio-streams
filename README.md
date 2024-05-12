[![Docs.rs](https://docs.rs/bio-streams/badge.svg)](https://docs.rs/bio-streams)
[![CI status](https://github.com/jeff-k/bio-streams/actions/workflows/rust.yml/badge.svg)](https://github.com/jeff-k/bio-streams/actions/workflows/rust.yml)

<div class="title-block" style="text-align: center;" align="center">

# bio-steams

### Types and datastructures for streaming genomics data

#### This crate is in early development. Contributions are very welcome.

Webassembly examples: [Remove non M. TB reads from streaming fastqs](https://jeff-k.github.io/fqdemo/), [amplicon based SARS-CoV-2 assembly](https://jeff-k.github.io/amplicon-tiling/)</div>

## Features

Shared `Record` type by `Fastq` and `Fasta` streams:

```rust
pub struct Record<T: for<'a> TryFrom<&'a [u8]> = Vec<u8>> {
    pub fields: Vec<u8>,
    pub seq: T,
    pub quality: Option<Vec<Phred>>, // fasta records set quality to `None`
}
```

Records can be read into custom types: `pub struct Fastq<R: BufRead, T = Seq<Dna>>`

## Examples

### Stream a pair of fastqs and check some conditions on their name fields
```rust
// Open a pair of gzipped fastq files as streams of `Record`s with `Seq<Dna>` sequences

let fq1: Fastq<BufReader<MultiGzDecoder<File>>> = Fastq::new(BufReader::new(
    MultiGzDecoder::new(File::open(&file1).unwrap()),
));

let fq2: Fastq<BufReader<MultiGzDecoder<File>>> = Fastq::new(BufReader::new(
    MultiGzDecoder::new(File::open(&file2).unwrap()),
));

for zipped in fq1.zip(fq2) {
    match zipped {
        (Ok(r1), Ok(r2)) => {
            // check that the last characters of the name strings are 1 and 2
            if r1.fields[r1.fields.len() - 1] != b'1' || r2.fields[r2.fields.len() - 1] != b'2'
            {
                eprintln!("paired records do not end in 1/2");
            }

            // check that the description fields are equal up to the last character
            if r1.fields[..r1.fields.len() - 1] != r2.fields[..r2.fields.len() - 1] {
                eprintln!("reads do not have the same names");
            }
        }
        _ => {
            eprintln!("Parse error in fastq files");
        }
    }
}
```

To run the `fqcheck` example program with read files `r1.fq.gz` and `f2.fq.gz`:

```
$ cargo build --example fqcheck --release
$ target/release/examples/fqcheck r1.fq.gz r2.fq.gz
```

### Count amino acid k-mers

```rust
// this opens a gzipped data stream and parses it into `Records` with `Seq<Amino>` sequence fields
let faa: Fasta<BufReader<File>, Seq<Amino>> =
    Fasta::new(BufReader::new(File::open(&faa_file).unwrap()));

// we can convert amino acid k-mers directly into usizes and use them to index into a table
let mut histogram = Box::new([0u64; 1 << (K * Amino::BITS as usize)]);

for contig in faa {
    // here "contig" is a fasta record
    for kmer in contig.unwrap().seq.kmers::<K>() {
        histogram[usize::from(kmer)] += 1;
    }
}
```


To run the `aminokmers` example program with fasta file `proteins.faa`:

```
$ cargo build --example fqcheck --release
$ target/release/examples/aminokmers proteins.faa
```

## Roadmap

input streams:

* fastq
* fasta
* TODO sam/bam
* TODO gfa

todo:

* quality score trait, `Phred` alias for `u8`
* futures::streams for async
* GAT lending iterator
* benchmark
* examples
