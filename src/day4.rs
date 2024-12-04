use std::fs;

fn check_word(
    chars: &Vec<char>,
    line_size: usize,
    word: &str,
    index: i32,
    dx: i32,
    dy: i32,
) -> i32 {
    if word.is_empty() {
        return 1;
    }
    let word_first_char = word.chars().nth(0).unwrap();
    let new_index = index + dx + dy * (line_size + 1) as i32;
    if new_index >= 0 && new_index < chars.len() as i32 {
        if chars[new_index as usize] == word_first_char {
            return check_word(&chars, line_size, &word[1..], new_index, dx, dy);
        }
    }
    0
}

fn part1(chars: &Vec<char>, line_size: usize) -> i32 {
    let word = "XMAS";
    let char = word.chars().nth(0).unwrap();
    let mut tot = 0;
    for i in 0..chars.len() {
        if chars[i] == char {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    tot += check_word(&chars, line_size, &word[1..], i as i32, dx, dy);
                }
            }
        } else {
            continue;
        }
    }
    tot
}
fn check_cross(chars: &Vec<char>, line_size: usize, index: i32) -> i32 {
    let index = index as usize;
    if index >= 1 + line_size
        && index + 1 + line_size < chars.len()
        && index as i32 - line_size as i32 - 2 > 0
    {
        let upl = chars[index - line_size - 2];
        let upr = chars[index - line_size];
        let botl = chars[index + line_size];
        let botr = chars[index + line_size + 2];
        let s = [upl, upr, botl, botr].iter().filter(|&&c| c == 'S').count() as i32;
        let m = [upl, upr, botl, botr].iter().filter(|&&c| c == 'M').count() as i32;

        if s == 2 && m == 2 && upl != botr {
            return 1;
        }
    }
    0
}

fn part2(chars: &Vec<char>, line_size: usize) -> i32 {
    let char = 'A';
    let mut tot = 0;
    for i in 0..chars.len() {
        if chars[i] == char {
            tot += check_cross(&chars, line_size, i as i32);
        }
    }
    tot
}

pub fn main() {
    match fs::read_to_string("input/day4.txt") {
        Ok(input) => {
            let chars: Vec<char> = input.chars().collect();
            let line_size = input.lines().next().map(|line| line.len()).unwrap_or(0);
            println!("{}", part1(&chars, line_size));
            println!("{}", part2(&chars, line_size));
        }
        Err(e) => {
            println!("Error reading file: {:?}", e);
            panic!();
        }
    }
}
