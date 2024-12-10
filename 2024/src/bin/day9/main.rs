use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use aoc_2024::*;
use itertools::Itertools;

aoc!(Day9);

impl Solution<Self> for Day9 {
    type Parsed = Vec<Option<usize>>;

    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::Answer = 1928;

    const SAMPLE_ANSWER_B: Self::Answer = 2858;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        Ok(input
            .chars()
            .enumerate()
            .filter(|(_, char)| char.is_numeric())
            .flat_map(|(i, char)| {
                let mut blocks = Vec::new();
                let len = char.to_digit(10).expect("Could not convert char to digit");
                if i % 2 == 1 {
                    for _ in 0..len {
                        blocks.push(None);
                    }
                } else {
                    let file_num = i / 2;
                    for _ in 0..len {
                        blocks.push(Some(file_num));
                    }
                }
                blocks.into_iter()
            })
            .collect_vec())
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let mut new_vec = input.clone();
        move_somes_to_start(&mut new_vec);
        Ok(new_vec
            .iter()
            .enumerate()
            .filter(|(_, option)| option.is_some())
            .map(|(i, option)| option.unwrap() * i)
            .sum())
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let result = move_files_to_left(input);
        Ok(result
            .iter()
            .enumerate()
            .filter(|(_, option)| option.is_some())
            .map(|(i, option)| option.unwrap() * i)
            .sum())
    }
}

fn move_somes_to_start(parsed: &mut Vec<Option<usize>>) {
    let mut left = 0;
    let mut right = parsed.len();

    while left < right {
        if parsed[left].is_some() {
            left += 1;
            continue;
        }

        right -= 1;
        while right > left && parsed[right].is_none() {
            right -= 1;
        }

        if left < right {
            parsed[left] = parsed[right].take();
            left += 1;
        }
    }
}

fn move_files_to_left(mut blocks: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let counts = blocks
        .iter()
        .filter(|option| option.is_some())
        .counts_by(|val| val.unwrap());
    let mut free_space_map = free_space_heapify(&blocks);

    if let Some(&max_file_id) = blocks.iter().flatten().max() {
        for file_id in (0..=max_file_id).rev() {
            // Locate the file's blocks
            if let Some(start_pos) = blocks.iter().position(|&val| val == Some(file_id)) {
                let file_size = *counts.get(&file_id).expect("No file with id.");

                // Look for the smallest free span that can fit the file
                let smallest_span = get_start_of_span(&mut free_space_map, file_size);

                if let Some((size, heap)) = smallest_span {
                    if let Some(Reverse(start)) = heap.peek() {
                        if *start >= start_pos {
                            continue;
                        }
                    }
                    if let Some(Reverse(start)) = heap.pop() {
                        for (i, pos) in (start_pos..start_pos + file_size).enumerate() {
                            blocks[start + i] = blocks[pos].take();
                        }
                        let remaining_size = size - file_size;
                        free_space_map
                            .entry(remaining_size)
                            .or_insert_with(BinaryHeap::new)
                            .push(Reverse(start + file_size));
                    }
                }
            }
        }
    }

    blocks
}

fn get_start_of_span<'a>(
    heap_map: &'a mut HashMap<usize, BinaryHeap<Reverse<usize>>>,
    span_size: usize,
) -> Option<(usize, &'a mut BinaryHeap<Reverse<usize>>)> {
    heap_map
        .iter_mut()
        .filter(|(size, _)| **size >= span_size)
        .min_by_key(|(_, heap)| {
            heap.peek().map_or(usize::MAX, |Reverse(start)| *start) // Use smallest start index
        })
        .map(|(size, heap)| (*size, heap)) // Return size and mutable heap reference
}

fn free_space_heapify(blocks: &Vec<Option<usize>>) -> HashMap<usize, BinaryHeap<Reverse<usize>>> {
    let mut free_space_map: HashMap<usize, BinaryHeap<Reverse<usize>>> = HashMap::new();

    let mut current_size = 0;
    let mut span_start = None;
    for i in 0..blocks.len() {
        if blocks[i].is_none() {
            if current_size == 0 {
                span_start = Some(i);
            }
            current_size += 1;
        } else if blocks[i].is_some() && span_start.is_some() {
            free_space_map
                .entry(current_size)
                .or_insert_with(BinaryHeap::new)
                .push(Reverse(span_start.unwrap()));
            current_size = 0;
            span_start = None;
        }
    }

    if let Some(span_start) = span_start {
        free_space_map
            .entry(current_size)
            .and_modify(|heap| heap.push(Reverse(span_start)))
            .or_insert_with(|| {
                let mut heap = BinaryHeap::new();
                heap.push(Reverse(span_start));
                heap
            });
    }

    free_space_map
}

fn print_file_space(vec: &[Option<usize>]) {
    vec.iter().for_each(|thing| match thing {
        Some(x) => print!("{}", x),
        None => print!("."),
    });
}
