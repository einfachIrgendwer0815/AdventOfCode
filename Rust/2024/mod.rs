use crate::error::NotFound;
use crate::year::{Part, Year};

pub struct Year2024;

impl Year for Year2024 {
    fn run(
        &self,
        day: u8,
        part: Part,
    ) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        match day {
            _ => Err(NotFound {
                year: 2024,
                day,
                part,
            }
            .into()),
        }
    }
}
