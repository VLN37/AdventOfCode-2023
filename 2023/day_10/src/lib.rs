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

pub enum Cardinal {
    East,
    West,
    North,
    South,
}

pub fn valid_move(direction: Cardinal, c: char) -> bool {
    match direction {
        Cardinal::North => "|LJS".contains(c),
        Cardinal::West => "7J-S".contains(c),
        Cardinal::East => "FL-JS".contains(c),
        Cardinal::South => "|7FS".contains(c),
    }
}
