use super::part1::{parse_number, Number};

struct Symbol {
    pub pos: u8,
    pub parts: Vec<u16>,
}

pub fn run() -> u32 {
    let input = include_str!("input.txt");
    gear_ratios(input.lines())
}

fn gear_ratios<'a, I: Iterator<Item = &'a str>>(lines: I) -> u32 {
    let mut symbols_old = Vec::with_capacity(8 * 128);
    let mut symbols_prev = Vec::with_capacity(8);
    let mut symbols_this = Vec::with_capacity(8);
    let mut symbols = [&mut symbols_prev, &mut symbols_this];

    let mut numbers_prev = Vec::with_capacity(8);
    let mut numbers_this = Vec::with_capacity(8);
    let mut numbers = [&mut numbers_prev, &mut numbers_this];

    for line in lines {
        let mut chars = line.chars().enumerate();

        while let Some((index, c)) = chars.next() {
            handle_chars(&mut symbols, &mut numbers, index, c, &mut chars)
        }

        symbols_old.append(symbols[0]);
        symbols[0].clear();
        symbols.swap(0, 1);

        numbers[0].clear();
        numbers.swap(0, 1);
    }

    let mut sum = 0;
    for s in symbols_old
        .iter()
        .chain(symbols[0].iter())
        .chain(symbols[1].iter())
    {
        if s.parts.len() == 2 {
            sum += *s.parts.first().unwrap() as u32 * *s.parts.last().unwrap() as u32;
        }
    }

    sum
}

fn handle_chars<I: Iterator<Item = (usize, char)>>(
    symbols: &mut [&mut Vec<Symbol>],
    numbers: &mut [&mut Vec<Number>],
    index: usize,
    c: char,
    mut chars: &mut I,
) {
    match c {
        '.' => (),
        '0'..='9' => {
            let ((start, end), num, next_c) = parse_number(&mut chars, c);
            let symbol_bounds = start.saturating_sub(1)..=end + 1;

            let (a, b) = symbols.split_at_mut(1);
            for s in a[0].iter_mut().chain(b[0].iter_mut()) {
                if symbol_bounds.contains(&s.pos) {
                    s.parts.push(num)
                }
            }

            numbers[1].push(Number {
                pos_start: start,
                pos_end: end,
                num,
            });

            if let Some((i, n)) = next_c {
                handle_chars(symbols, numbers, i, n, chars)
            }
        }
        _ => {
            if c.is_ascii_punctuation() {
                let mut sym = Symbol {
                    pos: index as u8,
                    parts: vec![],
                };

                for num in numbers[0].iter().chain(numbers[1].iter()) {
                    let symbold_bound = num.pos_start.saturating_sub(1)..=num.pos_end + 1;

                    if symbold_bound.contains(&sym.pos) {
                        sym.parts.push(num.num)
                    }
                }

                symbols[1].push(sym);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2() {
        let input = "467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
            .664.598..";

        assert_eq!(gear_ratios(input.lines()), 467835);
    }
}
