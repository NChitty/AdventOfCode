use aoc_2024::*;
use regex::Regex;

aoc!(Day3);

#[derive(Debug, Clone)]
struct Mul {
    is_do: bool,
    a: u64,
    b: u64,
}

impl Mul {
    fn product(&self) -> u64 {
        self.a * self.b
    }
}

impl Solution<Self> for Day3 {
    type Parsed = Vec<Mul>;

    type Answer = u64;

    const SAMPLE_ANSWER_A: Self::Answer = 161;

    const SAMPLE_ANSWER_B: Self::Answer = 48;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed> {
        let re = Regex::new(r"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))")?;
        let mut is_do = true;
        let mut muls = Vec::new();
        let captures = re.captures_iter(input);
        for capture in captures {
            if let Some(matched) = capture.get(0) {
                match matched.as_str() {
                    "do()" => is_do = true,
                    "don't()" => is_do = false,
                    _ => {
                        let mul = Mul {
                            is_do,
                            a: capture
                                .get(2)
                                .map(|str| {
                                    str.as_str().parse::<u64>().expect("Could not parse int")
                                })
                                .expect("Could not parse"),
                            b: capture
                                .get(3)
                                .map(|str| {
                                    str.as_str().parse::<u64>().expect("Could not parse int")
                                })
                                .expect("Could not parse"),
                        };
                        muls.push(mul);
                    }
                }
            }
        }
        Ok(muls)
    }

    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(input.iter().map(Mul::product).sum())
    }

    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(input.iter().filter(|mul| mul.is_do).map(Mul::product).sum())
    }
}
