use regex::Regex;
use std::fs;

fn extract_mul_patterns(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();

    re.captures_iter(input)
        .filter_map(|cap| {
            let x = cap[1].parse::<i32>().ok()?;
            let y = cap[2].parse::<i32>().ok()?;
            Some((x, y))
        })
        .collect()
}

fn part1(input: &String) -> i32 {
    let patterns = extract_mul_patterns(input);
    let mut tot = 0;
    for (x, y) in patterns {
        tot += x * y;
    }
    tot
}

pub fn main() {
    match fs::read_to_string("input/day3.txt") {
        Ok(input) => println!("{}", part1(&input)),
        Err(e) => {
            println!("Error reading file: {:?}", e);
            panic!();
        }
    }
}
