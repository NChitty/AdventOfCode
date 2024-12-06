use std::{collections::HashSet, usize};

use aoc_2024::*;
use dimensions_2::{unsigned::{Dimension, Point}, Direction};

aoc!(Day6);


#[derive(Clone, Copy, Debug)]
struct Guard {
    position: Point,
    direction: Direction,
}

impl Guard {

    fn step(
        &mut self,
        dimensions: Dimension,
        obstacles: &HashSet<Point>,
        visited: &mut HashSet<Point>,
    ) -> bool {
        let range = self.direction.scan(self.position.get(), dimensions.get());
        for (x, y) in range {
            if obstacles.contains(&Point::new(x, y)) {
                self.direction = self.direction.rotate_right();
                return true;
            }
            visited.insert(self.position);
            self.position = Point::new(x, y);
        }
        false
    }

    fn step_to_obstacle(
        &mut self,
        dimensions: Dimension,
        obstacles: &HashSet<Point>,
    ) -> bool {
        let range = self.direction.scan(self.position.get(), dimensions.get());
        for (x, y) in range {
            if obstacles.contains(&Point::new(x, y)) {
                self.direction = self.direction.rotate_right();
                return true;
            }
            self.position = Point::new(x, y);
        }
        false
    }

    fn patrol(
        &mut self,
        dimensions: Dimension,
        obstacles: &HashSet<Point>,
        visited: &mut HashSet<Point>,
    ) {
        while self.step(dimensions, obstacles, visited) {}
    }

    fn detect_loop(
        self,
        dimensions: Dimension,
        obstacles: &HashSet<Point>
    ) -> bool {
        let mut state: HashSet<(Point, Direction)> = HashSet::new();
        let mut temp_guard = self;
        while state.insert((temp_guard.position, temp_guard.direction)) {
            if !temp_guard.step_to_obstacle(dimensions, obstacles) {
                return false;
            }
        }
        true
    }
}

impl Solution<Self> for Day6 {
    type Parsed = (Dimension, HashSet<Point>, Guard);

    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::Answer = 41;

    const SAMPLE_ANSWER_B: Self::Answer = 6;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        let mut obstacles: HashSet<Point> = HashSet::new();
        let mut guard: Guard = Guard {
            position: Point::default(),
            direction: Direction::Up,
        };
        let dimensions = Dimension::new(
            input.lines().next().expect("No line").len(),
            input.lines().count(),
        );
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| match c {
                '#' => {
                    obstacles.insert(Point::new(x, y));
                }
                '>' => {
                    guard.direction = Direction::Right;
                    guard.position = Point::new(x, y);
                }
                '<' => {
                    guard.direction = Direction::Left;
                    guard.position = Point::new(x, y);
                }
                '^' => {
                    guard.direction = Direction::Up;
                    guard.position = Point::new(x, y);
                }
                'v' => {
                    guard.direction = Direction::Down;
                    guard.position = Point::new(x, y);
                }
                _ => (),
            });
        });
        Ok((dimensions, obstacles, guard))
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let (dimensions, obstacles, mut guard) = input;
        let mut visited: HashSet<Point> = HashSet::new();
        guard.patrol(dimensions, &obstacles, &mut visited);
        Ok(visited.len())
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let (dimensions, obstacles, guard) = input;
        let mut path_guard = guard;
        let mut visited: HashSet<Point> = HashSet::new();
        path_guard.patrol(dimensions, &obstacles, &mut visited);
        Ok(visited.iter().filter(|new_obstacle| {
            let mut new_obstacles = obstacles.clone();
            new_obstacles.insert(**new_obstacle);
            guard.detect_loop(dimensions, &new_obstacles)
        }).count())
    }
}
