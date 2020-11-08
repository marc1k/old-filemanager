use {
    std::fmt::{self, Display, Formatter},
    lazy_static::lazy_static
};

lazy_static! {
    pub static ref EMPTY_CELL: Cell = Cell::new("".to_string());
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    /// This is a `String`, not a `char` because some Unicode characters can take up
    /// more than one column of space.
    symbol: String,
}

impl Cell {
    pub fn new(symbol: String) -> Self {
        Self { symbol }
    }

    pub fn empty() -> Self {
        EMPTY_CELL.clone()
    }
}

impl Default for Cell {
    // Space cell
    fn default() -> Self {
        Self::new(" ".to_string())
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}
