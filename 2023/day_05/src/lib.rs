#[derive(Debug, Default)]
pub struct Instructions {
    pub dst: i64,
    pub src: i64,
    pub n:   i64,
}

impl From<&str> for Instructions {
    /// ### Format example
    /// "1 2 3"
    fn from(value: &str) -> Self {
        let mut iter = value.split(' ').map(|x| x.parse::<i64>().unwrap());
        Instructions {
            dst: iter.next().unwrap(),
            src: iter.next().unwrap(),
            n:   iter.next().unwrap(),
        }
    }
}
