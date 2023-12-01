use std::fs::read_to_string;

pub fn solve() {
    let filename = "src/day_1/input.txt";

    let lines = read_lines(filename);
    let calibration_value = add_lines(lines);

    println!("The calibration value is {}.", calibration_value);
}

pub fn add_lines(lines: Vec<String>) -> u32 {
    let calibration_values = lines
        .iter()
        .map(|l| get_calibration_value(&l))
        .collect::<Vec<u32>>();

    return calibration_values.iter().fold(0, |sum, n| sum + n);
}

pub fn get_calibration_value(s: &str) -> u32 {
    let preprocessed = replace_digit_text(s);

    let chars = [get_first_digit(&preprocessed), get_last_digit(&preprocessed)];
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

fn replace_digit_text(line: &str) -> String {
    let mut result = Vec::<char>::new();
    let chars = line.chars().collect::<Vec<char>>();

    let mut i = 0;

    while i < chars.len() {
        result.push(get_next_char(&chars[i..].to_vec()));

        i = i + 1;
    }

    return result.iter().collect()
}


fn get_next_char(chars: &Vec<char>) -> char {
    if matches_start("one", chars) {
        return '1';
    } else if matches_start("two", chars) {
        return '2';
    } else if matches_start("three", chars) {
        return '3';
    } else if matches_start("four", chars) {
        return '4';
    } else if matches_start("five", chars) {
        return '5';
    } else if matches_start("six", chars) {
        return '6';
    } else if matches_start("seven", chars) {
        return '7';
    } else if matches_start("eight", chars) {
        return '8';
    } else if matches_start("nine", chars) {
        return '9';
    } else {
        return chars.first().unwrap().clone();   
    }
}

fn matches_start(check: &str, chars: &Vec<char>) -> bool {
    if chars.len() < check.len() {
        return false;
    }

    return chars[0..check.len()].iter().collect::<String>() == check;
}

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)
        .collect();
}