pub fn run() -> u32 {
    let input = include_str!("input.txt");
    calibration_values_sum(input.lines())
}

fn calibration_values_sum<'a, I: Iterator<Item = &'a str>>(lines: I) -> u32 {
    let mut values = Vec::new();

    for line in lines {
        let mut left = None;
        let mut right = None;

        'inner: for c in line.chars() {
            if let '0'..='9' = c {
                left = Some(c.to_digit(10).unwrap());
                break 'inner;
            }
        }

        'inner: for c in line.chars().rev() {
            if let '0'..='9' = c {
                right = Some(c.to_digit(10).unwrap());
                break 'inner;
            }
        }

        let Some(left) = left else {
            continue;
        };

        let right = match right {
            Some(r) => r,
            None => left,
        };

        values.push((left * 10) + right)
    }

    values.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let mut lines = input.split('\n');

        assert_eq!(calibration_values_sum(&mut lines), 142)
    }
}
