use std::{
    collections::{HashMap, HashSet},
    isize,
};

use aoc_2024::*;
use dimensions_2::unsigned::{Dimension, Point};
use itertools::Itertools;

aoc!(Day8);

#[derive(Copy, Clone, Debug, Hash)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn double_from_ends(&self) -> Vec<Point> {
        let (x1, y1) = self.start.get();
        let (x2, y2) = self.end.get();

        let dx = x2 as isize - x1 as isize;
        let dy = y2 as isize - y1 as isize;

        let mut result = Vec::new();

        if let (Some(new_x1), Some(new_y1)) = (x2.checked_add_signed(dx), y2.checked_add_signed(dy))
        {
            result.push(Point::new(new_x1, new_y1));
        }

        if let (Some(new_x2), Some(new_y2)) =
            (x1.checked_add_signed(-dx), y1.checked_add_signed(-dy))
        {
            result.push(Point::new(new_x2, new_y2));
        };

        result
    }

    fn contains_point(&self, point: &Point) -> bool {
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

    fn extend_distances(&self, dimensions: Dimension) -> Vec<Point> {
        let (x1, y1) = self.start.get();
        let (x2, y2) = self.end.get();

        let dx = x2 as isize - x1 as isize;
        let dy = y2 as isize - y1 as isize;

        let mut result = Vec::new();

        let mut antinode = self.start;
        while dimensions.is_within_bounds_exclusive(antinode) {
            result.push(antinode);
            antinode += (-dx, -dy);
        }

        antinode = self.end;
        while dimensions.is_within_bounds_exclusive(antinode) {
            result.push(antinode);
            antinode += (dx, dy);
        }

        result
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.contains_point(&other.start) && self.contains_point(&other.end)
    }
}

impl Eq for Line {}

impl Solution<Self> for Day8 {
    type Parsed = (Dimension, HashMap<char, Vec<Point>>);

    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::Answer = 14;

    const SAMPLE_ANSWER_B: Self::Answer = 34;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        let dimension = Dimension::new(
            input.lines().next().expect("No lines").len(),
            input.lines().count(),
        );
        Ok((
            dimension,
            input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(move |(x, antenna)| match antenna {
                            letter if antenna.is_alphanumeric() => Some((letter, Point::new(x, y))),
                            _ => None,
                        })
                        .into_iter()
                })
                .into_group_map(),
        ))
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let (dimension, groups) = input;
        let lines = transform_to_lines(&groups);
        let antinodes: HashSet<Point> = lines
            .iter()
            .flat_map(|(_, lines)| lines)
            .flat_map(|line| line.double_from_ends())
            .filter(|point| dimension.is_within_bounds_exclusive(*point))
            .collect();
        Ok(antinodes.len())
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let (dimension, groups) = input;

        let lines = transform_to_lines(&groups);
        let antinodes: HashSet<Point> = lines
            .iter()
            .flat_map(|(_, lines)| lines)
            .flat_map(|line| line.extend_distances(dimension))
            .collect();

        Ok(antinodes.len())
    }
}

fn transform_to_lines(parsed: &HashMap<char, Vec<Point>>) -> HashMap<char, HashSet<Line>> {
    let mut lines = HashMap::new();

    for (antenna, pairs) in parsed {
        for i in 0..pairs.len() {
            for j in (i + 1)..pairs.len() {
                let point1 = pairs[i];
                let point2 = pairs[j];
                let mut default = HashSet::new();
                default.insert(Line::new(point1, point2));
                lines
                    .entry(*antenna)
                    .and_modify(|set: &mut HashSet<Line>| {
                        set.insert(Line::new(point1, point2));
                    })
                    .or_insert(default);
            }
        }
    }

    lines
}
