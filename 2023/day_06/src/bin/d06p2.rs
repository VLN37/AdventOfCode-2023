fn main() {
    let input = include_str!("../../resources/input.txt");
    let mut input = input
        .lines()
        .map(|x| x.split(':').last().unwrap())
        .map(|x| x.replace(' ', ""));

    let time = input.next().unwrap().parse::<usize>().unwrap();
    let distance = input.next().unwrap().parse::<usize>().unwrap();

    let mut n = 0;
    for i in 0..time {
        if (time - i) * i > distance {
            n += 1;
        }
    }
    println!("Result: {}", n);
}
