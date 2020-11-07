use {
    std::fmt::{self, Display, Formatter},
};

pub struct Cell {
    /// This is a `String`, not a `char`, because some Unicode characters can take up
    /// more than one column of space.
    symbol: String,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}
