pub fn find_player(vec: &[Vec<char>]) -> (usize, usize) {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut found = false;

    for (i, line) in vec.iter().enumerate() {
        if let Some(idx) = line.iter().position(|x| *x == 'S') {
            x = i;
            y = idx;
            found = true;
            break;
        }
    }
    if !found {
        panic!("char not found");
    }
    (x, y)
}

#[derive(Debug)]
pub enum Cardinal {
    East,
    West,
    North,
    South,
}

pub fn valid_move(direction: Cardinal, src: char, dst: char) -> bool {
    // println!("src {src} dst {dst} {:?}", direction);
    match direction {
        Cardinal::North => "|LJS".contains(src) && "|7F".contains(dst),
        Cardinal::West => "-J7S".contains(src) && "L-F".contains(dst),
        Cardinal::East => "L-FS".contains(src) && "-J7".contains(dst),
        Cardinal::South => "|7FS".contains(src) && "|LJ".contains(dst),
    }
}

pub fn back_to_where_i_started(maze: &mut [Vec<char>], x: usize, y: usize) -> bool {
    let sides = [
        maze[x + 1][y],
        maze[x - 1][y],
        maze[x][y + 1],
        maze[x][y - 1],
    ];
    for c in sides.iter() {
        if *c == 'S' {
            return true;
        }
    }
    false
}
