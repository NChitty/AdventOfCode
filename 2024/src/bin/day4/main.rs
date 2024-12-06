use aoc_2024::*;
use dimensions_2::unsigned::{Dimension, Point};
use itertools::Itertools;

const XMAS: &str = "XMAS";
const MAS: &str = "MAS";

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
        let dimension = Dimension::new(input[0].len(), input.len());
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
                let remaining_directions = Diagonals::
                    iter()
                    .filter(|&&direction| {
                        let (final_x, overflow_x) =
                            start_pos.0.overflowing_add_signed(direction.0 * 3);
                        let (final_y, overflow_y) =
                            start_pos.1.overflowing_add_signed(direction.1 * 3);
                        (!overflow_x && x_range.contains(&final_x))
                            && (!overflow_y && y_range.contains(&final_y))
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
                        word.push(input[cur_pos.1][cur_pos.0]);
                        cur_pos.0 = cur_pos.0.saturating_add_signed(direction.0);
                        cur_pos.1 = cur_pos.1.saturating_add_signed(direction.1);
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
        let y_range = 0..input.len();
        let x_range = 0..input[0].len();
        let mut candidates: Vec<(usize, usize)> = Vec::new();
        for (y, row) in input.iter().enumerate() {
            for (x, &character) in row.iter().enumerate() {
                if character == 'A' {
                    candidates.push((x, y));
                }
            }
        }
        let pos_pos = X_DIRECTIONS[0];
        let neg_neg = X_DIRECTIONS[1];
        let neg_pos = X_DIRECTIONS[2];
        let pos_neg = X_DIRECTIONS[3];

        Ok(candidates
            .iter()
            .filter(|pos| {
                let bottom_left = (
                    pos.0.overflowing_add_signed(neg_neg.0),
                    pos.1.overflowing_add_signed(neg_neg.1),
                );
                let bottom_right = (
                    pos.0.overflowing_add_signed(pos_neg.0),
                    pos.1.overflowing_add_signed(pos_neg.1),
                );
                let top_right = (
                    pos.0.overflowing_add_signed(pos_pos.0),
                    pos.1.overflowing_add_signed(pos_pos.1),
                );
                let top_left = (
                    pos.0.overflowing_add_signed(neg_pos.0),
                    pos.1.overflowing_add_signed(neg_pos.1),
                );
                if bottom_left.0 .1
                    || bottom_left.1 .1
                    || bottom_right.0 .1
                    || bottom_right.1 .1
                    || top_left.0 .1
                    || top_left.1 .1
                    || top_right.0 .1
                    || top_right.1 .1
                {
                    return false;
                }
                if !x_range.contains(&bottom_left.0 .0)
                    || !y_range.contains(&bottom_left.1 .0)
                    || !x_range.contains(&bottom_right.0 .0)
                    || !y_range.contains(&bottom_right.1 .0)
                    || !x_range.contains(&top_left.0 .0)
                    || !y_range.contains(&top_left.1 .0)
                    || !x_range.contains(&top_right.0 .0)
                    || !y_range.contains(&top_right.1 .0)
                {
                    return false;
                }

                let bottom_left = input[bottom_left.1.0][bottom_left.0.0];
                let bottom_right = input[bottom_right.1.0][bottom_right.0.0];
                let top_left = input[top_left.1.0][top_left.0.0];
                let top_right = input[top_right.1.0][top_right.0.0];

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
