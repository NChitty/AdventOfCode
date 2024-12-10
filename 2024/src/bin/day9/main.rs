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

    if let Some(&max_file_id) = blocks.iter().flatten().max() {
        for file_id in (0..=max_file_id).rev() {
            if let Some(start_pos) = blocks.iter().position(|val| *val == Some(file_id)) {
                let file_size = counts.get(&file_id).expect("No file with id.");

                if let Some(start) = find_free_space_span(&blocks, *file_size, 0, start_pos) {
                    for (i, pos) in (start_pos..start_pos+file_size).enumerate() {
                        blocks[start + i] = blocks[pos].take();
                    }
                }
            }
        }
    }

    blocks
}

fn find_free_space_span(
    blocks: &[Option<usize>],
    size: usize,
    start: usize,
    end: usize,
) -> Option<usize> {
    let mut current_size = 0;
    let mut span_start = None;

    for i in start..end {
        if blocks[i].is_none() {
            if current_size == 0 {
                span_start = Some(i);
            }
            current_size += 1;
            if current_size == size {
                return span_start;
            }
        } else {
            current_size = 0;
            span_start = None;
        }
    }

    None
}

fn print_file_space(vec: &[Option<usize>]) {
    vec.iter().for_each(|thing| match thing {
        Some(x) => print!("{}", x),
        None => print!("."),
    });
}
