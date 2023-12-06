#[derive(Debug, Default)]
pub struct Instructions {
    pub destination_start: usize,
    pub source_start:      usize,
    pub range:             usize,
}

impl From<&str> for Instructions {
    fn from(value: &str) -> Self {
        let mut iter = value.split(' ').map(|x| x.parse::<usize>().unwrap());
        Instructions {
            destination_start: iter.next().unwrap(),
            source_start:      iter.next().unwrap(),
            range:             iter.next().unwrap(),
        }
    }
}
