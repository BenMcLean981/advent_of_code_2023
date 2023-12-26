use std::str::FromStr;

pub struct Part {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

#[derive(Debug)]
pub struct ParsePartError;

impl FromStr for Part {
    type Err = ParsePartError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace('{', "").replace('}', "").replace('=', "");
        let split = s
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        let x = u32::from_str(&split[0][1..]).unwrap();
        let m = u32::from_str(&split[1][1..]).unwrap();
        let a = u32::from_str(&split[2][1..]).unwrap();
        let s = u32::from_str(&split[3][1..]).unwrap();

        return Ok(Part { x, m, a, s });
    }
}
