use crate::year_2023::day04::part1::parse_nums;

pub fn run() -> u32 {
    let input = include_str!("../../inputs/input_2023_4.txt");
    scratchcard_stack(input.lines())
}

fn scratchcard_stack<'a, I: Iterator<Item = &'a str>>(lines: I) -> u32 {
    let mut win_lookup = Vec::with_capacity(230);

    let mut winning_nums = Vec::with_capacity(10);
    let mut guessed_nums = Vec::with_capacity(10);
    for (index, line) in lines.enumerate() {
        let (winning, guessed) = {
            let colon_pos = line.find(':').unwrap();
            let num_table = line[colon_pos + 1..].trim();
            let bat_pos = num_table.find('|').unwrap();

            (num_table[..bat_pos].trim(), num_table[bat_pos + 1..].trim())
        };

        parse_nums(winning.split_whitespace(), &mut winning_nums);
        parse_nums(guessed.split_whitespace(), &mut guessed_nums);

        let correct_nums = count_correct_nums(&winning_nums, &guessed_nums) as usize;
        win_lookup.push(index + 1..=index + correct_nums);

        winning_nums.clear();
        guessed_nums.clear();
    }

    let diff_cards = win_lookup.len();
    let mut card_counts = vec![1u32; diff_cards];
    for card in 0..diff_cards {
        for c in win_lookup[card].clone() {
            card_counts[c] += card_counts[card];
        }
    }

    card_counts.iter().sum()
}

fn count_correct_nums(winning: &[u32], guessed: &[u32]) -> u32 {
    let mut correct_nums = 0;
    for gn in guessed.iter() {
        if winning.contains(gn) {
            correct_nums += 1;
        }
    }

    correct_nums
}
