use regex::Regex;
use std::cmp::Ordering;
use crate::util::read_content;

pub fn solve_a() {
    let contents = read_content("src/question_12/input.txt");
    let mut moons: Vec<_> = contents.lines().map(|line| parse_moon_data(line)).collect();
    run_steps(&mut moons, 1000);
    let total = calculate_energy(&moons);
    println!("Total: {}", total);
}

fn calculate_energy(moons: &Vec<Moon>) -> i32 {
    moons.iter()
        .map(|m|
            (m.position.x.abs() + m.position.y.abs() + m.position.z.abs()) *
                (m.velocity.x.abs() + m.velocity.y.abs() + m.velocity.z.abs()))
        .sum()
}

fn run_steps(moons: &mut Vec<Moon>, n: usize) {
    for step in 0..n {
        run_step(moons);
        print!("Step {:>#4} ", step);
        for i in 0..moons.len() {
            print!("({:>#4}[{:>#4}],{:>#4}[{:>#4}],{:>#4}[{:>#4}])    ",
                   moons[i].position.x, moons[i].velocity.x,
                   moons[i].position.y, moons[i].velocity.y,
                   moons[i].position.z, moons[i].velocity.z
            );
        }
        println!("");
    }
}

fn run_step(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        let moons_cloned = moons.clone();
        let other_moons: Vec<_> = moons_cloned.iter().enumerate()
            .filter(|(idx, _)| *idx != i)
            .map(|(_, m)| m)
            .collect();
        let moon = moons.get_mut(i).unwrap();
        apply_gravity(moon, &other_moons);
    }

    for i in 0..moons.len() {
        let moon = moons.get_mut(i).unwrap();
        apply_velocity(moon);
    }
}

fn apply_gravity(moon: &mut Moon, other_moons: &Vec<&Moon>) {
    for other in other_moons {
        moon.velocity.x += match moon.position.x.cmp(&other.position.x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        moon.velocity.y += match moon.position.y.cmp(&other.position.y) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        moon.velocity.z += match moon.position.z.cmp(&other.position.z) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
    }
}

fn apply_velocity(moon: &mut Moon) {
    moon.position.x += moon.velocity.x;
    moon.position.y += moon.velocity.y;
    moon.position.z += moon.velocity.z;
}

fn parse_moon_data(data: &str) -> Moon {
    let re = Regex::new(r"<x=(-?\d*\.?\d+), y=(-?\d*\.?\d+), z=(-?\d*\.?\d+)>").unwrap();
    let captures = re.captures(data).unwrap();
    let x: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
    let y: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
    let z: i32 = captures.get(3).unwrap().as_str().parse().unwrap();
    Moon::new(Vector3D::new(x, y, z), Vector3D::empty())
}

#[derive(Debug, PartialEq, Clone)]
struct Vector3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector3D {
    pub fn new(x: i32, y: i32, z: i32) -> Vector3D {
        Vector3D { x, y, z }
    }

    pub fn empty() -> Vector3D {
        Vector3D { x: 0, y: 0, z: 0 }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Moon {
    position: Vector3D,
    velocity: Vector3D,
}

impl Moon {
    pub fn new(position: Vector3D, velocity: Vector3D) -> Moon {
        Moon { position, velocity }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_moon_data() {
        let data = "<x=-12, y=0, z=223>";
        assert_eq!(parse_moon_data(data), Moon::new(Vector3D::new(-12, 0, 223), Vector3D::empty()));
    }

    #[test]
    fn test_apply_gravity() {
        let mut moon = Moon::new(Vector3D::new(-1, 0, 2), Vector3D::empty());
        let others = vec![
            Moon::new(Vector3D::new(2, -10, -7), Vector3D::empty()),
            Moon::new(Vector3D::new(4, -8, 8), Vector3D::empty()),
            Moon::new(Vector3D::new(3, 5, -1), Vector3D::empty()),
        ];
        apply_gravity(&mut moon, &others.iter().map(|m| m).collect::<Vec<_>>());
        assert_eq!(moon.velocity, Vector3D::new(3, -1, -1));
    }

    #[test]
    fn test_run_step() {
        let mut moons = vec![
            Moon::new(Vector3D::new(-1, 0, 2), Vector3D::empty()),
            Moon::new(Vector3D::new(2, -10, -7), Vector3D::empty()),
            Moon::new(Vector3D::new(4, -8, 8), Vector3D::empty()),
            Moon::new(Vector3D::new(3, 5, -1), Vector3D::empty()),
        ];

        run_step(&mut moons);

        assert_eq!(moons, vec![
            Moon::new(Vector3D::new(2, -1, 1), Vector3D::new(3, -1, -1)),
            Moon::new(Vector3D::new(3, -7, -4), Vector3D::new(1, 3, 3)),
            Moon::new(Vector3D::new(1, -7, 5), Vector3D::new(-3, 1, -3)),
            Moon::new(Vector3D::new(2, 2, 0), Vector3D::new(-1, -3, 1)),
        ]);
    }

    #[test]
    fn test_calculate_energy() {
        let mut moons = vec![
            Moon::new(Vector3D::new(-8, -10, 0), Vector3D::empty()),
            Moon::new(Vector3D::new(5, 5, 10), Vector3D::empty()),
            Moon::new(Vector3D::new(2, -7, 3), Vector3D::empty()),
            Moon::new(Vector3D::new(9, -8, -3), Vector3D::empty()),
        ];

        run_steps(&mut moons, 100);
        let total = calculate_energy(&moons);

        assert_eq!(total, 1940);
    }
}