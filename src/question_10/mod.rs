use std::collections::HashMap;
use itertools::Itertools;

pub fn solve_a() {}

fn parse_asteroids(map: &str) -> Vec<Asteroid> {
    let mut asteroids = Vec::new();
    for (y, line) in map.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                asteroids.push(Asteroid::new(x as i32, y as i32));
            }
        }
    }
    asteroids
}

fn find_best_asteroid(asteroids: &Vec<Asteroid>) -> Option<&Asteroid> {
    for asteroid in asteroids.iter() {
        let asteroid_slopes: HashMap<_, _> = asteroids.iter().map(|a| (calculate_slope(asteroid, a), a)).into_group_map();
    }

    None

    // 5,6 vs 2,6 => -3,0
    // 5,6 vs 1,6 => -4,0
    // 5,6 vs 9,6 => 4,0
    // 5,6 vs 8,5 => 3,-1
    // 5,6 vs 14,3 => 9,-3
}

fn calculate_slope(first: &Asteroid, second: &Asteroid) -> Slope {
    if second.x - first.x == 0 {
        return Slope::Vertical(second.y - first.y);
    }

    if second.y - first.y == 0 {
        return Slope::Horizontal(second.x - first.x);
    }

    Slope::Diagonal((second.y - first.y) as f32 / (second.x - first.x) as f32)
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Slope {
    Horizontal(i32),
    Vertical(i32),
    Diagonal(f32),
}

#[derive(Debug, PartialEq)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Asteroid {
    pub fn new(x: i32, y: i32) -> Asteroid {
        Asteroid {
            x,
            y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_asteroids() {
        let map = "\
        ......#.#.\n\
        #..#.#....";

        let asteroids = parse_asteroids(map);
        assert_eq!(asteroids, vec![
            Asteroid::new(6, 0),
            Asteroid::new(8, 0),
            Asteroid::new(0, 1),
            Asteroid::new(3, 1),
            Asteroid::new(5, 1)
        ])
    }

    #[test]
    fn test_calculate_slope_vertical() {
        let slope = calculate_slope(&Asteroid::new(6, 7), &Asteroid::new(6, 2));
        assert_eq!(slope, Slope::Vertical(-5));
    }

    #[test]
    fn test_calculate_slope_horizontal() {
        let slope = calculate_slope(&Asteroid::new(2, 5), &Asteroid::new(6, 5));
        assert_eq!(slope, Slope::Horizontal(4));
    }

    #[test]
    fn test_calculate_slope_diagonal() {
        let slope = calculate_slope(&Asteroid::new(9, 5), &Asteroid::new(6, 11));
        assert_eq!(slope, Slope::Diagonal(-2.0));
    }

    #[test]
    fn test_find_best_asteroid() {
        let map = "\
        ......#.#.\n\
        #..#.#....\n\
        ..#######.\n\
        .#.#.###..\n\
        .#..#.....\n\
        ..#....#.#\n\
        #..#....#.\n\
        .##.#..###\n\
        ##...#..#.\n\
        .#....####";

        let asteroids = parse_asteroids(map);
        let best = find_best_asteroid(&asteroids);
        assert_eq!(best.is_some(), true);
        assert_eq!(best.unwrap().x, 5);
        assert_eq!(best.unwrap().y, 8);
    }
}