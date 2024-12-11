use std::collections::HashMap;

pub fn run(input: &str) -> u64 {
    let mut stones = collect_stones(input);

    for _ in 0..25 {
        stones = blink(stones);
    }

    stones.into_values().sum()
}

pub(super) fn collect_stones(input: &str) -> HashMap<u64, u64> {
    let mut stones = HashMap::new();
    for i in input.trim().split(' ').map(|n| n.parse::<u64>().unwrap()) {
        stones.entry(i).and_modify(|val| *val += 1).or_insert(1);
    }

    stones
}

pub(super) fn blink(mut stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::with_capacity(stones.len());

    for (key, val) in stones.drain() {
        if key == 0 {
            new_stones.entry(1).and_modify(|v| *v += val).or_insert(val);
            continue;
        }

        let mut stone_str = key.to_string();
        let len = stone_str.len();
        if len % 2 == 0 {
            let right = stone_str.split_off(len / 2).parse().unwrap();
            let left = stone_str.parse().unwrap();
            new_stones
                .entry(left)
                .and_modify(|v| *v += val)
                .or_insert(val);
            new_stones
                .entry(right)
                .and_modify(|v| *v += val)
                .or_insert(val);

            continue;
        }

        new_stones
            .entry(key * 2024)
            .and_modify(|v| *v += val)
            .or_insert(val);
    }

    new_stones
}
