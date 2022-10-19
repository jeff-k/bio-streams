extern crate flate2;
extern crate futures;

use std::fs::File;
//use std::io::Bufreader;
use std::path::Path;

use flate2::read::MultiGzDecoder;
use futures::{StreamExt, Stream, AsyncRead};
use futures::io::BufReader;

pub struct Record<T> {
    name: Vec<u8>,
    seq: T,
    quality: Vec<u8>,
    info: Option<Vec<u8>>,
}

pub struct Fastq<T> {
    buf: BufReader<T>,
}

impl<T: AsyncRead> Fastq<T> {
    fn open(path: Path) -> Self {
        if path.ends_with(".gz") {
            Fastq { buf: BufReader::<T>::new(MultiGzDecoder::new(File::open(path).unwrap())) }
        } else {
            Fastq { buf: BufReader::<T>::new(File::open(path).unwrap()) }
        }
    }

    // from buffer
    fn new(bytes: &[u8]) -> Self {
        unimplemented!();
    }
}

impl<T> Stream for Fastq<T> {
}
