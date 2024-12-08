use pathfinding::prelude::astar;

use super::part1::parse;

pub fn run(input: &str) -> u32 {
    let base_map = parse(input);
    let base_height = base_map.len();
    let base_width = base_map[0].len();
    let height = 5 * base_height;
    let width = 5 * base_width;

    let goal = (width - 1, height - 1);
    let result = astar(
        &(0, 0),
        |&(x, y)| {
            let mut successor_coordinates = vec![];
            if x > 0 {
                successor_coordinates.push((x - 1, y));
            }
            if y > 0 {
                successor_coordinates.push((x, y - 1));
            }
            if x < width - 1 {
                successor_coordinates.push((x + 1, y));
            }
            if y < height - 1 {
                successor_coordinates.push((x, y + 1));
            }
            successor_coordinates
                .drain(..)
                .map(|(x, y)| ((x, y), calc_risk(&base_map, x, y, base_width, base_height)))
                .collect::<Vec<_>>()
        },
        |&(x, y)| (goal.0.abs_diff(x) + goal.1.abs_diff(y)) as u32,
        |n| n == &goal,
    )
    .unwrap();

    result.1
}

fn calc_risk(map: &[Vec<u32>], x: usize, y: usize, base_width: usize, base_height: usize) -> u32 {
    let base_x = x % base_width;
    let base_y = y % base_height;
    let map_part = x / base_width + y / base_height;

    let mut risk = map[base_y][base_x];
    risk += map_part as u32;

    while risk > 9 {
        risk -= 9;
    }
    risk
}
