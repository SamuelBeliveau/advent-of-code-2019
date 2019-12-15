use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub fn read_content(path: &str) -> String {
    let mut content = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut content)
        .expect("Failed to read file!");
    content
}

pub fn read_op_codes(path: &str) -> [i64; 2048] {
    let contents = read_content(path);
    let numbers: Vec<i64> = contents.split(",").map(|number| i64::from_str(number).unwrap()).collect();

    let mut op_codes = [0i64; 2048];
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