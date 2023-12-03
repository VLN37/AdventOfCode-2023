use std::collections::HashMap;

use day_03::{distance, star_coordinate, surroundings, GearCoordinate};

fn main() {
    let input = include_str!("../../resources/input.txt");
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut results: HashMap<_, Vec<i32>> = HashMap::new();

    let width = input.find('\n').unwrap() + 2;
    matrix.push(vec!['?'; width]);
    for line in &mut input.lines() {
        matrix.push(format!("?{line}?").chars().collect());
    }
    matrix.push(vec!['?'; width]);
    let height = matrix.len();

    let mut i = 1;
    let mut j = 1;
    while i < height - 1 {
        // dbg!(i);
        while j < width - 1 {
            if matrix[i][j].is_ascii_digit() {
                let slice = &matrix[i][j..];
                let distance = distance(slice);
                if surroundings(&matrix, i, j, distance) {
                    if let Some((x, y)) = star_coordinate(&matrix, i, j, distance) {
                        let str: String = matrix[i][j..j + distance].iter().collect();
                        let nbr = str.parse::<i32>().unwrap();
                        let coord = GearCoordinate::new(x, y);
                        if let Some(vec) = results.get_mut(&coord) {
                            vec.push(nbr)
                        } else {
                            results.insert(coord, vec![nbr]);
                        }
                    }
                }
                j += distance;
                continue;
            }
            j += 1;
        }
        j = 1;
        i += 1;
    }
    let result: i32 = results
        .values()
        .filter(|x| x.len() == 2)
        .map(|x| x.iter().product::<i32>())
        .sum();
    println!("Result: {result}");
}
