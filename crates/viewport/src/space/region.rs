use {
    super::{Position, Size},
};

/// A positioned region in 2D space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Region {
    pub origin: Position,
    size: Size,
}

impl Region {
    pub fn new(size: Size) -> Self {
        Self {
            origin: (0, 0).into(),
            size,
        }
    }

    pub fn width(&self) -> u16 {
        self.size.width
    }

    pub fn height(&self) -> u16 {
        self.size.height
    }

    pub fn area(&self) -> u32 {
        u32::from(self.size.width * self.size.height) 
    }

    pub fn set_origin(&mut self, pos: Position) {
        self.origin = pos;
    }

    pub fn contains(&self, pos: &Position) -> bool {
        if pos.x < self.origin.x || pos.x >= self.width() {
            return false;
        }

        if pos.y < self.origin.y || pos.y >= self.height() {
            return false;
        }
        
        true
    }

    /// Iterate over all `Position`s relative to the origin position of the `Region`.
    pub fn iter_relative(&self) -> impl Iterator<Item = Position> {
        let (w, h) = self.size.into();

        (0..h).flat_map(move |row| (0..w).map(move |col| (col, row).into()))
    }

    pub fn iter_absolute(&self) -> impl Iterator<Item = Position> {
        let origin = self.origin;

        self.iter_relative().map(move |pos| pos + origin)
    }
}
