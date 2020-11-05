use std::ops::{
    Add, Sub,
    Mul, Div
};

/// A definition of a `(x, y)` or `(col, row)` point in 2D space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position(pub u16, pub u16);

impl Default for Position {
    fn default() -> Self {
        Self(0, 0)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul for Position {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Div for Position {
    type Output = Self;

    fn div(self, mut rhs: Self) -> Self::Output {
        if rhs.0 == 0 {
            rhs.0 = 1;
        }

        if rhs.1 == 0 {
            rhs.1 = 1;
        }

        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}

/// A definition of some size `(width, height)`. It is relative strictly to itself, and has no
/// positional meaning on its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size(pub u16, pub u16);

impl Size {
    /// Builds and returns an `Iterator` over possible arbitrary `Position`s inside this size constraint.
    pub fn iter(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.1).flat_map(move |row| {
            (0..self.0).map(move |col| Position(col, row))
        })
    }

    pub fn width(&self) -> u16 {
        self.0
    }

    pub fn height(&self) -> u16 {
        self.1
    }
}

impl Default for Size {
    fn default() -> Self {
        Self(1, 1)
    }
}

impl Add for Size {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Size {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul for Size {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Div for Size {
    type Output = Self;

    fn div(self, mut rhs: Self) -> Self::Output {
        if rhs.0 == 0 {
            rhs.0 = 1;
        }

        if rhs.1 == 0 {
            rhs.1 = 1;
        }

        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}
