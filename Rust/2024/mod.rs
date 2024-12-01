use crate::error::NotFound;
use crate::year::{Part, Year};

mod day01 {
    pub const INPUT: &str = include_str!("../inputs/input_2024_1.txt");
    pub mod part1;
    pub mod part2;
}

pub struct Year2024;

impl Year for Year2024 {
    fn run(
        &self,
        day: u8,
        part: Part,
    ) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        match day {
            1 => match part {
                Part::Part1 => Ok(Box::new(day01::part1::run(day01::INPUT))),
                Part::Part2 => Ok(Box::new(day01::part2::run(day01::INPUT))),
            },
            _ => Err(NotFound {
                year: 2024,
                day,
                part,
            }
            .into()),
        }
    }
}
