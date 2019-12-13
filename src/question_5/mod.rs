use crate::util::{read_content};
use std::str::FromStr;
use crate::int_code::run_program;

pub fn solve_a() {
    let contents = read_content("src/question_5/input.txt");
    let numbers: Vec<i64> = contents.split(",").map(|number| i64::from_str(number).unwrap()).collect();
    println!("size: {}", numbers.len());

    let mut op_codes = [0i64; 1024];
    for (i, number) in numbers.iter().enumerate() {
        op_codes[i] = *number
    }

    let mut current_position = 0usize;

    run_program(&mut op_codes, &Vec::new(), &mut current_position, &mut 0);
}

