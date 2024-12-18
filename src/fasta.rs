#[cfg(test)]
mod tests {
    use super::*;
    use futures_test::task::noop_waker;
    use std::io::Cursor;
    use std::task::{Context, Poll};

    #[test]
    fn test_fasta_iterator() {
        let data = b">SEQ_ID_1
ACTCGATCGCGACG
ACACGATCGCGCGC
CATCGACTACGGCG
>SEQ_ID_2
GGGGGGGGGGGGGG\n";
        let reader = Cursor::new(data as &[u8]);
        let mut fasta: Fasta<Cursor<&[u8]>, Seq<Dna>> = Fasta::new(reader);

        let record1 = fasta.next().unwrap().unwrap();
        assert_eq!(record1.fields, b"SEQ_ID_1".to_vec());
        assert_eq!(
            record1.seq,
            dna!("ACTCGATCGCGACGACACGATCGCGCGCCATCGACTACGGCG")
        );

        let record2 = fasta
            .next()
            .expect("Expected a record")
            .expect("Expected valid record");
        assert_eq!(record2.fields, b"SEQ_ID_2".to_vec());
        assert_eq!(record2.seq, dna!("GGGGGGGGGGGGGG"));

        assert!(fasta.next().is_none(), "Expected no more records");
    }

    #[test]
    fn test_fasta_poll_next() {
        let data = b">SEQ_ID_1
AAAAAAAAAAAAAA
CCCCCCCCCCCCC
GGGGGGGGGGGG
>SEQ_ID_2
TTTTTTTTTTT\n";

        let reader = Cursor::new(data as &[u8]);
        let mut fasta: Pin<Box<Fasta<Cursor<&[u8]>, Seq<Dna>>>> =
            Pin::new(Box::new(Fasta::new(reader)));

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // Manual polling using poll_next
        match fasta.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.fields, b"SEQ_ID_1".to_vec());
                assert_eq!(record.seq, dna!("AAAAAAAAAAAAAACCCCCCCCCCCCCGGGGGGGGGGGG"));
            }
            _ => panic!("Unexpected result"),
        }

        match fasta.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.fields, b"SEQ_ID_2".to_vec());
                assert_eq!(record.seq, dna!("TTTTTTTTTTT"));
            }
            e => panic!("Unexpected result {:?}", e),
        }

        assert_eq!(fasta.as_mut().poll_next(&mut cx), Poll::Ready(None));
    }

    #[test]
    fn test_fasta_poll_next_with_crlf() {
        let data = b">SEQ_ID_1\r\nAAAAAAAAAAAAAA\r\nCCCCCCCCCCCCC\r\nGGGGGGGGGGGG\r\n>SEQ_ID_2\r\nTTTTTTTTTTT";

        let reader = Cursor::new(data as &[u8]);
        let mut fasta: Pin<Box<Fasta<Cursor<&[u8]>, Seq<Dna>>>> =
            Pin::new(Box::new(Fasta::new(reader)));

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // Manual polling using poll_next
        match fasta.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.fields, b"SEQ_ID_1".to_vec());
                assert_eq!(record.seq, dna!("AAAAAAAAAAAAAACCCCCCCCCCCCCGGGGGGGGGGGG"));
            }
            _ => panic!("Unexpected result"),
        }

        match fasta.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.fields, b"SEQ_ID_2".to_vec());
                assert_eq!(record.seq, dna!("TTTTTTTTTTT"));
            }
            e => panic!("Unexpected result {:?}", e),
        }

        assert_eq!(fasta.as_mut().poll_next(&mut cx), Poll::Ready(None));
    }

    #[test]
    fn test_fasta_poll_next_no_eol() {
        let data = b">SEQ_ID_X\nAAAAAAAAAAAAAACCCCCCCCCCCCCGGGGGGGGGGGGACGTAAA";

        let reader = Cursor::new(data as &[u8]);
        let mut fasta: Pin<Box<Fasta<Cursor<&[u8]>, Seq<Dna>>>> =
            Pin::new(Box::new(Fasta::new(reader)));

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // Manual polling using poll_next
        match fasta.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(record))) => {
                assert_eq!(record.fields, b"SEQ_ID_X".to_vec());
                assert_eq!(
                    record.seq,
                    dna!("AAAAAAAAAAAAAACCCCCCCCCCCCCGGGGGGGGGGGGACGTAAA")
                );
            }
            _ => panic!("Unexpected result"),
        }

        assert_eq!(fasta.as_mut().poll_next(&mut cx), Poll::Ready(None));
    }
}
