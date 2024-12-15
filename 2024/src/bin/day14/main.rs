use std::{collections::HashMap, usize};

use anyhow::{Context, Error};
use aoc_2024::*;
use dimensions_2::unsigned::{Dimensions, Point};
use itertools::Itertools;

aoc!(Day14);

#[derive(Clone, Copy, Debug)]
struct Robot {
    position: Point,
    velocity: (isize, isize),
}

impl Robot {
    fn move_robot(&mut self, dimensions: &Dimensions) {
        let (cur_x, cur_y) = self.position.get();
        let new_x = ((cur_x as isize + self.velocity.0).rem_euclid(dimensions.get_width() as isize))
            as usize;
        let new_y =
            ((cur_y as isize + self.velocity.1).rem_euclid(dimensions.get_len() as isize)) as usize;

        self.position = Point::new(new_x, new_y);
    }
}

fn parse_robot(input: &str) -> anyhow::Result<Robot> {
    let parts: Vec<&str> = input.split(['=', ',', ' ']).collect();
    let x = parts[1]
        .parse()
        .context(format!("Could not parse position x: {}", parts[1]))?;
    let y = parts[2]
        .parse()
        .context(format!("Could not parse position y: {}", parts[2]))?;
    let vx = parts[4]
        .parse()
        .context(format!("Could not parse velocity x: {}", parts[4]))?;
    let vy = parts[5]
        .parse()
        .context(format!("Could not parse velocity y: {}", parts[5]))?;
    Ok(Robot {
        position: Point::new(x, y),
        velocity: (vx, vy),
    })
}

impl Solution<Self> for Day14 {
    type Parsed = Vec<Robot>;

    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::Answer = 12;

    const SAMPLE_ANSWER_B: Self::Answer = 0;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        let robots: Vec<Robot> = input
            .lines()
            .map(|line| parse_robot(line))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(robots)
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let dimensions = Dimensions::new(101, 103);
        let mut robots = input.clone();
        for _ in 0..100 {
            robots
                .iter_mut()
                .for_each(|robot| robot.move_robot(&dimensions));
        }
        let quadrant_counts = count_robots_in_quadrants(&robots, &dimensions);
        Ok(quadrant_counts.iter().product())
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let dimensions = Dimensions::new(101, 103);
        let mut robots = input.clone();
        for seconds in 0..101*103 {
            robots
                .iter_mut()
                .for_each(|robot| robot.move_robot(&dimensions));
            if robots.iter().map(|robot| robot.position).all_unique() {
                return Ok(seconds);
            }
        }
        Err(Error::msg("No all unique"))
    }
}

fn count_robots_in_quadrants(robots: &[Robot], dimensions: &Dimensions) -> [usize; 4] {
    let mid_x = dimensions.get_width() / 2;
    let mid_y = dimensions.get_len() / 2;

    let mut quadrant_counts = [0; 4];

    for robot in robots {
        let (x, y) = robot.position.get();
        if x > mid_x && y > mid_y {
            quadrant_counts[0] += 1;
        } else if x < mid_x && y > mid_y {
            quadrant_counts[1] += 1;
        } else if x < mid_x && y < mid_y {
            quadrant_counts[2] += 1;
        } else if x > mid_x && y < mid_y {
            quadrant_counts[3] += 1;
        }
    }

    quadrant_counts
}

fn print_quadrants(robots: &[Robot], dimensions: &Dimensions) {
    let (width, len) = dimensions.get();
    let position_robot_map: HashMap<Point, Vec<&Robot>> =
        robots.iter().into_group_map_by(|robot| robot.position);
    for y in 0..len {
        if y == len / 2 {
            println!();
            continue;
        }
        for x in 0..width {
            if x ==  width / 2 {
                print!(" ");
                continue;
            }
            if position_robot_map.contains_key(&Point::new(x, y)) {
                print!("#");
                continue;
            }
            print!(".");
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use aoc_2024::dimensions_2::unsigned::{Dimensions, Point};

    use crate::Robot;

    #[test]
    fn test_move_robot_wrap() {
        let dimensions = Dimensions::new(11, 7);
        let mut robot = Robot { position: Point::new(8, 2), velocity: (2, -3) };
        let expected = Point::new(10, 6);
        robot.move_robot(&dimensions);
        assert_eq!(expected, robot.position);
    }
}
