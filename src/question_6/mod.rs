use regex::Regex;
use crate::util::read_content;
use std::collections::HashMap;

pub fn solve_a() {
    let mut contents = read_content("src/question_6/input.txt");
    let orbits: HashMap<_,_> = contents.lines().map(|l| extract_orbit_info(l)).collect();
    let mut orbits_count = 0u32;
    for (k, _) in &orbits {
        orbits_count += count_orbits(&k, &orbits);
    }
    println!("Number of orbits: {}", orbits_count);
}

pub fn solve_b() {
    let mut contents = read_content("src/question_6/input.txt");
    let orbits: HashMap<_,_> = contents.lines().map(|l| extract_orbit_info(l)).collect();

    let mut you_path = Vec::new();
    get_path(&"YOU".to_string(), &orbits, &mut you_path);
    println!("YOU path = {:?}", you_path);

    let mut santa_path = Vec::new();
    get_path(&"SAN".to_string(), &orbits, &mut santa_path);
    println!("SAN path = {:?}", santa_path);

    println!("Distance = {}", calculate_path_between(&mut you_path, &mut santa_path));
}

fn calculate_path_between(path_1: &mut Vec<String>, path_2: &mut Vec<String>) -> usize {
    path_1.reverse();
    path_2.reverse();

    let mut common_index = 0usize;

    while path_1[common_index] == path_2[common_index] {
        common_index += 1;
    }

    (path_1.len() - common_index) + (path_2.len() - common_index)
}

fn get_path(key: &String, orbits: &HashMap<String, String>, path: &mut Vec<String>) {
    match orbits.get(key) {
        Some(value) => {
            path.push(value.to_string());
            get_path(&value, orbits, path);
        }
        None => {}
    }
}

fn count_orbits(key: &String, orbits: &HashMap<String, String>) -> u32 {
    orbits.get(key).map_or_else(|| 0, |v| 1 + count_orbits(v, orbits))
}

fn extract_orbit_info(str: &str) -> (String, String) {
    let re = Regex::new(r"(.*?)\)(.*)").unwrap();
    let captures = re.captures(str).unwrap();
    (captures.get(2).unwrap().as_str().to_string(), captures.get(1).unwrap().as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_orbit_info() {
        assert_eq!(extract_orbit_info("A7S9)8AJ22"), ("8AJ22".to_string(), "A7S9".to_string()));
    }
}