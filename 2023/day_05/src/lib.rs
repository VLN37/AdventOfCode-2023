#[derive(Debug, Default)]
pub struct Instructions {
    pub src: usize,
    pub dst: usize,
    pub n:   usize,
}

impl From<&str> for Instructions {
    /// ### Format example
    /// "1 2 3"
    fn from(value: &str) -> Self {
        let mut iter = value.split(' ').map(|x| x.parse::<usize>().unwrap());
        Instructions {
            dst: iter.next().unwrap(),
            src: iter.next().unwrap(),
            n:   iter.next().unwrap(),
        }
    }
}

// impl<'a> FromIterator<&'a str> for Instructions {
//     fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
//         Vec::<Instructions>::from_iter(iter)
//     }
// }
// seed - soil
// soil - fert
// fert - water
// water - light
// light - temp
// temp - humidity
// humidity - location
