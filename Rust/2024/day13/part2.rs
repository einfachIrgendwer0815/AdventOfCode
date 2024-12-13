use ndarray::prelude::*;

use super::part1::{check, parse, solve};

pub fn run(input: &str) -> u64 {
    let machines = parse(input);
    let mut cost = 0;

    for mut machine in machines {
        machine.1 .0 += 10000000000000.0;
        machine.1 .1 += 10000000000000.0;

        let result: Array1<f64> = solve(machine).unwrap();
        let a = result[0].round();
        let b = result[1].round();

        if check(machine, a, b) {
            cost += a as u64 * 3;
            cost += b as u64;
        }
    }

    cost
}
