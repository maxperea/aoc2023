use num_integer::gcd;
use std::collections::HashMap;

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn least_common_denominator(numbers: &[u64]) -> u64 {
    numbers.iter().fold(1, |acc, &num| lcm(acc, num))
}

use regex::Regex;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let (sequence, map) = parse(input);
    let (mut node, mut steps) = ("AAA", 0);
    let mut direction_iter = sequence.chars().cycle();
    while node != "ZZZ" {
        match direction_iter.next().unwrap() {
            'L' => node = &map[node].0,
            'R' => node = &map[node].1,
            _ => panic!(),
        }
        steps += 1;
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (sequence, map) = parse(input);
    let mut start_nodes: Vec<(&str, u64)> = map
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|&node| (node, 0))
        .collect();
    let mut direction_iter = sequence.chars().cycle();
    while start_nodes.iter().any(|node| !node.0.ends_with("Z")) {
        let direction = direction_iter.next().unwrap();
        for node in start_nodes.iter_mut().filter(|node| !node.0.ends_with("Z")) {
            node.1 += 1;
            match direction {
                'L' => node.0 = map[node.0].0,
                'R' => node.0 = map[node.0].1,
                _ => panic!(),
            }
        }
    }
    let cycle_sizes: Vec<_> = start_nodes.iter().map(|t| t.1).collect();
    least_common_denominator(&cycle_sizes).into()
}

fn parse(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let (sequence, map_raw) = input.split_once("\n\n").unwrap();
    let mut map = HashMap::new();

    let re = Regex::new(r"\b[A-Za-z0-9]{3}\b").unwrap();
    for line in map_raw.lines() {
        let matches: Vec<_> = re.find_iter(line).map(|match_| match_.as_str()).collect();
        map.insert(matches[0], (matches[1], matches[2]));
    }

    (sequence, map)
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
    fn test_part_two() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let result = part_two(input);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_2_part_two() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
33A = (33B, XXX)
33B = (33C, 33C)
33C = (33D, 33D)
33D = (33Z, 33Z)
33Z = (33B, 33B)
";

        let result = part_two(input);
        assert_eq!(result, Some(12));
    }
}
