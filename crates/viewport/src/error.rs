use {
    crate::Bounds,
    std::{ error, fmt }
};

#[derive(Debug)]
pub enum Error {
    OutOfBounds(u16, u16, Bounds)
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {     
        match self {
            Error::OutOfBounds(ref col, ref row, ref bounds) => {
                write!(f, "the coordinates ({}, {}) are not in {:?}", col, row, bounds)
            },
        }
    }
}

