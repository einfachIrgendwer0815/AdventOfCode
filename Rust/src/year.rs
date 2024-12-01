use std::error::Error;
use std::fmt::Display;

pub trait Year {
    fn run(&self, day: u8, part: Part) -> Result<Box<dyn Display>, Box<dyn Error>>;
}

#[derive(Debug)]
pub enum Part {
    Part1,
    Part2,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Part1 => f.write_str("1"),
            Self::Part2 => f.write_str("2"),
        }
    }
}
