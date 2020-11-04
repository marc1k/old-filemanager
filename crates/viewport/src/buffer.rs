use {
    crate::{Bounds, Cell, Result},
    std::collections::HashSet,
};

pub struct Buffer {
    inner: Box<[Cell]>,
    bounds: Bounds,
}

impl std::fmt::Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.bounds.height {
            for col in 0..self.bounds.width {
                write!(f, "{}    ", self.at(col, row).unwrap()).unwrap();
            }
            write!(f, "\n\n").unwrap();
        }

        Ok(())
    }
}

impl Buffer {
    pub fn filled_with(fill: Cell, bounds: Bounds) -> Self {
        let inner = vec![fill; (bounds.width * bounds.height) as usize]
            .into_boxed_slice();

        Self { inner, bounds }
    }

    pub fn blank(bounds: Bounds) -> Self {
        Self::filled_with(Cell::default(), bounds)
    }

    // Indexing (col, row) as per (x, y) convention
    pub fn at_mut(&mut self, col: u16, row: u16) -> Result<&mut Cell> {
        Ok(&mut self.inner[self.bounds.index_of(col, row)?])
    }

    // Indexing (col, row) as per (x, y) convention
    pub fn at(&self, col: u16, row: u16) -> Result<&Cell> {
        Ok(&self.inner[self.bounds.index_of(col, row)?])
    }

    /// An `Iterator` over each coordinate within the `Buffer` and a mutable
    /// reference to the corresponding `Cell`.
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = ((u16, u16), &mut Cell)> {
        self.bounds.iter_interior().zip(self.inner.iter_mut())
    }

    /// An `Iterator` over each coordinate within the `Buffer` and an immutable
    /// reference to the corresponding `Cell`.
    pub fn iter(&self) -> impl Iterator<Item = ((u16, u16), &Cell)> {
        self.bounds.iter_interior().zip(self.inner.iter())
    }

    /// An `Iterator` over each coordinate in the intersection of the smaller specified `Buffer`
    /// and the current `Buffer`, and a mutable reference to the corresponding `Cell`.
    ///
    /// This can be thought of as narrowing the "scope" of the `Buffer` temporarily.
    ///
    /// Any overflow is elided. If the specified `Bounds` are completely
    /// overflowed, the `Iterator` returned will be empty.
    pub fn iter_intersect_mut<'b>(
        &'b mut self,
        bounds: &'b Bounds,
    ) -> impl Iterator<Item = ((u16, u16), &'b mut Cell)> + 'b {
        // Set of coordinates relative in the specified `Bounds` to this `Buffer`'s `Bounds`.
        let coords: HashSet<_> = bounds.iter().collect();

        self.iter_mut()
            .filter(move |(p, _)| coords.contains(p))
            .into_iter()
    }

    /// An `Iterator` over each coordinate in the intersection of the smaller specified `Buffer`
    /// and the current `Buffer`, and an immutable reference to the corresponding `Cell`.
    ///
    /// This can be thought of as narrowing the "scope" of the `Buffer` temporarily.
    ///
    /// Any overflow is elided. If the specified `Bounds` are completely
    /// overflowed, the `Iterator` returned will be empty.
    pub fn iter_intersect<'b>(
        &'b self,
        bounds: &'b Bounds,
    ) -> impl Iterator<Item = ((u16, u16), &'b Cell)> + 'b {
        // Set of coordinates in the specified `Bounds` relative to the current `Buffer`'s `Bounds`.
        let coords: HashSet<_> = bounds.iter().collect();

        self.iter()
            .filter(move |(p, _)| coords.contains(p))
            .into_iter()
    }

    /*pub fn diff(&self, other: Self) -> impl Iterator<Item = ((u16, u16), &Cell)> {
    }*/
}
