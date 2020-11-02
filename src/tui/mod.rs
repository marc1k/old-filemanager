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

/// The writer used to write to the terminal buffer
pub type W = std::io::BufWriter<std::io::Stderr>;

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
pub struct Dimension {
    pub width: u16,
    pub height: u16
}

impl From<(u16, u16)> for Dimension {
    fn from(pair: (u16, u16)) -> Self {     
        Self {
            width: pair.0,
            height: pair.1
        }
    }
}


