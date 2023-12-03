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
            let (qty, color) = balls.trim().split_once(' ').unwrap();
            match color.trim() {
                "red" => bag.r = qty.parse().unwrap(),
                "green" => bag.g = qty.parse().unwrap(),
                "blue" => bag.b = qty.parse().unwrap(),
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
        let mut game = Game::new();
        let (id, raw_game) = raw.split_once(':').unwrap();
        game.id = id.split(' ').last().unwrap().parse().unwrap();
        game.bags.extend(raw_game.trim().split(';').map(Bag::from));
        game
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
