use std::collections::{HashSet, VecDeque};

use aoc_2024::*;
use dimensions_2::{unsigned::Point, Direction};

aoc!(Day10);

impl Solution<Self> for Day10 {
    type Parsed = Vec<Vec<u8>>;

    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::Answer = 36;

    const SAMPLE_ANSWER_B: Self::Answer = 81;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect())
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let trailheads = find_trailheads(&input);

        let total_score: usize = trailheads
            .iter()
            .map(|&trailhead| score_trailhead(&input, trailhead))
            .sum();
        Ok(total_score)
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let trailheads = find_trailheads(&input);

        let total_rating: usize = trailheads
            .iter()
            .map(|&trailhead| rating_trailhead(&input, trailhead))
            .sum();

        Ok(total_rating)
    }
}

fn find_trailheads(map: &[Vec<u8>]) -> Vec<Point> {
    let mut trailheads = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if height == 0 {
                trailheads.push(Point::new(i, j));
            }
        }
    }
    trailheads
}

fn is_valid_move(map: &[Vec<u8>], current: Point, next: Point) -> bool {
    let (rows, cols) = (map.len(), map[0].len());
    let (x, y) = next.get();
    if x >= rows || y >= cols {
        return false;
    }
    let (cur_x, cur_y) = current.get();
    let current_height = map[cur_x][cur_y];
    let next_height = map[x][y];
    next_height == current_height + 1
}

fn score_trailhead(map: &[Vec<u8>], start: Point) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut reachable_nines = HashSet::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        let (x, y) = current.get();
        let current_height = map[x][y];

        if current_height == 9 {
            reachable_nines.insert(current);
            continue;
        }

        let directions = [
            Direction::Up.delta(),
            Direction::Down.delta(),
            Direction::Left.delta(),
            Direction::Right.delta(),
        ];

        for &dir in &directions {
            let next = current + dir;
            if is_valid_move(map, current, next) && !visited.contains(&next) {
                visited.insert(next);
                queue.push_back(next);
            }
        }
    }

    reachable_nines.len()
}

fn rating_trailhead(map: &[Vec<u8>], start: Point) -> usize {
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut trails = 0;
    count_trails(map, start, &mut visited, &mut trails);
    trails
}

fn count_trails(map: &[Vec<u8>], current: Point, visited: &mut HashSet<Point>, trails: &mut usize) {
    let (x, y) = current.get();
    let current_height = map[x][y];

    if current_height == 9 {
        *trails += 1;
        return;
    }

    let directions = [
        Direction::Up.delta(),
        Direction::Down.delta(),
        Direction::Left.delta(),
        Direction::Right.delta(),
    ];

    for &dir in &directions {
        let next = current + dir;
        if is_valid_move(map, current, next) && !visited.contains(&next) {
            visited.insert(next);
            count_trails(map, next, visited, trails);
            visited.remove(&next);
        }
    }
}
