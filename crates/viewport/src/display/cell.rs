use lazy_static::lazy_static;
use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

lazy_static! {
    /// A `Cell` sentinel that signifies the complete absence of data.
    pub static ref EMPTY_CELL: Cell = Cell::new("".to_string());
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    /// This is a `String`, not a `char` because some Unicode characters can take up
    /// more than one column of space.
    symbol: String
}

impl Cell {
    pub fn new(symbol: String) -> Self {
        Self { symbol }
    }

    /// Constructs a clone of the "empty" `Cell` sentinel value.
    pub fn empty() -> Self {
        EMPTY_CELL.clone()
    }

    pub fn is_empty(&self) -> bool {
        *self == *EMPTY_CELL
    }
}

impl Default for Cell {
    // Space cell
    fn default() -> Self {
        Self::new(" ".to_string())
    }
}

impl Display for Cell {
    /// The preferable way to access the `Cell`'s drawable data.
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.symbol)
    }
}
