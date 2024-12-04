pub fn run(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    let line_length = lines[0].chars().count();

    let mut count = 0;

    for line_idx in 0..lines.len().saturating_sub(2) {
        for char_idx in 0..line_length.saturating_sub(2) {
            let middle = lines[line_idx + 1].chars().nth(char_idx + 1).unwrap();
            if middle != 'A' {
                continue;
            }

            let (top_left, top_right) = {
                let mut chars = lines[line_idx].chars().skip(char_idx);
                (chars.next().unwrap(), chars.nth(1).unwrap())
            };
            let (bottom_left, bottom_right) = {
                let mut chars = lines[line_idx + 2].chars().skip(char_idx);
                (chars.next().unwrap(), chars.nth(1).unwrap())
            };

            if !((top_left == 'M' && bottom_right == 'S')
                || (top_left == 'S' && bottom_right == 'M'))
            {
                continue;
            }

            if !((top_right == 'M' && bottom_left == 'S')
                || (top_right == 'S' && bottom_left == 'M'))
            {
                continue;
            }

            count += 1;
        }
    }

    count
}
