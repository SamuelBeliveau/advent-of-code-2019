use crate::util::{read_content};
use std::str::FromStr;
use crate::int_code::run_program;

pub fn solve_a() {
    let mut op_codes = read_op_codes("src/question_5/input.txt");

    let mut current_position = 0usize;

    run_program(&mut op_codes, &Vec::new(), &mut current_position, &mut 0);
}

