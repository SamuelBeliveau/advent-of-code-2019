use std::collections::HashMap;
use itertools::Itertools;
use regex::internal::Input;
use crate::util::read_content;

pub fn solve_a() {
    let content = read_content("src/question_10/input.txt");
    let asteroids = parse_asteroids(&content[..]);
    let best = find_best_asteroid(&asteroids);
    println!("Best: {:?}", best);
}

pub fn solve_b() {
    // 0,inf => inf,0 => 0,-inf => -inf,0
}

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

fn find_best_asteroid(asteroids: &Vec<Asteroid>) -> (Option<&Asteroid>, usize) {
    let mut best: (Option<&Asteroid>, usize) = (None, 0);

    for asteroid in asteroids.iter() {
        let asteroid_slopes: HashMap<_, _> = asteroids.iter()
            .filter(|a| *a != asteroid)
            .map(|a| (calculate_unit_vector(asteroid, a), a))
            .map(|((x, y), a)| ((format!("{:.10}", x), format!("{:.10}", y)), a))
            .into_group_map();
        if asteroid_slopes.len() > best.1 {
            best = (Some(asteroid), asteroid_slopes.len());
        }
    }

    best
}

fn calculate_unit_vector(first: &Asteroid, second: &Asteroid) -> (f64, f64) {
    let vector = ((second.x - first.x) as f64, (second.y - first.y) as f64);
    let magnitude = (vector.0.powf(2.0) + vector.1.powf(2.0)).sqrt();
    (vector.0 / magnitude, (vector.1 / magnitude))
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
        assert_eq!(best.0.is_some(), true);
        assert_eq!(best.0.unwrap().x, 5);
        assert_eq!(best.0.unwrap().y, 8);
    }

    #[test]
    fn test_find_best_asteroid_2() {
        let map = "\
        .#..#\n\
        .....\n\
        #####\n\
        ....#\n\
        ...##";

        let asteroids = parse_asteroids(map);
        let best = find_best_asteroid(&asteroids);
        assert_eq!(best.0.is_some(), true);
        assert_eq!(best.0.unwrap().x, 3);
        assert_eq!(best.0.unwrap().y, 4);
    }

    #[test]
    fn test_calculate_unit_vector() {
        assert_eq!(calculate_unit_vector(&Asteroid::new(5, 6), &Asteroid::new(9, 2)), (0.7071067811865475, -0.7071067811865475));
    }
}