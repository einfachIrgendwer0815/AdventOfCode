pub fn run(input: &str) -> u32 {
    let (mut left_list, mut right_list) = build_lists(input);
    left_list.sort();
    right_list.sort();

    left_list
        .iter()
        .zip(right_list)
        .map(|(l, r)| match l.cmp(&r) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => l - r,
            std::cmp::Ordering::Less => r - l,
        })
        .sum()
}

pub(super) fn build_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .take(2)
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        left.push(nums[0]);
        right.push(nums[1]);
    }

    (left, right)
}
