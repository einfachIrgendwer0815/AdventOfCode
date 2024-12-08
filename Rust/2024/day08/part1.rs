use std::collections::HashSet;

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

            let cx = *ax as isize - ab_x;
            let cy = *ay as isize - ab_y;

            if !(cx < 0 || cx >= area_width as isize || cy < 0 || cy >= area_height as isize) {
                antinode_locations.insert((cx, cy));
            }
        }
    }

    antinode_locations.len()
}

pub(super) fn parse(input: &str) -> Vec<(usize, usize, char)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == '.' {
                    return None;
                }

                Some((x, y, c))
            })
        })
        .collect::<Vec<_>>()
}
