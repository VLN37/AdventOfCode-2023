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
