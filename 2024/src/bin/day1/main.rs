use std::collections::HashMap;

use aoc_2024::*;
use itertools::Itertools;

aoc!(Day1);

impl Solution<Day1> for Day1 {
    type Parsed = (Vec<u64>, Vec<u64>);

    type Answer = u64;

    const SAMPLE_ANSWER_A: Self::Answer = 11;

    const SAMPLE_ANSWER_B: Self::Answer = 31;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        let mut a = Vec::new();
        let mut b = Vec::new();
        input.lines().for_each(|line| {
            let mut split = line.split("   ");
            a.push(
                split
                    .next()
                    .expect("No first number")
                    .parse()
                    .expect("Could not parse"),
            );
            b.push(
                split
                    .next()
                    .expect("No second number")
                    .parse()
                    .expect("Could not parse"),
            );
        });
        Ok((a, b))
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let (a, b) = input;
        let sorted_a = a.iter().sorted();
        Ok(sorted_a
            .zip(b.iter().sorted())
            .map(|(a, b)| a.abs_diff(*b) as u64)
            .sum())
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let (a, b) = input;
        let mut occurences_map = HashMap::new();
        b.iter().for_each(|&val| {
            occurences_map
                .entry(val)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        });
        Ok(a.iter()
            .map(|&val| val * occurences_map.get(&val).unwrap_or(&0u64))
            .sum())
    }
}
