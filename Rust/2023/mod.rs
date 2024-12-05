use crate::error::NotFound;
use crate::year::{Part, Year};

pub struct Year2023;

impl Year for Year2023 {
    fn run(
        &self,
        day: u8,
        part: Part,
    ) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        Ok(match (day, part) {
            (1, Part::Part1) => Box::new(day01::part1::run()),
            (1, Part::Part2) => Box::new(day01::part2::run()),
            (2, Part::Part1) => Box::new(day02::part1::run()),
            (2, Part::Part2) => Box::new(day02::part2::run()),
            (3, Part::Part1) => Box::new(day03::part1::run()),
            (3, Part::Part2) => Box::new(day03::part2::run()),
            (4, Part::Part1) => Box::new(day04::part1::run()),
            (4, Part::Part2) => Box::new(day04::part2::run()),
            (5, Part::Part1) => Box::new(day05::part1::run()),
            _ => {
                return Err(NotFound {
                    year: 2023,
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

mod day02 {
    pub mod part1;
    pub mod part2;
}

mod day03 {
    pub mod part1;
    pub mod part2;
}

mod day04 {
    pub mod part1;
    pub mod part2;
}

mod day05 {
    pub mod part1;
}
