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
    let mut game = parse(input);
    game.sort();
    game.iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as u32 + 1) * hand.bid)
        .into()
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<usize>,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
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
        let mut self_jcount = self_counts[0];
        let mut other_jcount = other_counts[0];

        if self_jcount == 5 {
            self_jcount = 0;
        } else {
            self_counts[0] = 0;
        }

        if other_jcount == 5 {
            other_jcount = 0;
        } else {
            other_counts[0] = 0;
        }

        let mut self_vec: Vec<i32> = self_counts.into_iter().filter(|x| x > &0).collect();
        let mut other_vec: Vec<i32> = other_counts.into_iter().filter(|x| x > &0).collect();

        let self_max = self_vec.iter().max().unwrap().clone();
        let other_max = other_vec.iter().max().unwrap().clone();

        if let Some(element) = self_vec.iter_mut().find(|&&mut x| x == self_max) {
            *element += self_jcount;
        }

        if let Some(element) = other_vec.iter_mut().find(|&&mut x| x == other_max) {
            *element += other_jcount;
        }

        if self_vec.iter().max() != other_vec.iter().max() {
            return self_vec.iter().max().cmp(&other_vec.iter().max()).into();
        } else if self_vec.len() != other_vec.len() {
            return other_vec.len().cmp(&self_vec.len()).into();
        } else {
            return self.cards.cmp(&other.cards).into();
        }
    }
}

fn char_to_card(c: char) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 0,
        'T' => 10,
        x => x.to_digit(10).unwrap() as usize,
    }
}

fn parse_hand(input: &str) -> Hand {
    let (cards, bid) = input.split_once(" ").unwrap();
    let cards = cards.chars().map(char_to_card).collect();
    Hand {
        cards,
        bid: bid.parse().unwrap(),
    }
}

fn parse(input: &str) -> Vec<Hand> {
    input.lines().map(parse_hand).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(6440));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_hands() {
        assert!(parse_hand("J2T3K 0") < parse_hand("42T4K 0"));
        assert!(parse_hand("44443 0") < parse_hand("22222 0"));
        assert!(parse_hand("44433 0") < parse_hand("22422 0"));
        assert!(parse_hand("44432 0") < parse_hand("22233 0"));
        assert!(parse_hand("44732 0") < parse_hand("22533 0"));
        assert!(parse_hand("44332 0") < parse_hand("22233 0"));
        assert!(parse_hand("44732 0") < parse_hand("22533 0"));
        assert!(parse_hand("23456 0") < parse_hand("22345 0"));
        assert!(parse_hand("23456 0") < parse_hand("23457 0"));

        assert!(parse_hand("32T3K 0") < parse_hand("42T4J 0"));
        assert!(parse_hand("44443 0") < parse_hand("22J2J 0"));
        assert!(parse_hand("44433 0") < parse_hand("2242J 0"));
        assert!(parse_hand("44432 0") < parse_hand("22J33 0"));
        assert!(parse_hand("44732 0") < parse_hand("225J3 0"));
        assert!(parse_hand("44332 0") < parse_hand("J2233 0"));
        assert!(parse_hand("44732 0") < parse_hand("2J533 0"));
        assert!(parse_hand("23456 0") < parse_hand("22345 0"));
        assert!(parse_hand("23456 0") < parse_hand("2J457 0"));
    }
}
