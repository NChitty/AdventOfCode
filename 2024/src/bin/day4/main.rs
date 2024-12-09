use aoc_2024::*;
use dimensions_2::{extended::Diagonals, unsigned::{Dimensions, Point}};
use itertools::Itertools;

const XMAS: &str = "XMAS";

aoc!(Day4);

impl Solution<Self> for Day4 {
    type Parsed = Vec<Vec<char>>;

    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::Answer = 18;

    const SAMPLE_ANSWER_B: Self::Answer = 9;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        Ok(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let dimension = Dimensions::new(input[0].len(), input.len());
        let mut candidates: Vec<Point> = Vec::new();
        for (y, row) in input.iter().enumerate() {
            for (x, &character) in row.iter().enumerate() {
                if character == 'X' {
                    candidates.push(Point::new(x, y));
                }
            }
        }
        Ok(candidates
            .iter()
            .map(|start_pos| {
                let remaining_directions = Diagonals::iter()
                    .filter(|direction| {
                        let final_point = *start_pos + (direction.delta().0 * 3, direction.delta().1 * 3);
                        dimension.is_within_bounds_exclusive(final_point)
                    })
                    .collect_vec();
                (start_pos, remaining_directions)
            })
            .map(|(pos, remaining_directions)| {
                let mut words = 0;
                remaining_directions.iter().for_each(|direction| {
                    let mut cur_pos = pos.clone();
                    let mut word = "".to_string();
                    for _ in 0..4 {
                        let (x, y) = cur_pos.get();
                        word.push(input[y][x]);
                        cur_pos += direction.delta();
                    }
                    if word == XMAS {
                        words += 1;
                    }
                });
                words
            })
            .sum())
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let dimension = Dimensions::new(input[0].len(), input.len());
        let mut candidates: Vec<Point> = Vec::new();
        for (y, row) in input.iter().enumerate() {
            for (x, &character) in row.iter().enumerate() {
                if character == 'A' {
                    candidates.push(Point::new(x, y));
                }
            }
        }

        Ok(candidates
            .iter()
            .filter(|pos| {
                let bottom_left = **pos + Diagonals::DownLeft.delta();
                let bottom_right = **pos + Diagonals::DownRight.delta();
                let top_right = **pos + Diagonals::UpRight.delta();
                let top_left = **pos + Diagonals::UpLeft.delta();

                if !dimension.is_within_bounds_exclusive(bottom_left)
                    || !dimension.is_within_bounds_exclusive(bottom_right)
                    || !dimension.is_within_bounds_exclusive(top_left)
                    || !dimension.is_within_bounds_exclusive(top_right) {
                        return false;
                }

                let bottom_left = input[bottom_left.get().1][bottom_left.get().0];
                let bottom_right = input[bottom_right.get().1][bottom_right.get().0];
                let top_left = input[top_left.get().1][top_left.get().0];
                let top_right = input[top_right.get().1][top_right.get().0];

                match ((bottom_left, top_right), (bottom_right, top_left)) {
                    (('M', 'S'), ('M', 'S')) => true,
                    (('S', 'M'), ('M', 'S')) => true,
                    (('S', 'M'), ('S', 'M')) => true,
                    (('M', 'S'), ('S', 'M')) => true,
                    _ => false,
                }
            })
            .count())
    }
}
