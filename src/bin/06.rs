advent_of_code::solution!(6);
use itertools::Itertools;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let (map, mut start, mut direction) = parse_input(input);
    let visited = walk(&map, &mut start, &mut direction);
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

/// Tile in the map
#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            _ => panic!("Invalid tile"),
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn remove(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn get_in_map<'a>(&self, map: &'a Map) -> Option<&'a Tile> {
        let (x, y): (usize, usize) = [self.x, self.y]
            .iter()
            .map(|&n| n.try_into().expect(&format!("Invalid usize: {}", n)))
            .collect_tuple()?;

        map.get(y).and_then(|row| row.get(x))
    }

    fn from_direction(c: &char) -> Self {
        match c {
            '^' => Point::new(0, -1),
            'V' => Point::new(0, 1),
            '<' => Point::new(-1, 0),
            '>' => Point::new(1, 0),
            _ => panic!("Not expected character!"),
        }
    }

    fn rotate_cw(&mut self) {
        (self.x, self.y) = (self.y, -self.x);
    }

    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

type Map = Vec<Vec<Tile>>;

fn parse_input(input: &str) -> (Map, Point, Point) {
    let mut start = Point::new(0, 0);
    let mut direction = Point::new(0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '^' | 'v' | '<' | '>' => {
                        start = Point::new(col as i32, row as i32);
                        direction = Point::from_direction(&c);
                        Tile::Empty
                    }
                    _ => Tile::from_char(c),
                })
                .collect()
        })
        .collect();
    (map, start, direction)
}

fn walk(map: &Map, position: &mut Point, direction: &mut Point) -> HashSet<(i32, i32)> {
    let mut visited = HashSet::new();

    while let Some(&tile) = position.get_in_map(map) {
        println!("{:?}, {:?}", position.as_tuple(), direction.as_tuple());
        match tile {
            Tile::Empty => {
                visited.insert(position.as_tuple());
            }
            Tile::Wall => {
                *position = position.remove(direction);
                direction.rotate_cw();
            }
        }
        *position = position.add(direction);
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (map, start, direction) = parse_input(input);
        assert_eq!(map.len(), 10);
        assert_eq!(start, Point::new(4, 6));
        assert_eq!(direction, Point::new(0, 1));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
