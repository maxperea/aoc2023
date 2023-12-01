advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut nums = line.chars().filter(|c| c.is_ascii_digit());
                let first = nums.next().unwrap().to_digit(10).unwrap();
                let last = nums.last();
                if let Some(last) = last {
                    let last = last.to_digit(10).unwrap();
                    first * 10 + last
                } else {
                    first * 10 + first
                }
            })
            .sum(),
    )
}

pub fn part_two(input_raw: &str) -> Option<u32> {
    Some(
        input_raw
            .lines()
            .map(|line| {
                let input = replace_alpha_w_num(line);
                let mut nums = input.chars().filter(|c| c.is_ascii_digit());
                let first = nums.next().unwrap().to_digit(10).unwrap();
                let last = nums.last();
                if let Some(last) = last {
                    let last = last.to_digit(10).unwrap();
                    first * 10 + last
                } else {
                    first * 10 + first
                }
            })
            .sum(),
    )
}

fn nums() -> [(&'static str, &'static str); 9] {
    [
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ]
}

fn replace_alpha_w_num(input: &str) -> String {
    let mut result = input.to_string();
    for (word, num) in nums() {
        result = result.replacen(word, num, 1000);
    }
    result
}

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
