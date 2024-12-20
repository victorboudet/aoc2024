use regex::Regex;
use std::fs;

fn extract_mul_numbers(s: &str) -> Option<(i32, i32)> {
    let re = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();
    if let Some(caps) = re.captures(s) {
        let x = caps[1].parse().ok()?;
        let y = caps[2].parse().ok()?;
        Some((x, y))
    } else {
        None
    }
}

fn part1(s: &str) -> i32 {
    let mut tot = 0;
    for i in 0..s.len() {
        let shifted = &s[i..];
        if let Some((x, y)) = extract_mul_numbers(shifted) {
            tot += x * y;
        }
    }
    tot
}

fn part2(s: &str) -> i32 {
    let mut tot = 0;
    let mut activate = true;
    for i in 0..s.len() {
        let shifted = &s[i..];
        if shifted.starts_with("do()") {
            activate = true;
        }
        if shifted.starts_with("don't()") {
            activate = false;
        }
        if !activate {
            continue;
        }
        if let Some((x, y)) = extract_mul_numbers(shifted) {
            tot += x * y;
        }
    }
    tot
}

pub fn main(input: String) {
    let s = input.as_str();
    println!("{}", part1(&s));
    println!("{}", part2(&s));
}
