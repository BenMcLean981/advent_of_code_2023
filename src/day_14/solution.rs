use crate::utils::file_utils::read_lines;

pub fn solve() {
    let filename = "src/day_14/sample.txt";

    let binding = read_lines(filename);
    let lines = binding.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    println!("Day 14");
}
