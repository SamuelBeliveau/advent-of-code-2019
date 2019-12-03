use std::ops::Add;
use std::cmp::Ordering;
use crate::util::read_content;

pub fn solve_a() {
    let contents = read_content("src/question_3/input.txt");
    let lines = contents.lines().collect();
    match get_closest_intersection_distance(&lines) {
        Some(distance) => println!("Minimum distance is {}", distance),
        None => println!("No collision found...")
    }
}

pub fn solve_b() {
    let contents = read_content("src/question_3/input.txt");
    let lines = contents.lines().collect();
    match get_minimum_intersection_steps(&lines) {
        Some(steps) => println!("Minimum steps is {}", steps),
        None => println!("No collision found...")
    }
}

fn get_closest_intersection_distance(paths_str: &Vec<&str>) -> Option<u32> {
    let intersections = get_intersections(paths_str);
    let mut intersection_distances: Vec<u32> = intersections.iter().map(|i| (i.point.x.abs() + i.point.y.abs()) as u32).collect();

    println!("distances: {:?}", intersection_distances);

    intersection_distances.iter().filter(|d| **d > 0).min().map(|d| *d)
}

fn get_minimum_intersection_steps(paths_str: &Vec<&str>) -> Option<u32> {
    let intersections = get_intersections(paths_str);
    let mut intersection_steps: Vec<u32> = intersections.iter().map(|i| i.steps).collect();

    println!("steps: {:?}", intersection_steps);

    intersection_steps.iter().filter(|d| **d > 0).min().map(|d| *d)
}

fn get_intersections(paths_str: &Vec<&str>) -> Vec<Intersection> {
    let mut visited_paths: Vec<Path> = Vec::new();
    let mut intersections = Vec::new();

    for path_str in paths_str {
        let path = Path::from(path_str);
        &visited_paths.iter().flat_map(|vp| &vp.lines).for_each(|vl| {
            for l in &path.lines {
                match get_intersection(&l, vl) {
                    Some(intersection) => intersections.push(intersection),
                    None => ()
                }
            }
        });
        visited_paths.push(path);
    }

    intersections
}

fn get_next_point(origin: &Point, path: &str) -> Option<Point> {
    let mut direction: char = ' ';
    let mut amount_string = String::new();

    for (i, c) in path.chars().enumerate() {
        if i == 0 {
            direction = c;
        } else {
            amount_string.push(c);
        }
    }

    let amount: i32 = amount_string.parse().unwrap();

    match direction {
        'U' => Option::Some(origin + &Point::new(0, -amount)),
        'D' => Option::Some(origin + &Point::new(0, amount)),
        'L' => Option::Some(origin + &Point::new(-amount, 0)),
        'R' => Option::Some(origin + &Point::new(amount, 0)),
        _ => Option::None
    }
}

fn get_intersection(line1: &Line, line2: &Line) -> Option<Intersection> {
    // Case when line1 is horizontal and line2 vertical
    if in_interval(line2.start.x, line1.start.x, line1.end.x) && in_interval(line1.start.y, line2.start.y, line2.end.y) {
        let steps = (line2.start.x - line1.start.x).abs() as u32 +
            (line1.start.y - line2.start.y).abs() as u32 +
            line1.starting_steps +
            line2.starting_steps;
        return Option::Some(Intersection::new(Point::new(line2.start.x, line1.start.y), steps));
    }

    // Case when line1 is vertical and line2 horizontal
    if in_interval(line1.start.x, line2.start.x, line2.end.x) && in_interval(line2.start.y, line1.start.y, line1.end.y) {
        let steps = (line1.start.x - line2.start.x).abs() as u32 +
            (line2.start.y - line1.start.y).abs() as u32 +
            line1.starting_steps +
            line2.starting_steps;
        return Option::Some(Intersection::new(Point::new(line1.start.x, line2.start.y), steps));
    }

    Option::None
}

fn in_interval(value: i32, start: i32, end: i32) -> bool {
    (value >= start && value <= end) || (value <= start && value >= end)
}

struct Path {
    lines: Vec<Line>,
}

impl Path {
    pub fn from(str: &str) -> Path {
        let mut current_point = Point::new(0, 0);
        let mut lines: Vec<Line> = Vec::new();
        let mut steps_taken = 0u32;

        for part in str.split(",") {
            let next_point = get_next_point(&current_point, part).unwrap();
            let next_line = Line::new(current_point.clone(), next_point.clone(), steps_taken);
            steps_taken += next_line.distance();

            lines.push(next_line);
            current_point = next_point;
        }

        Path { lines }
    }
}

#[derive(Debug)]
struct Intersection {
    point: Point,
    steps: u32,
}

impl Intersection {
    pub fn new(point: Point, steps: u32) -> Intersection {
        Intersection {
            point,
            steps,
        }
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point && self.steps == other.steps
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
        }
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
    starting_steps: u32,
}

impl Line {
    pub fn new(start: Point, end: Point, starting_steps: u32) -> Line {
        Line {
            start,
            end,
            starting_steps,
        }
    }

