use std::collections::HashMap;
use itertools::Itertools;
use crate::util::read_content;
use std::cmp::Ordering;

pub fn solve_a() {
    let content = read_content("src/question_10/input.txt");
    let asteroids = parse_asteroids(&content[..]);
    let best = find_best_asteroid(&asteroids);
    println!("Best: {:?}", best);
}

pub fn solve_b() {
    // best was (23, 29)
    let content = read_content("src/question_10/input.txt");
    let asteroids = parse_asteroids(&content[..]);
    let base = asteroids.iter().find(|a| a.x == 23 && a.y == 29).unwrap();
    let last_destroyed = destroy_until(base, &asteroids, 200).unwrap();
    println!("Result: {}", (last_destroyed.x * 100) + last_destroyed.y);
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

fn destroy_until<'a>(base: &Asteroid, asteroids: &'a Vec<Asteroid>, last: u32) -> Option<&'a Asteroid> {
    let mut asteroid_degrees: Vec<(f64, Option<&Asteroid>)> = asteroids.iter()
        .filter(|a| *a != base)
        .map(|a| (calculate_degrees(base, a), a))
        .sorted_by(|a, b| {
            match b.0.partial_cmp(&a.0) {
                Some(Ordering::Equal) => calculate_distance(base, &a.1).partial_cmp(&calculate_distance(base, &b.1)).unwrap_or_else(|| Ordering::Equal),
                Some(ordering) => ordering,
                None => Ordering::Equal
            }
        })
        .map(|(d, a)| (d, Some(a)))
        .collect();

    if last > asteroids.len() as u32 - 1 {
        println!("Not enough asteroids to destroy!");
        return None;
    }

    let mut last_angle = -179f64;
    let mut asteroids_shot = 0u32;
    let mut last_asteroid_shot = None;

    while asteroids_shot < last {
        for i in 0..asteroid_degrees.len() {
            if asteroids_shot >= last {
                break;
            }

            let a = asteroid_degrees.get_mut(i).unwrap();
            if last_angle == a.0 {
                continue;
            }

            match a.1 {
                Some(asteroid) => {
                    println!("Asteroid at ({}, {}) shot down at angle {}!", asteroid.x, asteroid.y, a.0);
                    last_asteroid_shot = Some(asteroid);
                    *a = (a.0, None);
                    last_angle = a.0;
                    asteroids_shot += 1;
                }
                None => {
                    continue;
                }
            }
        }
    }

    last_asteroid_shot
}

fn calculate_distance(first: &Asteroid, second: &Asteroid) -> i32 {
    (second.x - first.x).abs() + (second.y - first.y).abs()
}

fn calculate_unit_vector(first: &Asteroid, second: &Asteroid) -> (f64, f64) {
    let vector = ((second.x - first.x) as f64, (second.y - first.y) as f64);
    let magnitude = (vector.0.powf(2.0) + vector.1.powf(2.0)).sqrt();
    (vector.0 / magnitude, (vector.1 / magnitude))
}

fn calculate_degrees(first: &Asteroid, second: &Asteroid) -> f64 {
    let vector = ((second.x - first.x) as f64, (second.y - first.y) as f64);
    vector.0.atan2(vector.1).to_degrees()
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

    #[test]
    fn test_calculate_degrees() {
        // top
        assert_eq!(calculate_degrees(&Asteroid::new(2, 2), &Asteroid::new(2, 0)), 180.0);

        assert_eq!(calculate_degrees(&Asteroid::new(2, 2), &Asteroid::new(4, 0)), 135.0);

        // right
        assert_eq!(calculate_degrees(&Asteroid::new(2, 2), &Asteroid::new(4, 2)), 90.0);

        assert_eq!(calculate_degrees(&Asteroid::new(2, 2), &Asteroid::new(4, 4)), 45.0);

        //bottom
        assert_eq!(calculate_degrees(&Asteroid::new(2, 2), &Asteroid::new(2, 4)), 0.0);

        assert_eq!(calculate_degrees(&Asteroid::new(2, 2), &Asteroid::new(0, 4)), -45.0);

        //left
        assert_eq!(calculate_degrees(&Asteroid::new(2, 2), &Asteroid::new(0, 2)), -90.0);

        assert_eq!(calculate_degrees(&Asteroid::new(2, 2), &Asteroid::new(0, 0)), -135.0);
    }

    #[test]
    fn test_destroy_until() {
        let map = "\
        .#....#####...#..\n\
        ##...##.#####..##\n\
        ##...#...#.#####.\n\
        ..#.....#...###..\n\
        ..#.#.....#....##\
        ";

        let asteroids = parse_asteroids(map);
        let base = asteroids.iter().find(|a| a.x == 8 && a.y == 3).unwrap();
        let last_destroyed = destroy_until(base, &asteroids, 34);
        assert_eq!(last_destroyed, Some(&Asteroid::new(16, 1)))
    }

    #[test]
    fn test_destroy_until_2() {
        let map = "\
        .#..##.###...#######\n\
        ##.############..##.\n\
        .#.######.########.#\n\
        .###.#######.####.#.\n\
        #####.##.#.##.###.##\n\
        ..#####..#.#########\n\
        ####################\n\
        #.####....###.#.#.##\n\
        ##.#################\n\
        #####.##.###..####..\n\
        ..######..##.#######\n\
        ####.##.####...##..#\n\
        .#####..#.######.###\n\
        ##...#.##########...\n\
        #.##########.#######\n\
        .####.#.###.###.#.##\n\
        ....##.##.###..#####\n\
        .#.#.###########.###\n\
        #.#.#.#####.####.###\n\
        ###.##.####.##.#..##
        ";

        let asteroids = parse_asteroids(map);
        let base = asteroids.iter().find(|a| a.x == 11 && a.y == 13).unwrap();

        let last_destroyed = destroy_until(base, &asteroids, 1);
        assert_eq!(last_destroyed, Some(&Asteroid::new(11, 12)));

        let last_destroyed = destroy_until(base, &asteroids, 2);
        assert_eq!(last_destroyed, Some(&Asteroid::new(12, 1)));

        let last_destroyed = destroy_until(base, &asteroids, 3);
        assert_eq!(last_destroyed, Some(&Asteroid::new(12, 2)));

        let last_destroyed = destroy_until(base, &asteroids, 10);
        assert_eq!(last_destroyed, Some(&Asteroid::new(12, 8)));

        let last_destroyed = destroy_until(base, &asteroids, 200);
        assert_eq!(last_destroyed, Some(&Asteroid::new(8, 2)));
    }
}