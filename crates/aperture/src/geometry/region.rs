use super::{
    Position,
    Size
};
use std::{
    borrow::Borrow,
    cmp::{
        max,
        min
    }
};

/// A positioned region in 2D space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Region {
    pub origin: Position,
    size: Size
}

impl Region {
    pub fn new(size: Size) -> Self {
        Self {
            origin: (0, 0).into(),
            size
        }
    }

    pub fn hsplit(self, by: u16) -> (Self, Self) {
        let by: f32 = (min(max(by, 0), 100) as f32) / 100.;

        let h1 = ((self.height() as f32) * by).round() as u16;
        let h2 = self.height() - h1;

        let first = Self {
            origin: self.origin,
            size: Size::of(self.width(), h1)
        };

        let second = Self {
            origin: self.origin + (0, first.height()).into(),
            size: Size::of(self.width(), h2)
        };

        (first, second)
    }

    pub fn vsplit(self, by: u16) -> (Self, Self) {
        let by: f32 = (min(max(by, 0), 100) as f32) / 100.;

        let w1 = ((self.width() as f32) * by).round() as u16;
        let w2 = self.width() - w1;

        let first = Self {
            origin: self.origin,
            size: Size::of(w1, self.height())
        };

        let second = Self {
            origin: self.origin + (first.width(), 0).into(),
            size: Size::of(w2, self.height())
        };

        (first, second)
    }

    pub fn width(&self) -> u16 {
        self.size.width
    }

    pub fn height(&self) -> u16 {
        self.size.height
    }

    pub fn area(&self) -> u32 {
        u32::from(self.size.width * self.size.height)
    }

    pub fn set_origin(&mut self, pos: Position) {
        self.origin = pos;
    }

    pub fn contains<P: Borrow<Position>>(&self, pos: P) -> bool {
        let pos = pos.borrow();

        if pos.x < self.origin.x || pos.x >= self.width() {
            return false;
        }

        if pos.y < self.origin.y || pos.y >= self.height() {
            return false;
        }

        true
    }

    /// Iterate over all `Position`s relative to the origin position of the
    /// `Region`.
    pub fn iter_relative(&self) -> impl Iterator<Item = Position> {
        let (w, h) = self.size.into();

        (0..h).flat_map(move |row| (0..w).map(move |col| (col, row).into()))
    }

    pub fn iter_absolute(&self) -> impl Iterator<Item = Position> {
        let origin = self.origin;

        self.iter_relative().map(move |pos| pos + origin)
    }
}
