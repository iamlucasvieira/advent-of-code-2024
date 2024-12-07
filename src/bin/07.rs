advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let operations = parse_input(input);
    operations
        .iter()
        .filter(|(target, values)| buildable_with_sum_and_mult(*target, values))
        .map(|(target, _)| *target)
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let operations = parse_input(input);
    operations
        .iter()
        .filter(|(target, values)| buildable_with_sum_mult_and_concat(*target, values))
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

fn buildable_with_sum_and_mult(mut target: u64, values: &[u64]) -> bool {
    let n_numbers = values.len();
    if let Some(&value) = values.last() {
        if target % value != 0 {
            if target < value {
                return false;
            }
            target -= value;
            return buildable_with_sum_and_mult(target, &values[..n_numbers - 1]);
        } else {
            return buildable_with_sum_and_mult(target / value, &values[..n_numbers - 1])
                || buildable_with_sum_and_mult(target - value, &values[..n_numbers - 1]);
        }
    }
    target == 0
}

// if len is the same should return zero
fn remove_concatenatable(target: u64, value: u64) -> u64 {
    let target_str = target.to_string();
    let value_str = value.to_string();
    if target_str == value_str {
        return 0;
    } else if target_str.ends_with(&value_str) {
        let target_excluding_end = target_str[..target_str.len() - value_str.len()]
            .parse()
            .expect("Should be a number");
        return target_excluding_end;
    }
    target
}
fn buildable_with_sum_mult_and_concat(target: u64, values: &[u64]) -> bool {
    let n_numbers = values.len();
    if let Some(&value) = values.last() {
        let target_str = target.to_string();
        let value_str = value.to_string();
        let is_divisible = target % value == 0;
        let is_concatenatable = target_str.ends_with(&value_str);
        if !is_divisible && !is_concatenatable {
            if target < value {
                return false;
            }
            return buildable_with_sum_mult_and_concat(target - value, &values[..n_numbers - 1]);
        } else {
            let mut divisible_result = false;
            let mut concatenatable_result = false;
            let mut subtract_result = false;
            if is_divisible {
                divisible_result =
                    buildable_with_sum_mult_and_concat(target / value, &values[..n_numbers - 1]);
            }

            if is_concatenatable {
                let target_excluding_end = remove_concatenatable(target, value);
                concatenatable_result = buildable_with_sum_mult_and_concat(
                    target_excluding_end,
                    &values[..n_numbers - 1],
                );
            }

            if target > value {
                subtract_result =
                    buildable_with_sum_mult_and_concat(target - value, &values[..n_numbers - 1]);
            }

            return subtract_result || divisible_result || concatenatable_result;
        }
    }
    target == 0
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
