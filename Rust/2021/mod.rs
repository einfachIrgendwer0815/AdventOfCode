use crate::error::NotFound;
use crate::year::{Part, Year};

pub struct Year2021;

impl Year for Year2021 {
    fn run(
        &self,
        day: u8,
        part: Part,
    ) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        Ok(match (day, part) {
            (1, Part::Part1) => Box::new(day01::part1::run()),
            (1, Part::Part2) => Box::new(day01::part2::run()),
            (15, Part::Part1) => Box::new(day15::part1::run(day15::INPUT)),
            (15, Part::Part2) => Box::new(day15::part2::run(day15::INPUT)),
            _ => {
                return Err(NotFound {
                    year: 2021,
                    day,
                    part,
                }
                .into())
            }
        })
    }
}

mod day01 {
    pub mod part1;
    pub mod part2;
}

mod day15 {
    pub const INPUT: &str = include_str!("../inputs/input_2021_15.txt");
    pub mod part1;
    pub mod part2;
}
