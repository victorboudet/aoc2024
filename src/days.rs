use std::fs;

pub mod day3;
pub mod day4;
pub mod day5;

fn read_day_input(day: usize) -> String {
    println!("{}", format!("inputs/day{}.txt", day));
    match fs::read_to_string(format!("inputs/day{}.txt", day)) {
        Ok(input) => return input,
        Err(e) => {
            println!("Error reading file: {:?}", e);
            panic!();
        }
    }
}

pub fn launch_day(day: usize) {
    let input = read_day_input(day);

    let days: Vec<fn(String)> = vec![day3::main, day3::main, day3::main, day4::main, day5::main];

    if day > 0 && day <= days.len() {
        days[day - 1](input);
    } else {
        panic!("Invalid day: {}", day);
    }
}
