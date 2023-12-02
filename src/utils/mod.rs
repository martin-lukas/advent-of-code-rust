use std::fmt;
use std::fs;

pub fn puzzle_input(year: AocYear, day: u8) -> String {
    fs::read_to_string(format!("src/y{year}/day{day}/input.txt")).unwrap()
}

pub enum AocYear {
    Y22,
    Y23,
}

impl fmt::Display for AocYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            Self::Y22 => "22",
            Self::Y23 => "23",
        };
        write!(f, "{res}")
    }
}
