pub(super) const WIDTH: i32 = 101;
pub(super) const HEIGHT: i32 = 103;

pub fn run(input: &str) -> u32 {
    let robots = parse(input);
    let h_middle = (WIDTH - 1) / 2;
    let v_middle = (HEIGHT - 1) / 2;

    let mut quadrants = [0; 4];

    for robot in &robots {
        let final_x = robot.position.0 + 100 * robot.velocity.0;
        let final_y = robot.position.1 + 100 * robot.velocity.1;

        let wrapped_x = final_x.rem_euclid(WIDTH);
        let wrapped_y = final_y.rem_euclid(HEIGHT);
        let mut quadrant = 0;

        if wrapped_x == h_middle || wrapped_y == v_middle {
            continue;
        }
        if wrapped_x > h_middle {
            quadrant += 1;
        }
        if wrapped_y > v_middle {
            quadrant += 2;
        }

        quadrants[quadrant] += 1;
    }

    quadrants.iter().product()
}

#[derive(Debug)]
pub(super) struct Robot {
    pub position: (i32, i32),
    pub velocity: (i32, i32),
}

pub(super) fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|l| {
            let mut parts = l
                .split(' ')
                .flat_map(|i| i[2..].split(','))
                .map(|i| i.parse().unwrap());

            let (x, y) = (parts.next().unwrap(), parts.next().unwrap());
            let (dx, dy) = (parts.next().unwrap(), parts.next().unwrap());

            Robot {
                position: (x, y),
                velocity: (dx, dy),
            }
        })
        .collect::<Vec<_>>()
}
