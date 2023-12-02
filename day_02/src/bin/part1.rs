use day_02::{Bag, Game};

fn main() {
    let magical_bag = Bag::factory(12, 13, 14);
    let mut valid_games: Vec<usize> = Vec::new();

    let input = include_str!("../../resources/input.txt");
    for (id, raw_game) in input.lines().enumerate() {
        let game = Game::from(raw_game);
        let mut valid = true;
        for bag in game.bags {
            if !magical_bag.holds(&bag) {
                valid = false;
                break;
            }
        }
        if valid {
            valid_games.push(id + 1);
        }
    }
    let result: usize = valid_games.iter().sum();
    println!("Result: {result}");
}
