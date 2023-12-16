use std::str::FromStr;

use advent_of_code_2023::day_15::instruction::{Instruction, Operation};

#[test]
pub fn from_str_remove() {
    let actual = Instruction::from_str("rn-").unwrap();
    let expected = Instruction::new("rn-", "rn", Operation::Remove);

    assert_eq!(expected, actual);
}

#[test]
pub fn from_str_insert() {
    let actual = Instruction::from_str("rn=12").unwrap();
    let expected = Instruction::new("rn=12", "rn", Operation::Insert(12));

    assert_eq!(expected, actual);
}
