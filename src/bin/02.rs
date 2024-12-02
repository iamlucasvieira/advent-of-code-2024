advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let count = input.iter().filter(|row| is_safe(row)).count() as u32;
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let count = input.iter().filter(|row| is_any_subset_safe(row)).count() as u32;
    Some(count)
}

fn is_any_subset_safe(row: &[i32]) -> bool {
    row.iter().enumerate().any(|(i, _)| {
        let mut sub_row = row.to_vec();
        sub_row.remove(i);
        is_safe(&sub_row)
    })
}

/// is_safe returns true when all ascending or descening and diff is <= 3
fn is_safe(row: &[i32]) -> bool {
    is_safe_in_direction(row, |a, b| a < b) || is_safe_in_direction(row, |a, b| a > b)
}

/// is_safe_in_direction returns true when all ascending or descening
/// direction is a function argument
fn is_safe_in_direction(row: &[i32], direction: fn(&i32, &i32) -> bool) -> bool {
    row.iter()
        .zip(row.iter().skip(1))
        .all(|(a, b)| direction(a, b) && (b - a).abs() <= 3)
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().expect("Failed to parse number"))
                .collect::<Vec<i32>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_parse_input() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result = parse_input(input);
        assert_eq!(result.len(), 6);
        assert_eq!(result[0].len(), 5);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
