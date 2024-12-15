use std::isize;

use anyhow::Context;
use aoc_2024::*;
use dimensions_2::unsigned::Point;

aoc!(Day13);

#[derive(Clone, Copy, Debug)]
struct Entry {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

impl Solution<Self> for Day13 {
    type Parsed = Vec<Entry>;

    type Answer = isize;

    const SAMPLE_ANSWER_A: Self::Answer = 480;

    const SAMPLE_ANSWER_B: Self::Answer = 875318608908;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        let mut entries = Vec::new();

        for chunk in input.split("\n\n") {
            let mut lines = chunk.lines();

            let button_a_line = lines.next().unwrap();
            let button_b_line = lines.next().unwrap();
            let prize_line = lines.next().unwrap();

            let parse_button = |line: &str| -> anyhow::Result<Point> {
                let parts: Vec<&str> = line.split(['+', ':', ',']).collect();
                Ok(Point::new(
                    parts[2]
                        .trim()
                        .parse()
                        .context(format!("Could not parse button x: {}", parts[2]))?,
                    parts[4]
                        .trim()
                        .parse()
                        .context(format!("Could not parse button y: {}", parts[4]))?,
                ))
            };

            let parse_prize = |line: &str| -> anyhow::Result<Point> {
                let parts: Vec<&str> = line.split(['=', ',']).collect();
                Ok(Point::new(
                    parts[1]
                        .trim()
                        .parse()
                        .context(format!("Could not parse prize x: {}", parts[1]))?,
                    parts[3]
                        .trim()
                        .parse()
                        .context(format!("Could not parse prize y: {}", parts[3]))?,
                ))
            };

            let button_a = parse_button(button_a_line)?;
            let button_b = parse_button(button_b_line)?;
            let prize = parse_prize(prize_line)?;

            entries.push(Entry {
                button_a,
                button_b,
                prize,
            });
        }

        Ok(entries)
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(input
            .iter()
            .filter_map(|entry| solve_press_counts(entry, 0))
            .sum())
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(input
            .iter()
            .filter_map(|entry| solve_press_counts(entry, 10_000_000_000_000))
            .sum())
    }
}

fn solve_press_counts(entry: &Entry, offset: isize) -> Option<isize> {
    let (ax, ay) = (entry.button_a.get().0 as isize, entry.button_a.get().1 as isize);
    let (bx, by) = (entry.button_b.get().0 as isize, entry.button_b.get().1 as isize);
    let (px, py) = (entry.prize.get().0 as isize + offset, entry.prize.get().1 as isize + offset);

    let pb = (px * ay - py * ax) / (bx * ay - ax * by);
    let pa = (px - pb * bx) / ax;
    if pa * ax + pb * bx == px && pa * ay + pb * by == py {
        Some(pa * 3 + pb)
    } else {
        None
    }
}
