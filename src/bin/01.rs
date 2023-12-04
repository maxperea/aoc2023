advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let numbers: Vec<_> = line.chars().filter_map(|s| s.to_digit(10)).collect();
                match numbers.as_slice() {
                    [] => 0,
                    [first] => first * 11,
                    [first, .., last] => first * 10 + last,
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let alphas_replaced_with_num = NUMS.iter().fold(input.to_string(), |acc, (word, num)| {
        acc.replacen(word, num, 1000)
    });
    part_one(&alphas_replaced_with_num)
}

const NUMS: [(&'static str, &'static str); 9] = [
    ("one", "one1one"),
    ("two", "two2two"),
    ("three", "three3three"),
    ("four", "four4four"),
    ("five", "five5five"),
    ("six", "six6six"),
    ("seven", "seven7seven"),
    ("eight", "eight8eight"),
    ("nine", "nine9nine"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }
}
