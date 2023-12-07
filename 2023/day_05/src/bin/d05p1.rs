use day_05::Instructions;

fn main() {
    let input = include_str!("../../resources/small_input.txt");
    let seeds: Vec<i64> = input[..input.find("\n\n").unwrap()]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut clean = input
        .split('\n')
        .filter(|x| x.find(':').is_none())
        .skip(1)
        .collect::<Vec<&str>>()
        .join("\n");
    clean.pop();
    let mut instructions: Vec<Vec<Instructions>> = Vec::new();
    for value in clean.split("\n\n") {
        instructions.push(value.split('\n').map(Instructions::from).collect());
    }
    let mut results: Vec<i64> = Vec::new();
    for mut seed in seeds {
        for vec in instructions.iter() {
            for line in vec {
                if seed >= line.src && seed < line.src + line.n {
                    seed += line.dst - line.src;
                    break;
                }
            }
        }
        results.push(seed);
    }
    println!("Result: {}", results.iter().min().unwrap());
}
