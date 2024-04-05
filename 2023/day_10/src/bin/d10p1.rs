use day_10::*;

fn main() {
    let input = include_str!("../../resources/input.txt");
    let mut maze: Vec<Vec<char>> =
        input.lines().map(|x| Vec::from_iter(x.chars())).collect();

    let (mut x, mut y) = find_player(&maze);
    let init_pos = (x, y);

    let mut distance: usize = 0;
    let mut steps: Vec<(usize, usize)> = Vec::new();

    dbg!(init_pos, x, y);
    loop {
        if valid_move(Cardinal::North, maze[x + 1][y]) {
            if (x, y) != init_pos {
                maze[x][y] = 'x';
            }
            x += 1;
        } else if valid_move(Cardinal::South, maze[x - 1][y]) {
            if (x, y) != init_pos {
                maze[x][y] = 'x';
            }
            x -= 1;
        } else if valid_move(Cardinal::East, maze[x][y + 1]) {
            if (x, y) != init_pos {
                maze[x][y] = 'x';
            }
            y += 1;
        } else if valid_move(Cardinal::West, maze[x][y - 1]) {
            if (x, y) != init_pos {
                maze[x][y] = 'x';
            }
            y -= 1;
        } else {
            panic!("invalid move");
        }
        distance += 1;
        steps.push((x, y));
        dbg!(maze[x][y]);
        dbg!(x, y);
        if (x, y) == init_pos {
            println!("exit found");
            break;
        }
    }
    dbg!(distance);
    dbg!(steps);
}
