use std::fmt::{Debug, Display};

pub trait Solution {
    type Parsed: Debug + Clone;
    type Answer: Debug + Display + PartialEq;
    const INPUT: &'static str;
    const SAMPLE_INPUT: &'static str;
    const SAMPLE_ANSWER_A: &'static str;
    const SAMPLE_ANSWER_B: &'static str;

    fn parse(input: &str) -> Self::Parsed;
    fn part_a(input: Self::Parsed) -> Self::Answer;
    fn part_b(input: Self::Parsed) -> Self::Answer;
}
