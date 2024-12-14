use std::collections::HashSet;

use super::part1::parse;
use super::part1::Robot;
use super::part1::{HEIGHT, WIDTH};

pub fn run(input: &str) -> u32 {
    let mut robots = parse(input);

    let mut i = 0;
    loop {
        i += 1;

        for robot in &mut robots {
            robot.position.0 = (robot.position.0 + robot.velocity.0).rem_euclid(WIDTH);
            robot.position.1 = (robot.position.1 + robot.velocity.1).rem_euclid(HEIGHT);
        }

        if is_tree(&robots) {
            return i;
        }
    }
}

fn is_tree(robots: &[Robot]) -> bool {
    let mut set = HashSet::new();
    robots.iter().all(|r| set.insert(r.position))
}
