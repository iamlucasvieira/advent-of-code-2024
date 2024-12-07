advent_of_code::solution!(6);
use std::collections::HashSet;
use std::ops::{Add, Sub};

pub fn part_one(input: &str) -> Option<u32> {
    let (map, mut start, mut direction) = parse_input(input);
    let visited = walk_with_obstruction(&map, &mut start, &mut direction, None)?;
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start, direction) = parse_input(input);
    let cyclic_paths = find_cyclic_paths(&map, start, direction);
    Some(cyclic_paths as u32)
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

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    /// Returns the tile at the given point from the map if valid.
    fn get_in_map<'a>(&self, map: &'a Map) -> Option<&'a Tile> {
        if self.x < 0 || self.y < 0 {
            return None;
        }
        let (x, y) = (self.x as usize, self.y as usize);
        map.get(y)?.get(x)
    }

    fn from_direction(c: char) -> Self {
        match c {
            '^' => Point::new(0, -1),
            'v' => Point::new(0, 1),
            '<' => Point::new(-1, 0),
            '>' => Point::new(1, 0),
            _ => panic!("Unexpected direction character: {}", c),
        }
    }

    /// Rotates the point direction 90 degrees counter-clockwise.
    fn rotate_ccw(&mut self) {
        let (old_x, old_y) = (self.x, self.y);
        self.x = -old_y;
        self.y = old_x;
    }
}

/// Implement addition for Point to simplify arithmetic
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

/// Implement subtraction for Point to simplify arithmetic
impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
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
                    '^' | 'V' | '<' | '>' => {
                        start = Point::new(col as i32, row as i32);
                        direction = Point::from_direction(c);
                        Tile::Empty
                    }
                    _ => Tile::from_char(c),
                })
                .collect()
        })
        .collect();
    (map, start, direction)
}

fn walk_with_obstruction(
    map: &Map,
    position: &mut Point,
    direction: &mut Point,
    obstruction: Option<Point>,
) -> Option<HashSet<Point>> {
    let mut visited_positions = HashSet::new();
    let mut visited_states = HashSet::new();

    while let Some(&tile) = position.get_in_map(map) {
        let effective_tile = if Some(*position) == obstruction {
            Tile::Wall
        } else {
            tile
        };

        match effective_tile {
            Tile::Empty => {
                visited_positions.insert(*position);

                // Check for cycle
                let state = (*position, *direction);
                if !visited_states.insert(state) {
                    return None; // Cycle found
                }
            }
            Tile::Wall => {
                *position = *position - *direction;
                direction.rotate_ccw();
            }
        }

        *position = *position + *direction;
    }
    Some(visited_positions)
}

fn find_cyclic_paths(map: &Map, start: Point, direction: Point) -> usize {
    let possible_obstructions =
        walk_with_obstruction(map, &mut start.clone(), &mut direction.clone(), None)
            .unwrap()
            .iter()
            .filter(|&&point| point != start)
            .cloned()
            .collect::<Vec<_>>();

    possible_obstructions
        .iter()
        .map(|&obstruction| {
            let mut new_start = start;
            let mut new_direction = direction;
            walk_with_obstruction(map, &mut new_start, &mut new_direction, Some(obstruction))
        })
        .filter(|visited| visited.is_none())
        .count()
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
        assert_eq!(direction, Point::new(0, -1));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
