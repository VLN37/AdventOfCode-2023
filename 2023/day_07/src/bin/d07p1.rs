use day_07::Game;
use itertools::Itertools;

fn main() {
    let input = include_str!("../../resources/input.txt");

    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        let mut game = Game::from(hand.chars().collect::<Vec<char>>());
        game.bid = bid.parse::<usize>().unwrap();
        games.push(game);
    }
    games.iter_mut().for_each(|x| x.compute());

    let mut result = 0;
    for (i, game) in games.iter().sorted().enumerate() {
        result += (i + 1) * game.bid;
    }
    println!("Result: {result}");
}
