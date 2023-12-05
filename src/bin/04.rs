use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(parse_line)
        .map(
            |(winning, mine)| match winning.intersection(&mine).count() {
                0 => 0,
                x => 2_u32.pow(x as u32 - 1),
            },
        )
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards: Vec<u32> = input
        .lines()
        .map(parse_line)
        .map(|(winning, mine)| winning.intersection(&mine).count() as u32)
        .collect();
    let mut card_count = cards.len() as u32;
    let mut queue: VecDeque<u32> = (1..card_count).collect();
    cards.insert(0, 0); // Fix index
    while !queue.is_empty() {
        let card = queue.pop_front().unwrap();
        let winning_nums = cards.get(card as usize).unwrap().clone();
        let mut range_deque: VecDeque<u32> = ((card + 1)..=(card + winning_nums)).collect();
        queue.append(&mut range_deque);
        card_count += winning_nums;
    }
    Some(card_count)
}

fn parse_line(line: &str) -> (HashSet<i32>, HashSet<i32>) {
    let line_data = line.split(':').nth(1).unwrap_or("").trim();
    let parts: Vec<&str> = line_data.split('|').collect();
    let first_part = parts[0]
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let second_part = parts[1]
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    (first_part, second_part)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
