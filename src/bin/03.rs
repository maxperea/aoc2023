use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut nums: Vec<u32> = vec![];
    for i in 0..matrix.len() {
        let mut current_num = vec![];
        let mut is_part = false;
        for j in 0..matrix[i].len() {
            if matrix[i][j].is_digit(10) {
                current_num.push(&matrix[i][j]);
                is_part |= check_adjacent(i as i32, j as i32, &matrix);
            }
            if j + 1 == matrix[i].len() || !matrix[i][j + 1].is_digit(10) {
                if is_part {
                    let num: String = current_num.clone().into_iter().collect();
                    nums.push(num.parse().unwrap());
                }
                current_num = vec![];
                is_part = false;
            }
        }
    }
    nums.iter().sum::<u32>().into()
}

fn check_adjacent(i: i32, j: i32, matrix: &Vec<Vec<char>>) -> bool {
    for di in [-1, 0, 1] {
        for dj in [-1, 0, 1] {
            if (i + di) > 0
                && (i + di) < matrix.len() as i32
                && (j + dj) > 0
                && (j + dj) < matrix.len() as i32
            {
                if !matrix[(i + di) as usize][(j + dj) as usize].is_digit(10)
                    && (matrix[(i + di) as usize][(j + dj) as usize] != '.')
                {
                    return true;
                }
            }
        }
    }
    false
}

fn check_adjacent_for_gear(i: i32, j: i32, matrix: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for di in [-1, 0, 1] {
        for dj in [-1, 0, 1] {
            if (i + di) > 0
                && (i + di) < matrix.len() as i32
                && (j + dj) > 0
                && (j + dj) < matrix.len() as i32
            {
                if matrix[(i + di) as usize][(j + dj) as usize] == '*' {
                    return Some(((i + di) as usize, (j + dj) as usize));
                }
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut nums: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for i in 0..matrix.len() {
        let mut current_num = vec![];
        let mut gear = None;
        for j in 0..matrix[i].len() {
            if matrix[i][j].is_digit(10) {
                current_num.push(&matrix[i][j]);
                if gear.is_none() {
                    gear = check_adjacent_for_gear(i as i32, j as i32, &matrix);
                }
            }
            if j + 1 == matrix[i].len() || !matrix[i][j + 1].is_digit(10) {
                if let Some(gear) = gear {
                    let num: u32 = current_num
                        .clone()
                        .into_iter()
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    nums.entry(gear).or_insert(vec![]).push(num)
                }
                current_num = vec![];
                gear = None;
            }
        }
    }
    nums.values()
        .filter(|l| l.len() > 1)
        .map(|l| l.iter().product::<u32>())
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
