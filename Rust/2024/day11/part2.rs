use super::part1::blink;
use super::part1::collect_stones;

pub fn run(input: &str) -> u64 {
    let mut stones = collect_stones(input);

    for _ in 0..75 {
        stones = blink(stones);
    }

    stones.into_values().sum()
}
