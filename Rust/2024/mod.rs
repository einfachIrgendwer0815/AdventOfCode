use crate::error::NotFound;
use crate::year::{Part, Year};

pub struct Year2024;

impl Year for Year2024 {
    fn run(
        &self,
        day: u8,
        part: Part,
    ) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        Ok(match (day, part) {
            (1, Part::Part1) => Box::new(day01::part1::run(day01::INPUT)),
            (1, Part::Part2) => Box::new(day01::part2::run(day01::INPUT)),
            (2, Part::Part1) => Box::new(day02::part1::run(day02::INPUT)),
            (2, Part::Part2) => Box::new(day02::part2::run(day02::INPUT)),
            (3, Part::Part1) => Box::new(day03::part1::run(day03::INPUT)),
            (3, Part::Part2) => Box::new(day03::part2::run(day03::INPUT)),
            (4, Part::Part1) => Box::new(day04::part1::run(day04::INPUT)),
            (4, Part::Part2) => Box::new(day04::part2::run(day04::INPUT)),
            (5, Part::Part1) => Box::new(day05::part1::run(day05::INPUT)),
            (5, Part::Part2) => Box::new(day05::part2::run(day05::INPUT)),
            (6, Part::Part1) => Box::new(day06::part1::run(day06::INPUT)),
            (6, Part::Part2) => Box::new(day06::part2::run(day06::INPUT)),
            (7, Part::Part1) => Box::new(day07::part1::run(day07::INPUT)),
            (7, Part::Part2) => Box::new(day07::part2::run(day07::INPUT)),
            _ => {
                return Err(NotFound {
                    year: 2024,
                    day,
                    part,
                }
                .into())
            }
        })
    }
}

mod day01 {
    pub const INPUT: &str = include_str!("../inputs/input_2024_1.txt");
    pub mod part1;
    pub mod part2;
}

mod day02 {
    pub const INPUT: &str = include_str!("../inputs/input_2024_2.txt");
    pub mod part1;
    pub mod part2;
}

mod day03 {
    pub const INPUT: &str = include_str!("../inputs/input_2024_3.txt");
    pub mod part1;
    pub mod part2;
}

mod day04 {
    pub const INPUT: &str = include_str!("../inputs/input_2024_4.txt");
    pub mod part1;
    pub mod part2;
}

mod day05 {
    pub const INPUT: &str = include_str!("../inputs/input_2024_5.txt");
    pub mod part1;
    pub mod part2;
}

mod day06 {
    pub const INPUT: &str = include_str!("../inputs/input_2024_6.txt");
    pub mod part1;
    pub mod part2;
}

mod day07 {
    pub const INPUT: &str = include_str!("../inputs/input_2024_7.txt");
    pub mod part1;
    pub mod part2;
}
