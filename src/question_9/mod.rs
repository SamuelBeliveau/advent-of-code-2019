use crate::int_code::run_program;
use crate::util::read_content;
use std::str::FromStr;

pub fn solve_a() {
    let contents = read_content("src/question_9/input.txt");
    let numbers: Vec<i64> = contents.split(",").map(|number| i64::from_str(number).unwrap()).collect();
    println!("Numbers count: {}", numbers.len());

    let mut op_codes_master = [0i64; 2048];
    for (i, number) in numbers.iter().enumerate() {
        op_codes_master[i] = *number
    }

    run_sensor_boost(&mut op_codes_master, &vec![2]);
}

fn run_sensor_boost(op_codes: &mut [i64], inputs: &Vec<i64>) {
    let mut current_position = 0;
    let mut relative_base = 0;
    while run_program(op_codes, inputs, &mut current_position, &mut relative_base).is_some() {}
}