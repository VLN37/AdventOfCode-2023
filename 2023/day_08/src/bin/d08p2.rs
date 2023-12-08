use day_08::Place;

fn main() {
    let input = include_str!("../../resources/input.txt");
    let (d, p) = input.split_once("\n\n").unwrap();

    let places: Vec<Place> = p.lines().map(Place::from).collect();

    let mut i = 0;
    let mut currs: Vec<Place> = places
        .iter()
        .filter(|x| x.place.ends_with('A'))
        .cloned()
        .collect();
    dbg!(currs.len());
    'outer: loop {
        for c in d.chars() {
            let mut next: Vec<Place> = Vec::with_capacity(currs.len());
            for curr in &currs {
                if c == 'L' {
                    let place = places.iter().find(|x| curr.left == x.place).unwrap();
                    next.push(place.clone());
                } else if c == 'R' {
                    let place = places.iter().find(|x| curr.right == x.place).unwrap();
                    next.push(place.clone());
                }
            }
            currs = next;
            // dbg!(&currs);
            i += 1;
            if currs.iter().all(|x| x.place.ends_with('Z')) {
                break 'outer;
            }
            if i % 1000000 == 0 {
                dbg!(i / 1000000);
            }
        }
    }
    println!("Result: {i}");
}
