advent_of_code::solution!(5);
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, sequences) = parse_input(input);
    let valid_sequences = sequences
        .iter()
        .filter(|sequence| is_sorted(&rules, sequence))
        .map(|sequence| sequence[sequence.len() / 2])
        .sum();
    Some(valid_sequences)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, sequences) = parse_input(input);
    let now = Instant::now();
    let _sorted_sequences: u32 = sequences
        .iter()
        .filter(|sequence| !is_sorted(&rules, sequence))
        .map(|sequence| {
            let s = sorted_sequence(&rules, sequence);
            s[s.len() / 2]
        })
        .sum();
    println!("Execution time: {:?}", now.elapsed());

    let now = Instant::now();
    let sorted_with_graph = sequences
        .iter()
        .filter(|sequence| !is_sorted(&rules, sequence))
        .map(|sequence| {
            let (adj, mut in_degree) = rules_to_graph(&rules, sequence);
            let s = topological_sort(&adj, &mut in_degree);
            s[s.len() / 2]
        })
        .sum();
    println!("Execution time: {:?}", now.elapsed());
    Some(sorted_with_graph)
}

/// Rules has as key the number and value a set of numbers that should be ordered
/// after the key number.
type Rules = HashMap<u32, HashSet<u32>>;

/// Sequences is a list of sequences of numbers.
type Sequences = Vec<Vec<u32>>;

/// Sort the sequence based on the rules.
fn sorted_sequence(rules: &Rules, sequence: &[u32]) -> Vec<u32> {
    let mut sequence = sequence.to_vec();
    let mut seen = HashMap::new();
    let mut i = 0;
    while i < sequence.len() {
        let a = sequence[i];
        if let Some(set) = rules.get(&a) {
            for b in set {
                if let Some(j) = seen.get(b).copied() {
                    let value = sequence.remove(j);
                    sequence.push(value);
                    seen.remove(b);
                    for (_, v) in seen.iter_mut() {
                        if *v > j {
                            *v -= 1;
                        }
                    }
                    i -= 1
                }
            }
        }
        seen.insert(a, i);
        i += 1;
    }
    sequence
}

/// Checks if a sequence is sorted based on the provided rules.
fn is_sorted(rules: &Rules, sequence: &[u32]) -> bool {
    sequence
        .iter()
        .tuple_windows()
        .all(|(&a, &b)| is_ordered(rules, a, b))
}

/// Determines if two numbers are in the correct order based on the rules.
fn is_ordered(rules: &Rules, a: u32, b: u32) -> bool {
    match rules.get(&a) {
        Some(set) => set.contains(&b),
        None => false,
    }
}

type Graph = HashMap<u32, HashSet<u32>>;
type InDegree = HashMap<u32, usize>;

fn rules_to_graph(rules: &Rules, sequence: &[u32]) -> (Graph, InDegree) {
    let mut adj: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();
    let mut sequence_set: HashSet<u32> = HashSet::new();

    // Initialize adjacency list and in-degree count
    for &node in sequence {
        adj.entry(node).or_default();
        in_degree.entry(node).or_insert(0);
        sequence_set.insert(node);
    }

    for (&from, destinations) in rules {
        for &destination in destinations {
            if sequence_set.contains(&from) && sequence_set.contains(&destination) {
                adj.entry(from).or_default().insert(destination);
                in_degree.entry(destination).and_modify(|x| *x += 1);
            }
        }
    }

    (adj, in_degree)
}

fn topological_sort(adj: &Graph, in_degree: &mut InDegree) -> Vec<u32> {
    let mut queue = VecDeque::new();
    for (&node, &degree) in in_degree.iter() {
        if degree == 0 {
            queue.push_back(node);
        }
    }

    let mut sorted = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted.push(node);
        if let Some(neighbors) = adj.get(&node) {
            for &neighbor in neighbors {
                let degree = in_degree.get_mut(&neighbor).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }
    sorted
}

pub fn parse_input(input: &str) -> (Rules, Sequences) {
    let (first_part, second_part) = input.split_once("\n\n").expect("Invalid input");

    let rules: Rules = first_part
        .lines()
        .map(|line| {
            let (a, b) = line
                .split('|')
                .map(|x| x.parse::<u32>().expect("Not numeric value in input"))
                .collect_tuple()
                .expect("More than two values in input");
            (a, b)
        })
        .fold(HashMap::new(), |mut acc, (a, b)| {
            acc.entry(a).or_default().insert(b);
            acc
        });

    let sequences = second_part
        .lines()
        .map(|line| {
            line.trim()
                .split(',')
                .map(|x| x.parse::<u32>().expect("Not numeric value in input"))
                .collect()
        })
        .collect();

    (rules, sequences)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (rules, sequences) = parse_input(input);
        assert_eq!(rules.len(), 6);
        assert_eq!(sequences.len(), 6);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
