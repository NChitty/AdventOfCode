use core::panic;

use aoc_2024::*;
use dimensions_2::{unsigned::Point, Direction};
use itertools::Itertools;

aoc!(Day15);

#[derive(Clone, Copy, Debug)]
enum MapObject {
    Air,
    Box,
    Robot,
    Wall,
}

use MapObject::*;

#[derive(Clone, Copy, Debug)]
enum MapObject2 {
    Base(MapObject),
    BoxLeft,
    BoxRight,
}

use MapObject2::*;

#[derive(Clone, Debug)]
struct State {
    map: Vec<Vec<MapObject>>,
    robot: Point,
    moves: Vec<Direction>,
}

impl State {
    fn update(&mut self, direction: &Direction) {
        let (nx, ny) = (self.robot + direction.delta()).get();
        let map_object = self.map[ny][nx];
        match map_object {
            Air => {
                let (x, y) = self.robot.get();
                self.map[y][x] = Air;
                self.map[ny][nx] = Robot;
                self.robot = Point::new(nx, ny);
            }
            Box => {
                let (mut cur_x, mut cur_y) = (nx, ny);
                while let Box = self.map[cur_y][cur_x] {
                    cur_x = cur_x.wrapping_add_signed(direction.delta().0);
                    cur_y = cur_y.wrapping_add_signed(direction.delta().1);
                }
                let not_box = self.map[cur_y][cur_x];
                match not_box {
                    Air => {
                        let (x, y) = self.robot.get();
                        self.map[cur_y][cur_x] = Box;
                        self.map[y][x] = Air;
                        self.map[ny][nx] = Robot;
                        self.robot = Point::new(nx, ny);
                    }
                    Box => unreachable!(),
                    Robot => panic!("There's a second robot!"),
                    Wall => (),
                }
            }
            Wall => (),
            Robot => panic!("New position is the robot."),
        }
    }

    fn print_state(&self) {
        let map = self.map.clone();
        for row in map {
            for obj in row {
                match obj {
                    Air => print!("."),
                    Box => print!("O"),
                    Robot => print!("@"),
                    Wall => print!("#"),
                }
            }
            println!();
        }
    }
}

#[derive(Clone, Debug)]
struct State2 {
    map: Vec<Vec<MapObject2>>,
    robot: Point,
    moves: Vec<Direction>,
}

impl State2 {
    fn from_part_a(state: &State) -> Self {
        let map = state
            .map
            .iter()
            .map(|row| {
                row.iter()
                    .flat_map(|obj| match obj {
                        Air | Wall => vec![Base(*obj), Base(*obj)],
                        Box => vec![BoxLeft, BoxRight],
                        Robot => vec![Base(Robot), Base(Air)],
                    })
                    .collect_vec()
            })
            .collect_vec();

        Self {
            map,
            robot: Point::new(state.robot.get().0 * 2, state.robot.get().1),
            moves: state.moves.clone(),
        }
    }

    fn update(&mut self, direction: &Direction) {
        let (nx, ny) = (self.robot + direction.delta()).get();
        let map_object = self.map[ny][nx];
        match map_object {
            Base(Air) => {
                let (x, y) = self.robot.get();
                self.map[y][x] = Base(Air);
                self.map[ny][nx] = Base(Robot);
                self.robot = Point::new(nx, ny);
            }
            BoxLeft | BoxRight => {
                let try_push = self.try_push(self.robot, direction);
                if try_push {
                    self.push(self.robot, *direction);
                    let (x, y) = self.robot.get();
                    self.map[y][x] = Base(Air);
                    self.map[ny][nx] = Base(Robot);
                    self.robot = Point::new(nx, ny);
                }
            }
            Base(Wall) => (),
            Base(Robot) => panic!("New position is the robot."),
            Base(Box) => unreachable!("Box in v2 state."),
        }
    }

    fn try_push(&self, start_pos: Point, direction: &Direction) -> bool {
        let new_point = start_pos + direction.delta();
        let (nx, ny) = new_point.get();
        let map_obj = self.map[ny][nx];
        match (map_obj, direction) {
            (Base(Air), _) => true,
            (Base(Wall), _) => false,
            (Base(Robot), _) => panic!("Second robot"),
            (Base(Box), _) => unreachable!(),
            (obj, Direction::Right) => match obj {
                BoxLeft | BoxRight => self.try_push(new_point, direction),
                _ => unreachable!(),
            },
            (obj, Direction::Left) => match obj {
                BoxLeft | BoxRight => self.try_push(new_point, direction),
                _ => unreachable!(),
            },
            (obj, Direction::Up) | (obj, Direction::Down) => match obj {
                BoxLeft => {
                    self.try_push(new_point, direction)
                        && self.try_push(new_point + (1, 0), direction)
                }
                BoxRight => {
                    self.try_push(new_point, direction)
                        && self.try_push(new_point + (-1, 0), direction)
                }
                _ => unreachable!(),
            },
        }
    }

