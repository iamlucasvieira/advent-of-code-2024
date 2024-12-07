advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let operations = parse_input(input);
    operations
        .iter()
        .filter(|(target, values)| can_build_target(*target, values, false))
        .map(|(target, _)| *target)
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let operations = parse_input(input);
    operations
        .iter()
        .filter(|(target, values)| can_build_target(*target, values, true))
        .map(|(target, _)| *target)
        .sum::<u64>()
        .into()
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            line.split_once(':')
                .map(|(left, right)| {
                    (
                        left.parse().expect("First number should be a number"),
                        right
                            .split_whitespace()
                            .map(|n| n.parse().expect("Second number should be a number"))
                            .collect(),
                    )
                })
                .expect("Line should contain a colon")
        })
        .collect()
}

fn can_build_target(target: u64, values: &[u64], allow_concat: bool) -> bool {
    match values {
        [] => target == 0,
        [.., value] => {
            let without_last = &values[..values.len() - 1];

            let divisible = target % *value == 0;
            let subtractable = target >= *value;

            // Try subtraction (equivalent to using '+', because we move left-to-right and
            // interpret adding a negative after rearrangement. Here it directly represents trying target - value.)
            let subtract_option = if subtractable {
                can_build_target(target - *value, without_last, allow_concat)
            } else {
                false
            };

            // Try division (like a previous multiplication)
            let divide_option = if divisible {
                can_build_target(target / *value, without_last, allow_concat)
            } else {
                false
            };

            // Try concatenation if allowed
            let concat_option = if allow_concat && can_concatenate(target, *value) {
                let adjusted = remove_concatenatable(target, *value);
                can_build_target(adjusted, without_last, allow_concat)
            } else {
                false
            };

            subtract_option || divide_option || concat_option
        }
    }
}

/// Check if `value` can be concatenated onto the end of `target`.
fn can_concatenate(target: u64, value: u64) -> bool {
    target.to_string().ends_with(&value.to_string())
}

/// Removes the concatenated portion (value) from the end of `target`.
fn remove_concatenatable(target: u64, value: u64) -> u64 {
    let target_str = target.to_string();
    let value_str = value.to_string();

    if target_str == value_str {
        0
    } else if let Some(prefix) = target_str.strip_suffix(&value_str) {
        prefix.parse().expect("Prefix should be a number")
    } else {
        target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "0: 1 2\n1: 2 3\n2: 3 4\n";
        let expected = vec![(0, vec![1, 2]), (1, vec![2, 3]), (2, vec![3, 4])];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
