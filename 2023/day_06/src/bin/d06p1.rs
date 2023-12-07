fn main() {
    let input = include_str!("../../resources/input.txt");
    let mut input = input.lines().map(|x| x.split(':').last().unwrap());

    let t = input.next().unwrap();
    let d = input.next().unwrap();

    let mut time: Vec<usize> = Vec::new();
    time.extend(t.split_whitespace().map(|x| x.parse::<usize>().unwrap()));
    let mut distance: Vec<usize> = Vec::new();
    distance.extend(d.split_whitespace().map(|x| x.parse::<usize>().unwrap()));

    let mut results = Vec::new();
    for (&t, &d) in time.iter().zip(distance.iter()) {
        let mut n = 0;
        for i in 0..t {
            if (t - i) * i > d {
                n += 1;
            }
        }
        results.push(n);
    }
    println!("Result: {}", results.iter().product::<usize>());
}
