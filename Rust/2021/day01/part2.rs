use std::collections::VecDeque;

struct Group {
    first: Option<u32>,
    second: Option<u32>,
    third: Option<u32>,
    count: u8,
}

impl Group {
    pub fn new() -> Group {
        Group {
            first: None,
            second: None,
            third: None,
            count: 0,
        }
    }

    pub fn push(&mut self, value: u32) -> Result<(), &'static str> {
        match self.count {
            0u8 => {
                self.first = Some(value);
            }
            1u8 => {
                self.second = Some(value);
            }
            2u8 => {
                self.third = Some(value);
            }
            _ => {
                return Err("Group full");
            }
        }

        self.count += 1u8;

        Ok(())
    }

    pub fn is_full(&self) -> bool {
        self.count == 3
    }

    pub fn unwrap(self) -> Result<(u32, u32, u32), &'static str> {
        if !self.is_full() {
            Err("Group not full")
        } else {
            Ok((
                self.first.unwrap(),
                self.second.unwrap(),
                self.third.unwrap(),
            ))
        }
    }
}

pub fn run() -> u32 {
    let mut depth = Vec::<u32>::new();
    let input = include_str!("../../inputs/input_2021_1.txt");

    for i in input.split('\n') {
        if i.is_empty() {
            continue;
        }
        depth.push(i.parse().unwrap());
    }

    count_group_increases(&depth)
}

fn count_group_increases(depths: &Vec<u32>) -> u32 {
    let mut is_first = true;
    let mut increases = 0;
    let mut last = 0;
    let mut groups: VecDeque<Group> = VecDeque::new();

    for d in depths {
        groups.push_back(Group::new());

        for g in groups.iter_mut() {
            g.push(*d).unwrap();
        }

        if groups.front().unwrap().is_full() {
            let group = groups.pop_front().unwrap();

            let (a, b, c) = group.unwrap().unwrap();
            let sum = a + b + c;

            if sum > last {
                if is_first {
                    is_first = false;
                } else {
                    increases += 1;
                }
            }
            last = sum;
        }
    }

    increases
}
