use core::marker::PhantomData;
use std::io::BufRead;

//use futures::{AsyncRead, Stream, StreamExt};
//use futures::task::{Context, Poll};
//use futures::io::BufReader;

use crate::{Alignment, Record};

pub struct Sam<R: BufRead, T = Vec<u8>> {
    buf: R,
    p: PhantomData<T>,
}

impl<R: BufRead, T> Sam<R, T> {
    pub fn new(buf: R) -> Self {
        Sam {
            buf,
            p: PhantomData,
        }
    }
}

impl<R: BufRead, T: From<Vec<u8>>> Iterator for Sam<R, T> {
    type Item = Alignment<Record<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
