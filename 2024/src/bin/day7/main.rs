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
            .filter(|(target, operators)| recursive(Some(*target), operators, operators.len() - 1))
            .map(|(target, _)| target)
            .sum())
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(input
            .iter()
            .filter(|(target, operators)| {
                recursive_with_concat(Some(*target), operators, operators.len() - 1)
            })
            .map(|(target, _)| target)
            .sum())
    }
}

fn recursive(target: Option<usize>, numbers: &[usize], index: usize) -> bool {
    let Some(target) = target else {
        return false;
    };
    if index == 0 {
        return numbers[0] == target;
    }

    let operand = numbers[index];
    let mut is_solved = false;
    if target % operand == 0 {
        is_solved = recursive(Some(target / operand), numbers, index - 1);
    }

    return is_solved || recursive(target.checked_sub(operand), numbers, index - 1);
}

fn recursive_with_concat(target: Option<usize>, numbers: &[usize], index: usize) -> bool {
    let Some(target) = target else {
        return false;
    };

    if index == 0 {
        return numbers[0] == target;
    }

    let operand = numbers[index];
    let mut is_solved = false;
    let operand_len = operand
        .checked_ilog10()
        .unwrap_or(0) + 1;
    let target_len = target
        .checked_ilog10()
        .unwrap_or(0) + 1;
    if target_len > 1 && get_last_digits(target, operand_len) == operand {
        let deconcat = get_first_digits(target, operand_len);
        is_solved =
            recursive_with_concat(Some(deconcat), numbers, index - 1);
    }

    if target % operand == 0 {
        is_solved = is_solved || recursive_with_concat(Some(target / operand), numbers, index - 1);
    }


    return is_solved || recursive_with_concat(target.checked_sub(operand), numbers, index - 1);
}

fn get_first_digits(num: usize, num_digits: u32) -> usize {
    num / 10usize.pow(num_digits)
}

fn get_last_digits(num: usize, num_digits: u32) -> usize {
    num % 10usize.pow(num_digits)
}
