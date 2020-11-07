mod display;
mod space;

pub use {
    display::{Cell},
    space::{Bounds, Position, Size},
};

#[cfg(test)]
mod tests {
    #[test]
    fn bounds() {}
}
