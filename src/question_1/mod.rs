use crate::util::read_content;
use std::str::FromStr;

pub fn solve_a() {
    let content = read_content("src/question_1/input.txt");
    let fuel_sum: i32 = content
        .lines()
        .map(|line| calculate_fuel(i32::from_str(line).unwrap()))
        .sum();
    println!("Fuel sum is {}", fuel_sum);
}

pub fn solve_b() {
    let content = read_content("src/question_1/input.txt");
    let fuel_sum: i32 = content
        .lines()
        .map(|line| calculate_total_fuel(i32::from_str(line).unwrap()))
        .sum();
    println!("Fuel total sum is {}", fuel_sum);
}

fn calculate_fuel(mass: i32) -> i32 {
    return (mass / 3) - 2;
}

fn calculate_total_fuel(mass: i32) -> i32 {
    let mut total = 0;
    let mut fuel = calculate_fuel(mass);

    while fuel > 0 {
        total = total + fuel;
        fuel = calculate_fuel(fuel);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn test_calculate_total_fuel() {
        assert_eq!(calculate_total_fuel(14), 2);
        assert_eq!(calculate_total_fuel(1969), 966);
        assert_eq!(calculate_total_fuel(100756), 50346);
    }
}