/// Definition of some area
#[derive(Debug)]
pub struct Bounds {
    pub origin: (u16, u16),
    pub dim: (u16, u16)
}

impl Bounds {
    pub fn new(origin: (u16, u16), dim: (u16, u16)) -> Self {
        Self {
            origin,
            dim
        }
    }

    /// Coordinates relative to the viewport origin
    pub fn coords(&self) -> (u16, u16, u16, u16) {
        let (x, y) = self.origin;
        let (width, height) = self.dim;
        (
            x,
            y,
            x + width,
            y + height
        )
    }

    pub fn width(&self) -> u16 {
        self.dim.0
    }

    pub fn height(&self) -> u16 {
        self.dim.1
    }
}
