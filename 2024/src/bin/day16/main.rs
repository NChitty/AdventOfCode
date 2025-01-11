use std::{
    collections::{HashMap, HashSet, VecDeque},
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
        let (distances, _prev) = djikstras((input.start, Direction::Right), &input.maze, input.end);
        let (x, y) = input.end.get();
        Ok(distances[y][x])
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let (_distances, prev) = djikstras((input.start, Direction::Right), &input.maze, input.end);

        let mut paths = HashSet::new();
        let mut queue = vec![(input.end, Direction::Up)];
        paths.insert(input.end);
        while let Some(cur) = queue.pop() {
            if let Some(prev_set) = prev.get(&cur) {
                for &(point, direction) in prev_set {
                    paths.insert(point);
                    queue.push((point, direction));
                }
            }
        }
        // print_path(&input.maze, &paths);
        Ok(paths.len())
    }
}

fn djikstras(
    start: (Point, Direction),
    maze: &Vec<Vec<char>>,
    end: Point,
) -> (
    Vec<Vec<usize>>,
    HashMap<(Point, Direction), HashSet<(Point, Direction)>>,
) {
    let mut dist: Vec<Vec<usize>> = vec![vec![usize::MAX; maze[0].len()]; maze.len()];
    let mut prev: HashMap<(Point, Direction), HashSet<(Point, Direction)>> = HashMap::new();
    let mut queue: VecDeque<(Point, Direction, usize)> = VecDeque::new();
    queue.push_back((start.0, start.1, 0));
    dist[start.0.get().1][start.0.get().0] = 0;

    while let Some(cur) = queue.pop_front() {
        let neighbors = [cur.1.rotate_right(), cur.1.rotate_left(), cur.1];

        for dir in neighbors {
            let new_point = cur.0 + dir.delta();
            let (nx, ny) = new_point.get();
            if maze[ny][nx] != '#' {
                let new_score = match dir {
                    _ if dir == cur.1 => cur.2 + 1,
                    _ => cur.2 + 1001,
                };

                if dist[end.get().1][end.get().0] < new_score && maze[ny][nx] != 'E' {
                    continue;
                }

                if new_score < dist[ny][nx] {
                    prev.entry((new_point, dir))
                        .and_modify(|set| {
                            set.clear();
                            set.insert((cur.0, cur.1));
                        })
                        .or_insert_with(|| {
                            let mut set = HashSet::new();
                            set.insert((cur.0, cur.1));
                            set
                        });
                } else {
                    prev.entry((new_point, dir)).and_modify(|set| {
                        set.insert((cur.0, cur.1));
                    });
                }

                if dist[ny][nx] >= new_score {
                    dist[ny][nx] = new_score;

                    if maze[ny][nx] == 'E' {
                        continue;
                    }

                    match dir {
                        _ if dir == cur.1 => queue.push_back((new_point, dir, new_score)),
                        _ => queue.push_back((new_point, dir, new_score)),
                    }
                }
            }
        }
    }

    (dist, prev)
}

fn print_path(maze: &Vec<Vec<char>>, paths: &HashSet<Point>) {
    for (y, row) in maze.iter().enumerate() {
        for (x, map_char) in row.iter().enumerate() {
            if paths.contains(&Point::new(x, y)) {
                print!("O");
            } else {
                print!("{}", map_char);
            }
        }
        println!();
    }
}
