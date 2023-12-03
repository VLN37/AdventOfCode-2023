use day_02::{Bag, Game};

fn main() {
    let magical_bag = Bag::factory(12, 13, 14);
    let mut valid_games: Vec<u32> = Vec::new();
    let input = include_str!("../../resources/input.txt");

    'outer: for game in input.lines().map(Game::from) {
        for bag in &game.bags {
            if !magical_bag.holds(bag) {
                continue 'outer;
            }
        }
        valid_games.push(game.id);
    }
    let result: u32 = valid_games.iter().sum();
    println!("Result: {result}");
}
