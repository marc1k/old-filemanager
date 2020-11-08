mod display;
mod space;

pub use {
    display::{Buffer, Cell},
    space::{Region, Position, Size},
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_contains() {
        let reg = Region::new((5, 5).into());

        assert!(reg.contains((0, 0).into()) == true);
        assert!(reg.contains((10, 0).into()) == false);
        assert!(reg.contains((5, 5).into()) == false);
        assert!(reg.contains((3, 2).into()) == true);
    }

    #[test]
    fn region_contains_iter() {
        let reg = Region::new((5, 5).into());

        for pos in reg.iter_relative() {
            assert!(reg.contains(pos));
        }
    }

    #[test]
    fn buf() {

    }
}
