use std::collections::HashMap;

use aoc_2024::*;
use itertools::Itertools;

aoc!(Day5);

impl Solution<Self> for Day5 {
    type Parsed = (Vec<(u64, u64)>, Vec<Vec<u64>>);

    type Answer = u64;

    const SAMPLE_ANSWER_A: Self::Answer = 143;

    const SAMPLE_ANSWER_B: Self::Answer = 123;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        let page_order: Vec<(u64, u64)> = input
            .lines()
            .take_while(|&line| line != "")
            .map(|line| {
                line.split("|")
                    .filter_map(|num| num.parse::<u64>().ok())
                    .collect_tuple()
                    .expect("Could not parse tuple")
            })
            .collect_vec();
        let updates: Vec<Vec<u64>> = input
            .lines()
            .skip_while(|&line| line != "")
            .into_iter()
            .map(|update| {
                update
                    .split(",")
                    .filter_map(|num| num.parse::<u64>().ok())
                    .collect()
            })
            .filter(|vec: &Vec<u64>| !vec.is_empty())
            .collect();
        Ok((page_order, updates))
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let (page_order, updates) = input;
        Ok(updates
            .iter()
            .filter(|update| is_correctly_ordered(update, &page_order))
            .map(|update| get_middle(update))
            .sum())
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let (page_order, updates) = input;
        let lookup = page_order.clone().into_iter().into_group_map();
        Ok(updates
            .iter()
            .filter(|update| !is_correctly_ordered(update, &page_order))
            .map(|update| fix(update, &lookup))
            .map(|update| get_middle(&update))
            .sum())
    }
}

fn is_correctly_ordered(update: &[u64], orders: &[(u64, u64)]) -> bool {
    let index_num_map: HashMap<u64, usize> = update
        .iter()
        .enumerate()
        .map(|(idx, val)| (*val, idx))
        .collect();

    orders.iter().all(|(first, second)| {
        let (first_idx, second_idx) = (index_num_map.get(first), index_num_map.get(second));
        match (first_idx, second_idx) {
            (None, None) => true,
            (None, Some(_)) => true,
            (Some(_), None) => true,
            (Some(first), Some(second)) => first < second,
        }
    })
}

fn get_middle(update: &[u64]) -> u64 {
    update[update.len() / 2]
}

fn fix(update: &[u64], lookup: &HashMap<u64, Vec<u64>>) -> Vec<u64> {
    update
        .to_vec()
        .into_iter()
        .sorted_unstable_by(|a, b| ordering(a, b, lookup))
        .collect_vec()
}

fn ordering(a: &u64, b: &u64, lookup: &HashMap<u64, Vec<u64>>) -> std::cmp::Ordering {
    if let Some(before) = lookup.get(a) {
        return match before.contains(b) {
            true => std::cmp::Ordering::Less,
            false => std::cmp::Ordering::Equal,
        };
    }
    std::cmp::Ordering::Equal
}