    pub fn distance(&self) -> u32 {
        ((self.end.x - self.start.x).abs() + (self.end.y - self.start.y).abs()) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_intersection_horizontal_vertical() {
        let line1 = Line::new(Point::new(0, 0), Point::new(5, 0), 0);
        let line2 = Line::new(Point::new(3, -2), Point::new(3, 2), 0);
        assert_eq!(get_intersection(&line1, &line2), Option::Some(Intersection::new(Point::new(3, 0), 5)));

        let line1 = Line::new(Point::new(0, 0), Point::new(5, 0), 0);
        let line2 = Line::new(Point::new(0, -2), Point::new(0, 2), 0);
        assert_eq!(get_intersection(&line1, &line2), Option::Some(Intersection::new(Point::new(0, 0), 2)));

        let line1 = Line::new(Point::new(0, 0), Point::new(5, 0), 0);
        let line2 = Line::new(Point::new(5, -2), Point::new(5, 2), 0);
        assert_eq!(get_intersection(&line1, &line2), Option::Some(Intersection::new(Point::new(5, 0), 7)));

        let line1 = Line::new(Point::new(0, 0), Point::new(5, 0), 0);
        let line2 = Line::new(Point::new(-1, -2), Point::new(-1, 2), 0);
        assert_eq!(get_intersection(&line1, &line2), Option::None);

        let line1 = Line::new(Point::new(0, 0), Point::new(5, 0), 0);
        let line2 = Line::new(Point::new(6, -2), Point::new(6, 2), 0);
        assert_eq!(get_intersection(&line1, &line2), Option::None);
    }

    #[test]
    fn test_get_intersection_vertical_horizontal() {
        let line2 = Line::new(Point::new(0, 0), Point::new(5, 0), 0);
        let line1 = Line::new(Point::new(3, -2), Point::new(3, 2), 0);
        assert_eq!(get_intersection(&line1, &line2), Option::Some(Intersection::new(Point::new(3, 0), 5)));

        let line2 = Line::new(Point::new(0, 0), Point::new(5, 0), 0);
        let line1 = Line::new(Point::new(0, -2), Point::new(0, 2), 0);
        assert_eq!(get_intersection(&line1, &line2), Option::Some(Intersection::new(Point::new(0, 0), 2)));

        let line2 = Line::new(Point::new(0, 0), Point::new(5, 0), 0);
        let line1 = Line::new(Point::new(5, -2), Point::new(5, 2), 0);
        assert_eq!(get_intersection(&line1, &line2), Option::Some(Intersection::new(Point::new(5, 0), 7)));

        let line2 = Line::new(Point::new(0, 0), Point::new(5, 0), 0);
        let line1 = Line::new(Point::new(-1, -2), Point::new(-1, 2), 0);
        assert_eq!(get_intersection(&line1, &line2), Option::None);

        let line2 = Line::new(Point::new(0, 0), Point::new(5, 0), 0);
        let line1 = Line::new(Point::new(6, -2), Point::new(6, 2), 0);
        assert_eq!(get_intersection(&line1, &line2), Option::None);
    }

    #[test]
    fn test_get_intersection_reverse_points() {
        let line1 = Line::new(Point::new(5, 0), Point::new(0, 0), 0);
        let line2 = Line::new(Point::new(3, 2), Point::new(3, -2), 0);
        assert_eq!(get_intersection(&line1, &line2), Option::Some(Intersection::new(Point::new(3, 0), 4)));

        let line2 = Line::new(Point::new(5, 0), Point::new(0, 0), 0);
        let line1 = Line::new(Point::new(0, 2), Point::new(0, -2), 0);
        assert_eq!(get_intersection(&line1, &line2), Option::Some(Intersection::new(Point::new(0, 0), 7)));

        let line2 = Line::new(Point::new(5, 0), Point::new(0, 0), 0);
        let line1 = Line::new(Point::new(5, 2), Point::new(5, -2), 0);
        assert_eq!(get_intersection(&line1, &line2), Option::Some(Intersection::new(Point::new(5, 0), 2)));

        let line2 = Line::new(Point::new(5, 0), Point::new(0, 0), 0);
        let line1 = Line::new(Point::new(-1, 2), Point::new(-1, -2), 0);
        assert_eq!(get_intersection(&line1, &line2), Option::None);

        let line2 = Line::new(Point::new(5, 0), Point::new(0, 0), 0);
        let line1 = Line::new(Point::new(6, 2), Point::new(6, -2), 0);
        assert_eq!(get_intersection(&line1, &line2), Option::None);
    }

    #[test]
    fn test_get_next_point() {
        assert_eq!(get_next_point(&Point::new(0, 0), "U87"), Option::Some(Point::new(0, -87)));
        assert_eq!(get_next_point(&Point::new(0, 0), "D15"), Option::Some(Point::new(0, 15)));
        assert_eq!(get_next_point(&Point::new(0, 0), "L32"), Option::Some(Point::new(-32, 0)));
        assert_eq!(get_next_point(&Point::new(0, 0), "R99"), Option::Some(Point::new(99, 0)));
        assert_eq!(get_next_point(&Point::new(45, -34), "U15"), Option::Some(Point::new(45, -49)));
    }

    #[test]
    fn test_path_from() {
        let path = Path::from("U78,R3,D10,L90");
        assert_eq!(path.lines.len(), 4);
        assert_eq!(path.lines[0].end, Point::new(0, -78));
        assert_eq!(path.lines[1].end, Point::new(3, -78));
        assert_eq!(path.lines[2].end, Point::new(3, -68));
        assert_eq!(path.lines[3].end, Point::new(-87, -68));
    }

    #[test]
    fn test_get_closest_intersection_distance() {
        let paths = vec!["R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"];
        assert_eq!(get_closest_intersection_distance(&paths), Option::Some(159));

        let paths = vec!["R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"];
        assert_eq!(get_closest_intersection_distance(&paths), Option::Some(135));
    }

    #[test]
    fn test_get_minimum_intersection_steps() {
        let paths = vec!["R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"];
        assert_eq!(get_minimum_intersection_steps(&paths), Option::Some(610));

        let paths = vec!["R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"];
        assert_eq!(get_minimum_intersection_steps(&paths), Option::Some(410));
    }
}