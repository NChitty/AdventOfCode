use aoc_2024::{aoc, Solution};
use itertools::Itertools;

aoc!(Day2);

impl Solution for Day2 {
    type Parsed = Vec<Vec<u64>>;

    type Answer = u64;

    const INPUT: &'static str = include_str!("input.txt");

    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    const SAMPLE_ANSWER_A: Self::Answer = 2;

    const SAMPLE_ANSWER_B: Self::Answer = 4;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<u64>().expect("Could not parse number"))
                    .collect_vec()
            })
            .collect_vec())
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(input.iter().filter(|report| is_safe(report)).count() as u64)
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(input
            .iter()
            .filter(|report| {
                if is_safe(report) {
                    return true;
                }

                for i in 0..report.len() {
                    let mut clone = report.to_vec().clone();
                    clone.remove(i);

                    if is_safe(&clone) {
                        return true;
                    }
                }

                false
            })
            .count() as u64)
    }
}

fn is_safe(report: &[u64]) -> bool {
    let mut increasing: Option<bool> = None;
    report.windows(2).all(|window| {
        let diff = window[1].abs_diff(window[0]);
        if increasing == None {
            increasing = Some(window[0] < window[1]);
        }
        let allowed_change = match increasing {
            Some(true) => window[0] < window[1],
            Some(false) => window[0] > window[1],
            None => panic!("Set increasing flag"),
        };
        diff != 0 && diff <= 3 && allowed_change
    })
}
