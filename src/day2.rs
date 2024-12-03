use std::io::{self, BufRead, Error};

fn parse_line(line: Result<String, Error>) -> Vec<i32> {
    match line {
        Ok(l) => {
            let mut parts = l.split_whitespace();
            let mut res = vec![];
            for part in parts {
                res.push(part.parse::<i32>().unwrap());
            }
            return res;
        }
        Err(e) => {
            println!("Error reading line: {:?}", e);
            panic!();
        }
    }
}

fn is_increasing(vec: &[i32]) -> bool {
    vec.windows(2).all(|w| w[0] < w[1])
}

fn is_decreasing(vec: &[i32]) -> bool {
    vec.windows(2).all(|w| w[0] > w[1])
}

fn test_data(data: Vec<i32>) -> bool {
    let mut prev = -1;

    if !is_decreasing(&data) && !is_increasing(&data) {
        return false;
    }
    for num in data {
        if prev == -1 {
            prev = num;
            continue;
        }
        if (num - prev).abs() > 3 {
            prev = -1;
            break;
        }
        prev = num;
    }
    if prev != -1 {
        return true;
    }
    false
}

fn part1(datas: Vec<Vec<i32>>) -> i32 {
    let mut safe: i32 = 0;
    for data in datas {
        if test_data(data) {
            safe += 1;
        }
    }
    safe
}

fn part2(datas: Vec<Vec<i32>>) -> i32 {
    let mut safe: i32 = 0;
    for data in datas {
        for i in 0..data.len() {
            let mut data_copy = data.clone();
            data_copy.remove(i);
            if test_data(data_copy) {
                safe += 1;
                break;
            }
        }
    }
    safe
}

pub fn day2() {
    let stdin = io::stdin();
    let mut datas: Vec<Vec<i32>> = vec![];
    for line_res in stdin.lock().lines() {
        datas.push(parse_line(line_res))
    }
    println!("{}", part1(datas.clone()));
    println!("{}", part2(datas));
}
