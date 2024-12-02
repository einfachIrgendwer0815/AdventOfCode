use super::part1::{check_report, parse_reports};

pub fn run(input: &str) -> u32 {
    let reports = parse_reports(input);
    reports
        .iter()
        .filter(|report| {
            if !check_report(report) {
                for i in 0..report.len() {
                    let mut shortened_report = (*report).clone();
                    shortened_report.remove(i);
                    if check_report(&shortened_report) {
                        return true;
                    }
                }
                return false;
            }

            true
        })
        .count() as u32
}
