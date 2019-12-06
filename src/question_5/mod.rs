use crate::util::{extract_numbers, read_content};
use std::io::stdin;
use std::str::FromStr;

pub fn solve_a() {
    let contents = read_content("src/question_5/input.txt");
    let numbers: Vec<i32> = contents.split(",").map(|number| i32::from_str(number).unwrap()).collect();
    println!("size: {}", numbers.len());

    let mut op_codes = [0i32; 1024];
    for (i, number) in numbers.iter().enumerate() {
        op_codes[i] = *number
    }

    run_program(&mut op_codes);
}

fn run_program(op_codes: &mut [i32]) {
    let mut current_position = 0usize;
    let mut current_instruction = get_next_instruction(&op_codes, &mut current_position);

    while current_instruction.op_code != OpCode::End {
        match current_instruction.op_code {
            OpCode::Add => {
                let first_operand = get_parameter_value(&current_instruction.parameters[0], &op_codes);
                let second_operand = get_parameter_value(&current_instruction.parameters[1], &op_codes);

                op_codes[current_instruction.parameters[2].value as usize] = first_operand + second_operand;
            }
            OpCode::Multiply => {
                let first_operand = get_parameter_value(&current_instruction.parameters[0], &op_codes);
                let second_operand = get_parameter_value(&current_instruction.parameters[1], &op_codes);

                op_codes[current_instruction.parameters[2].value as usize] = first_operand * second_operand;
            }
            OpCode::Input => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(n) => {
                        op_codes[current_instruction.parameters[0].value as usize] = input.trim().parse().unwrap();
                    }
                    Err(error) => println!("error: {}", error)
                }
            }
            OpCode::Output => {
                println!("Output is: {}", op_codes[current_instruction.parameters[0].value as usize])
            }
            OpCode::JumpIfTrue => {
                let first_param = get_parameter_value(&current_instruction.parameters[0], &op_codes);
                let second_param = get_parameter_value(&current_instruction.parameters[1], &op_codes);
                if first_param != 0 {
                    current_position = second_param as usize;
                }
            }
            OpCode::JumpIfFalse => {
                let first_param = get_parameter_value(&current_instruction.parameters[0], &op_codes);
                let second_param = get_parameter_value(&current_instruction.parameters[1], &op_codes);
                if first_param == 0 {
                    current_position = second_param as usize;
                }
            }
            OpCode::LessThan => {
                let first_param = get_parameter_value(&current_instruction.parameters[0], &op_codes);
                let second_param = get_parameter_value(&current_instruction.parameters[1], &op_codes);
                op_codes[current_instruction.parameters[2].value as usize] = if first_param < second_param { 1 } else { 0 };
            }
            OpCode::Equal => {
                let first_param = get_parameter_value(&current_instruction.parameters[0], &op_codes);
                let second_param = get_parameter_value(&current_instruction.parameters[1], &op_codes);
                op_codes[current_instruction.parameters[2].value as usize] = if first_param == second_param { 1 } else { 0 };
            }
            _ => {}
        }

        current_instruction = get_next_instruction(&op_codes, &mut current_position);
    }
}

fn get_parameter_value(parameter: &Parameter, op_codes: &[i32]) -> i32 {
    if parameter.mode == ParameterMode::Position {
        op_codes[parameter.value as usize]
    } else {
        parameter.value
    }
}

fn get_next_instruction(op_codes: &[i32], current_index: &mut usize) -> Instruction {
    let metadata = extract_numbers(op_codes[*current_index] as u32);
    let metadata_len = metadata.len();

    let tens = if metadata_len >= 2 { metadata[metadata_len - 2] } else { 0 };

    let op_code = number_to_opcode((tens * 10) + metadata[metadata_len - 1]);
    let number_of_params = number_of_parameters(&op_code) as usize;
    let mut parameters = Vec::new();

    for i in 1..=number_of_params {
        parameters.push(Parameter {
            value: op_codes[*current_index + i],
            mode: if metadata_len >= 2 + i && metadata[metadata_len - 2 - i] == 1 {
                ParameterMode::Immediate
            } else {
                ParameterMode::Position
            },
        });
    }

    *current_index = *current_index + number_of_params + 1;

    Instruction {
        op_code,
        parameters,
    }
}

fn number_to_opcode(number: u8) -> OpCode {
    match number {
        1 => OpCode::Add,
        2 => OpCode::Multiply,
        3 => OpCode::Input,
        4 => OpCode::Output,
        5 => OpCode::JumpIfTrue,
        6 => OpCode::JumpIfFalse,
        7 => OpCode::LessThan,
        8 => OpCode::Equal,
        99 => OpCode::End,
        _ => OpCode::Unknown
    }
}

fn number_of_parameters(op_code: &OpCode) -> u8 {
    match op_code {
        OpCode::Add | OpCode::Multiply | OpCode::LessThan | OpCode::Equal => 3,
        OpCode::JumpIfTrue | OpCode::JumpIfFalse => 2,
        OpCode::Input | OpCode::Output => 1,
        _ => 0
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    op_code: OpCode,
    parameters: Vec<Parameter>,
}

#[derive(Debug, PartialEq)]
struct Parameter {
    value: i32,
    mode: ParameterMode,
}

#[derive(Debug, PartialEq)]
enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equal,
    End,
    Unknown,
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_instruction() {
        let array = [101, 2, 3, 3, 3, 4];
        let mut current_index = 0usize;

        assert_eq!(get_next_instruction(&array, &mut current_index), Instruction {
            op_code: OpCode::Add,
            parameters: vec![Parameter {
                value: 2,
                mode: ParameterMode::Immediate,
            }, Parameter {
                value: 3,
                mode: ParameterMode::Position,
            }, Parameter {
                value: 3,
                mode: ParameterMode::Position,
            }],
        });
        assert_eq!(current_index, 4);

        assert_eq!(get_next_instruction(&array, &mut current_index), Instruction {
            op_code: OpCode::Input,
            parameters: vec![Parameter {
                value: 4,
                mode: ParameterMode::Position,
            }],
        });
        assert_eq!(current_index, 6);
    }

    #[test]
    fn test_run_program() {
        let mut array = [101, 2, 3, 3, 1102, 90, 80, 1, 99];
        let mut current_index = 0usize;

        run_program(&mut array);

        assert_eq!(array, [101, 7200, 3, 5, 1102, 90, 80, 1, 99]);
    }
}