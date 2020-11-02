use {
    std::{
        fmt::{
            self, 
            Display, 
            Formatter 
        },
    },
    crossterm::style::{
        Color,
        Attribute
    }
};

/// A cell of a buffer that can contain exactly one `char`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cell {
    /// Content of the cell
    pub content: char,

    /// Foreground color
    fg: Color,

    /// Background color
    bg: Color,

}

impl Cell {
    pub fn new(content: char) -> Self {
        Self {
            content,
            fg: Color::Reset,
            bg: Color::Reset
        }
    }

    pub fn fg(&mut self, color: Color) -> &mut Self {
        self.fg = color;

        self
    }

    pub fn bg(&mut self, color: Color) -> &mut Self {
        self.bg = color;

        self
    }

    pub fn clear(&mut self) {
        *self = Cell::default();
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            content: ' ',
            fg: Color::Reset,
            bg: Color::Reset
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {     
        write!(f, "{}", self.content)
    }
}
