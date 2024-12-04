use regex::Regex;

pub fn run(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("compiling regex failed");

    input
        .lines()
        .flat_map(|l| re.captures_iter(l))
        .map(|c| {
            let a = c
                .get(1)
                .map(|a| a.as_str().parse::<u32>().unwrap())
                .unwrap_or_default();
            let b = c
                .get(2)
                .map(|b| b.as_str().parse::<u32>().unwrap())
                .unwrap_or_default();

            a * b
        })
        .sum()
}
