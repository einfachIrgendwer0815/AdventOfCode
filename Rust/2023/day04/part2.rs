use crate::year_2023::day04::part1::parse_nums;

pub fn run() -> u32 {
    let input = include_str!("input.txt");
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(scratchcard_stack(input.lines()), 30)
    }
}
