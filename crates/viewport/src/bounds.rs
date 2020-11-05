use crate::{ 
    Result,
    Error,
    ensure,
    Position, Size 
};

#[derive(Debug, Clone)]
pub struct Bounds {
    pub origin: Position,
    pub dim: Size
}

impl Bounds {
    /// Creates `Bounds` of `size` at `(0, 0)`.
    pub fn of(size: Size) -> Self {
        Self {
            origin: Position::default(),
            dim: size
        }
    }

    /// Consuming builder for origin `Position`
    pub fn at_origin(mut self, origin: Position) -> Self {
        self.origin = origin;

        self
    }

    /// Returns the index the `Position` is at `(col, row)` when flattened.
    pub fn index_of(&self, col: u16, row: u16) -> Result<usize> {
        ensure!(
            col < self.dim.width() && row < self.dim.height(),
            Error::OutOfBounds(col, row, self.clone())
        );

        let n = (row * self.dim.width()) + col;

        Ok(n as usize)
    }

    /// Builds and returns an `Iterator` of all the `Position`s in the `Bounds` starting
    /// from the `origin`.
    pub fn iter(&self) -> impl Iterator<Item = Position> + '_ {
        self.dim.iter().map(move |pos| pos + self.origin)
    }
}
