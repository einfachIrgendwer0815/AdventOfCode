use aoc_traits::NotFound;
use aoc_traits::Part;
use aoc_traits::Year;

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
            _ => Err(NotFound {
                year: 2023,
                day,
                part,
            }
            .into()),
        }
    }
}
