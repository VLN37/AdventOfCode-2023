#[derive(Debug, Default)]
pub struct Bag {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl Bag {
    pub fn new() -> Self { Bag::default() }

    pub fn factory(r: u32, g: u32, b: u32) -> Self { Bag { r, g, b } }

    pub fn holds(&self, bag: &Bag) -> bool {
        self.r >= bag.r && self.g >= bag.g && self.b >= bag.b
    }
}

impl From<&str> for Bag {
    fn from(raw: &str) -> Self {
        let mut bag = Bag::new();
        for balls in raw.split(',') {
            let mut pair = balls.trim().split(' ');
            let qty: u32 = pair.next().unwrap().parse().unwrap();
            let color: &str = pair.next().unwrap().trim();
            match color {
                "red" => bag.r += qty,
                "green" => bag.g += qty,
                "blue" => bag.b += qty,
                _ => panic!("invalid ball color"),
            }
        }
        bag
    }
}

#[derive(Debug, Default)]
pub struct Game {
    pub bags: Vec<Bag>,
}

impl Game {
    pub fn new() -> Self { Game::default() }
}

impl From<&str> for Game {
    fn from(raw: &str) -> Self {
        let mut result = Game::new();
        let game = raw.split(':').last().unwrap().trim().to_string();
        for raw_bag in game.split(';') {
            result.bags.push(Bag::from(raw_bag));
        }
        // dbg!(&result.bags);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day2p1() {
        let bag = Bag::factory(12, 13, 14);
        assert!(!bag.holds(&Bag::factory(13, 13, 13)));
    }
}
