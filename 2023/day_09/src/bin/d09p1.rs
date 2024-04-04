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

    let mut next_values: Vec<i32> = Vec::new();
    let mut previous_values: Vec<i32> = Vec::new();

    for sequence in sequences {
        let mut reductions: Vec<Vec<i32>> = Vec::new();
        reductions.push(sequence);
        loop {
            let mut next_step: Vec<i32> = Vec::new();
            let s = reductions.last().unwrap();

            for i in 0..s.len() - 1 {
                next_step.push(s[i + 1] - s[i]);
            }
            if next_step.iter().all(|x| *x == 0) {
                reductions.push(next_step);
                reductions.last_mut().unwrap().insert(0, 0);
                for i in (1..reductions.len()).rev() {
                    let curr = reductions[i].first().unwrap().to_owned();
                    let next = reductions[i - 1].first().unwrap().to_owned();
                    reductions[i - 1].insert(0, next - curr);
                }
                let sum = reductions.iter().map(|x| *x.last().unwrap()).sum();
                next_values.push(sum);
                let prev = reductions.first().unwrap().first().unwrap().to_owned();
                previous_values.push(prev);
                break;
            }
            reductions.push(next_step);
        }
        dbg!(&reductions);
    }
    dbg!(&next_values);
    dbg!(&previous_values);
    let next: i32 = next_values.iter().sum();
    dbg!(next);
    let prev: i32 = previous_values.iter().sum();
    dbg!(prev);
}
