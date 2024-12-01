pub fn run() -> u32 {
    let input = include_str!("../../inputs/input_2023_2.txt");
    possible_games(input.lines())
}

fn possible_games<'a, I: Iterator<Item = &'a str>>(lines: I) -> u32 {
    let mut sum = 0;

    for line in lines {
        let mut chars = line.chars();
        let id = parse_id(&mut chars);

        let mut chars = chars.skip(1);
        let mut possible = true;
        'set_iter: while let Some(set) = parse_set(&mut chars) {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                possible = false;
                break 'set_iter;
            }
        }

        if possible {
            sum += id;
        }
    }

    sum
}

fn parse_id<I: Iterator<Item = char>>(line: &mut I) -> u32 {
    let mut id_chars = String::new();
    for c in line.skip(5) {
        if c == ':' {
            break;
        }

        id_chars.push(c);
    }

    id_chars.parse::<u32>().unwrap()
}

pub(super) fn parse_set<I: Iterator<Item = char>>(line: &mut I) -> Option<CubeSet> {
    let mut set_text = String::new();

    for c in line {
        if c == ';' {
            break;
        }

        set_text.push(c);
    }

    if set_text.is_empty() {
        return None;
    }

    let mut set = CubeSet::default();
    for cube in set_text.split(", ") {
        let cube_info = cube.split_whitespace().collect::<Vec<_>>();

        if cube.len() < 2 {
            continue;
        }

        let num = cube_info.first().unwrap().parse::<u32>().unwrap();
        let cube_color = cube_info.get(1).unwrap();

        match *cube_color {
            "red" => set.red = num,
            "green" => set.green = num,
            "blue" => set.blue = num,
            _ => unreachable!(),
        }
    }

    Some(set)
}

#[derive(Debug, Default)]
pub(super) struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}
