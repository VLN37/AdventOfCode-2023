use std::collections::HashMap;

use day_05::Instructions;

fn main() {
    // seed - soil
    // soil - fert
    // fert - water
    // water - light
    // light - temp
    // temp - humidity
    // humidity - location
    let input = include_str!("../../resources/small_input.txt");
    let seeds: Vec<usize> = input[..input.find("\n\n").unwrap()]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    dbg!(1);

    let mut seed_soil: HashMap<usize, usize> = HashMap::new();
    let mut soil_fert: HashMap<usize, usize> = HashMap::new();
    let mut fert_water: HashMap<usize, usize> = HashMap::new();
    let mut water_light: HashMap<usize, usize> = HashMap::new();
    let mut light_temp: HashMap<usize, usize> = HashMap::new();
    let mut temp_humid: HashMap<usize, usize> = HashMap::new();
    let mut humid_locale: HashMap<usize, usize> = HashMap::new();
    let split = input
        .split('\n')
        .filter(|x| x.find(':').is_none())
        .skip(1)
        .collect::<Vec<&str>>();

    dbg!(2);
    let mut start = 0;
    let mut end = 0;
    while !split[end].is_empty() {
        end += 1;
    }
    let instructions = split[start..end].iter().map(|x| Instructions::from(*x));
    for range in instructions {
        dbg!("instruction...");
        for i in 0..range.n {
            seed_soil.insert(range.src + i, range.dst + i);
        }
    }
    end += 1;
    start = end;

    dbg!(3);
    while !split[end].is_empty() {
        end += 1;
    }
    let instructions = split[start..end].iter().map(|x| Instructions::from(*x));
    for range in instructions {
        dbg!("instruction...");
        for i in 0..range.n {
            soil_fert.insert(range.src + i, range.dst + i);
        }
    }
    end += 1;
    start = end;

    dbg!(4);
    while !split[end].is_empty() {
        end += 1;
    }
    let instructions = split[start..end].iter().map(|x| Instructions::from(*x));
    for range in instructions {
        dbg!("instruction...");
        for i in 0..range.n {
            fert_water.insert(range.src + i, range.dst + i);
        }
    }
    end += 1;
    start = end;

    dbg!(5);
    while !split[end].is_empty() {
        end += 1;
    }
    let instructions = split[start..end].iter().map(|x| Instructions::from(*x));
    for range in instructions {
        dbg!("instruction...");
        for i in 0..range.n {
            water_light.insert(range.src + i, range.dst + i);
        }
    }
    end += 1;
    start = end;

    dbg!(6);
    while !split[end].is_empty() {
        end += 1;
    }
    let instructions = split[start..end].iter().map(|x| Instructions::from(*x));
    for range in instructions {
        dbg!("instruction...");
        for i in 0..range.n {
            light_temp.insert(range.src + i, range.dst + i);
        }
    }
    end += 1;
    start = end;

    dbg!(7);
    while !split[end].is_empty() {
        end += 1;
    }
    let instructions = split[start..end].iter().map(|x| Instructions::from(*x));
    for range in instructions {
        dbg!("instruction...");
        for i in 0..range.n {
            temp_humid.insert(range.src + i, range.dst + i);
        }
    }
    end += 1;
    start = end;

    dbg!(8);
    while !split[end].is_empty() {
        end += 1;
    }
    let instructions = split[start..end].iter().map(|x| Instructions::from(*x));
    for range in instructions {
        dbg!("instruction...");
        for i in 0..range.n {
            humid_locale.insert(range.src + i, range.dst + i);
        }
    }
    // end += 1;
    // start = end;

    dbg!(9);
    let mut results: Vec<usize> = Vec::new();
    for seed in seeds {
        dbg!(1);
        let soil = seed_soil.get(&seed).unwrap_or(&seed);
        dbg!(2);
        let fert = soil_fert.get(soil).unwrap_or(soil);
        let water = fert_water.get(fert).unwrap_or(fert);
        let light = water_light.get(water).unwrap_or(water);
        let temp = light_temp.get(light).unwrap_or(light);
        let humid = temp_humid.get(temp).unwrap_or(temp);
        let locale = humid_locale.get(humid).unwrap_or(humid);
        results.push(*locale);
    }
    dbg!(results.iter().min().unwrap());
    // dbg!(&split[start..end]);
    // split.iter().chu;
    // dbg!(split);
    // dbg!(_soil_fert);
}
