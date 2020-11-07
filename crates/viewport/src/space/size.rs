use std::ops::{Add, AddAssign, Sub, SubAssign};

/// A positionless size in 2D space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
    width: u16,
    height: u16,
}

impl Size {
    pub fn area(&self) -> u32 {
        u32::from(self.width * self.height)
    }
}

/// Allow conversions from a `u16` pair.
impl From<(u16, u16)> for Size {
    fn from(pair: (u16, u16)) -> Self {
        Self {
            width: pair.0,
            height: pair.1,
        }
    }
}

/// Allow conversions to a `u16` pair.
impl From<Size> for (u16, u16) {
    fn from(size: Size) -> (u16, u16) {
        (size.width, size.height)
    }
}
