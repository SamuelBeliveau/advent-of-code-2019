use std::collections::HashMap;
use crate::int_code::run_program;
use crate::util::{read_op_codes, print_positions};

pub fn solve_a() {
    let mut op_codes = read_op_codes("src/question_11/input.txt");

    let amount = paint(&mut op_codes, 0).len();
    println!("Amount: {}", amount);
}

pub fn solve_b() {
    let mut op_codes = read_op_codes("src/question_11/input.txt");

    let painted = paint(&mut op_codes, 1);

    print_positions(&painted, |c| if *c.unwrap_or_else(|| &0) == 0 { " " } else { "#" });
}

fn paint(op_codes: &mut [i64], starting_color: i64) -> HashMap<(i64, i64), i64> {
    let mut painted = HashMap::new();
    let mut current_position = (0i64, 0i64);
    let mut current_direction = Direction::Up;
    let mut current_op_codes_pos = 0;
    let mut relative_base = 0;

    painted.entry(current_position).or_insert(starting_color);

    loop {
        let current_entry = painted.entry(current_position).or_insert(0);
        let current_color: &mut i64 = current_entry;

        match run_program(op_codes, &vec![*current_color], &mut current_op_codes_pos, &mut relative_base) {
            Some(color) => {
                *current_color = color;
            }
            None => {
                println!("Finished (when getting color info)!");
                break;
            }
        }

        match run_program(op_codes, &vec![*current_color], &mut current_op_codes_pos, &mut relative_base) {
            Some(instruction) => {
                current_direction = turn(current_direction, instruction);
                current_position = move_forward(current_position, &current_direction);
            }
            None => {
                println!("Finished (when getting direction info)!");
                break;
            }
        }
    }

    painted
}

fn turn(direction: Direction, instruction: i64) -> Direction {
    match direction {
        Direction::Up => if instruction == 0 { Direction::Left } else { Direction::Right },
        Direction::Right => if instruction == 0 { Direction::Up } else { Direction::Down },
        Direction::Down => if instruction == 0 { Direction::Right } else { Direction::Left },
        Direction::Left => if instruction == 0 { Direction::Down } else { Direction::Up },
    }
}

fn move_forward((x, y): (i64, i64), direction: &Direction) -> (i64, i64) {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Right => (x + 1, y),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
    }
}

#[derive(Debug)]
enum Direction {
    Up, Right, Down, Left
}