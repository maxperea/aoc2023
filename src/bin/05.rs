use std::iter;

advent_of_code::solution!(5);

type MapEntry = (i64, i64, i64);
type Map = Vec<MapEntry>;
type Seed = i64;

pub fn part_one(input: &str) -> Option<Seed> {
    let (maps, seeds) = parse(input);
    seeds
        .iter()
        .map(|&seed| maps.iter().fold(seed, |acc, map| apply_map(acc, map)))
        .min()
}

pub fn part_two(input: &str) -> Option<Seed> {
    let (maps, seeds) = parse(input);
    let init_map: Map = seeds
        .chunks(2)
        .map(|part| (part[0], part[0] + part[1], 0))
        .collect();

    iter::once(init_map)
        .chain(maps)
        .reduce(sum_maps)
        .unwrap()
        .iter()
        .map(|(start, _, shift)| start.clone())
        .min()
}

fn apply_map(seed: Seed, map: &[MapEntry]) -> Seed {
    for &(start, end, shift) in map {
        if seed >= start && seed <= end {
            return seed + shift;
        }
    }
    seed
}

fn sum_maps(mut source: Map, mut target: Map) -> Map {
    source.sort_by_key(|s| s.0);
    target.sort_by_key(|s| s.0);

    println!("{:?}", source);
    println!("{:?}", target);
    let mut sum: Map = vec![];
    for &(start, end, shift) in source.iter() {
        let mut s_start = start;
        let s_end = end;
        if s_end <= target.iter().next().unwrap().0 {
            sum.push((s_start, s_end, shift));
            continue;
        }

        while s_start <= s_end {
            if s_start >= target.iter().last().unwrap().1 {
                println!("after...");
                sum.push((s_start, s_end, shift));
                break;
            }
            for &(t_start, t_end, t_shift) in target.iter() {
                println!("{} {}", s_start, s_end);
                if s_end >= t_start && s_start < t_start {
                    println!("first hit");
                    sum.push((s_start, t_start - 1, shift));
                    s_start = t_start;
                } else if s_start >= t_start && s_end <= t_end {
                    println!("inside");
                    sum.push((s_start, s_end, shift + t_shift));
                    s_start = s_end + 1;
                    break;
                } else if s_start <= t_start && s_end >= t_end {
                    println!("third hit");
                    sum.push((t_start, t_end, shift + t_shift));
                    s_start = t_end + 1;
                } else if s_start >= t_start && s_start <= t_end {
                    println!("lower hit");
                    sum.push((s_start, t_end, shift + t_shift));
                    s_start = t_end + 1;
                } else if s_end < t_start {
                    sum.push((s_start, s_end, shift));
                    s_start = s_end + 1;
                    break;
                } else {
                    println!("{:?}", source);
                    println!("{:?}", target);
                    println!("no hit");
                }
            }
        }
    }
    println!("{:?}\n=======\n", sum);
    let sum = sum
        .iter()
        .map(|(start, end, shift)| (start + shift, end + shift, 0))
        .collect();
    println!("{:?}\n=======\n", sum);
    sum
}

fn parse(input: &str) -> (Vec<Map>, Vec<Seed>) {
    let seeds: Vec<Seed> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|num| num.parse::<i64>().ok())
        .collect();

    let maps = input
        .split("\n\n")
        .skip(1)
        .map(|group| {
            group
                .lines()
                .skip(1)
                .map(|line| {
                    let nums: Vec<_> = line
                        .split_whitespace()
                        .filter_map(|num| num.parse::<i64>().ok())
                        .collect();
                    (nums[1], nums[1] + nums[2] - 1, nums[0] - nums[1])
                })
                .collect()
        })
        .collect();
    (maps, seeds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_sum_maps() {
        //Inside
        let first = vec![(0, 50, 4)];
        let second = vec![(25, 40, 7)];
        let expected = vec![(4, 28, 0), (36, 51, 0), (45, 54, 0)];
        assert_eq!(expected, sum_maps(first, second));

        // Upper
        let first = vec![(0, 27, 4)];
        let second = vec![(25, 40, 7)];
        let expected = vec![(4, 28, 0), (36, 38, 0)];
        assert_eq!(expected, sum_maps(first, second));

        // Lower
        let first = vec![(30, 50, 4)];
        let second = vec![(25, 40, 7)];
        let expected = vec![(41, 51, 0), (45, 54, 0)];
        assert_eq!(expected, sum_maps(first, second));

        // Around
        let first = vec![(10, 15, 4)];
        let second = vec![(0, 20, 1)];
        let expected = vec![(15, 20, 0)];
        assert_eq!(expected, sum_maps(first, second));

        // Outside below
        let first = vec![(10, 15, 4)];
        let second = vec![(0, 5, 1)];
        let expected = vec![(14, 19, 0)];
        assert_eq!(expected, sum_maps(first, second));

        // Outside above
        let first = vec![(0, 5, 4)];
        let second = vec![(10, 20, 1)];
        let expected = vec![(4, 9, 0)];
        assert_eq!(expected, sum_maps(first, second));

        // Middle
        let first = vec![(30, 35, 1)];
        let second = vec![(5, 20, 1), (40, 45, 1)];
        let expected = vec![(31, 36, 0)];
        assert_eq!(expected, sum_maps(first, second));
    }
}
