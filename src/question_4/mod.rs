use crate::util::extract_numbers;

pub fn solve_a() {
    let min = 152085u32;
    let max = 670283u32;
    let mut passwords_count = 0u32;

    for i in min..=max {
        if has_correct_format(extract_numbers(i)) {
            passwords_count += 1;
        }
    }

    println!("Count: {}", passwords_count);
}

fn has_correct_format(numbers: Vec<u8>) -> bool {
    let mut i = 0usize;
    let mut twins_found = false;

    while i < numbers.len() {
        if i + 1 == numbers.len() {
            break;
        }

        if numbers[i] == numbers[i + 1] {
            let mut dupes = 2usize;
            for j in (i + 1)..(numbers.len() - 1) {
                if numbers[j] == numbers[j + 1] {
                    dupes += 1;
                } else {
                    break;
                }
            }

            if dupes > 2 {
                i += dupes - 1;
                continue;
            }

            twins_found = true;
        }

        if numbers[i] > numbers[i + 1] {
            return false;
        }

        i += 1;
    }
    twins_found
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::extract_numbers;

    #[test]
    fn test_extract_numbers() {
        assert_eq!(extract_numbers(123456), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_has_correct_format() {
        assert_eq!(has_correct_format(vec![1, 2, 3, 4, 5, 6]), false);
        assert_eq!(has_correct_format(vec![1, 2, 2, 4, 5, 6]), true);
        assert_eq!(has_correct_format(vec![1, 2, 2, 4, 5, 4]), false);
        assert_eq!(has_correct_format(vec![1, 1, 1, 1, 1, 1]), false);
        assert_eq!(has_correct_format(vec![1, 1, 1, 1, 2, 2]), true);
        assert_eq!(has_correct_format(vec![1, 2, 3, 4, 4, 4]), false);
        assert_eq!(has_correct_format(vec![1, 1, 2, 2, 3, 3]), true);
        assert_eq!(has_correct_format(vec![1, 2, 6, 7, 7, 7]), false);
    }
}