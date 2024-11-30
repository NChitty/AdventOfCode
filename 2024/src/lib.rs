use std::fmt::{Debug, Display};

pub trait Solution {
    type Parsed: Debug + Clone;
    type Answer: Debug + Display + PartialEq;
    const INPUT: &'static str;
    const SAMPLE_INPUT: &'static str;
    const SAMPLE_ANSWER_A: Self::Answer;
    const SAMPLE_ANSWER_B: Self::Answer;

    fn parse(input: &str) -> anyhow::Result<Self::Parsed>;
    fn part_a(input: Self::Parsed) -> anyhow::Result<Self::Answer>;
    fn part_b(input: Self::Parsed) -> anyhow::Result<Self::Answer>;

    fn part_a_test(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Self::part_a(input)
    }

    fn part_b_test(input: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Self::part_b(input)
    }

    fn test_part_a() -> anyhow::Result<()> {
        assert_eq!(
            Self::parse(Self::SAMPLE_INPUT).and_then(Self::part_a_test)?,
            Self::SAMPLE_ANSWER_A
        );
        let shared = Self::parse(Self::INPUT)?;
        println!("a: {}", Self::part_a(shared)?);
        Ok(())
    }

    fn test_part_b() -> anyhow::Result<()> {
        assert_eq!(
            Self::parse(Self::SAMPLE_INPUT).and_then(Self::part_b_test)?,
            Self::SAMPLE_ANSWER_B
        );
        let shared = Self::parse(Self::INPUT)?;
        println!("b: {}", Self::part_b(shared)?);
        Ok(())
    }

    fn main() -> anyhow::Result<()> {
        let input = time("Parse", || Self::parse(Self::INPUT))?;
        let arg = std::env::args().nth(1);
        match arg.as_deref() {
            Some("a") => {
                let a = time("Part a", || Self::part_a(input))?;
                println!("a: {a}");
            }
            Some("b") => {
                let b = time("Part b", || Self::part_b(input))?;
                println!("b: {b}");
            }
            _ => {
                let a = time("Part a", || Self::part_a(input.clone()))?;
                let b = time("Part b", || Self::part_b(input.clone()))?;
                println!("a: {a}");
                println!("b: {b}");
            }
        }
        Ok(())
    }
}

fn time<T>(tag: &str, f: impl FnOnce() -> T) -> T {
    let start = std::time::Instant::now();
    let ans = f();
    println!("{tag} took {:?}", start.elapsed());
    ans
}

#[macro_export]
macro_rules! aoc {
    ($day:ident) => {
        struct $day;

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn a() -> anyhow::Result<()> {
                $day::test_part_a()
            }

            #[test]
            fn b() -> anyhow::Result<()> {
                $day::test_part_b()
            }
        }

        fn main() -> anyhow::Result<()> {
            $day::main()
        }
    };
}
