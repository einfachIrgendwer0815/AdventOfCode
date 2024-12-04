use regex::Regex;

pub fn run(input: &str) -> u32 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)")
        .expect("compiling regex failed");

    let mut mul_enabled = true;

    input
        .lines()
        .flat_map(|l| re.captures_iter(l))
        .filter_map(|c| {
            match c.get(0).unwrap().as_str() {
                "do()" => mul_enabled = true,
                "don't()" => mul_enabled = false,
                _ if mul_enabled => {
                    let a = c
                        .name("a")
                        .map(|a| a.as_str().parse::<u32>().unwrap())
                        .unwrap_or_default();
                    let b = c
                        .name("b")
                        .map(|b| b.as_str().parse::<u32>().unwrap())
                        .unwrap_or_default();
                    return Some(a * b);
                }
                _ => {}
            }
            None
        })
        .sum()
}
