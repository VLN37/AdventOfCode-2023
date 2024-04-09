use day_11::Universe;

fn main() {
    let input = include_str!("../../resources/input.txt");
    let mut uni = Universe::from(input);
    uni.expand();
    println!("expand");
    let mut galaxies = uni.galaxies();
    let clone = galaxies.clone();
    println!("clone");
    // dbg!(&galaxies);
    for g in galaxies.iter_mut() {
        println!("g");
        g.distances = clone
            .iter()
            .map(|x| (g.y - x.y).abs() + (g.x - x.x).abs())
            .collect();
        dbg!(g.id, g.x, g.y, &g.distances);
    }
    let sum: i64 = galaxies
        .iter()
        .map(|x| x.distances.iter().sum::<i64>())
        .sum();
    dbg!(sum / 2);
}
