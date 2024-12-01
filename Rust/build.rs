use std::fs;
use std::process::ExitCode;

const INPUTS: &str = include_str!("inputs.txt");

fn main() -> ExitCode {
    println!("cargo::rerun-if-changed=inputs.txt");
    println!("cargo::rerun-if-env-changed=AOC_SESSION");
    println!("cargo::rerun-if-changed=inputs");

    if !fs::exists("./inputs").unwrap() {
        fs::create_dir("./inputs").unwrap();
    }

    let mut needed_inputs = parse_needed_inputs();
    let mut existing_inputs = vec![];

    for entry in fs::read_dir("./inputs").unwrap() {
        let Ok(entry) = entry else { continue };

        if !entry.file_type().unwrap().is_file() {
            continue;
        }

        let file_name = entry
            .path()
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if !file_name.starts_with("input") {
            continue;
        }

        let parts = file_name
            .split('_')
            .skip(1)
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        existing_inputs.push((parts[0], parts[1]));
    }

    let needed_inputs = needed_inputs
        .drain(..)
        .filter(|day| !existing_inputs.contains(day))
        .collect::<Vec<_>>();

    if needed_inputs.is_empty() {
        return ExitCode::SUCCESS;
    }

    let session_cookie = std::env::var("AOC_SESSION")
        .expect("Error: AOC_SESSION not set, unable to download puzzle input.");

    for (year, day) in needed_inputs {
        println!("Downloading year {} day {}...", year, day);
        let success = std::process::Command::new("curl")
            .arg(format!(
                "https://adventofcode.com/{}/day/{}/input",
                year, day
            ))
            .arg("-H")
            .arg(format!("Cookie: session={}", session_cookie))
            .arg("-o")
            .arg(format!("./inputs/input_{}_{}.txt", year, day))
            .status()
            .unwrap()
            .success();

        if !success {
            eprint!("Failed to fetch {} {}.", year, day);
            return ExitCode::FAILURE;
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    ExitCode::SUCCESS
}

fn parse_needed_inputs() -> Vec<(u32, u32)> {
    let mut inputs = vec![];
    for line in INPUTS.lines() {
        let parts = line
            .trim()
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        inputs.push((parts[0], parts[1]));
    }

    inputs
}
