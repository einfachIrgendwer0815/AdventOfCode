use std::collections::HashSet;

use super::part1::parse;

pub fn run(input: &str) -> usize {
    let area_height = input.lines().count();
    let area_width = input.lines().next().unwrap().trim().chars().count();

    let antennas = parse(input);
    let mut antinode_locations = HashSet::new();

    for (ax, ay, t1) in &antennas {
        for (bx, by, t2) in &antennas {
            if t1 != t2 || (ax == bx && ay == by) {
                continue;
            }

            let ab_x = *bx as isize - *ax as isize;
            let ab_y = *by as isize - *ay as isize;

            let mut cx = *ax as isize;
            let mut cy = *ay as isize;

            while !(cx < 0 || cx >= area_width as isize || cy < 0 || cy >= area_height as isize) {
                antinode_locations.insert((cx, cy));

                cx -= ab_x;
                cy -= ab_y;
            }
        }
    }

    antinode_locations.len()
}
