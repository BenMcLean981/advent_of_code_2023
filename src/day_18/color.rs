use std::str::FromStr;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        return Color { r, g, b };
    }
}

#[derive(Debug)]
pub struct ColorParseError;

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.len() == 7 {
            let r = u8::from_str_radix(&s[1..3], 16).unwrap();
            let g = u8::from_str_radix(&s[3..5], 16).unwrap();
            let b = u8::from_str_radix(&s[5..7], 16).unwrap();

            return Ok(Color::new(r, g, b));
        } else if s.len() == 6 {
            let r = u8::from_str_radix(&s[0..2], 16).unwrap();
            let g = u8::from_str_radix(&s[2..4], 16).unwrap();
            let b = u8::from_str_radix(&s[4..6], 16).unwrap();

            return Ok(Color::new(r, g, b));
        } else {
            return Err(ColorParseError);
        }
    }
}
