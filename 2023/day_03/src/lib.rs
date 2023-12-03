use core::panic;

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct GearCoordinate {
    x: usize,
    y: usize,
}

impl GearCoordinate {
    pub fn new(x: usize, y: usize) -> Self { Self { x, y } }
}

pub fn distance(slice: &[char]) -> usize {
    for (i, c) in slice.iter().enumerate() {
        // println!("{c} - {i}");
        if !c.is_ascii_digit() {
            // println!("{c}");
            return i;
        }
    }
    panic!("should be found");
}

fn is_marker(c: char) -> bool { c != '.' && c != '?' && c.is_ascii_punctuation() }
fn is_star(c: char) -> bool { c == '*' }

// ? matrix: &Vec<Vec<char>>,
pub fn surroundings(matrix: &[Vec<char>], i: usize, j: usize, distance: usize) -> bool {
    for j in j..j + distance {
        // println!("current: {}", matrix[i][j]);
        let cases = [
            (i + 1, j),
            (i - 1, j),
            (i, j - 1),
            (i, j + 1),
            (i - 1, j - 1),
            (i + 1, j + 1),
            (i - 1, j + 1),
            (i + 1, j - 1),
        ];
        for (x, y) in cases {
            if is_marker(matrix[x][y]) {
                return true;
            }
        }
    }
    false
}

pub fn star_coordinate(
    matrix: &[Vec<char>],
    i: usize,
    j: usize,
    distance: usize,
) -> Option<(usize, usize)> {
    for offset in 0..distance {
        let j = j + offset;
        let cases = [
            (i + 1, j),
            (i - 1, j),
            (i, j - 1),
            (i, j + 1),
            (i - 1, j - 1),
            (i + 1, j + 1),
            (i - 1, j + 1),
            (i + 1, j - 1),
        ];
        for (x, y) in cases {
            if is_star(matrix[x][y]) {
                return Some((x, y));
            }
        }
    }
    None
}
