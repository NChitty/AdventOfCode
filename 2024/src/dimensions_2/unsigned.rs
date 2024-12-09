use std::{
    isize,
    ops::{Add, AddAssign},
};

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

impl Add<Point> for Point {
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
pub struct Dimensions {
    width: usize,
    len: usize,
}

impl Dimensions {
    pub fn new(width: usize, len: usize) -> Self {
        Self { width, len }
    }

    pub fn get(&self) -> (usize, usize) {
        (self.len, self.width)
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

#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub fn get_start(self) -> Point {
        self.start
    }

    pub fn get_end(self) -> Point {
        self.end
    }

    pub fn slope(self) -> isize {
        (self.end.get().1 as isize - self.start.get().1 as isize)
            / (self.end.get().0 as isize - self.start.get().0 as isize)
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        let (dx1, dy1) = (
            self.end.get().0 as isize - self.start.get().0 as isize,
            self.end.get().1 as isize - self.start.get().1 as isize,
        );
        let (dx2, dy2) = (
            point.get().0 as isize - self.start.get().0 as isize,
            point.get().1 as isize - self.start.get().1 as isize,
        );

        let cross_product = dy1 * dx2 - dx1 * dy2;

        cross_product == 0
    }
}

impl PartialEq for Line {
    /// A line for AOC purposes is one co-linear with self, this implementation checks this by
    /// checking if both points in other are on self.
    fn eq(&self, other: &Self) -> bool {
        self.contains_point(&other.start) && self.contains_point(&other.end)
    }
}

impl Eq for Line {}
