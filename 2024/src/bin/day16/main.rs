use std::{
    collections::{HashSet, VecDeque},
    usize,
};

use aoc_2024::*;
use dimensions_2::{unsigned::Point, Direction};
use itertools::Itertools;

aoc!(Day16);

#[derive(Clone, Debug)]
struct Maze {
    start: Point,
    end: Point,
    maze: Vec<Vec<char>>,
}

impl Solution<Self> for Day16 {
    type Parsed = Maze;

    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::Answer = 7036;

    const SAMPLE_ANSWER_B: Self::Answer = 45;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        let mut start = Point::default();
        let mut end = Point::default();
        let maze = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| match char {
                        'S' => {
                            start = Point::new(x, y);
                            'S'
                        }
                        'E' => {
                            end = Point::new(x, y);
                            'E'
                        }
                        c => c,
                    })
                    .collect_vec()
            })
            .collect_vec();
        Ok(Maze { start, end, maze })
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let distances = djikstras((input.start, Direction::Right), &input.maze);
        let (x, y) = input.end.get();
        Ok(distances[y][x])
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        todo!()
    }
}

fn djikstras(start: (Point, Direction), maze: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut dist: Vec<Vec<usize>> = vec![vec![usize::MAX; maze[0].len()]; maze.len()];
    let mut queue: VecDeque<(Point, Direction)> = VecDeque::new();
    queue.push_back(start);
    dist[start.0.get().1][start.0.get().0] = 0;

    while let Some(cur) = queue.pop_front() {
        let (x, y) = cur.0.get();
        let neighbors = [cur.1, cur.1.rotate_left(), cur.1.rotate_right()];

        for dir in neighbors {
            let new_point = cur.0 + dir.delta();
            let (nx, ny) = new_point.get();
            if maze[ny][nx] == '.' || maze[ny][nx] == 'E' {
                if dir == cur.1 {
                    if dist[ny][nx] >= dist[y][x] + 1001 {
                        dist[ny][nx] = dist[y][x] + 1;
                        queue.push_back((new_point, dir));
                    }
                } else {
                    if dist[ny][nx] >= dist[y][x] + 1 {
                        dist[ny][nx] = dist[y][x] + 1001;
                        queue.push_back((new_point, dir));
                    }
                }
            }
        }
    }

    dist
}
