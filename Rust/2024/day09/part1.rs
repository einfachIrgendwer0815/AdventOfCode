pub fn run(input: &str) -> u128 {
    let memory_map = parse(input);
    let mut left = 0;
    let mut right = memory_map.len() - 1;
    right -= 1 - (memory_map.len() % 2);

    let mut left_block_id = 0;
    let mut right_block_id = (right / 2) as u128;
    let mut position = 0;
    let mut right_files_left = memory_map[right];

    let mut sum = 0u128;

    while left < right {
        if left % 2 == 0 {
            for _ in 0..memory_map[left] {
                sum += position * left_block_id;
                position += 1;
            }

            left += 1;
            left_block_id += 1;
        } else {
            let mut empty_space = memory_map[left];

            while empty_space > 0 && left < right {
                if right_files_left > 0 {
                    sum += position * right_block_id;
                    position += 1;
                    right_files_left -= 1;
                    empty_space -= 1;
                } else {
                    right -= 2;
                    right_block_id -= 1;
                    right_files_left = memory_map[right];
                }
            }

            left += 1;
        }
    }

    if left == right {
        while right_files_left > 0 {
            sum += position * right_block_id;
            position += 1;
            right_files_left -= 1;
        }
    }

    sum
}

pub(super) fn parse(input: &str) -> Vec<u8> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>()
}
