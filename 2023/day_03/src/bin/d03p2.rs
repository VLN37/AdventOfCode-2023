use std::collections::HashMap;

use day_03::{distance, star_coordinates, surroundings, GearCoordinates};

fn main() {
    let input = include_str!("../../resources/small_input.txt");
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut results: HashMap<_, Vec<i32>> = HashMap::new();

    for line in input.lines() {
        matrix.push(line.chars().collect())
    }
    let width = matrix[0].len() + 2;
    let height = matrix.len() + 2;

    for line in &mut matrix {
        line.insert(0, '?');
        line.push('?');
    }

    matrix.insert(0, vec!['?'; width]);
    matrix.push(vec!['?'; width]);

    for line in &matrix {
        println!("{:?}", line);
    }
    println!("-------------");

    let mut i = 1;
    let mut j = 1;
    while i < height - 1 {
        // dbg!(i);
        while j < width - 1 {
            if matrix[i][j].is_ascii_digit() {
                let slice = &matrix[i][j..];
                let distance = distance(slice);
                // println!("distance {distance}");
                // println!("slice {:?}", slice);
                if surroundings(&matrix, i, j, distance) {
                    if let Some((x, y)) = star_coordinates(&matrix, i, j, distance) {
                        println!("FOUND");
                        let str: String = matrix[i][j..j + distance].iter().collect();
                        let nbr = str.parse::<i32>().unwrap();
                        let coord = GearCoordinates::new(x, y);
                        let opt = results.get_mut(&coord);
                        match opt {
                            Some(vec) => vec.push(nbr),
                            None => {
                                results.insert(coord, vec![nbr]);
                            }
                        }
                    }
                } else {
                    println!("NOT FOUND");
                }
                println!("-------------");
                j += distance;
                continue;
            }
            j += 1;
        }
        j = 1;
        i += 1;
    }
    dbg!(&results);
    let result: i32 = results
        .values()
        .filter(|x| x.len() == 2)
        .map(|x| x.iter().product::<i32>())
        .sum();
    println!("Result: {result}");
}
