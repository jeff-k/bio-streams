use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::Range;
use std::borrow::Cow;

use crate::error::ParseError;

pub struct Alignment<'a, S: for<'b> TryFrom<&'b [u8], Error = ParseError> = String> {
    pub(crate) data: Cow<'a, [u8]>,
    pub(crate) _p: PhantomData<S>,
}

impl<S: for<'b> TryFrom<&'b [u8], Error = ParseError>> Alignment<'_, S> {
    pub fn ref_id(&self) -> S {
        todo!()
    }
}

pub struct PseudoAlignment<'a, S: for<'b> TryFrom<&'b [u8], Error = ParseError> = String> {
    pub(crate) data: Cow<'a, [u8]>,
    pub(crate) _p: PhantomData<S>,
}

impl<S: for<'b> TryFrom<&'b [u8], Error = ParseError>> PseudoAlignment<'_, S> {
    pub fn ref_id(&self) -> S {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn record_field() {
        assert!(false);
    }
}
