pub fn run() -> u32 {
    let mut depth = Vec::<u32>::new();
    let input = include_str!("input.txt");

    for i in input.split('\n') {
        if i.is_empty() {
            continue;
        }
        depth.push(i.parse().unwrap());
    }

    count_increases(&depth)
}

fn count_increases(depths: &Vec<u32>) -> u32 {
    let mut increases = 0;
    let mut last = depths[0];

    for d in depths {
        if d > &last {
            increases += 1;
        }
        last = *d;
    }

    increases
}
