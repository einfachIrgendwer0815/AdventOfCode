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

pub(super) fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|l| {
            let split = l.split(":").take(2).collect::<Vec<_>>();
            let target = split[0].parse::<u64>().unwrap();
            let numbers = split[1]
                .trim()
                .split(' ')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            (target, numbers)
        })
        .collect::<Vec<_>>()
}

fn check_equation(target: u64, numbers: &[u64], acc: u64) -> bool {
    if numbers.len() == 1 {
        return acc + numbers[0] == target || acc * numbers[0] == target;
    }

    check_equation(target, &numbers[1..], acc + numbers[0])
        || check_equation(target, &numbers[1..], acc * numbers[0])
}
