use {
    crate::{
        Result,
        Error,
        Bounds,
        Cell,
    },
    std::ops::Sub,
};

pub struct Buffer {
    inner: Box<[Cell]>,
    bounds: Bounds
}

impl Buffer {
    pub fn filled_with(fill: Cell, bounds: Bounds) -> Self {
        let inner = vec![fill; (bounds.width * bounds.height) as usize].into_boxed_slice();

        Self {
            inner,
            bounds
        }
    }

    pub fn blank(bounds: Bounds) -> Self {
        Self::filled_with(Cell::default(), bounds)
    }

    fn index_of(&self, col: u16, row: u16) -> Result<usize> {
        if col >= self.bounds.width || row >= self.bounds.height {
            return Err(Error::OutOfBounds(col, row, self.bounds.clone()));
        }

        let idx = (row * self.bounds.width) + col;

        Ok(idx as usize)
    }

    // Indexing (col, row) as per (x, y) convention
    pub fn at_mut(&mut self, col: u16, row: u16) -> Result<&mut Cell> {
        Ok(&mut self.inner[self.index_of(col, row)?])
    }

    // Indexing (col, row) as per (x, y) convention
    pub fn at(&self, col: u16, row: u16) -> Result<&Cell> {
        Ok(&self.inner[self.index_of(col, row)?])
    }
}



