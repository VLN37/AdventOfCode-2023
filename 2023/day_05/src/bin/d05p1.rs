use day_05::Instructions;

// seed - soil
// soil - fert
// fert - water
// water - light
// light - temp
// temp - humidity
// humidity - location

fn main() {
    let input = include_str!("../../resources/input.txt");
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
    for (i, value) in clean.split("\n\n").enumerate() {
        instructions.insert(i, value.split('\n').map(Instructions::from).collect());
    }
    let mut results: Vec<i64> = Vec::new();
    for seed in seeds {
        let mut n = seed;
        for vec in instructions.iter() {
            for line in vec {
                if n >= line.src && n < line.src + line.n {
                    n = line.dst + n - line.src;
                    // println!("new {n}");
                    break;
                }
            }
        }
        results.push(n);
    }
    println!("Result: {}", results.iter().min().unwrap());
}
