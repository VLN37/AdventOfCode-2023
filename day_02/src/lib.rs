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
    /// ## Example format
    /// 4 blue, 17 green
    /// 12 red, 17 blue, 3 green
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
    pub id:   u32,
}

impl Game {
    pub fn new() -> Self { Game::default() }
}

impl From<&str> for Game {
    /// ## Example format
    /// Game 51: 4 blue, 17 green; 3 blue, 17 green, 1 red; 6 green, 8 blue
    fn from(raw: &str) -> Self {
        let mut result = Game::new();
        let iter = raw.split(':');
        let raw_game = iter.last().unwrap().trim().to_string();

        let iter = raw.split(':').nth(0).unwrap().split(' ');
        result.id = iter.last().unwrap().parse().unwrap();
        for raw_bag in raw_game.split(';') {
            result.bags.push(Bag::from(raw_bag));
        }
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
