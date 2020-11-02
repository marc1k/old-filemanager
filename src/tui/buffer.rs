use {
    super::{
        Region,
        Cell,
        Position,
        Dimension
    },
    std::{
        borrow::Borrow,
        fmt::{ self, Debug, Formatter },
        ops::{ Index, IndexMut }
    }
};

pub struct Buffer {
    pub region: Region,

    /// `Cell`s in contiguous representation
    /// Boxed slice to discourage easy length mutation 
    content: Box<[Cell]>
}

impl Buffer {
    /// Constructs a `Buffer` with a specified `Region`, filled with a specified `Cell`
    pub fn filled<C: Borrow<Cell>>(region: Region, cell: C) -> Self {
        let content = vec![cell.borrow().clone(); region.area() as usize]
            .into_boxed_slice();

        Self {
            region,
            content
        }
    }

    /// Constructs a blank `Buffer`
    pub fn blank(region: Region) -> Self {
        Buffer::filled(region, Cell::default())
    }

    pub fn iter(&self) -> impl Iterator<Item = (Position, &Cell) > {
        self.region.iter().zip(self.content.iter())         
    }

    pub fn row(&self, row: u16) -> &[Cell] {
        let row_flat = (self.region.width() * row) as usize;

        &self.content[row_flat .. (row_flat + self.region.width() as usize)]
    }

    pub fn row_mut(&mut self, row: u16) -> &mut [Cell] {
        let row_flat = (self.region.width() * row) as usize;

        &mut self.content[row_flat .. (row_flat + self.region.width() as usize)]
    }
}

impl Index<[u16; 2]> for Buffer {
    type Output = Cell;

    fn index(&self, idx: [u16; 2]) -> &Self::Output {     
        let [col, row] = idx;

        &self.row(row)[col as usize]
    }
}

impl IndexMut<[u16; 2]> for Buffer {
    fn index_mut(&mut self, idx: [u16; 2]) -> &mut Self::Output {     
        let [col, row] = idx;

        &mut self.row_mut(row)[col as usize]
    }
}

impl Debug for Buffer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {     
        for row in 0..self.region.height() {
            for col in 0..self.region.width() {
                write!(f, "{}    ", self[[col, row]])?;
            }
            write!(f, "\n\n")?;
        }

        Ok(())
    }
}
