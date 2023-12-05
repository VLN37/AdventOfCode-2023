use std::collections::HashSet;

// 1 2 4 8 16
fn geometric_progression(n: usize, ratio: usize) -> usize {
    if n < 1 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    ratio * geometric_progression(n - 1, ratio)
}

fn main() {
    let str = include_str!("../../resources/input.txt");

    let mut results: Vec<usize> = Vec::new();
    for line in str.lines().map(|x| x.split(':').last().unwrap()) {
        let (ticket, numbers) = line.split_once('|').unwrap();
        let ticket_numbers: HashSet<usize> = ticket
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let my_numbers: HashSet<usize> = numbers
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let qty = ticket_numbers.intersection(&my_numbers).count();
        results.push(geometric_progression(qty, 2));
    }
    println!("Result: {}", results.iter().sum::<usize>());
}
