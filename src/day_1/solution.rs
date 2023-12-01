use std::fs::read_to_string;

pub fn solve() {
    let filename = "src/day_1/input.txt";

    let lines = read_lines(filename);
    let calibration_value = add_lines(lines);

    println!("The calibration value is {}.", calibration_value);
}

pub fn add_lines(lines: Vec<String>) -> u32 {
    return lines.iter().map(|l| get_calibration_value(&l)).fold(0, |sum, n| sum + n);
}

pub fn get_calibration_value(s: &str) -> u32 {
    let chars = [get_first_digit(s), get_last_digit(s)];
    let num_string: String = chars.iter().collect();

    return num_string.parse().unwrap();
}

fn get_first_digit(s: &str) -> char {
    let radix = 10;
    return s.chars().find(|c| c.is_digit(radix)).unwrap();
}


fn get_last_digit(s: &str) -> char {
    let radix = 10;
    return s.chars().rev().find(|c| c.is_digit(radix)).unwrap();
}

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)
        .collect();
}