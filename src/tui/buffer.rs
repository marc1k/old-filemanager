use {
    super::{
        Bounds,
        Cell,
    },
    std::{
        ops::{ Index, Sub }
    }
};

pub struct Buffer {
    pub bounds: Bounds,

    /// `Cell`s in contiguous representation
    /// Boxed slice to discourage easy length mutation 
    content: Box<[Cell]>
}

impl Buffer {
    /// Initializes a blank buffer
    pub fn blank(bounds: Bounds) -> Self {
        let len = bounds.width() * bounds.height();
        let content = vec![Cell::default(); len as usize].into_boxed_slice();

        Self {
            bounds,
            content
        }
    }
}

/// Returns a `[Cell]` which can be indexed into (allows n-dimensional array syntax, i.e. `buf[i][j]`)
impl Index<u16> for Buffer {
    type Output = [Cell];

    fn index(&self, row: u16) -> &Self::Output {     
        // Starting index of row 
        let idx = (self.bounds.width() * row) as usize;

        &self.content[idx .. (idx + self.bounds.width() as usize)]
    }
}

