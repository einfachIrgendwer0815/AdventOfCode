use aoc_traits::NotFound;
use aoc_traits::Part;
use aoc_traits::Year;

pub struct Year2023;

impl Year for Year2023 {
    fn run(
        &self,
        day: u8,
        part: Part,
    ) -> Result<Box<dyn std::fmt::Display>, Box<dyn std::error::Error>> {
        Err(NotFound {
            year: 2023,
            day,
            part,
        }
        .into())
    }
}
