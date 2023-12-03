use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Eq, Hash)]
struct GearCoordinates {
    x: u32,
    y: u32,
}

impl GearCoordinates {
    fn new(x: u32, y: u32) -> Self { Self { x, y } }
}

// verify if hashed keys are compared by pointer or by equality
fn main() {
    dbg!(GearCoordinates::new(1, 2));
    let mut map = HashMap::new();
    map.insert(GearCoordinates::new(1, 2), vec![1, 2, 3]);
    dbg!(map.get(&GearCoordinates::new(1, 2)));
}
