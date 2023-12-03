use day_03::{distance, surroundings};

fn main() {
    let input = include_str!("../../resources/input.txt");
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut results: Vec<usize> = Vec::new();

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
                println!("distance {distance}");
                println!("slice {:?}", slice);
                if surroundings(&matrix, i, j, distance) {
                    println!("FOUND");
                    let str: String = matrix[i][j..j + distance].iter().collect();
                    results.push(str.parse().unwrap());
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
    println!("Result: {}", results.iter().sum::<usize>());
}
