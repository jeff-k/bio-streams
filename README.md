<div class="title-block" style="text-align: center;" align="center">

# bio-steams

Streaming parsers for genomics data

[![Docs.rs](https://docs.rs/bio-streams/badge.svg)](https://docs.rs/bio-streams)
[![CI status](https://github.com/jeff-k/bio-streams/actions/workflows/rust.yml/badge.svg)](https://github.com/jeff-k/bio-streams/actions/workflows/rust.yml)

</div>

This is an avant-garde runtime-agnostic API that relies on type inference to configure streams of genomic data.

## Features

### Parsers for popular formats

- [] fastq
- [] fasta
- [] sam/bam
- [] gfa
- [] paf

### Zero-copy datastructures

Shared `SeqRecord` type for `Fastq`, `Fasta`, and streams of records that carry sequences:

```rust
pub struct SeqRecord<S: for<'a> TryFrom<&'a [u8]> = &'a [u8]> { ... }
```

Records can be read into custom types like bit-packed sequences or SIMD-backed kmers:

```rust
use bio_seq::prelude::*;

todo!()
```

### Lazy parsing configured through generic types

Lazy parsing for record members (CIGAR strings/Phred scores/Sequences)

### Webassembly friendly async

This crate implements nightly std::futures traits.

Examples:
    * [Remove non M. TB reads from streaming fastqs](https://jeff-k.github.io/fqdemo/), [amplicon based SARS-CoV-2 assembly](https://jeff-k.github.io/amplicon-tiling/)</div>

### Combinators for streams of data

* zipping streams with buffers
* mapping quality filter to streams
* create a stream of mapped sequences

## TODO

* SIMD parallel scanning for record boundaries
* Branch prediction hints for record boundary checks
* Checking multiple delimiters in parallel
* Prefetch hints for sequential access patterns
* Buffer preallocation based on typical FASTQ record sizes
* Reusing buffer space by sliding windows rather than clearing
* SIMD-optimized scanning for record delimiters (@, +, newlines)
* Memory alignment of the buffer for SIMD operations
* Custom allocator for the buffer optimized for append/clear pattern## Examples

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
