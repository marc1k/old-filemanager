use {
    crate::space::{Region, Position, Size},
    super::Cell,
    std::{
        borrow::Borrow,
        fmt::{
            Result as FmtResult,
            Formatter,
            Debug
        },
        ops::{
            Add, AddAssign,
            Sub, SubAssign,
            Index, IndexMut
        },
    }
};

#[derive(Clone)]
pub struct Buffer {
    region: Region,

    /// Flat representation of a 2D `Cell` matrix (row-major)
    content: Box<[Cell]>,
}

impl Buffer {
    pub fn fill<C: Borrow<Cell>>(cell: C, region: Region) -> Self {
        let cell = cell.borrow().clone(); 
        let content = vec![cell; region.area() as usize].into_boxed_slice();

        Self {
            content,
            region
        }
    }

    pub fn blank(region: Region) -> Self {
        let cell: Cell = Default::default();

        Self::fill(cell, region)
    }

    fn row(&self, y: u16) -> &[Cell] {
        let row = usize::from(y * self.region.width());
        &self.content[row .. row + self.region.width() as usize]
    }

    fn row_mut(&mut self, y: u16) -> &mut [Cell] {
        let row = usize::from(y * self.region.width());
        &mut self.content[row .. row + self.region.width() as usize]
    }

    pub fn iter_relative(&self) -> impl Iterator<Item = (Position, &Cell)> {
        self.region.iter_relative().zip(self.content.iter())
    }

    pub fn iter_absolute(&self) -> impl Iterator<Item = (Position, &Cell)> {
        self.region.iter_absolute().zip(self.content.iter())
    }
}

impl Debug for Buffer {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for row in 0..self.region.height() {
            for col in 0..self.region.width() {
                write!(f, "{}    ", self[&(col, row).into()])?;
            }
            write!(f, "\n\n")?;
        }

        Ok(())
    }
}

impl Index<&Position> for Buffer {
    type Output = Cell;

    fn index(&self, pos: &Position) -> &Self::Output {
        &self.row(pos.y)[pos.x as usize]
    }
}

impl IndexMut<&Position> for Buffer {
    fn index_mut(&mut self, pos: &Position) -> &mut Self::Output {
        &mut self.row_mut(pos.y)[pos.x as usize]
    }
}

// non symmetrical overwrite operation
impl Add for Buffer {
    type Output = Self;
    
    fn add(mut self, rhs: Self) -> Self::Output {
        for (pos, cell) in rhs.iter_absolute() {
            if self.region.contains(&pos) {
                self[&pos] = (*cell).clone();
            }
        }

        self
    }
}

impl AddAssign for Buffer {
    fn add_assign(&mut self, rhs: Self) {
        *self = (*self).clone() + rhs;
    }
}

impl Sub for Buffer {
    type Output = Self;
    
    fn sub(mut self, rhs: Self) -> Self::Output {
        for (pos, cell) in rhs.iter_absolute() {
            // lhs & rhs contain same cell at same position
            if self.region.contains(&pos) && self[&pos] == *cell {
                self[&pos] = Cell::empty();          
            }
        }

        self
    }
}

