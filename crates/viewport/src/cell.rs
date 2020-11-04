#[derive(Debug, Clone)]
pub struct Cell {
    pub content: String,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            content: ' '.to_string(),
        }
    }
}
