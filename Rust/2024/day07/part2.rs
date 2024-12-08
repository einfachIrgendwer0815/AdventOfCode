use super::part1::parse;

pub fn run(input: &str) -> u64 {
    parse(input)
        .iter()
        .filter_map(|(target, numbers)| {
            if check_equation(*target, &numbers[1..], numbers[0]) {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

fn check_equation(target: u64, numbers: &[u64], acc: u64) -> bool {
    if numbers.len() == 1 {
        return acc + numbers[0] == target
            || acc * numbers[0] == target
            || concatenate(acc, numbers[0]) == target;
    }

    check_equation(target, &numbers[1..], acc + numbers[0])
        || check_equation(target, &numbers[1..], acc * numbers[0])
        || check_equation(target, &numbers[1..], concatenate(acc, numbers[0]))
}

fn concatenate(a: u64, b: u64) -> u64 {
    let mut c = a.to_string();
    c.push_str(&b.to_string());
    c.parse().unwrap()
}
