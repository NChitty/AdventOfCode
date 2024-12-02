use aoc_2024::*;
use itertools::Itertools;

aoc!(Day2);

impl Solution<Self> for Day2 {
    type Parsed = Vec<Vec<u64>>;

    type Answer = u64;

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
                let mut increasing: Option<bool> = None;
                let to_remove = report.windows(2).position(|window| {
                    let diff = window[1].abs_diff(window[0]);
                    if increasing == None {
                        increasing = Some(window[0] < window[1]);
                    }
                    let allowed_change = match increasing {
                        Some(true) => window[0] < window[1],
                        Some(false) => window[0] > window[1],
                        None => panic!("Set increasing flag"),
                    };
                    diff == 0 || diff > 3 || !allowed_change
                });
                if let Some(remove_index) = to_remove {
                    let mut clone_first = report.to_vec().clone();
                    let mut clone_second = report.to_vec().clone();
                    let mut clone_third = report.to_vec().clone();
                    clone_first.remove(remove_index.saturating_sub(1));
                    clone_second.remove(remove_index);
                    clone_third.remove(remove_index + 1);
                    return is_safe(&clone_first) || is_safe(&clone_second) || is_safe(&clone_third);
                }
                to_remove.is_none()
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
