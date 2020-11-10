use super::Cell;
use crate::space::{
    Position,
    Region
};
use std::{
    borrow::Borrow,
    fmt::{
        Debug,
        Formatter,
        Result as FmtResult
    },
    ops::{
        Add,
        AddAssign,
        Index,
        IndexMut,
        Sub,
        SubAssign
    }
};

#[derive(Clone)]
pub struct Buffer {
    /// The area of the screen the content is restricted to.
    region: Region,

    /// Flat representation of a 2D `Cell` matrix (row-major).
    content: Box<[Cell]>
}

impl Buffer {
    /// Fills a `Buffer` with clones of the specified `Cell`.
    pub fn fill<C: Borrow<Cell>>(cell: C, region: Region) -> Self {
        let cell = cell.borrow().clone();
        let content = vec![cell; region.area() as usize].into_boxed_slice();

        Self { content, region }
    }

    /// Creates a `Buffer` filled with space `Cell`s.
    ///
    /// This is not to be confused with an *empty* `Buffer`.
    /// Space `Cell`s are iterable, while empty `Cell`s are not.
    pub fn blank(region: Region) -> Self {
        let cell: Cell = Default::default();

        Self::fill(cell, region)
    }

    /// Creates a `Buffer` filled with *empty* `Cell`s.
    ///
    /// This is not to be confused with a *blank* `Buffer`.
    /// Space `Cell`s are iterable, while empty `Cell`s are not.
    pub fn empty(region: Region) -> Self {
        let cell = Cell::empty();

        Self::fill(cell, region)
    }

    /// Returns a reference to the row at the given `y`.
    fn row(&self, y: u16) -> &[Cell] {
        let row = usize::from(y * self.region.width());
        &self.content[row..row + self.region.width() as usize]
    }

    /// Returns a mutable reference row at the given `y`.
    fn row_mut(&mut self, y: u16) -> &mut [Cell] {
        let row = usize::from(y * self.region.width());
        &mut self.content[row..row + self.region.width() as usize]
    }

    /// Returns an `Iterator` over each `Position` in the `Region`, paired with
    /// the respective `Cell`s at that point.
    ///
    /// Each `Position` is relative to the origin of the `Buffer` itself,
    /// *not* the environment it was defined in.
    pub fn iter_relative(&self) -> impl Iterator<Item = (Position, &Cell)> {
        self.region
            .iter_relative()
            .zip(self.content.iter())
            .filter(|(_, cell)| !cell.is_empty())
    }

    /// Returns an `Iterator` over each `Position` in the `Region`, paired with
    /// the respective `Cell`s at that point.
    ///
    /// Each `Position` is *absolute*. It is relative to the environment the
    /// `Buffer` was defined in.
    pub fn iter_absolute(&self) -> impl Iterator<Item = (Position, &Cell)> {
        self.region
            .iter_absolute()
            .zip(self.content.iter())
            .filter(|(_, cell)| !cell.is_empty())
    }
}

impl Debug for Buffer {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for row in 0..self.region.height() {
            for col in 0..self.region.width() {
                write!(f, "{}    ", self[Position::of(col, row)])?;
            }
            write!(f, "\n\n")?;
        }

        Ok(())
    }
}

impl<P> Index<P> for Buffer
where
    P: Borrow<Position>
{
    type Output = Cell;

    fn index(&self, pos: P) -> &Self::Output {
        let pos = pos.borrow();

        &self.row(pos.y)[pos.x as usize]
    }
}

impl<P> IndexMut<P> for Buffer
where
    P: Borrow<Position>
{
    fn index_mut(&mut self, pos: P) -> &mut Self::Output {
        let pos = pos.borrow();

        &mut self.row_mut(pos.y)[pos.x as usize]
    }
}

impl Add for Buffer {
    type Output = Self;

    /// A non-symmetrical overwrite operation.
    fn add(mut self, rhs: Self) -> Self::Output {
        for (pos, cell) in rhs.iter_absolute() {
            if self.region.contains(pos) {
                self[pos] = (*cell).clone();
            }
        }

        self
    }
}

impl AddAssign for Buffer {
    fn add_assign(&mut self, rhs: Self) {
        // The clone is cheap because the original `self` is dropped right
        // after assignment.
        *self = (*self).clone() + rhs;
    }
}

impl Sub for Buffer {
    type Output = Self;

    /// A non-symmetrical subtraction operation.
    fn sub(mut self, rhs: Self) -> Self::Output {
        for (pos, cell) in rhs.iter_absolute() {
            if self.region.contains(pos) && self[pos] == *cell {
                self[pos] = Cell::empty();
            }
        }

        self
    }
}

impl SubAssign for Buffer {
    fn sub_assign(&mut self, rhs: Self) {
        // The clone is cheap because the original `self` is dropped right
        // after assignment.
        *self = (*self).clone() - rhs;
    }
}
