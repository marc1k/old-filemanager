mod buffer;
mod cell;
mod region;
mod viewport;

pub use {
    buffer::Buffer,
    cell::Cell,
    region::Region,
    viewport::Viewport,
};

use std::convert::From;

#[derive(Debug, Clone)]
pub struct Position {
    pub col: u16,
    pub row: u16,
}

impl From<(u16, u16)> for Position {
    fn from(pair: (u16, u16)) -> Self {     
        Self {
            col: pair.0,
            row: pair.1
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dimensions {
    pub width: u16,
    pub height: u16
}

impl From<(u16, u16)> for Dimensions {
    fn from(pair: (u16, u16)) -> Self {     
        Self {
            width: pair.0,
            height: pair.1
        }
    }
}


