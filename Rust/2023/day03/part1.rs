pub fn run() -> u32 {
    let input = include_str!("../../inputs/input_2023_3.txt");
    missing_part(input.lines())
}

pub(super) type Symbol = u8;

pub(super) struct Number {
    pub pos_start: u8,
    pub pos_end: u8,
    pub num: u16,
}

fn missing_part<'a, I: Iterator<Item = &'a str>>(lines: I) -> u32 {
    let mut symbols_prev = Vec::with_capacity(8);
    let mut symbols_this = Vec::with_capacity(8);
    let mut symbols = [&mut symbols_prev, &mut symbols_this];

    let mut numbers_prev = Vec::with_capacity(8);
    let mut numbers_this = Vec::with_capacity(8);
    let mut numbers = [&mut numbers_prev, &mut numbers_this];

    let mut sum = 0;

    for line in lines {
        let mut chars = line.chars().enumerate();

        while let Some((index, c)) = chars.next() {
            handle_chars(&mut symbols, &mut numbers, index, c, &mut chars, &mut sum)
        }

        symbols[0].clear();
        symbols.swap(0, 1);

        numbers[0].clear();
        numbers.swap(0, 1);
    }

    sum
}

fn handle_chars<I: Iterator<Item = (usize, char)>>(
    symbols: &mut [&mut Vec<Symbol>],
    numbers: &mut [&mut Vec<Number>],
    index: usize,
    c: char,
    mut chars: &mut I,
    sum: &mut u32,
) {
    match c {
        '.' => (),
        '0'..='9' => {
            let ((start, end), num, next_c) = parse_number(&mut chars, c);
            let symbol_bounds = start.saturating_sub(1)..=end + 1;

            let mut found = false;
            for s in symbols[0].iter().chain(symbols[1].iter()) {
                if symbol_bounds.contains(s) {
                    *sum += num as u32;
                    found = true;
                    break;
                }
            }

            if !found {
                numbers[1].push(Number {
                    pos_start: start,
                    pos_end: end,
                    num,
                })
            }

            if let Some((i, n)) = next_c {
                handle_chars(symbols, numbers, i, n, chars, sum)
            }
        }
        _ => {
            if c.is_ascii_punctuation() {
                symbols[1].push(index as u8);

                let mut to_remove = vec![];
                for (i, n) in numbers[0].iter().enumerate() {
                    let bounds = n.pos_start.saturating_sub(1)..=n.pos_end + 1;

                    if bounds.contains(&(index as u8)) {
                        to_remove.push(i);
                    }
                }
                for (c, i) in to_remove.into_iter().enumerate() {
                    let num = numbers[0].remove(i - c).num;
                    *sum += num as u32;
                }

                let mut to_remove = vec![];
                for (i, n) in numbers[1].iter().enumerate() {
                    let bounds = n.pos_start.saturating_sub(1)..=n.pos_end + 1;

                    if bounds.contains(&(index as u8)) {
                        to_remove.push(i);
                    }
                }
                for (c, i) in to_remove.into_iter().enumerate() {
                    let num = numbers[1].remove(i - c).num;
                    *sum += num as u32;
                }
            }
        }
    }
}

pub(super) fn parse_number<I: Iterator<Item = (usize, char)>>(
    chars: &mut I,
    start: char,
) -> ((u8, u8), u16, Option<(usize, char)>) {
    let mut num_chars = String::new();
    num_chars.push(start);

    let mut start = None;
    for (t, c) in chars {
        if let '0'..='9' = c {
            num_chars.push(c);

            if start.is_none() {
                start = Some((t - 1) as u8);
            }
        } else {
            return (
                ((t - (num_chars.len())) as u8, (t - 1) as u8),
                num_chars.parse().unwrap(),
                Some((t, c)),
            );
        }
    }

    let start = start.unwrap();
    (
        (start, start + ((num_chars.len() - 1) as u8)),
        num_chars.parse().unwrap(),
        None,
    )
}
