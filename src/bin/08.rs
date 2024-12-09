advent_of_code::solution!(8);
use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, true))
}

fn solve(input: &str, multi_mode: bool) -> u32 {
    parse_input(input, multi_mode).len() as u32
}

fn iter_char_locations(input: &str) -> impl Iterator<Item = (char, i32, i32)> + '_ {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(move |(x, c)| (c, x as i32, y as i32))
    })
}

type Antennas = HashMap<char, HashSet<(i32, i32)>>;
type Antinodes = HashSet<(i32, i32)>;

fn within_bounds(x: i32, y: i32, max_x: i32, max_y: i32) -> bool {
    x >= 0 && x < max_x && y >= 0 && y < max_y
}

fn insert_antinode(
    antinodes: &mut Antinodes,
    start: (i32, i32),
    dir: (i32, i32),
    max: (i32, i32),
    multi_mode: bool,
) {
    let (max_x, max_y) = max;
    let (mut x, mut y) = start;
    let (dx, dy) = dir;

    if multi_mode {
        while within_bounds(x, y, max_x, max_y) {
            antinodes.insert((x, y));
            x += dx;
            y += dy;
        }
    } else {
        let (nx, ny) = (x + dx, y + dy);
        if within_bounds(nx, ny, max_x, max_y) {
            antinodes.insert((nx, ny));
        }
    }
}

fn parse_input(input: &str, multi_mode: bool) -> Antinodes {
    let rows = input.lines().count() as i32;
    let cols = input.lines().next().unwrap_or("").chars().count() as i32;
    let max = (cols, rows);

    let mut antenna_locations: Antennas = HashMap::new();
    let mut antinodes: Antinodes = HashSet::new();

    for (c, x, y) in iter_char_locations(input) {
        if c == '.' {
            continue;
        }

        if let Some(coordinates) = antenna_locations.get(&c) {
            for &(other_x, other_y) in coordinates {
                let dx = x - other_x;
                let dy = y - other_y;

                insert_antinode(&mut antinodes, (x, y), (dx, dy), max, multi_mode);
                insert_antinode(
                    &mut antinodes,
                    (other_x, other_y),
                    (-dx, -dy),
                    max,
                    multi_mode,
                );
            }
        }

        antenna_locations.entry(c).or_default().insert((x, y));
    }

    antinodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_char_locations() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let total = iter_char_locations(input).count();
        assert_eq!(total, 144)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
