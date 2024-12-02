use std::io::Error;
use std::io::{self, BufRead};

fn get_numbers(line_res: Result<String, Error>) -> (i32, i32) {
    match line_res {
        Ok(line) => {
            let mut parts = line.split_whitespace();
            let left = parts.next().unwrap().parse::<i32>().unwrap();
            let right = parts.next().unwrap().parse::<i32>().unwrap();
            return (left, right);
        }
        Err(e) => {
            println!("Error reading line: {:?}", e);
            panic!();
        }
    }
}

fn part1(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    let mut tot_dist = 0;
    left.sort();
    right.sort();
    for i in 0..left.len() {
        tot_dist += (left[i] - right[i]).abs();
    }
    tot_dist
}

fn part2(right: Vec<i32>, left: Vec<i32>) -> i32 {
    let mut tot = 0;
    for i in left {
        let it = right.iter().filter(|&&x| x == i).count();
        tot += i * it as i32;
    }
    tot
}

pub fn day1() {
    let stdin = io::stdin();
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for line_res in stdin.lock().lines() {
        let (l, r) = get_numbers(line_res);
        left.push(l);
        right.push(r);
    }
    println!("{}", part1(left.clone(), right.clone()));
    println!("{}", part2(left, right));
}
