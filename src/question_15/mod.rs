use crate::util::{read_op_codes};
use crate::int_code::run_program;
use std::collections::HashMap;
use std::borrow::BorrowMut;

pub fn solve_b() {
    let mut op_codes = read_op_codes("src/question_15/input.txt");
    let map = discover_map(&mut op_codes);

    let oxygen_location = &map.iter()
        .filter(|(_, tile)| tile.tile_type == TileType::Oxygen)
        .map(|(pos, _)| pos)
        .nth(0)
        .expect("Oxygen tank was not found!");
    println!("Oxygen location is at {:?}", oxygen_location);

    let position = **oxygen_location;
    let distance = fill(&map, position, &Direction::EAST, 0);
    println!("Distance: {}", distance);
}

fn fill(map: &HashMap<(i64, i64), Tile>, position: (i64, i64), direction: &Direction, distance: u32) -> u32 {
    let mut curr_distance = distance;
    let possibilities = [
        (Direction::NORTH, get_new_position(&Direction::NORTH, position), map.get(&get_new_position(&Direction::NORTH, position)).unwrap()),
        (Direction::SOUTH, get_new_position(&Direction::SOUTH, position), map.get(&get_new_position(&Direction::SOUTH, position)).unwrap()),
        (Direction::WEST, get_new_position(&Direction::WEST, position), map.get(&get_new_position(&Direction::WEST, position)).unwrap()),
        (Direction::EAST, get_new_position(&Direction::EAST, position), map.get(&get_new_position(&Direction::EAST, position)).unwrap()),
    ];
    let possibilities: Vec<_> = possibilities.iter().filter(|(d, ..)| *d != get_opposite(direction)).collect();

    for possibility in possibilities.iter() {
        if possibility.2.tile_type != TileType::Wall {
            curr_distance = curr_distance.max(fill(map, possibility.1, &possibility.0, distance + 1));
        }
    }

    curr_distance
}

fn get_opposite(direction: &Direction) -> Direction {
    match direction {
        Direction::NORTH => Direction::SOUTH,
        Direction::SOUTH => Direction::NORTH,
        Direction::WEST => Direction::EAST,
        Direction::EAST => Direction::WEST,
    }
}

fn discover_map(op_codes: &mut [i64; 4096]) -> HashMap<(i64, i64), Tile> {
    let mut op_codes_pos = 0;
    let mut op_codes_base = 0;

    let mut current_position = (0, 0);
    let mut current_direction = Direction::NORTH;
    let mut inputs = vec![(current_direction as i64).clone()];
    let mut map = HashMap::new();

    while let Some(output) = run_program(op_codes, &inputs, &mut op_codes_pos, &mut op_codes_base) {
        let tile_type = TileType::from(output);
        match tile_type {
            TileType::Wall => {
                let next_position = get_new_position(&current_direction, current_position);
                map.insert(next_position, Tile::new(tile_type));
            }
            TileType::Oxygen => {
                current_position = get_new_position(&current_direction, current_position);
                let tile = map.entry(current_position)
                    .or_insert(Tile::new(tile_type))
                    .borrow_mut();
                tile.visited += 1;

                // (-16, 14)
            }
            _ => {
                current_position = get_new_position(&current_direction, current_position);
                let tile = map.entry(current_position)
                    .or_insert(Tile::new(tile_type))
                    .borrow_mut();
                tile.visited += 1;
            }
        }

        current_direction = get_new_direction(current_position, &map);
        inputs = vec![(current_direction as i64).clone()];

        if map.len() == 1660 {
            break;
        }
    }
    map
}

fn get_new_direction(position: (i64, i64), map: &HashMap<(i64, i64), Tile>) -> Direction {
    let unknown = Tile::new(TileType::Unknown);
    let possibilities = [
        (Direction::NORTH, map.get(&get_new_position(&Direction::NORTH, position)).unwrap_or_else(|| &unknown)),
        (Direction::SOUTH, map.get(&get_new_position(&Direction::SOUTH, position)).unwrap_or_else(|| &unknown)),
        (Direction::WEST, map.get(&get_new_position(&Direction::WEST, position)).unwrap_or_else(|| &unknown)),
        (Direction::EAST, map.get(&get_new_position(&Direction::EAST, position)).unwrap_or_else(|| &unknown)),
    ];

    for possibility in possibilities.iter() {
        if possibility.1.tile_type == TileType::Unknown {
            return possibility.0;
        }
    }

    possibilities.iter()
        .filter(|(_, t)| t.tile_type != TileType::Wall)
        .min_by(|(_, a), (_, b)| a.visited.cmp(&b.visited))
        .map(|(d, _)| *d)
        .unwrap_or_else(|| Direction::NORTH)
}

fn get_new_position(direction: &Direction, position: (i64, i64)) -> (i64, i64) {
    match direction {
        Direction::NORTH => (position.0, position.1 - 1),
        Direction::SOUTH => (position.0, position.1 + 1),
        Direction::WEST => (position.0 - 1, position.1),
        Direction::EAST => (position.0 + 1, position.1),
    }
}

#[derive(Clone)]
struct Tile {
    tile_type: TileType,
    visited: u32,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Tile {
        Tile {
            tile_type,
            visited: 0,
        }
    }
}

#[derive(PartialEq, Clone)]
enum TileType {
    Empty,
    Wall,
    Oxygen,
    Robot,
    Unknown,
}

impl TileType {
    pub fn from(value: i64) -> TileType {
        match value {
            0 => TileType::Wall,
            2 => TileType::Oxygen,
            3 => TileType::Robot,
            _ => TileType::Empty,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    NORTH = 1,
    SOUTH = 2,
    WEST = 3,
    EAST = 4,
}