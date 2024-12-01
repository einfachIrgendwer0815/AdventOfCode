use std::error::Error;
use std::fmt::Display;

use crate::year::Part;

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
