use day_08::Place;
use num_integer::lcm;

fn main() {
    let input = include_str!("../../resources/input.txt");
    let (d, p) = input.split_once("\n\n").unwrap();

    let places: Vec<Place> = p.lines().map(Place::from).collect();
    let starting_places: Vec<Place> = places
        .iter()
        .filter(|x| x.place.ends_with('A'))
        .cloned()
        .collect();
    dbg!(&starting_places);
    let mut results: Vec<usize> = Vec::new();
    'outer: for p in &starting_places {
        let mut curr = p;
        let mut i = 0;
        dbg!(&p);
        loop {
            for c in d.chars() {
                if c == 'L' {
                    curr = places.iter().find(|x| curr.left == x.place).unwrap();
                } else if c == 'R' {
                    curr = places.iter().find(|x| curr.right == x.place).unwrap();
                }
                i += 1;
                if curr.place.ends_with('Z') {
                    results.push(i);
                    continue 'outer;
                }
            }
        }
    }

    let mut x = lcm(results[0], results[1]);
    dbg!(&results);
    for y in results.into_iter().skip(2) {
        x = lcm(x, y);
    }
    println!("x: {x}");
}
