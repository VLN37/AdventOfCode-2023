use day_10::*;

fn recurse(
    maze: &mut Vec<Vec<char>>,
    steps: &mut Vec<(usize, usize)>,
    mut x: usize,
    mut y: usize,
    distance: usize,
) -> bool {
    let mut viable = false;
    // dbg!(maze[x][y]);
    // dbg!(maze
    // .to_owned()
    // .iter()
    // .map(|x| x.to_owned().iter().collect::<String>())
    // .collect::<Vec<String>>());
    println!("{distance}");
    if valid_move(Cardinal::South, maze[x][y], maze[x + 1][y]) {
        if maze[x][y] != 'S' {
            maze[x][y] = 'x';
        }
        // dbg!("recurse", maze[x][y]);
        // dbg!('&');
        x += 1;
        viable = recurse(maze, steps, x, y, distance + 1);
        if !viable {
            maze[x][y] = 'n';
        }
    } else if valid_move(Cardinal::North, maze[x][y], maze[x - 1][y]) {
        if maze[x][y] != 'S' {
            maze[x][y] = 'x';
        }
        // dbg!("recurse", maze[x][y]);
        // dbg!('&');
        x -= 1;
        viable = recurse(maze, steps, x, y, distance + 1);
        if !viable {
            maze[x][y] = 'n';
        }
    } else if valid_move(Cardinal::East, maze[x][y], maze[x][y + 1]) {
        if maze[x][y] != 'S' {
            maze[x][y] = 'x';
        }
        // dbg!("recurse", maze[x][y]);
        // dbg!('&');
        y += 1;
        viable = recurse(maze, steps, x, y, distance + 1);
        if !viable {
            maze[x][y] = 'n';
        }
    } else if valid_move(Cardinal::West, maze[x][y], maze[x][y - 1]) {
        if maze[x][y] != 'S' {
            maze[x][y] = 'x';
        }
        // dbg!("recurse", maze[x][y]);
        // dbg!('&');
        y -= 1;
        viable = recurse(maze, steps, x, y, distance + 1);
        if !viable {
            maze[x][y] = 'n';
        }
    }
    if viable {
        // dbg!("push", (x, y), maze[x][y]);
        steps.push((x, y));
        return true;
    }
    if !viable && back_to_where_i_started(maze, x, y) {
        // dbg!("push end", (x, y), maze[x][y]);
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

    dbg!(init_pos, x, y);
    recurse(&mut maze, &mut steps, x, y, 0);
    dbg!(steps.len());
    dbg!(steps.len() / 2);
    // dbg!(steps);
    // dbg!(maze);
    dbg!(maze
        .to_owned()
        .iter()
        .map(|x| x.to_owned().iter().collect::<String>())
        .collect::<Vec<String>>());
}
