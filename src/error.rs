use core::error;
use core::fmt;

#[derive(Debug, PartialEq)]
pub enum FastxError {
    InvalidSeparationLine,
    InvalidId(String),
    TruncatedRecord,
    InvalidSequence(String),
    InvalidQuality,
    FileError,
}

impl fmt::Display for FastxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FastxError::InvalidSeparationLine => write!(f, "Invalid separation character"),
            FastxError::InvalidId(id) => write!(f, "Invalid id: {id}"),
            FastxError::TruncatedRecord => write!(f, "Truncated record"),
            FastxError::InvalidSequence(seq) => write!(f, "Invalid sequence: {seq}"),
            FastxError::InvalidQuality => write!(f, "Invalid quailty string"),
            FastxError::FileError => write!(f, "File error"),
        }
    }
}

impl error::Error for FastxError {}
