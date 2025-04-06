<div class="title-block" style="text-align: center;" align="center">

# bio-steams
[![Docs.rs](https://docs.rs/bio-streams/badge.svg)](https://docs.rs/bio-streams)
[![CI status](https://github.com/jeff-k/bio-streams/actions/workflows/rust.yml/badge.svg)](https://github.com/jeff-k/bio-streams/actions/workflows/rust.yml)
### Streaming parsers for genomics data

</div>

An avant-garde async streaming API for genomics

## Features

### Parsers for popular formats

- [ ] fastq
- [ ] fasta
- [ ] sam/bam
- [ ] gfa
- [ ] paf

### Zero-copy datastructures

### Lazy parsing configured through generic types

Lazy parsing for record members (CIGAR strings/Phred scores/Sequences)

### Webassembly friendly async

### Combinators for streams of data

Using `StreamExt`

* zipping streams with buffers
* mapping quality filter to streams
* create a stream of mapped sequences

## Examples

* [Remove non M. TB reads from streaming fastqs](https://jeff-k.github.io/fqdemo/), [amplicon based SARS-CoV-2 assembly](https://jeff-k.github.io/amplicon-tiling/)</div>

### Stream a pair of fastqs and check some conditions on their name fields

### Count amino acid k-mers

## TODO

* Branch prediction hints for record boundary checks
* Checking multiple delimiters in parallel
* Prefetch hints for sequential access patterns
* Buffer preallocation based on typical FASTQ record sizes
* Reusing buffer space by sliding windows rather than clearing
* SIMD-optimized scanning for record delimiters (@, +, newlines)
* Memory alignment of the buffer for SIMD operations
* Custom allocator for the buffer optimized for append/clear pattern

