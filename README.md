# bio-streams

## examples

To run the `fqcheck` example program with read files `r1.fq.gz` and `f2.fq.gz`:

```
$ cargo build --example fqcheck --release
$ target/release/examples/fqcheck r1.fq.gz r2.fq.gz
```

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
