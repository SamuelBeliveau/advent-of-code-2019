use crate::util::{read_op_codes, print_positions};
use crate::int_code::run_program;
use std::collections::HashMap;
use std::io::stdin;

pub fn solve_a() {
    let mut op_codes = read_op_codes("src/question_15/input.txt");
    let mut op_codes_pos = 0;
    let mut op_codes_base = 0;

    let mut current_position = (0, 0);
    let mut current_direction = Direction::NORTH;
    let mut inputs = vec![(current_direction as i64).clone()];
    let mut map = HashMap::new();
    let mut i = 0;

    while let Some(output) = run_program(&mut op_codes, &inputs, &mut op_codes_pos, &mut op_codes_base) {
        match output {
            0 => {
                let next_position = get_new_position(&current_direction, current_position);
                map.insert(next_position, output);
            } // wall
            2 => {
                current_position = get_new_position(&current_direction, current_position);
                map.insert(current_position.clone(), output);
                println!("Found oxygen!");
            } // oxygen system
            _ => {
                current_position = get_new_position(&current_direction, current_position);
                map.insert(current_position.clone(), output);
            } // empty
        }

        current_direction = get_new_direction(current_position, &map);
        inputs = vec![(current_direction as i64).clone()];

        print!("{}[2J", 27 as char);
        print_positions(&map, |option| option.map(|val| match *val {
            0 => "#",
            2 => "O",
            _ => "."
        }).unwrap_or_else(|| " "));
        i += 1;
        if i % 10 == 0 {
            let mut input = String::new();
            stdin().read_line(&mut input);
        }
    }
}

fn get_new_direction(position: (i64, i64), map: &HashMap<(i64, i64), i64>) -> Direction {
    let context = [
        (Direction::NORTH, map.get(&get_new_position(&Direction::NORTH, position))),
        (Direction::SOUTH, map.get(&get_new_position(&Direction::SOUTH, position))),
        (Direction::WEST, map.get(&get_new_position(&Direction::WEST, position))),
        (Direction::EAST, map.get(&get_new_position(&Direction::EAST, position))),
    ];

    for possibility in context.iter() {
        if possibility.1.is_none() {
            return possibility.0;
        }
    }

    for possibility in context.iter() {
        if *possibility.1.unwrap() != 0 {
            return possibility.0;
        }
    }
    Direction::NORTH
}

fn get_new_position(direction: &Direction, position: (i64, i64)) -> (i64, i64) {
    match direction {
        Direction::NORTH => (position.0, position.1 - 1),
        Direction::SOUTH => (position.0, position.1 + 1),
        Direction::WEST => (position.0 - 1, position.1),
        Direction::EAST => (position.0 + 1, position.1),
    }
}

#[derive(Copy, Clone)]
enum Direction {
    NORTH = 1,
    SOUTH = 2,
    WEST = 3,
    EAST = 4,
}