use std::collections::HashMap;

use super::part1::build_lists;

pub fn run(input: &str) -> u32 {
    let (left_list, right_list) = build_lists(input);

    let mut count_map = HashMap::new();
    for location_id in &right_list {
        match count_map.get_mut(location_id) {
            Some(val) => *val += 1,
            None => {
                count_map.insert(*location_id, 1);
            }
        }
    }

    left_list
        .iter()
        .map(|n| n * count_map.get(n).unwrap_or(&0))
        .sum()
}
