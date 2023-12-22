advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    input
        .lines()
        .map(parse_line)
        .map(all_levels)
        .map(next_value)
        .sum::<i64>()
        .into()
}

pub fn part_two(input: &str) -> Option<i64> {
    input
        .lines()
        .map(parse_line)
        .map(all_levels)
        .map(previous_value)
        .sum::<i64>()
        .into()
}

fn parse_line(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .filter_map(|num| num.parse::<i64>().ok())
        .collect()
}

fn next_level(level: &[i64]) -> Vec<i64> {
    level.windows(2).fold(vec![], |mut acc, new| {
        acc.push(new[1] - new[0]);
        acc
    })
}

fn all_levels(level: Vec<i64>) -> Vec<Vec<i64>> {
    let mut all_levels: Vec<Vec<i64>> = vec![level.iter().map(|x| *x).collect()];
    while !all_levels.last().unwrap().iter().all(|x| *x == 0) {
        all_levels.push(next_level(all_levels.last().unwrap()));
    }
    all_levels
}

fn next_value(mut all_levels: Vec<Vec<i64>>) -> i64 {
    all_levels.last_mut().unwrap().push(0);
    for i in (0..all_levels.len() - 1).rev() {
        let x = all_levels[i].last().unwrap().clone();
        let y = all_levels[i + 1].last().unwrap().clone();
        all_levels[i].push(x + y);
    }
    *all_levels.first().unwrap().last().unwrap()
}

fn previous_value(mut all_levels: Vec<Vec<i64>>) -> i64 {
    all_levels.last_mut().unwrap().insert(0, 0);
    for i in (0..all_levels.len() - 1).rev() {
        let x = all_levels[i].first().unwrap().clone();
        let y = all_levels[i + 1].first().unwrap().clone();
        all_levels[i].insert(0, x - y);
    }
    *all_levels.first().unwrap().first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_levels_test() {
        assert_eq!(
            all_levels(vec![1, 2, 3]),
            vec![vec![1, 2, 3], vec![1, 1], vec![0]]
        );
    }

    #[test]
    fn next_level_test() {
        assert_eq!(next_level(&[1, 2, 3]), [1, 1]);
        assert_eq!(next_level(&[1, 1]), [0]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
