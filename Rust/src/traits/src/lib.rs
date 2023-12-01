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

#[derive(Debug)]
pub struct NotFound {
    pub year: u32,
    pub day: u8,
    pub part: Part,
}

impl Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Year {} has no day {} part {}.",
            self.year, self.day, self.part
        ))
    }
}

impl Error for NotFound {}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Part1 => f.write_str("1"),
            Self::Part2 => f.write_str("2"),
        }
    }
}
