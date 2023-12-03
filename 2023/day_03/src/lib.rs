use core::panic;

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct GearCoordinates {
    x: usize,
    y: usize,
}

impl GearCoordinates {
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
    for offset in 0..distance {
        let j = j + offset;
        // println!("current: {}", matrix[i][j]);
        if is_marker(matrix[i + 1][j])
            || is_marker(matrix[i - 1][j])
            || is_marker(matrix[i][j - 1])
            || is_marker(matrix[i][j + 1])
            || is_marker(matrix[i + 1][j + 1])
            || is_marker(matrix[i - 1][j - 1])
            || is_marker(matrix[i + 1][j - 1])
            || is_marker(matrix[i - 1][j + 1])
        {
            return true;
        }
    }
    false
}

pub fn star_coordinates(
    matrix: &[Vec<char>],
    i: usize,
    j: usize,
    distance: usize,
) -> Option<(usize, usize)> {
    for offset in 0..distance {
        let j = j + offset;
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
            if is_star(matrix[x][y]) {
                return Some((x, y));
            }
        }
    }
    None
}
