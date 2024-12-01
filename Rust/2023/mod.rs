use crate::error::NotFound;
use crate::year::{Part, Year};

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

pub struct Year2023;

impl Year for Year2023 {
    fn run(
        &self,
        day: u8,
        part: Part,
    ) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        match day {
            1 => Ok(match part {
                Part::Part1 => Box::new(day01::part1::run()),
                Part::Part2 => Box::new(day01::part2::run()),
            }),
            2 => Ok(match part {
                Part::Part1 => Box::new(day02::part1::run()),
                Part::Part2 => Box::new(day02::part2::run()),
            }),
            3 => Ok(match part {
                Part::Part1 => Box::new(day03::part1::run()),
                Part::Part2 => Box::new(day03::part2::run()),
            }),
            4 => Ok(match part {
                Part::Part1 => Box::new(day04::part1::run()),
                Part::Part2 => Box::new(day04::part2::run()),
            }),
            5 => Ok(match part {
                Part::Part1 => Box::new(day05::part1::run()),
                Part::Part2 => {
                    return Err(NotFound {
                        year: 2023,
                        day: 5,
                        part: Part::Part2,
                    }
                    .into())
                }
            }),
            _ => Err(NotFound {
                year: 2023,
                day,
                part,
            }
            .into()),
        }
    }
}
