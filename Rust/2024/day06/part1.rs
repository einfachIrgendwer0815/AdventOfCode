pub fn run(input: &str) -> usize {
    let (mut field, mut guard_pos) = parse(input);
    let mut guard_direction = GuardDirection::Up;

    let field_height = field.len();
    let field_width = field[0].len();

    loop {
        field[guard_pos.1][guard_pos.0] = Field::Visited;

        let Some(next_pos) = calc_next_pos(guard_pos, guard_direction, field_height, field_width)
        else {
            break;
        };

        if field[next_pos.1][next_pos.0] == Field::Barrier {
            guard_direction = guard_direction.turn_right();
        } else {
            guard_pos = next_pos;
        }
    }

    field
        .iter()
        .flat_map(|l| l.iter())
        .filter(|f| **f == Field::Visited)
        .count()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Field {
    Unvisited,
    Visited,
    Barrier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

impl GuardDirection {
    pub fn turn_right(&self) -> Self {
        match *self {
            GuardDirection::Up => GuardDirection::Right,
            GuardDirection::Right => GuardDirection::Down,
            GuardDirection::Down => GuardDirection::Left,
            GuardDirection::Left => GuardDirection::Up,
        }
    }
}

pub(super) fn calc_next_pos(
    pos: (usize, usize),
    direction: GuardDirection,
    field_height: usize,
    field_width: usize,
) -> Option<(usize, usize)> {
    let new_x = match direction {
        GuardDirection::Left => {
            if pos.0 == 0 {
                return None;
            } else {
                pos.0 - 1
            }
        }
        GuardDirection::Right => {
            if pos.0 >= field_width - 1 {
                return None;
            } else {
                pos.0 + 1
            }
        }
        _ => pos.0,
    };

    let new_y = match direction {
        GuardDirection::Up => {
            if pos.1 == 0 {
                return None;
            } else {
                pos.1 - 1
            }
        }
        GuardDirection::Down => {
            if pos.1 >= field_height - 1 {
                return None;
            } else {
                pos.1 + 1
            }
        }
        _ => pos.1,
    };

    Some((new_x, new_y))
}

pub(super) fn parse(input: &str) -> (Vec<Vec<Field>>, (usize, usize)) {
    let mut guard_pos = (0, 0);
    let field = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Field::Unvisited,
                    '#' => Field::Barrier,
                    '^' => {
                        guard_pos = (x, y);
                        Field::Unvisited
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (field, guard_pos)
}
