use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::collections::HashMap;

pub fn read_content(path: &str) -> String {
    let mut content = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut content)
        .expect("Failed to read file!");
    content
}

pub fn read_op_codes(path: &str) -> [i64; 4096] {
    let contents = read_content(path);
    let numbers: Vec<i64> = contents.split(",").map(|number| i64::from_str(number).unwrap()).collect();

    let mut op_codes = [0i64; 4096];
    for (i, number) in numbers.iter().enumerate() {
        op_codes[i] = *number
    }
    op_codes
}

pub fn extract_numbers(number: u32) -> Vec<u8> {
    let mut n = number;
    let mut numbers = Vec::new();

    while n > 0 {
        numbers.push((n % 10) as u8);
        n = n / 10;
    }

    numbers.reverse();

    numbers
}

pub fn print_positions<'a, T, F>(hash_map: &HashMap<(i64, i64), T>, map_value: F)
    where F: Fn(Option<&T>) -> &'a str {
    let mut boundaries = (std::i64::MAX, std::i64::MIN, std::i64::MAX, std::i64::MIN);
    for key in hash_map.keys() {
        if key.0 < boundaries.0 {
            boundaries.0 = key.0;
        }
        if key.0 > boundaries.1 {
            boundaries.1 = key.0;
        }
        if key.1 < boundaries.2 {
            boundaries.2 = key.1;
        }
        if key.1 > boundaries.3 {
            boundaries.3 = key.1;
        }
    }

    for y in boundaries.2..=boundaries.3 {
        for x in boundaries.0..=boundaries.1 {
            print!("{}", map_value(hash_map.get(&(x, y))));
        }
        println!("");
    }
}