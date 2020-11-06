use {
    crate::{Result, Error, Position, Size},
    std::borrow::Borrow,
    opal_macros::ensure,
};

/// A definition of some area on the screen, with an origin `Position` and a `Size` (i.e.
/// dimensions).
#[derive(Debug, Clone)]
pub struct Bounds {
    pub origin: Position,
    pub dim: Size,
}

impl Bounds {
    /// Creates `Bounds` of `size` at `(0, 0)`.
    pub fn of(size: Size) -> Self {
        Self {
            origin: Position::default(),
            dim: size,
        }
    }

    /// Consuming builder for origin `Position`
    pub fn at_origin(mut self, origin: Position) -> Self {
        self.origin = origin;

        self
    }

    pub fn contains<P: Borrow<Position>>(&self, pos: P) -> bool {
        let pos = pos.borrow();

        if pos.x() < self.origin.x() || pos.x() > self.dim.width() {
            return false;
        }

        if pos.y() < self.origin.y() || pos.y() > self.dim.height() {
            return false;
        }

        true
    }

    /// Returns the index the `Position` is at when flattened.
    pub fn index_of(&self, pos: Position) -> Result<usize> {
        ensure!(
            pos.x() < self.dim.width() && pos.y() < self.dim.height(),
            Error::OutOfBounds(pos.y(), pos.x(), self.clone())
        );

        let n = (pos.y() * self.dim.width()) + pos.x();

        Ok(n as usize)
    }

    /// Builds and returns an `Iterator` of all the `Position`s in the `Bounds` starting
    /// from the `origin`, traversing columns first.
    pub fn iter(&self) -> impl Iterator<Item = Position> + '_ {
        self.dim.iter().map(move |pos| pos + self.origin)
    }
}
