advent_of_code::solution!(4);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_input(input);
    let map = find_words(data);
    let xmas = map.get("XMAS").unwrap_or(&0);
    let samx = map.get("SAMX").unwrap_or(&0);
    let sum = xmas + samx;
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_input(input);
    Some(find_all_cross_mass(data))
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let board = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    board
}

fn find_words(board: Vec<Vec<char>>) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    let rows = board.len();
    let cols = board[0].len();

    // we need a map of the number of occurences
    for y in 0..rows {
        for x in 0..cols {
            // Horizontal
            if x + 4 <= cols {
                let word = board[y][x..x + 4].iter().collect();
                *map.entry(word).or_insert(0) += 1;
            }

            // Vertical
            if y + 4 <= rows {
                let word = (0..4).map(|k| board[y + k][x]).collect();
                *map.entry(word).or_insert(0) += 1;
            }

            // Diagonal
            if y + 4 <= rows && x + 4 <= cols {
                let word = (0..4).map(|k| board[y + k][x + k]).collect();
                *map.entry(word).or_insert(0) += 1;
            }

            // Opposite Diagonal
            if y + 4 <= rows && x >= 3 {
                let word = (0..4).map(|k| board[y + k][x - k]).collect();
                *map.entry(word).or_insert(0) += 1;
            }
        }
    }

    map
}

fn find_all_cross_mass(board: Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    let rows = board.len();
    let cols = board[0].len();
    for y in 1..rows - 1 {
        for x in 1..cols - 1 {
            if board[y][x] == 'A' {
                let top_left = board[y - 1][x - 1];
                let top_right = board[y - 1][x + 1];
                let bottom_left = board[y + 1][x - 1];
                let bottom_right = board[y + 1][x + 1];

                let first_diagonal: String = [top_left, bottom_right].iter().collect();
                let second_diagonal: String = [top_right, bottom_left].iter().collect();

                let first_satisfies = first_diagonal == "SM" || first_diagonal == "MS";
                let second_satisfies = second_diagonal == "SM" || second_diagonal == "MS";

                if first_satisfies && second_satisfies {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_parse_input_diagonal() {
        let input = r#"T...T
.E.E.
..S..
.T.T.
"#;
        let data = parse_input(&input);
        let map = find_words(data);
        let test_count = map.get("TEST").expect("Should be in map");
        assert_eq!(*test_count, 2)
    }
}
