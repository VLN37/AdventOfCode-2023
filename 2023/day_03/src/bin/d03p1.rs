use day_03::{distance, surroundings};

fn main() {
    let input = include_str!("../../resources/input.txt");
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut results: Vec<usize> = Vec::new();

    let width = input.find('\n').unwrap() + 2;
    matrix.push(vec!['?'; width]);
    for line in &mut input.lines() {
        matrix.push(format!("?{line}?").chars().collect());
    }
    matrix.push(vec!['?'; width]);
    let height = matrix.len();

    let mut i = 1;
    let mut j = 1;
    while i < height {
        while j < width {
            if matrix[i][j].is_ascii_digit() {
                let slice = &matrix[i][j..];
                let distance = distance(slice);
                if surroundings(&matrix, i, j, distance) {
                    let str: String = matrix[i][j..j + distance].iter().collect();
                    results.push(str.parse().unwrap());
                }
                j += distance;
                continue;
            }
            j += 1;
        }
        j = 1;
        i += 1;
    }
    println!("Result: {}", results.iter().sum::<usize>());
}
