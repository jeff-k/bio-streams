#![allow(clippy::module_name_repetitions)]
use core::error::Error;
use core::fmt;

#[derive(Debug)]
pub enum ParseError {
    InvalidDelimiter,
    InvalidId(String),
    TruncatedRecord,
    InvalidSequence(String),
    InvalidQuality,
    FileError,
    IoError,
    InvalidFields,
    LengthMismatch,
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDelimiter => write!(f, "Invalid separation character"),
            Self::InvalidId(id) => write!(f, "Invalid id: {id}"),
            Self::TruncatedRecord => write!(f, "Truncated record"),
            Self::InvalidSequence(seq) => write!(f, "Invalid sequence: {seq}"),
            Self::InvalidQuality => write!(f, "Invalid quailty string"),
            Self::FileError => write!(f, "File error"),
            Self::InvalidFields => write!(f, "Invalid data fields"),
            Self::IoError => write!(f, "IO error"),
            Self::LengthMismatch => write!(f, "Mismatched sequence length"),
        }
    }
}
