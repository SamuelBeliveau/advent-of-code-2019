use std::fs::File;
use std::io::Read;
use std::str::Lines;

pub fn read_content(path: &str) -> String {
    let mut content = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut content)
        .expect("Failed to read file!");
    content
}