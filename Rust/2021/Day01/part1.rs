use std::fs;

fn main() {
    let mut depth = Vec::<u32>::new();
    let input = fs::read_to_string("input.txt").expect("Could not read input.");

    for i in input.split("\n") {
        if i == "" { continue; }
        depth.push(i.parse().unwrap());
    }

    println!("{}", count_increases(&depth))
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
