use core::marker::PhantomData;

use std::task::{Context, Poll};

use std::marker::Unpin;
use std::pin::Pin;

use futures::stream::Stream;
use std::io::BufRead;

use crate::Record;

pub struct Fastq<R: BufRead, T = Vec<u8>> {
    buf: Pin<Box<R>>,
    p: PhantomData<T>,
}

impl<R: BufRead + Unpin, T: From<Vec<u8>> + Unpin> Unpin for Fastq<R, T> {}

impl<R: BufRead + Into<Box<R>>, T> Fastq<R, T> {
    pub fn new(buf: R) -> Self {
        Fastq {
            buf: Box::pin(buf),
            p: PhantomData,
        }
    }
}

impl<R: BufRead + Unpin, T: From<Vec<u8>> + Unpin> Stream for Fastq<R, T> {
    type Item = Result<Record<T>, String>;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<<Self as Stream>::Item>> {
        let mut fields: Vec<u8> = Vec::new();
        let mut seq: Vec<u8> = Vec::new();
        let mut sep: Vec<u8> = Vec::new();
        let mut quality: Vec<u8> = Vec::new();

        let r = unsafe {
            let mut x = self.get_unchecked_mut().buf.as_mut();

            x.read_until(b'\n', &mut fields)
                .map_err(|e| e.to_string())?;
            x.read_until(b'\n', &mut seq).map_err(|e| e.to_string())?;
            x.read_until(b'\n', &mut sep).map_err(|e| e.to_string())?;
            x.read_until(b'\n', &mut quality).map_err(|e| e.to_string())
        };

        match r {
            Ok(n) if n == 0 => Poll::Ready(None),
            Ok(_n) => Poll::Ready(Some(Ok(Record {
                fields: fields.into(),
                seq: seq.into(),
                quality: None,
            }))),
            Err(e) => Poll::Ready(Some(Err(e))),
        }
    }
}
