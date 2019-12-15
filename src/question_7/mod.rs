use crate::util::{read_op_codes};
use crate::int_code::run_program;
use itertools::Itertools;
use std::cell::RefCell;

pub fn solve_a() {
    let op_codes_master = read_op_codes("src/question_7/input.txt");

    let mut max_output = 0;

    let possible_phases = vec![0, 1, 2, 3, 4];
    let all_possibilities = possible_phases.iter()
        .cartesian_product(possible_phases.iter())
        .cartesian_product(possible_phases.iter())
        .cartesian_product(possible_phases.iter())
        .cartesian_product(possible_phases.iter());

    for possibility in all_possibilities {
        let ((((first, second), third), fourth), fifth) = possibility;
        if first == second || first == third || first == fourth || first == fifth || second == third ||
            second == fourth || second == fifth || third == fourth || third == fifth || fourth == fifth {
            continue;
        }

        println!("Test: {}, {}, {}, {}, {}", first, second, third, fourth, fifth);
        let mut op_codes = op_codes_master.clone();

        let output = run_amplifiers(&mut op_codes, &[*first, *second, *third, *fourth, *fifth]);
        if output > max_output {
            max_output = output;
        }
    }

    println!("Max output is {}", max_output);
}

pub fn solve_b() {
    let mut op_codes_master = read_op_codes("src/question_7/input.txt");

    let mut max_output = 0;

    let possible_phases = vec![5, 6, 7, 8, 9];
    let all_possibilities = possible_phases.iter()
        .cartesian_product(possible_phases.iter())
        .cartesian_product(possible_phases.iter())
        .cartesian_product(possible_phases.iter())
        .cartesian_product(possible_phases.iter());

    for possibility in all_possibilities {
        let ((((first, second), third), fourth), fifth) = possibility;
        if first == second || first == third || first == fourth || first == fifth || second == third ||
            second == fourth || second == fifth || third == fourth || third == fifth || fourth == fifth {
            continue;
        }

        println!("Test: {}, {}, {}, {}, {}", first, second, third, fourth, fifth);

        let output = run_amplifiers_feedback_loop(&mut op_codes_master, &[*first, *second, *third, *fourth, *fifth]);
        if output > max_output {
            max_output = output;
        }
    }

    println!("Max output is {}", max_output);
}

fn run_amplifiers(op_codes: &mut [i64], phase_sequence: &[i64]) -> i64 {
    let mut output = 0;
    for phase in phase_sequence {
        // TODO: op_codes should be cloned each time?
        let mut current_position = 0usize;
        output = run_amplifier(op_codes, *phase, output, &mut current_position);
    }
    output
}

fn run_amplifiers_feedback_loop(op_codes: &[i64], phase_sequence: &[i64]) -> i64 {
    let mut current_amplifier_index = 0usize;
    let mut amplifiers = vec![
        op_codes.clone().to_vec(),
        op_codes.clone().to_vec(),
        op_codes.clone().to_vec(),
        op_codes.clone().to_vec(),
        op_codes.clone().to_vec()
    ];
    let mut amplifiers_positions = vec![RefCell::new(0usize); 5];
    let mut output = 0;

    let mut current_amplifier;

    for _ in 0..phase_sequence.len() {
        current_amplifier = &mut amplifiers[current_amplifier_index];
        output = run_amplifier(&mut current_amplifier[..], phase_sequence[current_amplifier_index], output, &mut amplifiers_positions[current_amplifier_index].borrow_mut());
        current_amplifier_index = (current_amplifier_index + 1) % 5;
    }

    while output != -1 {
        current_amplifier = &mut amplifiers[current_amplifier_index];
        let current_output = run_amplifier(&mut current_amplifier[..], output, 0, &mut amplifiers_positions[current_amplifier_index].borrow_mut());
        if current_output == -1 {
            break;
        }
        output = current_output;
        current_amplifier_index = (current_amplifier_index + 1) % 5;
    }

    output
}

fn run_amplifier(op_codes: &mut [i64], phase: i64, input: i64, current_position: &mut usize) -> i64 {
    let inputs = vec![phase, input];
    run_program(op_codes, &inputs, current_position, &mut 0).unwrap_or_else(|| -1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_amplifiers_1() {
        let mut op_codes = [3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
        let phase_sequence = [4, 3, 2, 1, 0];

        assert_eq!(run_amplifiers(&mut op_codes, &phase_sequence), 43210);
    }

    #[test]
    fn test_run_amplifiers_2() {
        let mut op_codes = [3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23,
            101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0];
        let phase_sequence = [0, 1, 2, 3, 4];

        assert_eq!(run_amplifiers(&mut op_codes, &phase_sequence), 54321);
    }

    #[test]
    fn test_run_amplifiers_3() {
        let mut op_codes = [3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33,
            1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0];
        let phase_sequence = [1, 0, 4, 3, 2];

        assert_eq!(run_amplifiers(&mut op_codes, &phase_sequence), 65210);
    }

    #[test]
    fn test_run_amplifiers_feedback_loop_1() {
        let mut op_codes = [3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
            27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5];
        let phase_sequence = [9, 8, 7, 6, 5];

        assert_eq!(run_amplifiers_feedback_loop(&mut op_codes, &phase_sequence), 139629729);
    }

    #[test]
    fn test_run_amplifiers_feedback_loop_2() {
        let mut op_codes = [3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10];
        let phase_sequence = [9, 7, 8, 5, 6];

        assert_eq!(run_amplifiers_feedback_loop(&mut op_codes, &phase_sequence), 18216);
    }
}