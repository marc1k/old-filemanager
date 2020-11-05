use {
    crate::Bounds,
    std::{error, fmt},
};

#[macro_export]
macro_rules! ensure {
    ($predicate:expr, $err:expr) => {
        if !$predicate {
            return Err($err);
        }
    };
}

#[derive(Debug)]
pub enum Error {
    OutOfBounds(u16, u16, Bounds),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::OutOfBounds(ref col, ref row, ref bounds) => write!(
                f,
                "the coordinates ({}, {}) are not in {:?}",
                col, row, bounds
            ),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
