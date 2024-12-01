use clap::{value_parser, Arg, Command};

pub fn build() -> Command {
    Command::new("advent-of-code")
        .arg(
            Arg::new("year")
                .value_parser(["2021", "2023"])
                .required(true),
        )
        .arg(
            Arg::new("day")
                .value_parser(value_parser!(u8).range(1..=25))
                .required(true),
        )
        .arg(
            Arg::new("part")
                .value_parser(value_parser!(u8).range(1..=2))
                .required(true),
        )
}
