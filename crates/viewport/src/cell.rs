use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub content: String,
}

impl Cell {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            content: ' '.to_string(),
        }
    }
}
