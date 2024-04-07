use day_10::*;

fn recurse(
    maze: &mut Vec<Vec<char>>,
    steps: &mut Vec<(usize, usize)>,
    x: usize,
    y: usize,
) -> bool {
    let mut viable = false;
    if valid_move(Cardinal::South, maze[x][y], maze[x + 1][y]) {
        if maze[x][y] != 'S' {
            maze[x][y] = 'x';
        }
        viable = recurse(maze, steps, x + 1, y);
    }
    if valid_move(Cardinal::North, maze[x][y], maze[x - 1][y]) {
        if maze[x][y] != 'S' {
            maze[x][y] = 'x';
        }
        viable = recurse(maze, steps, x - 1, y);
    }
    if valid_move(Cardinal::East, maze[x][y], maze[x][y + 1]) {
        if maze[x][y] != 'S' {
            maze[x][y] = 'x';
        }
        viable = recurse(maze, steps, x, y + 1);
    }
    if valid_move(Cardinal::West, maze[x][y], maze[x][y - 1]) {
        if maze[x][y] != 'S' {
            maze[x][y] = 'x';
        }
        viable = recurse(maze, steps, x, y - 1);
    }
    if viable {
        steps.push((x, y));
        return true;
    }
    if !viable && back_to_where_i_started(maze, x, y) {
        maze[x][y] = 'x';
        return true;
    }
    maze[x][y] = 'n';
    false
}

fn main() {
    let input = include_str!("../../resources/input.txt");
    let mut maze: Vec<Vec<char>> =
        input.lines().map(|x| Vec::from_iter(x.chars())).collect();

    // dbg!(&maze);
    let (x, y) = find_player(&maze);
    let init_pos = (x, y);

    let mut steps: Vec<(usize, usize)> = Vec::new();
    dbg!(init_pos);
    recurse(&mut maze, &mut steps, x, y);
    dbg!(maze
        .iter()
        .map(|x| x.iter().collect::<String>())
        .collect::<Vec<String>>());
    println!("Maximum distance: {}", steps.len() / 2 + 1)
}
