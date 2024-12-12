use super::part1::parse;

pub fn run(input: &str) -> u128 {
    let mut memory_map = parse(input);
    let mut idx = memory_map.len() - 1;
    idx -= 1 - (memory_map.len() % 2);

    let mut block_id = (idx / 2) as u32;
    let mut position: u32 = memory_map.iter().map(|m| *m as u32).take(idx).sum();

    let mut spaces = memory_map
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(i, item)| if i % 2 != 0 { Some(item) } else { None })
        .collect::<Vec<_>>();

    let mut spaces_used = vec![0; spaces.len()];

    let mut sum = 0u128;

    while idx > 0 {
        match find_free_space(&memory_map, &spaces, &spaces_used, idx) {
            Some((space_idx, mut space_pos)) => {
                spaces[space_idx] -= memory_map[idx];
                spaces_used[space_idx] += memory_map[idx];

                while memory_map[idx] > 0 {
                    sum += space_pos as u128 * block_id as u128;
                    memory_map[idx] -= 1;
                    space_pos += 1;
                }
            }
            None => {
                while memory_map[idx] > 0 {
                    let pos_offset = memory_map[idx] - 1;
                    sum += (position + pos_offset as u32) as u128 * block_id as u128;
                    memory_map[idx] -= 1;
                }
            }
        }

        position -= memory_map[idx - 1] as u32 + memory_map[idx - 2] as u32;
        idx -= 2;
        block_id -= 1;
    }

    sum
}

fn find_free_space(
    mmap: &[u8],
    spaces: &[u8],
    spaces_used: &[u8],
    file_idx: usize,
) -> Option<(usize, u32)> {
    let mut idx = 1;
    let mut pos = mmap[0] as u32;

    while idx < file_idx {
        let spaces_idx = idx / 2;
        if spaces[spaces_idx] >= mmap[file_idx] {
            return Some((spaces_idx, pos + spaces_used[spaces_idx] as u32));
        }

        pos += mmap[idx] as u32 + mmap[idx + 1] as u32;
        idx += 2;
    }

    None
}