    fn push(&mut self, start_pos: Point, direction: Direction) {
        let (sx, sy) = start_pos.get();
        let new_point = start_pos + direction.delta();
        let (nx, ny) = new_point.get();

        let map_obj = self.map[ny][nx];

        match (map_obj, direction) {
            (Base(Air), _) => {
                let temp = self.map[ny][nx];
                self.map[ny][nx] = self.map[sy][sx];
                self.map[sy][sx] = temp;
            }
            (Base(Wall), _) => (),
            (Base(_), _) => unreachable!(),
            (obj, Direction::Right) => match obj {
                BoxLeft | BoxRight => {
                    self.push(new_point, direction);
                    let temp = self.map[ny][nx];
                    self.map[ny][nx] = self.map[sy][sx];
                    self.map[sy][sx] = temp;
                }
                _ => unreachable!(),
            },
            (obj, Direction::Left) => match obj {
                BoxLeft | BoxRight => {
                    self.push(new_point, direction);
                    let temp = self.map[ny][nx];
                    self.map[ny][nx] = self.map[sy][sx];
                    self.map[sy][sx] = temp;
                }
                _ => unreachable!(),
            },
            (obj, Direction::Up) | (obj, Direction::Down) => match obj {
                BoxLeft => {
                    self.push(new_point, direction);
                    self.push(new_point + (1, 0), direction);
                    let temp1 = self.map[ny][nx + 1];
                    let temp2 = self.map[ny][nx];
                    let temp3 = self.map[sy][sx];

                    self.map[ny][nx + 1] = temp2;
                    self.map[ny][nx] = temp3;
                    self.map[sy][sx] = temp1;
                }
                BoxRight => {
                    self.push(new_point, direction);
                    self.push(new_point + (-1, 0), direction);
                    let temp1 = self.map[ny][nx - 1];
                    let temp2 = self.map[ny][nx];
                    let temp3 = self.map[sy][sx];

                    self.map[ny][nx - 1] = temp2;
                    self.map[ny][nx] = temp3;
                    self.map[sy][sx] = temp1;
                }
                _ => unreachable!(),
            },
        }
    }

    fn print_state(&self) {
        let map = self.map.clone();
        for row in map {
            for obj in row {
                match obj {
                    Base(Air) => print!("."),
                    BoxLeft => print!("["),
                    BoxRight => print!("]"),
                    Base(Robot) => print!("@"),
                    Base(Wall) => print!("#"),
                    Base(Box) => unreachable!("Boxes in state 2"),
                }
            }
            println!();
        }
    }
}

impl Solution<Self> for Day15 {
    type Parsed = State;

    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::Answer = 10092;

    const SAMPLE_ANSWER_B: Self::Answer = 9021;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        let mut robot = Point::new(0, 0);
        let map = input
            .lines()
            .enumerate()
            .take_while(|(_, line)| *line != "")
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| match char {
                        '.' => Air,
                        '@' => {
                            robot = Point::new(x, y);
                            Robot
                        }
                        'O' => Box,
                        '#' => Wall,
                        _ => panic!("Unknown object to parse"),
                    })
                    .collect_vec()
            })
            .collect_vec();
        let moves = input
            .lines()
            .skip_while(|line| *line != "")
            .skip(1)
            .flat_map(|line| {
                line.chars()
                    .map(|char| match char {
                        '>' => Direction::Right,
                        'v' => Direction::Down,
                        '<' => Direction::Left,
                        '^' => Direction::Up,
                        _ => panic!("Unknown direction to parse"),
                    })
                    .collect_vec()
                    .into_iter()
            })
            .collect_vec();
        Ok(State { map, robot, moves })
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let mut state = input.clone();

        input.moves.iter().enumerate().for_each(|(_i, direction)| state.update(direction));

        Ok(state
            .map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, obj)| match obj {
                        Air => None,
                        Box => Some(x + 100 * y),
                        Robot => None,
                        Wall => None,
                    })
                    .collect_vec()
                    .into_iter()
            })
            .sum())
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let mut state = State2::from_part_a(&input);

        input
            .moves
            .iter()
            .enumerate()
            .for_each(|(_i, direction)| state.update(direction));


        Ok(state
            .map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, obj)| match obj {
                        Base(_) => None,
                        BoxLeft => Some(x + 100 * y),
                        _ => None,
                    })
                    .collect_vec()
                    .into_iter()
            })
            .sum())
    }
}
