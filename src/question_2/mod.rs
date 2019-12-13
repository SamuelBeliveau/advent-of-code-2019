use crate::util::read_content;
use std::str::FromStr;

pub fn solve_a() {
    let contents = read_content("src/question_2/input.txt");
    let numbers: Vec<usize> = contents.split(",").map(|number| usize::from_str(number).unwrap()).collect();

    let mut op_codes = [0usize; 512];
    for (i, number) in numbers.iter().enumerate() {
        match i {
            1 => op_codes[i] = 12,
            2 => op_codes[i] = 2,
            _ => op_codes[i] = *number
        }
    }

    run_program(&mut op_codes);
    println!("Opcode at 0 is {}", op_codes[0]);
}

pub fn solve_b() {
    let contents = read_content("src/question_2/input.txt");
    let numbers: Vec<usize> = contents.split(",").map(|number| usize::from_str(number).unwrap()).collect();

    let mut op_codes;

    for i in 0..=99 {
        for j in 0..=99 {
            op_codes = [0; 512];
            for (index, number) in numbers.iter().enumerate() {
                match index {
                    1 => op_codes[index] = i,
                    2 => op_codes[index] = j,
                    _ => op_codes[index] = *number
                }
            }

            run_program(&mut op_codes);
            if op_codes[0] == 19690720 {
                println!("Found! Result = 100*{}+{}={}", i, j, (100*i) + j);
                return;
            }
        }
    }
}

fn run_program(op_codes: &mut [usize]) {
    let mut current_position = 0usize;
    let mut current_opcode = op_codes[current_position];

    while current_opcode != 99 {
        match current_opcode {
            1 => {
                op_codes[op_codes[current_position + 3]] = op_codes[op_codes[current_position + 1]] + op_codes[op_codes[current_position + 2]];
            }
            2 => {
                op_codes[op_codes[current_position + 3]] = op_codes[op_codes[current_position + 1]] * op_codes[op_codes[current_position + 2]];
            }
            _ => {}
        }
        current_position += 4;
        current_opcode = op_codes[current_position];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program_1() {
        let mut op_codes = [1, 0, 0, 0, 99];
        run_program(&mut op_codes);
        assert_eq!(op_codes, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_run_program_2() {
        let mut op_codes = [2, 3, 0, 3, 99];
        run_program(&mut op_codes);
        assert_eq!(op_codes, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_run_program_3() {
        let mut op_codes = [2, 4, 4, 5, 99, 0];
        run_program(&mut op_codes);
        assert_eq!(op_codes, [2, 4, 4, 5, 99, 9801]);
    }
}