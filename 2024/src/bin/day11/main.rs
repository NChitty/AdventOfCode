use std::collections::HashMap;

use anyhow::Context;
use aoc_2024::*;
use itertools::Itertools;

aoc!(Day11);

impl Solution<Self> for Day11 {
    type Parsed = Vec<u64>;

    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::Answer = 55312;

    const SAMPLE_ANSWER_B: Self::Answer = 0;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        input
            .split_whitespace()
            .map(|num| num.parse::<u64>().context("Could not parse int"))
            .collect()
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let mut stone_counts: HashMap<u64, usize> = input.into_iter().counts_by(|stone| stone);
        for _ in 0..25 {
            transform_stones(&mut stone_counts);
        }
        Ok(stone_counts.values().sum())
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let mut stone_counts: HashMap<u64, usize> = input.into_iter().counts_by(|stone| stone);
        for _ in 0..75 {
            transform_stones(&mut stone_counts);
        }
        Ok(stone_counts.values().sum())
    }
}

fn transform_stones(stones_counts: &mut HashMap<u64, usize>) {
    let mut new_counts = HashMap::new();

    for (&stone, &count) in stones_counts.iter() {
        if stone == 0 {
            *new_counts.entry(1).or_insert(0) += count;
        } else {
            let digits = stone.to_string();
            if digits.len() % 2 == 0 {
                let mid = digits.len() / 2;
                let left_part = digits[..mid].parse::<u64>().unwrap_or(0);
                let right_part = digits[mid..].parse::<u64>().unwrap_or(0);
                *new_counts.entry(left_part).or_insert(0) += count;
                *new_counts.entry(right_part).or_insert(0) += count;
            } else {
                *new_counts.entry(stone * 2024).or_insert(0) += count;
            }
        }
    }

    *stones_counts = new_counts;
}
