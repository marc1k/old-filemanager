use {
    crate::{
        Bounds,
        Cell,
        Position,
        Size,
        Result
    },
};

/// An intermediary between a caller writing data and the *real* terminal buffer.
pub struct Buffer {
    /// Boxed slice of `Cell`s to discourage trivial length mutation.
    content: Box<[Cell]>,

    /// The bounds of which this `Buffer` is constrained by.
    pub bounds: Bounds,
}

impl std::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.bounds.dim.height() {
            for col in 0..self.bounds.dim.width() {
                let pos = Position(col, row);
                write!(f, "{}    ", self.cell_at(pos).unwrap()).unwrap();
            }
            write!(f, "\n\n").unwrap();
        }

        Ok(())
    }
}

impl Buffer {
    /// Creates a `Buffer` filled with specified `Cell`s. 
    pub fn filled_with(fill: Cell, bounds: Bounds) -> Self {
        let content = vec![
            fill; 
            (bounds.dim.width() * bounds.dim.height()) as usize
        ].into_boxed_slice();

        Self { content, bounds }
    }

    /// Creates a blank `Buffer` within specified `Bounds`.
    pub fn blank(bounds: Bounds) -> Self {
        Self::filled_with(Cell::default(), bounds)
    }

    /// Returns the `Cell` at the specified `Position` if valid, and `Error` otherwise.
    pub fn cell_at(&self, pos: Position) -> Result<&Cell> {
        Ok(&self.content[self.bounds.index_of(pos)?])
    }

    /*pub fn line_at(&self, row: u16 -> Result<&[Cell]> {
    }*/

}
