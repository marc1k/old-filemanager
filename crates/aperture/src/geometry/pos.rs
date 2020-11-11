use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign
};

/// A point in 2D space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: u16,
    pub y: u16
}

impl Position {
    pub fn of(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

/// Allow conversions from a `u16` pair.
impl From<(u16, u16)> for Position {
    fn from(pair: (u16, u16)) -> Position {
        Position {
            x: pair.0,
            y: pair.1
        }
    }
}

/// Allow conversions to a `u16` pair.
impl From<Position> for (u16, u16) {
    fn from(pos: Position) -> (u16, u16) {
        (pos.x, pos.y)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
