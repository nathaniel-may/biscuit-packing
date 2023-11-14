use std::error::Error as StdError;
use std::fmt;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    BiscuitsBelowOne,
    StartGreaterThanEnd,
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BiscuitsBelowOne => {
                write!(f, "The number of biscuits to arrange must be at least 1.")
            }
            Self::StartGreaterThanEnd => {
                write!(f, "start must be greater than the end.")
            }
        }
    }
}
