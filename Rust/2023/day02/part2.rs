use super::part1::{parse_set, CubeSet};

pub fn run() -> u32 {
    let input = include_str!("input.txt");
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(minimal_number_of_cubes(input.lines()), 2286)
    }
}
