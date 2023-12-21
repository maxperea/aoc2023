advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    parse(input)
        .iter()
        .map(race_to_score)
        .product::<u32>()
        .into()
}

type Race = (i64, i64);

fn race_to_score(race: &Race) -> u32 {
    (0..=race.0).fold(0, |total, i| match i * (race.0 - i) > race.1 {
        true => total + 1,
        false => total,
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    race_to_score(&parse_part_2(input)).into()
}

fn parse(input: &str) -> Vec<Race> {
    let nums: Vec<_> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .collect();
    nums[0]
        .iter()
        .zip(nums[1].iter())
        .map(|(&a, &b)| (a, b))
        .collect()
}

fn parse_part_2(input: &str) -> Race {
    let nums: Vec<_> = input
        .lines()
        .filter_map(|line| {
            let (_, end) = line.split_once(":").unwrap();
            let no_whitespace: String = end.chars().filter(|c| !c.is_whitespace()).collect();
            no_whitespace.parse::<i64>().ok()
        })
        .collect();
    (nums[0], nums[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_to_score() {
        let result = race_to_score(&(7, 9));
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
