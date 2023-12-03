use day_02::Game;

fn main() {
    let mut powers: Vec<u32> = Vec::new();
    let input = include_str!("../../resources/input.txt");

    for raw_game in input.lines() {
        let game = Game::from(raw_game);
        let red: u32 = game.bags.iter().map(|x| x.r).max().unwrap();
        let green: u32 = game.bags.iter().map(|x| x.g).max().unwrap();
        let blue: u32 = game.bags.iter().map(|x| x.b).max().unwrap();
        powers.push(red * green * blue);
    }
    let result: u32 = powers.iter().sum();
    println!("Result: {result}");
}
