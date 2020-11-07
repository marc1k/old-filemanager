use {
    super::{Position, Size},
};

/// A positioned region in 2D space.
pub struct Bounds {
    origin: Position,
    size: Size,
}

impl Bounds {
    pub fn new(size: Size) -> Self {
        Self {
            origin: (0, 0).into(),
            size,
        }
    }

    pub fn set_origin(&mut self, pos: Position) {
        self.origin = pos;
    }

    /// Iterate over all `Position`s relative to the origin position of the `Bounds`.
    pub fn iter_relative(&self) -> impl Iterator<Item = Position> {
        let (w, h) = self.size.into();

        (0..w).flat_map(move |col| (0..h).map(move |row| (col, row).into()))
    }

    pub fn iter_absolute(&'_ self) -> impl Iterator<Item = Position> + '_ {
        self.iter_relative().map(move |pos| pos + self.origin)
    }
}
