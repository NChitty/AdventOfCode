use std::collections::{HashMap, HashSet};

use aoc_2024::*;
use dimensions_2::{unsigned::Point, Direction};
use itertools::Itertools;

aoc!(Day12);

impl Solution<Self> for Day12 {
    type Parsed = Vec<Vec<char>>;

    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::Answer = 140;

    const SAMPLE_ANSWER_B: Self::Answer = 80;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| line.chars().into_iter().collect())
            .collect())
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(calculate_regions(input)
            .into_iter()
            .map(|(_letter, area, perimeter)| area * perimeter)
            .sum())
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(calculate_regions_sides(input)
            .into_iter()
            .map(|(_letter, area, sides)| {
                area * sides
            })
            .sum())
    }
}

fn calculate_regions(grid: Vec<Vec<char>>) -> Vec<(char, usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut regions = Vec::new();

    // Iterate through the grid to find regions
    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                let letter = grid[r][c];
                let (area, perimeter) = flood_fill(&grid, &mut visited, Point::new(c, r));
                regions.push((letter, area, perimeter));
            }
        }
    }

    regions
}

fn calculate_regions_sides(grid: Vec<Vec<char>>) -> Vec<(char, usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut regions = Vec::new();

    // Iterate through the grid to find regions
    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                let letter = grid[r][c];
                let (area, sides) = flood_fill_side_tracking(&grid, &mut visited, Point::new(c, r));
                regions.push((letter, area, sides));
            }
        }
    }

    regions
}

fn flood_fill_side_tracking(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start: Point,
) -> (usize, usize) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut stack = vec![start];
    let mut area = 0;
    let mut sides: HashSet<(Direction, Point)> = HashSet::new();
    let mut sides_count = 0;

    let letter = grid[start.get().1][start.get().0];
    visited[start.get().1][start.get().0] = true;

    while let Some(cur_point) = stack.pop() {
        area += 1;

        for direction in Direction::iter() {
            let (nx, ny) = (cur_point + direction.delta()).get();
            if ny < rows && nx < cols {
                if grid[ny][nx] == letter {
                    if !visited[ny][nx] {
                        visited[ny][nx] = true;
                        stack.push(cur_point + direction.delta());
                    }
                } else {
                    sides.insert((direction, cur_point + direction.delta()));
                }
            } else {
                sides.insert((direction, cur_point + direction.delta()));
            }
        }
    }

    let edges = sides.into_iter().into_group_map_by(|(dir, _point)| *dir);
    for dir in Direction::iter() {
        let vec = edges.get(&dir).unwrap().into_iter().map(|tuple| tuple.1).collect_vec();
        match dir {
            Direction::Up | Direction::Down => {
                let (mut prev_x, mut prev_y) = (0usize, 0usize);
                vec.iter()
                    .sorted_by(|a, b| match a.get().1.cmp(&b.get().1) {
                        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => a.get().0.cmp(&b.get().0),
                        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                    })
                    .for_each(|point| {
                        if point.get().1 != prev_y {
                            sides_count += 1;
                        } else if prev_x.abs_diff(point.get().0) > 1 {
                            sides_count += 1;
                        }
                        (prev_x, prev_y) = point.get();
                    });
            }

            Direction::Left | Direction::Right => {
                let (mut prev_x, mut prev_y) = (0usize, 0usize);
                vec.iter()
                    .sorted_by(|a, b| match a.get().0.cmp(&b.get().0) {
                        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => a.get().1.cmp(&b.get().1),
                        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                    })
                    .for_each(|point| {
                        if point.get().0 != prev_x {
                            sides_count += 1;
                        } else if prev_y.abs_diff(point.get().1) > 1 {
                            sides_count += 1;
                        }
                        (prev_x, prev_y) = point.get();
                    });
            }
        }
    }

    (area, sides_count)
}

fn flood_fill(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, start: Point) -> (usize, usize) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut stack = vec![start];
    let mut area = 0;
    let mut perimeter = 0;

    let letter = grid[start.get().1][start.get().0];
    visited[start.get().1][start.get().0] = true;

    while let Some(cur_point) = stack.pop() {
        area += 1;

        // Explore neighbors
        for direction in Direction::iter() {
            let new_point = cur_point + direction.delta();

            if new_point.get().1 < rows && new_point.get().0 < cols {
                let (nx, ny) = new_point.get();
                if grid[ny][nx] == letter && !visited[ny][nx] {
                    visited[ny][nx] = true;
                    stack.push(new_point);
                } else if grid[ny][nx] != letter {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    (area, perimeter)
}
