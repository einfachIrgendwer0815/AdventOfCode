use aho_corasick::AhoCorasick;

pub fn run(input: &str) -> u32 {
    let searcher =
        AhoCorasick::new(["XMAS", "SAMX"]).expect("unable to compile Aho-Corasick automaton");

    let number_lines = input.lines().count();
    let line_length = input
        .lines()
        .next()
        .map(|l| l.chars().count())
        .unwrap_or_default();

    let horizontal: u32 = input
        .lines()
        .map(|l| searcher.find_overlapping_iter(l).count() as u32)
        .sum();

    let vertical: u32 = (0..line_length)
        .map(|idx| {
            input
                .lines()
                .map(|l| l.chars().nth(idx).unwrap())
                .collect::<String>()
        })
        .map(|l| searcher.find_overlapping_iter(&l).count() as u32)
        .sum();

    let diagonal_top_left_bottom_right: u32 =
        // diagonals starting at the first character of each line
        (0..number_lines).map(|idx| {
            input
                .lines()
                .skip(idx)
                .take(line_length)
                .enumerate()
                .map(|(idx_in_diagonal, line)| line.chars().nth(idx_in_diagonal).unwrap())
                .collect::<String>()
        })
        .chain(
            // diagonals starting at characters 1 to `line_length - 1` of the first line
            (1..line_length).map(|idx| {
                input
                    .lines()
                    .take(line_length - idx)
                    .enumerate()
                    .map(|(idx_in_diagonal, line)| line.chars().nth(idx + idx_in_diagonal).unwrap())
                    .collect::<String>()
            })
        )
        .map(|l| searcher.find_overlapping_iter(&l).count() as u32)
        .sum();

    let diagonal_top_right_bottom_left: u32 =
        // diagonals starting at the last character of each line
        (0..number_lines).map(|idx| {
            input
                .lines()
                .skip(idx)
                .take(line_length)
                .enumerate()
                .map(|(idx_in_diagonal, line)| line.chars().rev().nth(idx_in_diagonal).unwrap())
                .collect::<String>()
        })
        .chain(
            // diagonals starting at characters `line_length - 1` to 1 of the first line
            (1..line_length).map(|idx| {
                input
                    .lines()
                    .take(line_length - idx)
                    .enumerate()
                    .map(|(idx_in_diagonal, line)| {
                        line.chars().rev().nth(idx + idx_in_diagonal).unwrap()
                    })
                    .collect::<String>()
            })
        )
        .map(|l| searcher.find_overlapping_iter(&l).count() as u32)
        .sum();

    horizontal + vertical + diagonal_top_left_bottom_right + diagonal_top_right_bottom_left
}
