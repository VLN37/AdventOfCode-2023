#[derive(Debug)]
pub struct Place {
    pub place: String,
    pub left:  String,
    pub right: String,
}

impl From<&str> for Place {
    fn from(value: &str) -> Self {
        let v: String = value.chars().filter(|c| !"(),=".contains(*c)).collect();
        let mut iter = v.split_whitespace();
        Place {
            place: iter.next().unwrap().into(),
            left:  iter.next().unwrap().into(),
            right: iter.next().unwrap().into(),
        }
    }
}
