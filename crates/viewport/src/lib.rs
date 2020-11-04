mod bounds;
mod buffer;
mod cell;
mod error;

pub use {
    bounds::Bounds,
    buffer::Buffer,
    cell::Cell,
    error::Error
};

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer() -> Result<()> {
        let mut buf = Buffer::blank(Bounds::of(10, 10).at_origin(0, 0));

        buf.at_mut(5, 5)?.content = ".".to_string();
        assert_eq!(buf.at(5, 5)?.content, ".".to_owned());

        Ok(())
    }
}
