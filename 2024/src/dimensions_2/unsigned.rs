use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn get(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl Add<Point> for Point{
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<(isize, isize)> for Point {
    type Output = Self;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Self {
            x: self.x.wrapping_add_signed(rhs.0),
            y: self.y.wrapping_add_signed(rhs.1),
        }
    }
}

impl AddAssign<(isize, isize)> for Point {
    fn add_assign(&mut self, rhs: (isize, isize)) {
        self.x = self.x.wrapping_add_signed(rhs.0);
        self.y = self.y.wrapping_add_signed(rhs.1);
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Dimension {
    width: usize,
    len: usize,
}

impl Dimension {
    pub fn new(width: usize, len: usize) -> Self {
        Self { width, len }
    }

    pub fn get(&self) -> (usize, usize) {
        (self.len,
        self.width)
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_len(&self) -> usize {
        self.len
    }

    pub fn is_within_bounds_exclusive(&self, point: Point) -> bool {
        let (x, y) = point.get();
        self.len > y && self.width > x
    }

    pub fn is_within_bounds_inclusive(&self, point: Point) -> bool {
        let (x, y) = point.get();
        self.len >= y && self.width >= x
    }
}
