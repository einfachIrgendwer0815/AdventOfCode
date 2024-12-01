pub fn run() -> u32 {
    let input = include_str!("../../inputs/input_2023_4.txt");
    scratchcard_points(input.lines())
}

fn scratchcard_points<'a, I: Iterator<Item = &'a str>>(lines: I) -> u32 {
    let mut sum = 0;
    let mut winning_nums = Vec::with_capacity(10);
    let mut guessed_nums = Vec::with_capacity(10);

    for line in lines {
        let (winning, guessed) = {
            let colon_pos = line.find(':').unwrap();
            let num_table = line[colon_pos + 1..].trim();
            let bat_pos = num_table.find('|').unwrap();

            (num_table[..bat_pos].trim(), num_table[bat_pos + 1..].trim())
        };

        parse_nums(winning.split_whitespace(), &mut winning_nums);
        parse_nums(guessed.split_whitespace(), &mut guessed_nums);

        sum += calc_card_points(&winning_nums, &guessed_nums);

        winning_nums.clear();
        guessed_nums.clear();
    }

    sum
}

fn calc_card_points(winning: &[u32], guessed: &[u32]) -> u32 {
    let mut card_points = 0;
    for gn in guessed.iter() {
        if winning.contains(gn) {
            if card_points > 0 {
                card_points *= 2;
            } else {
                card_points = 1;
            }
        }
    }

    card_points
}

pub(super) fn parse_nums<'a, I: Iterator<Item = &'a str>>(nums: I, buf: &mut Vec<u32>) {
    for n in nums {
        if n.is_empty() {
            continue;
        }

        buf.push(n.parse().unwrap())
    }
}
