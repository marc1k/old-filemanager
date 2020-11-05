use {
    crate::{
        Bounds,
        Cell,
        Position,
        Size,
        Result
    },
};

pub struct Buffer {
    inner: Box<[Cell]>,
    bounds: Bounds,
}

/*impl std::fmt::Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.bounds.height {
            for col in 0..self.bounds.width {
                write!(f, "{}    ", self.at(col, row).unwrap()).unwrap();
            }
            write!(f, "\n\n").unwrap();
        }

        Ok(())
    }
}*/

impl Buffer {
    /// Creates a `Buffer` filled with specified `Cell`s. 
    pub fn filled_with(fill: Cell, bounds: Bounds) -> Self {
        let inner = vec![fill; (bounds.dim.width() * bounds.dim.height()) as usize]
            .into_boxed_slice();

        Self { inner, bounds }
    }

    /// Creates a blank `Buffer` within specified `Bounds`.
    pub fn blank(bounds: Bounds) -> Self {
        Self::filled_with(Cell::default(), bounds)
    }

}
