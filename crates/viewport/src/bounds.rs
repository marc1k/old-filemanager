use crate::{ensure, Error, Result};

#[derive(Debug, Clone)]
pub struct Bounds {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Bounds {
    pub fn of(width: u16, height: u16) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    // Consuming builder
    pub fn at_origin(mut self, x: u16, y: u16) -> Self {
        self.x = x;
        self.y = y;

        self
    }

    /// The index of the coordinate at (col, row) in the `Bounds` relative to itself.
    pub fn index_of(&self, col: u16, row: u16) -> Result<usize> {
        ensure!(
            col < self.width && row < self.height,
            Error::OutOfBounds(col, row, self.clone())
        );

        let idx = (row * self.width) + col;

        Ok(idx as usize)
    }

    /// An `Iterator` over each possible coordinate within the `Bounds` in form `(col, row)`
    /// relative to itself.
    pub fn iter_interior(&self) -> impl Iterator<Item = (u16, u16)> + '_ {
        (0..self.height)
            .flat_map(move |row| (0..self.width).map(move |col| (col, row)))
    }

    /// An `Iterator` over each possible coordinate of the `Bounds` relative to its
    /// environment.
    pub fn iter(&self) -> impl Iterator<Item = (u16, u16)> + '_ {
        (self.y..self.y + self.height).flat_map(move |row| {
            (self.x..self.x + self.width).map(move |col| (col, row))
        })
    }
}
