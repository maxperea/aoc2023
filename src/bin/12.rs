advent_of_code::solution!(12);
use std::collections::VecDeque;

// ????????????      3,2,1
//                   111 11 1 -> 6 zeroes at 4 pos , start+end can be empty
// 3 groups = 4 positions
// Put zeroes between groups
// 4 zeroes at 4 positions
// 4, 0, 0, 0
// 4!
//

// ???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3
// 15 groups, 39 length, 16 positions, 25 1s, 14 0s
// Put zeroes between groups
// 0 zeroes at 16 positions -> 1 arrangement

// ?###??????????###??????????###??????????###??????????###???????? 3,2,1,3,2,1,3,2,1,3,2,1,3,2,1,
// 15 groups, 64 length, 16 positions, 30 1s, 34 0s
// Put zeroes between groups.
// 20 zeroes left, 16 positions

// ?###???????? 3,2,1
// ??????? 2,1

pub fn part_one(input: &str) -> Option<usize> {
    let nums = parse(input);
    nums.iter()
        .map(|(springs, counts)| {
            get_all(springs.to_vec())
                .iter()
                .map(count_consecutive)
                .filter(|c| c == counts)
                .count()
        })
        .sum::<usize>()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let nums = parse(input);
    nums.iter()
        .map(|(springs, counts)| {
            let last = get_all(springs.to_vec())
                .iter()
                .map(count_consecutive)
                .filter(|c| c == counts)
                .count();
            let mut append_q = springs.to_vec();
            // match append_q.last().unwrap() {
            //     '#' => (),
            //     '.' => append_q.insert(0, '?'),
            //     '?' => append_q.push('?'),
            //     _ => panic!(),
            // };
            // append_q.insert(0, '?');
            // append_q.insert(0, append_q.last().unwrap().clone());
            append_q.push('?');
            let others = get_all(append_q)
                .iter()
                .map(count_consecutive)
                .filter(|c| c == counts)
                .count();

            let res = others * others * others * others * last;
            println!("{}", res);
            res
        })
        .sum::<usize>()
        .into()
}

// ???.### 1,1,3
// ???.###.???.###????.###

fn parse(input: &str) -> Vec<(Vec<char>, Vec<i32>)> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(" ").unwrap();
            let counts = second
                .split(',')
                .filter_map(|c| c.parse::<i32>().ok())
                .collect();
            (first.chars().collect(), counts)
        })
        .collect()
}

fn count_consecutive(arr: &Vec<char>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut count = 0;
    for c in arr {
        if *c == '#' {
            count += 1;
        } else if count > 0 {
            result.push(count);
            count = 0;
        }
    }
    // Handle the last group if it's ones
    if count > 0 {
        result.push(count);
    }
    result
}

fn get_all(input: Vec<char>) -> Vec<Vec<char>> {
    let mut queue = VecDeque::new();
    let mut combinations = Vec::new();
    queue.push_back(input);

    while let Some(current) = queue.pop_front() {
        match current.iter().position(|&c| c == '?') {
            Some(index) => {
                let mut replace_with_hash = current.clone();
                replace_with_hash[index] = '#';
                queue.push_back(replace_with_hash);

                let mut replace_with_dot = current;
                replace_with_dot[index] = '.';
                queue.push_back(replace_with_dot);
            }
            None => combinations.push(current),
        }
    }

    combinations
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_thesis() {
    //     println!("{:?}", (part_one("?###????????? 3,2,1")));
    //     println!("{}", 15 * 15 * 15 * 15 * 10);
    //     panic!();
    // }

    // #[test]
    // fn test_all() {
    //     println!("{:?}", get_all(vec!['?', '?', '?']));
    //     panic!()
    // }

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(21));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
        panic!();
    }
}
