use crate::int_code::run_program;
use crate::util::{read_op_codes};

pub fn solve_a() {
    let mut op_codes_master = read_op_codes("src/question_9/input.txt");

    run_sensor_boost(&mut op_codes_master, &vec![2]);
}

fn run_sensor_boost(op_codes: &mut [i64], inputs: &Vec<i64>) {
    let mut current_position = 0;
    let mut relative_base = 0;
    while run_program(op_codes, inputs, &mut current_position, &mut relative_base).is_some() {}
}