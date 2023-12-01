fn first_and_last(str: &str) -> u32 {
    let mut string = String::new();
    let idx1 = str.find(|c: char| c.is_ascii_digit()).unwrap();
    let idx2 = str.rfind(|c: char| c.is_ascii_digit()).unwrap();
    let bytes = str.as_bytes();

    string.push(bytes[idx1] as char);
    string.push(bytes[idx2] as char);
    string.parse().unwrap()
}

fn main() {
    let input = include_str!("../../resources/input.txt");
    let res = input.lines().map(first_and_last).sum::<u32>();
    println!("Result: {}", res);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_1_part_1() {
        assert_eq!(first_and_last("rhqrpdx2sqhgxzknr2foursnrcfthree"), 22);
        assert_eq!(first_and_last("2"), 22);
        assert_eq!(first_and_last("2onetwo2"), 22);
        assert_eq!(first_and_last("2bmckl"), 22);
        assert_eq!(first_and_last("9four6dk7gvv"), 97);
        assert_eq!(first_and_last("six43fourthree2"), 42);
    }
}
