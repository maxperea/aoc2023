advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut game = parse(input);
    game.sort();
    game.iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as u32 + 1) * hand.bid)
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand {
    cards: Vec<usize>,
    bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut self_counts = [0; 15];
        let mut other_counts = [0; 15];
        for &card in &self.cards {
            self_counts[card] += 1;
        }
        for &card in &other.cards {
            other_counts[card] += 1;
        }
        // let self_jcount = self_counts[11];
        // let other_jcount = self_counts[11];
        let self_vec: Vec<_> = self_counts.iter().filter(|x| x > &&0).collect();
        let other_vec: Vec<_> = other_counts.iter().filter(|x| x > &&0).collect();

        if self_vec.iter().max() != other_vec.iter().max() {
            self_vec.iter().max().cmp(&other_vec.iter().max())
        } else if self_vec.len() != other_vec.len() {
            other_vec.len().cmp(&self_vec.len())
        } else {
            self.cards.cmp(&other.cards)
        }
        .into()
    }
}

fn char_to_card(c: char) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        x => x.to_digit(10).unwrap() as usize,
    }
}

fn parse(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|hand| {
            let (cards, bid) = hand.split_once(" ").unwrap();
            let cards = cards.chars().map(char_to_card).collect();
            Hand {
                cards,
                bid: bid.parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
