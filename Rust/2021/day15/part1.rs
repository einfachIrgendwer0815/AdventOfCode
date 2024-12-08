use pathfinding::prelude::astar;

pub fn run(input: &str) -> u32 {
    let map = parse(input);
    let height = map.len();
    let width = map[0].len();

    let goal = (width - 1, height - 1);
    let result = astar(
        &(0, 0),
        |&(x, y)| {
            let mut successors = vec![];
            if x > 0 {
                successors.push(((x - 1, y), map[y][x - 1]));
            }
            if y > 0 {
                successors.push(((x, y - 1), map[y - 1][x]));
            }
            if x < width - 1 {
                successors.push(((x + 1, y), map[y][x + 1]));
            }
            if y < height - 1 {
                successors.push(((x, y + 1), map[y + 1][x]));
            }
            successors
        },
        |&(x, y)| (goal.0.abs_diff(x) + goal.1.abs_diff(y)) as u32,
        |n| n == &goal,
    )
    .unwrap();

    result.1
}

pub(super) fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
