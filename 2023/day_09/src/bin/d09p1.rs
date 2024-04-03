fn main() {
    let input = include_str!("../../resources/input.txt");

    let lines: Vec<String> = input.lines().map(String::from).collect();

    let mut sequences: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let v: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        sequences.push(v);
    }

    let mut extrapolations: Vec<i32> = Vec::new();

    for sequence in sequences {
        let mut reductions: Vec<Vec<i32>> = Vec::new();
        reductions.push(sequence);
        loop {
            let mut next_step: Vec<i32> = Vec::new();
            let s = reductions.last().unwrap();

            for i in 0..s.len() - 1 {
                next_step.push(s[i + 1] - s[i]);
            }
            if s.iter().all(|x| *x == 0) {
                let sum = reductions.into_iter().map(|x| *x.last().unwrap()).sum();
                extrapolations.push(sum);
                break;
            }
            reductions.push(next_step);
        }
    }
    let result: i32 = extrapolations.iter().sum();
    dbg!(result);
}
