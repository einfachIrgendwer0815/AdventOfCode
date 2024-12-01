use super::part1::{parse_set, CubeSet};

pub fn run() -> u32 {
    let input = include_str!("../../inputs/input_2023_2.txt");
    minimal_number_of_cubes(input.lines())
}

fn minimal_number_of_cubes<'a, I: Iterator<Item = &'a str>>(lines: I) -> u32 {
    let mut power_sum = 0;

    for line in lines {
        let mut before_colon = true;
        let mut chars = line
            .chars()
            .filter(|c| {
                if c == &':' {
                    before_colon = false;
                    false
                } else {
                    !before_colon
                }
            })
            .skip(1);

        let mut minimal_set = CubeSet::default();
        while let Some(set) = parse_set(&mut chars) {
            if set.red > minimal_set.red {
                minimal_set.red = set.red
            }

            if set.green > minimal_set.green {
                minimal_set.green = set.green
            }

            if set.blue > minimal_set.blue {
                minimal_set.blue = set.blue
            }
        }

        power_sum += minimal_set.red * minimal_set.green * minimal_set.blue;
    }

    power_sum
}
