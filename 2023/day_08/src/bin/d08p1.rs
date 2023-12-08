use day_08::Place;

fn main() {
    let input = include_str!("../../resources/input.txt");
    let (d, p) = input.split_once("\n\n").unwrap();

    let places: Vec<Place> = p.lines().map(Place::from).collect();

    let mut i = 0;
    let mut curr = places.iter().find(|x| x.place == "AAA").unwrap();
    'outer: loop {
        for c in d.chars() {
            if c == 'L' {
                curr = places.iter().find(|x| curr.left == x.place).unwrap();
            } else if c == 'R' {
                curr = places.iter().find(|x| curr.right == x.place).unwrap();
            }
            i += 1;
            if curr.place == "ZZZ" {
                break 'outer;
            }
        }
    }
    println!("Result: {i}");
}
