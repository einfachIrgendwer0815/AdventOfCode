use ndarray::prelude::*;
use ndarray_linalg::Solve;

type Button = (f64, f64);
pub(super) type Machine = ([Button; 2], (f64, f64));

pub fn run(input: &str) -> u32 {
    let machines = parse(input);
    let mut cost = 0;

    for machine in machines {
        let result: Array1<f64> = solve(machine).unwrap();
        let a = result[0].round();
        let b = result[1].round();

        if check(machine, a, b) {
            cost += a as u32 * 3;
            cost += b as u32;
        }
    }

    cost
}

#[inline]
pub(super) fn check(machine: Machine, a: f64, b: f64) -> bool {
    a * machine.0[0].0 + b * machine.0[1].0 == machine.1 .0
        && a * machine.0[0].1 + b * machine.0[1].1 == machine.1 .1
}

pub(super) fn solve(machine: Machine) -> ndarray_linalg::error::Result<Array1<f64>> {
    let a: Array2<f64> = array![
        [machine.0[0].0, machine.0[1].0],
        [machine.0[0].1, machine.0[1].1]
    ];
    let b: Array1<f64> = array![machine.1 .0, machine.1 .1];
    a.solve_into(b)
}

pub(super) fn parse(input: &str) -> Vec<Machine> {
    let mut machines = vec![];

    let mut button_a = None;
    let mut button_b = None;
    for (idx, line) in input.lines().enumerate() {
        match idx % 4 {
            0 | 1 => {
                let mut moves = line.split(':').nth(1).unwrap().trim().split(", ");
                let x = moves
                    .next()
                    .unwrap()
                    .split('+')
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap();
                let y = moves
                    .next()
                    .unwrap()
                    .split('+')
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap();

                if idx % 4 == 0 {
                    button_a = Some((x, y));
                } else {
                    button_b = Some((x, y));
                }
            }
            2 => {
                let mut coordinates = line.split(':').nth(1).unwrap().trim().split(", ");
                let x = coordinates
                    .next()
                    .unwrap()
                    .split('=')
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap();
                let y = coordinates
                    .next()
                    .unwrap()
                    .split('=')
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap();

                let goal = (x, y);

                let btn_a = button_a.take().unwrap();
                let btn_b = button_b.take().unwrap();

                machines.push(([btn_a, btn_b], goal));
            }
            3 => {}
            _ => unreachable!(),
        }
    }

    machines
}
