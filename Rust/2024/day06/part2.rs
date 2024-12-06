use std::collections::HashSet;

use super::part1::calc_next_pos;
use super::part1::parse;
use super::part1::Field;
use super::part1::GuardDirection;

pub fn run(input: &str) -> usize {
    let (mut field, mut guard_pos) = parse(input);
    let mut guard_direction = GuardDirection::Up;

    let initial_guard_pos = guard_pos;
    let initial_guard_direction = guard_direction;

    let field_height = field.len();
    let field_width = field[0].len();

    let mut possible_barrier_positions = HashSet::with_capacity(2_000);

    loop {
        let Some(next_pos) = calc_next_pos(guard_pos, guard_direction, field_height, field_width)
        else {
            break;
        };

        if field[next_pos.1][next_pos.0] == Field::Barrier {
            guard_direction = guard_direction.turn_right();
            continue;
        }

        field[next_pos.1][next_pos.0] = Field::Barrier;

        if is_loop(
            initial_guard_pos,
            initial_guard_direction,
            &field,
            field_height,
            field_width,
        ) {
            possible_barrier_positions.insert(next_pos);
        }

        field[next_pos.1][next_pos.0] = Field::Unvisited;
        guard_pos = next_pos;
    }

    possible_barrier_positions.len()
}

fn is_loop(
    mut pos: (usize, usize),
    mut direction: GuardDirection,
    field: &[Vec<Field>],
    field_height: usize,
    field_width: usize,
) -> bool {
    let mut visited_fields = HashSet::with_capacity(8_000);
    loop {
        if visited_fields.contains(&(pos, direction)) {
            return true;
        }

        visited_fields.insert((pos, direction));

        let Some(next_pos) = calc_next_pos(pos, direction, field_height, field_width) else {
            return false;
        };

        if field[next_pos.1][next_pos.0] == Field::Barrier {
            direction = direction.turn_right();
        } else {
            pos = next_pos;
        }
    }
}
