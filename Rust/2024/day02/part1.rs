pub fn run(input: &str) -> u32 {
    let reports = parse_reports(input);
    reports.iter().filter(|report| check_report(report)).count() as u32
}

#[derive(PartialEq, Eq)]
enum Direction {
    Increasing,
    Decreasing,
}

pub(super) fn check_report(report: &[u32]) -> bool {
    let mut direction = None;
    let mut previous = report[0];
    for i in report.iter().skip(1).cloned() {
        if i == previous || (i as i32 - previous as i32).abs() > 3 {
            return false;
        }
        match &direction {
            Some(dir) => {
                if (dir == &Direction::Increasing && i < previous)
                    || (dir == &Direction::Decreasing && i > previous)
                {
                    return false;
                }
            }
            None => {
                direction = if i > previous {
                    Some(Direction::Increasing)
                } else {
                    Some(Direction::Decreasing)
                }
            }
        }

        previous = i;
    }
    true
}

pub(super) fn parse_reports(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .split(' ')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
