#[derive(Debug, Clone, Copy)]
pub enum Direction {
    LEFT,
    RIGHT,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::LEFT,
            'R' => Self::RIGHT,
            _ => panic!("Invalid direction"),
        }
    }
}
