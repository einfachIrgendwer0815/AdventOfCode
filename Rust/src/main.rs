mod clap_app;
mod error;
mod year;

#[path = "../2021/mod.rs"]
mod year_2021;
#[path = "../2023/mod.rs"]
mod year_2023;
#[path = "../2024/mod.rs"]
mod year_2024;

use year::{Part, Year};

fn main() {
    let app = clap_app::build().get_matches();

    let year = app.get_one::<String>("year").unwrap();
    let day = app.get_one::<u8>("day").unwrap();
    let part = match app.get_one::<u8>("part").unwrap() {
        1 => Part::Part1,
        2 => Part::Part2,
        _ => unreachable!(),
    };

    let year: Box<dyn Year> = match year.as_str() {
        "2021" => Box::new(year_2021::Year2021),
        "2023" => Box::new(year_2023::Year2023),
        "2024" => Box::new(year_2024::Year2024),
        _ => unreachable!(),
    };

    match year.run(*day, part) {
        Ok(d) => println!("{d}"),
        Err(e) => println!("{e}"),
    }
}
