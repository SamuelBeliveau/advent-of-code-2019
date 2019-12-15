use crate::util::{read_op_codes, print_positions};
use std::collections::HashMap;
use crate::int_code::run_program;

pub fn solve_ab() {
    let mut op_codes = read_op_codes("src/question_13/input.txt");
    let mut current_position = 0;
    let mut relative_base = 0;

    let mut tiles: HashMap<(i64, i64), Tile> = HashMap::new();
    let mut ball_position;
    let mut paddle_position= (0, 0);
    let mut inputs = Vec::new();

    // Insert fake coins :)
    op_codes[0] = 2;

    let mut inst;

    loop {
        inst = [
            run_program(&mut op_codes, &inputs, &mut current_position, &mut relative_base),
            run_program(&mut op_codes, &inputs, &mut current_position, &mut relative_base),
            run_program(&mut op_codes, &inputs, &mut current_position, &mut relative_base)
        ];

        if inst.iter().any(|o| o.is_none()) {
            let number_of_blocks = tiles.iter().filter(|kvp| *kvp.1 == Tile::Block).count();
            println!("Number of blocks: {}", number_of_blocks);

            print_positions(&tiles, |t| format_tile(t));
            return;
        }

        let first = inst[0].unwrap();
        let second = inst[1].unwrap();
        let third = inst[2].unwrap();

        if first == -1 && second == 0 {
            println!("Score is now: {}", third);
            continue;
        }

        let tile = map_to_tile(third);
        let mut should_print = false;

        match tile {
            Tile::Ball => {
                ball_position = (first, second);
                let diff_x = ball_position.0 - paddle_position.0;
                inputs = vec![if diff_x < 0 { -1 } else if diff_x > 0 { 1 } else { 0 }];
                should_print = true;
            }
            Tile::HorizontalPaddle => {
                paddle_position = (first, second);
            }
            _ => {}
        }

        tiles.insert((first, second), tile);

        if should_print {
            print_positions(&tiles, |t| format_tile(t));
        }
    }
}

fn format_tile<'a>(tile: Option<&Tile>) -> &'a str {
    match tile {
        None => " ",
        Some(t) => {
            match t {
                Tile::Empty => " ",
                Tile::Wall => "|",
                Tile::Block => "#",
                Tile::HorizontalPaddle => "_",
                Tile::Ball => "*",
            }
        }
    }
}

fn map_to_tile(tile_id: i64) -> Tile {
    match tile_id {
        1 => Tile::Wall,
        2 => Tile::Block,
        3 => Tile::HorizontalPaddle,
        4 => Tile::Ball,
        _ => Tile::Empty,
    }
}

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}