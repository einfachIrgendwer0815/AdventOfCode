use std::collections::HashMap;

type Update = Vec<Vec<u32>>;
type Rules = HashMap<u32, Vec<u32>>;

pub fn run(input: &str) -> u32 {
    let (updates, rules) = parse(input);
    updates
        .iter()
        .filter_map(|u| {
            if update_correctly_ordered(u, &rules) {
                Some(u[u.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

pub(super) fn update_correctly_ordered(update: &[u32], rules: &Rules) -> bool {
    let mut previous_items = &update[0..0];
    let mut remaining_items = &update[0..];

    while !remaining_items.is_empty() {
        let current_item = remaining_items[0];
        let rule_set = rules.get(&current_item).unwrap();

        if rule_set.iter().any(|r| previous_items.contains(r)) {
            return false;
        }

        previous_items = &update[0..previous_items.len() + 1];
        remaining_items = &remaining_items[1..];
    }

    true
}

pub(super) fn parse(input: &str) -> (Update, Rules) {
    let mut parts = input.split("\n\n").map(str::lines);
    let rules_parts = parts.next().unwrap().map(|l| {
        l.split('|')
            .map(|p| p.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    });

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    for i in rules_parts {
        match rules.get_mut(&i[0]) {
            Some(entry) => entry.push(i[1]),
            None => {
                rules.insert(i[0], vec![i[1]]);
            }
        }
    }

    let updates = parts
        .next()
        .unwrap()
        .map(|l| {
            l.split(',')
                .map(|p| p.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (updates, rules)
}
