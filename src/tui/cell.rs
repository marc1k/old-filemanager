use std::fmt::{
    self, 
    Display, 
    Formatter 
};

/// A cell of a buffer that can contain exactly one `char`
#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub content: char
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            content: ' '
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {     
        write!(f, "{}", self.content)
    }
}

