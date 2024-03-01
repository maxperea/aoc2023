advent_of_code::solution!(11);

use advent_of_code::utility::*;

pub fn part_one(input: &str) -> Option<usize> {
    let image: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    image_to_paths(image, 2).iter().sum::<usize>().into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let image: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let shrink_distances = image_to_paths(image.clone(), 0);
    let original_distances = image_to_paths(image.clone(), 1);
    let expansion_rate = 1_000_000;
    original_distances
        .iter()
        .zip(shrink_distances.iter())
        .map(|(shrink, orig)| (shrink - orig) * expansion_rate + orig)
        .sum::<usize>()
        .into()
}

fn image_to_paths(image: Vec<Vec<char>>, expansion: usize) -> Vec<usize> {
    paths(galaxies(transpose(expand(
        transpose(expand(image, expansion)),
        expansion,
    ))))
}

fn expand(board: Vec<Vec<char>>, times: usize) -> Vec<Vec<char>> {
    board
        .iter()
        .flat_map(|row| match row.iter().all(|&c| c == '.') {
            true => vec![row.clone(); times],
            false => vec![row.clone()],
        })
        .collect()
}

fn galaxies(image: Vec<Vec<char>>) -> Vec<Position> {
    let mut positions = vec![];
    for y in 0..image.len() {
        for x in 0..image[0].len() {
            if image[y][x] == '#' {
                positions.push((x, y));
            }
        }
    }
    positions
}

fn paths(positions: Vec<Position>) -> Vec<usize> {
    let mut paths = vec![];
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            paths.push(manhattan_distance(positions[i], positions[j]));
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
