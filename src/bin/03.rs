advent_of_code::solution!(3);

#[derive(Debug, PartialEq)]
enum Instructions {
    Mul(usize),
    Do(usize),
    Dont(usize),
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_input(input);
    let instructions = find_instructions_in_str(&data);
    let mul_locations = filter_mul(instructions);
    let products = get_products_inside_mul(&data, &mul_locations);

    let sum: u32 = products.iter().sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_input(input);
    let instructions = find_instructions_in_str(&data);
    let mul_locations = filter_mul_with_dont(instructions);
    let products = get_products_inside_mul(&data, &mul_locations);

    let sum: u32 = products.iter().sum();
    Some(sum)
}

fn find_instructions_in_str(input: &str) -> Vec<Instructions> {
    const MUL_PREFIX: &[u8] = b"mul(";
    const DO_PREFIX: &[u8] = b"do(";
    const DONT_PREFIX: &[u8] = b"don't(";

    let max_len = MUL_PREFIX.len().max(DO_PREFIX.len()).max(DONT_PREFIX.len());
    input
        .as_bytes()
        .windows(max_len)
        .enumerate()
        .filter_map(|(i, window)| {
            if window.starts_with(MUL_PREFIX) {
                Some(Instructions::Mul(i))
            } else if window.starts_with(DO_PREFIX) {
                Some(Instructions::Do(i))
            } else if window.starts_with(DONT_PREFIX) {
                Some(Instructions::Dont(i))
            } else {
                None
            }
        })
        .collect()
}

fn filter_mul_with_dont(input: Vec<Instructions>) -> Vec<usize> {
    let mut dont_is_active = false;
    let filterd = input
        .into_iter()
        .filter(|instruction| match instruction {
            Instructions::Dont(_) => {
                dont_is_active = true;
                false
            }
            Instructions::Do(_) => {
                dont_is_active = false;
                false
            }
            _ => !dont_is_active,
        })
        .collect();
    filter_mul(filterd)
}

fn filter_mul(i: Vec<Instructions>) -> Vec<usize> {
    i.iter()
        .filter_map(|instruction| match instruction {
            Instructions::Mul(i) => Some(*i),
            _ => None,
        })
        .collect()
}

fn parse_mul_expression(input: &str, start: usize) -> Option<u32> {
    let after_mul = start + 4; // Position after "mul("
    if after_mul >= input.len() {
        return None;
    }

    // Extract the substring starting after "mul("
    let remaining = &input[after_mul..];

    // Find the position of the closing ')' relative to `remaining`
    let closing_paren = remaining.find(')')?;
    let inside = &remaining[..closing_paren];

    // Split the inside by comma
    let mut parts = inside.split(',').map(|s| s.trim());

    // Parse the first number
    let first_str = parts.next()?;
    let first_number: u32 = first_str.parse().ok()?;

    // Parse the second number
    let second_str = parts.next()?;
    let second_number: u32 = second_str.parse().ok()?;

    // Ensure there are no extra parts
    if parts.next().is_some() {
        return None;
    }

    Some(first_number * second_number)
}

/// Extracts and multiplies the two numbers inside each "mul(a, b)" expression.
/// Returns a vector of products.
fn get_products_inside_mul(input: &str, locations: &[usize]) -> Vec<u32> {
    locations
        .iter()
        .filter_map(|&start| parse_mul_expression(input, start))
        .collect()
}

fn parse_input(input: &str) -> String {
    input.lines().collect::<Vec<&str>>().join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = r#"abc
def
ghi"#;
        assert_eq!(parse_input(input), "abcdefghi");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }

    #[test]
    fn test_find_instructions_in_str() {
        let input = "mul(1, 2)do(3, 4)don't(5, 6)";
        assert_eq!(
            find_instructions_in_str(input),
            vec![
                Instructions::Mul(0),
                Instructions::Do(9),
                Instructions::Dont(17)
            ]
        );
    }

    #[test]
    fn test_filter_do_and_donts() {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        let instructions = find_instructions_in_str(input);
        let filtered = filter_mul_with_dont(instructions);
        assert_eq!(filtered, vec![1, 64]);
    }
}
