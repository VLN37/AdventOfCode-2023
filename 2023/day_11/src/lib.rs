#[derive(Default, Clone)]
pub struct Galaxy {
    pub id:        usize,
    pub x:         i64,
    pub y:         i64,
    pub distances: Vec<i64>,
}

pub struct Universe {
    pub uni: Vec<Vec<char>>,
}

const EMPTY: char = '.';
const GALAXY: char = '#';

impl Universe {
    pub fn expand(&mut self) {
        for (i, line) in self.uni.clone().iter().enumerate().rev() {
            if line.iter().all(|x| *x == '.') {
                self.uni.insert(i, line.clone());
            }
        }
        for i in (0..self.uni.first().unwrap().len()).rev() {
            if self.uni.iter().map(|x| x[i]).all(|x| x == EMPTY) {
                for line in &mut self.uni {
                    line.insert(i, EMPTY);
                }
            }
        }
    }

    pub fn galaxies(&self) -> Vec<Galaxy> {
        let mut galaxies: Vec<Galaxy> = Vec::new();
        let mut id = 1;
        for x in 0..self.height() {
            // dbg!(self.length());
            for y in 0..self.length() {
                // dbg!(self.height());
                if self.uni[x][y] == GALAXY {
                    galaxies.push(Galaxy {
                        x: x as i64,
                        y: y as i64,
                        id,
                        ..Default::default()
                    });
                    id += 1;
                }
            }
        }
        galaxies
    }

    pub fn length(&self) -> usize { self.uni.first().unwrap().len() }

    pub fn height(&self) -> usize { self.uni.len() }

    pub fn dump(&self) {
        println!(
            "dimensions: x={} y={}",
            self.uni.len(),
            self.uni.first().unwrap().len()
        );
        let dump: Vec<String> = self
            .uni
            .iter()
            .map(|x| String::from_iter(x.iter()))
            .collect();
        dbg!(dump);
    }
}

impl From<&str> for Universe {
    fn from(input: &str) -> Self {
        Universe {
            uni: input.lines().map(|x| x.chars().collect()).collect(),
        }
    }
}
