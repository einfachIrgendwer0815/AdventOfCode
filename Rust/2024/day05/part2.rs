use std::cmp::Ordering;

use super::part1::parse;
use super::part1::update_correctly_ordered;

pub fn run(input: &str) -> u32 {
    let (mut updates, rules) = parse(input);

    updates
        .iter_mut()
        .filter_map(|u| {
            if !update_correctly_ordered(u, &rules) {
                u.sort_by(|a, b| {
                    if rules.get(a).unwrap().contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                Some(u[u.len() / 2])
            } else {
                None
            }
        })
        .sum()
}
