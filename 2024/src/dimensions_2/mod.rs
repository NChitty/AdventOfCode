use std::array::IntoIter;

// pub mod signed;
pub mod unsigned;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn iter() -> IntoIter<Direction, 4> {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .into_iter()
    }

    pub fn scan(
        self,
        position: (usize, usize),
        dimensions: (usize, usize),
    ) -> Box<dyn Iterator<Item = (usize, usize)>> {
        let (x, y) = position;
        let (width, height) = dimensions;

        match self {
            Direction::Up => Box::new((0..=y).rev().map(move |ny| (x, ny))),
            Direction::Down => Box::new((y + 1..=height).map(move |ny| (x, ny))),
            Direction::Left => Box::new((0..=x).rev().map(move |nx| (nx, y))),
            Direction::Right => Box::new((x + 1..=width).map(move |nx| (nx, y))),
        }
    }

    pub fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn rotate_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    pub fn inverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

pub mod extended {
    use std::vec::IntoIter;

    use itertools::Itertools;

    use super::Direction;

    pub enum Diagonals {
        Cardinal(Direction),
        UpRight,
        UpLeft,
        DownRight,
        DownLeft,
    }

    impl Diagonals {
        pub fn delta(&self) -> (isize, isize) {
            match self {
                Diagonals::Cardinal(dir) => dir.delta(),
                Diagonals::UpRight => (1, -1),
                Diagonals::UpLeft => (-1, -1),
                Diagonals::DownRight => (1, 1),
                Diagonals::DownLeft => (-1, 1),
            }
        }

        pub fn iter() -> IntoIter<Diagonals> {
            let mut vec = Direction::iter()
                .map(|direction| Diagonals::Cardinal(direction))
                .collect_vec();
            vec.push(Diagonals::UpLeft);
            vec.push(Diagonals::UpRight);
            vec.push(Diagonals::DownLeft);
            vec.push(Diagonals::DownRight);
            vec.into_iter()
        }
    }
}
