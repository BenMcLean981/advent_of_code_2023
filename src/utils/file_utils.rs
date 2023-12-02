use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from)
        .collect();
}
