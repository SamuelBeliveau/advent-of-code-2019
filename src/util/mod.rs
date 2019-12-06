use std::fs::File;
use std::io::Read;

pub fn read_content(path: &str) -> String {
    let mut content = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut content)
        .expect("Failed to read file!");
    content
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