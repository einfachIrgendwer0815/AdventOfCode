mod clap_app;

use aoc_traits::Part;
use aoc_traits::Year;

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
        _ => unreachable!(),
    };

    match year.run(*day, part) {
        Ok(d) => println!("{d}"),
        Err(e) => println!("{e}"),
    }
}
