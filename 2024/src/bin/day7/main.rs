use std::usize;

use aoc_2024::*;
use itertools::Itertools;

aoc!(Day7);

impl Solution<Self> for Day7 {
    type Parsed = Vec<(usize, Vec<usize>)>;

    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::Answer = 3749;

    const SAMPLE_ANSWER_B: Self::Answer = 11387;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                let mut split_colon = line.split(":");
                let result = split_colon
                    .next()
                    .expect("Could not split at colon")
                    .trim()
                    .parse()
                    .expect("Could not parse to usize");
                let operators = split_colon
                    .next()
                    .expect("No operators")
                    .trim()
                    .split_whitespace()
                    .map(|operators| {
                        operators
                            .parse()
                            .expect("Could not parse operator to usize")
                    })
                    .collect_vec();
                (result, operators)
            })
            .collect_vec())
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(input
            .iter()
            .filter(|(target, operators)| brute_force(*target, operators))
            .map(|(target, _)| target)
            .sum())
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(input
            .iter()
            .filter(|(target, operators)| brute_force_with_concatenation(*target, operators))
            .map(|(target, _)| target)
            .sum())
    }
}

fn brute_force(target: usize, numbers: &[usize]) -> bool {
    let num_len = numbers.len();
    if num_len < 2 {
        return false;
    }

    let num_combinations = 1 << (num_len - 1);

    for i in 0..num_combinations {
        let mut result = numbers[0];

        for (j, &num) in numbers.iter().enumerate().skip(1) {
            if (i >> (j - 1)) & 1 == 1 {
                result += num;
            } else {
                result *= num;
            }
        }

        if result == target {
            return true;
        }
    }

    false
}

fn brute_force_with_concatenation(target: usize, numbers: &[usize]) -> bool {
    let n = numbers.len();
    if n < 2 {
        return false;
    }

    let num_combinations = 3usize.pow((n - 1) as u32);

    for combination in 0..num_combinations {
        let mut result = numbers[0];
        let mut current_combination = combination;

        for i in 1..n {
            let operator = current_combination % 3;
            current_combination /= 3;

            match operator {
                0 => {
                    result += numbers[i];
                }
                1 => {
                    result *= numbers[i];
                }
                2 => {
                    if let Some(concat_result) = concatenate_numbers(result, numbers[i]) {
                        result = concat_result;
                    } else {
                        result = usize::MAX;
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }

        if result == target {
            return true;
        }
    }

    false
}

fn concatenate_numbers(a: usize, b: usize) -> Option<usize> {
    let concatenated = format!("{}{}", a, b).parse::<usize>().ok()?;
    Some(concatenated)
}
