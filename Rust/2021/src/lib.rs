use aoc_traits::NotFound;
use aoc_traits::Part;
use aoc_traits::Year;

mod day01;

pub struct Year2021;

impl Year for Year2021 {
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
            _ => Err(NotFound {
                year: 2021,
                day,
                part,
            }
            .into()),
        }
    }
}
