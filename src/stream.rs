//extern crate futures;

use core::marker::PhantomData;
//use futures::io::AsyncBufRead;
//use futures::AsyncBufReadExt;

use std::task::{Context, Poll};

use std::marker::Unpin;
use std::pin::Pin;

//use futures::io::BufReader;
//use futures::stream::{Stream, StreamExt};
use futures::stream::Stream;
use std::io::BufRead;
//use std::io::BufReader;

use crate::Record;

pub struct Fastq<R: BufRead, T = Vec<u8>> {
    buf: Pin<Box<R>>,
    p: PhantomData<T>,
}

impl<R: BufRead + Into<Box<R>>, T> Fastq<R, T> {
    pub fn new(buf: R) -> Self {
        Fastq {
            buf: Box::pin(buf),
            p: PhantomData,
        }
    }
}

/*
impl<R: BufRead, T: From<Vec<u8>> + Unpin> Stream for Fastq<R, T> {
    type Item = Record<T>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<<Self as Stream>::Item>> {

        let ref b = self.get_mut().buf.as_mut();



        let mut x: Vec<u8> = Vec::new();
        let nbs = b.read_until(b'\n', &mut x);

        Poll::Ready(None)
    }
}
*/

/*
    //let n_bs = self.get_mut()
    //    .buf
    //    .as_mut()
    match n1.read_until(b'\n', &mut name) {
        Ok(0) => task::Poll::Ready(None),
        Ok(n) => task::Poll::Ready(Some(Record {
            fields: name,
            seq: vec![].into(),
            //          seq: seq.into(),
            quality: None,
            //            quality: Some(quality),
        })),

        _ => task::Poll::Ready(None),
    }
}
        if n_bs == 0 {
            // the stream has properly ended
            return task::Poll::Ready(None);
        }
        //        assert_eq!(name[0], b'@');
        let s_bs = self.buf.read_until(b'\n', &mut seq).unwrap();

        // one option is to just consume the +\n line:
        //self.buf.consume(2);

        let d_bs = self.buf.read_until(b'\n', &mut delim).unwrap();
        assert_eq!(d_bs, 2);

        let q_bs = self.buf.read_until(b'\n', &mut quality).unwrap();
        // read equal number of bytes for sequence and quality string
        //        assert_eq!(s_bs, q_bs);
        name.truncate(n_bs - 1);
//        seq.truncate(s_bs - 1);
//        quality.truncate(q_bs - 1);

        task::Poll::Ready(Some(Record {
            fields: name,
            seq: vec![].into(),
            //          seq: seq.into(),
            quality: None,
//            quality: Some(quality),
        }))
    }
    */

/*
impl<R: BufRead, T: From<Vec<u8>> + Unpin> Stream for Fastq<R, T> {
    type Item = Record<T>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<<Self as Stream>::Item>> {
        let mut name: Vec<u8> = Vec::new();
        //        let mut seq: Vec<u8> = Vec::new();
        //        let mut delim: Vec<u8> = Vec::new();
        //        let mut quality: Vec<u8> = Vec::new();

        let n1 = self.get_mut().buf.as_mut();

        //let n_bs = self.get_mut()
        //    .buf
        //    .as_mut()
        match n1.read_until(b'\n', &mut name) {
            Ok(0) => task::Poll::Ready(None),
            Ok(n) => task::Poll::Ready(Some(Record {
                fields: name,
                seq: vec![].into(),
                //          seq: seq.into(),
                quality: None,
                //            quality: Some(quality),
            })),

            _ => task::Poll::Ready(None),
        }
    }
            if n_bs == 0 {
                // the stream has properly ended
                return task::Poll::Ready(None);
            }
            //        assert_eq!(name[0], b'@');
            let s_bs = self.buf.read_until(b'\n', &mut seq).unwrap();

            // one option is to just consume the +\n line:
            //self.buf.consume(2);

            let d_bs = self.buf.read_until(b'\n', &mut delim).unwrap();
            assert_eq!(d_bs, 2);

            let q_bs = self.buf.read_until(b'\n', &mut quality).unwrap();
            // read equal number of bytes for sequence and quality string
            //        assert_eq!(s_bs, q_bs);
            name.truncate(n_bs - 1);
    //        seq.truncate(s_bs - 1);
    //        quality.truncate(q_bs - 1);

            task::Poll::Ready(Some(Record {
                fields: name,
                seq: vec![].into(),
                //          seq: seq.into(),
                quality: None,
    //            quality: Some(quality),
            }))
        }
        */
