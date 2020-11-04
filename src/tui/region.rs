use super::{
    Position,
    Dimensions
};

/// Representation of a region in 2D space.
/// All positions and dimensions are of the form (col, row).
#[derive(Debug, Clone)]
pub struct Region {
    pub origin: Position,
    pub dim: Dimensions
}

impl Region {
    /// Constructs a new `Region` defaulting to origin `(0, 0)`.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            origin: Position::from((0, 0)),
            dim: Dimensions::from((width, height))
        }
    }

    /// Iterates over each `Position` in the `Region`, disregarding the origin `Position`.
    pub fn iter(&self) -> impl Iterator<Item = Position> {
        let Dimensions { width, height } = self.dim;

        (0 .. height).into_iter()
            .map(move |row|
                (0 .. width).into_iter()
                    .map(move |col|
                        Position::from((col, row))
                    )
            )
            .flatten()
    }

    /// Sets the origin.
    pub fn origin(mut self, col: u16, row: u16) -> Self {
        self.origin = Position::from((col, row)); 

        self
    }

    pub fn area(&self) -> u16 {
        self.dim.width * self.dim.height
    }

    pub fn width(&self) -> u16 {
        self.dim.width
    }

    pub fn height(&self) -> u16 {
        self.dim.height
    }
    
}
