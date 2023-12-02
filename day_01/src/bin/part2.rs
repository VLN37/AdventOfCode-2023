use std::collections::HashMap;

// fn first_digit(str: &str) -> Option<char> {
//     for char in str.as_bytes().iter() {
//         if char.is_ascii_digit() {
//             return Some(*char as char);
//         }
//     }
//     None
// }

fn first_and_last(str: &str) -> u32 {
    let mut string = String::new();
    let idx1 = str.find(|c: char| c.is_ascii_digit()).unwrap();
    let idx2 = str.rfind(|c: char| c.is_ascii_digit()).unwrap();
    let bytes = str.as_bytes();

    string.push(bytes[idx1].into());
    string.push(bytes[idx2].into());
    string.parse().unwrap()
}

fn main() {
    let numbers_as_strings: Vec<String> = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
        String::from("five"),
        String::from("six"),
        String::from("seven"),
        String::from("eight"),
        String::from("nine"),
        String::from("ten"),
    ];
    let conversion_table = HashMap::from([
        (String::from("one"), String::from("1")),
        (String::from("two"), String::from("2")),
        (String::from("three"), String::from("3")),
        (String::from("four"), String::from("4")),
        (String::from("five"), String::from("5")),
        (String::from("six"), String::from("6")),
        (String::from("seven"), String::from("7")),
        (String::from("eight"), String::from("8")),
        (String::from("nine"), String::from("9")),
    ]);

    let text = include_str!("../../resources/input.txt");
    let mut line_results: Vec<String> = Vec::new();
    let mut _found = false;

    for line in text.lines() {
        let mut result = String::new();
        let mut translate = String::new();
        let mut i = 0;
        while i < line.len() {
            _found = false;
            for word in &numbers_as_strings {
                if line[i..].starts_with(word) {
                    translate.push_str(&conversion_table[word]);
                    i += word.len();
                    _found = true;
                    break;
                }
            }
            if !_found {
                translate.push(line.chars().nth(i).unwrap());
                i += 1;
            }
        }

        for char in translate.chars() {
            if char.is_ascii_digit() {
                result.push(char);
                break;
            }
        }

        translate.clear();
        i = line.len() - 1;

        while i != usize::MAX {
            _found = false;
            for word in &numbers_as_strings {
                if line[i..].starts_with(word) {
                    translate.push_str(&conversion_table[word]);
                    // i -= word.len();
                    _found = true;
                    break;
                }
            }
            if !_found {
                translate.push(line.chars().nth(i).unwrap());
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        for char in translate.chars() {
            if char.is_ascii_digit() {
                result.push(char);
                break;
            }
        }
        line_results.push(result);
    }
    println!(
        "result: {}",
        line_results.iter().map(|x| first_and_last(x)).sum::<u32>()
    )
}
