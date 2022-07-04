use std::ops::Add;
use std::error;
use std::fmt;
use std::result;
pub trait RangeQuerier {
    type EntryType: Add<Output = Self::EntryType> + Default + Copy;
    fn update_val(&mut self, index: usize, val: Self::EntryType) -> Result<()>;
    fn query_range(&self, left: usize, right: usize) -> Result<Self::EntryType>;
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    
    UpdateIndexNotValid {
        index: usize,
        length: usize,
    },

    QueryRangeNotValid {
        left: usize,
        right: usize,
        length: usize,
    },

    ZeroLengthConstruction {},
}


impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::UpdateIndexNotValid { .. } => {
                "the index attempted to update is out of bounds"
            }
            ErrorKind::QueryRangeNotValid { .. } => {
                "the range queried is out of bounds"
            }
            ErrorKind::ZeroLengthConstruction { .. } => {
                "attempted to construct a segment tree on zero elements"
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::UpdateIndexNotValid { index, length } => write!(
                f,
                "The update failed because the index {} is \
                 at least the size {} of the underlying vector",
                index, length
            ),
            ErrorKind::QueryRangeNotValid { left, right, length } => write!(
                f,
                "The query failed because the range {}-{} does \
                 not belong to the range 0-{}",
                left, right, length
            ),
            ErrorKind::ZeroLengthConstruction {  } => write!(
                f,
                "The construction failed the underlying vector \
                of a segment tree must have positive size",
            ),
        }
    }
}