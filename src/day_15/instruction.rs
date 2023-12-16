use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    rep: String,
    pub label: String,
    pub operation: Operation,
}

impl Instruction {
    pub fn new(rep: &str, label: &str, operation: Operation) -> Self {
        return Instruction {
            rep: rep.to_string(),
            label: label.to_string(),
            operation,
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rep = s;
        let label = rep.split(|c| c == '-' || c == '=').nth(0).unwrap();
        let op_start = label.len();
        let op_str = &rep[op_start..];

        let operation = Operation::from_str(op_str).unwrap();

        return Ok(Instruction::new(rep, label, operation));
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Remove,
    Insert(u32),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseOperationError;

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0).unwrap() {
            '-' => Ok(Operation::Remove),
            '=' => Ok(Operation::Insert(
                u32::from_str(s.chars().skip(1).collect::<String>().as_str())
                    .unwrap(),
            )),
            _ => panic!(),
        }
    }
}
