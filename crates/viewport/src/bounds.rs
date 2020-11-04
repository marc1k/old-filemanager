#[derive(Debug, Clone)]
pub struct Bounds {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16
}

impl Bounds {
    pub fn of(width: u16, height: u16) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height
        }
    }

    // Consuming builder
    pub fn at_origin(mut self, x: u16, y: u16) -> Self {
        self.x = x;
        self.y = y;

        self
    }
}
