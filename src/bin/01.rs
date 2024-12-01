advent_of_code::solution!(1);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let [first, second] = parse_input(input);
    let result = first
        .iter()
        .sorted()
        .zip(second.iter().sorted())
        .map(|(a, b)| a.max(b) - a.min(b))
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let [first, second] = parse_input(input);
    let result = first
        .iter()
        .map(|a| {
            let occurences = second.iter().filter(|&b| a == b).count();
            a * occurences as u32
        })
        .sum();
    Some(result)
}

/// Parse the input into a 2D array of u32
///
/// Each line contains two numbers separated by a space
fn parse_input(input: &str) -> [Vec<u32>; 2] {
    let (first, second) = input
        .lines()
        .flat_map({
            |line| {
                line.split_whitespace()
                    .map(|num| num.parse::<u32>().expect("Failed to parse number"))
            }
        })
        .tuples()
        .unzip();
    [first, second]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_parse_input() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let [first, second] = parse_input(input);
        assert_eq!(first.len(), 6);
        assert_eq!(second.len(), 6);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
