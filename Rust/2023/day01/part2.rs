const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const DIGITS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn run() -> u32 {
    let input = include_str!("../../inputs/input_2023_1.txt");
    calibration_values_sum(input.lines())
}

fn calibration_values_sum<'a, I: Iterator<Item = &'a str>>(lines: I) -> u32 {
    let mut values = Vec::new();

    for line in lines {
        let mut line_values = Vec::<(usize, u32)>::new();

        for (word_index, w) in WORDS.iter().enumerate() {
            for (index, _) in line.match_indices(w) {
                line_values.push((index, word_index as u32 + 1))
            }
        }

        for (digit_index, c) in DIGITS.iter().enumerate() {
            for (index, _) in line.match_indices(*c) {
                line_values.push((index, digit_index as u32 + 1))
            }
        }

        line_values.sort_by(|a, b| a.0.cmp(&b.0));

        if line_values.is_empty() {
            continue;
        }

        let (_, first) = line_values.first().unwrap();
        let (_, last) = line_values.last().unwrap();

        values.push((first * 10) + last)
    }

    values.iter().sum()
}
