use day_10::*;

fn main() {
    let input = include_str!("../../resources/input.txt");

    let mut maze = Maze::from(input);
    maze.find_player();
    maze.pipe_jump();
    maze.report();
}
