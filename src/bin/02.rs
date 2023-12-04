advent_of_code::solution!(2);

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Debug)]
enum Cubes {
    Blue(u32),
    Red(u32),
    Green(u32),
}
type Draw = Vec<Cubes>;
type Game = Vec<Draw>;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(read_line)
        .enumerate()
        .filter_map(|(i, game)| {
            (!game.iter().flatten().any(|cube| match cube {
                Cubes::Blue(x) => x > &MAX_BLUE,
                Cubes::Red(x) => x > &MAX_RED,
                Cubes::Green(x) => x > &MAX_GREEN,
            }))
            .then(|| (i + 1) as u32)
        })
        .sum::<u32>()
        .into()
}

fn read_line(line: &str) -> Game {
    let without_prefix = line.split(": ").last().unwrap();
    without_prefix
        .split("; ")
        .map(|game| {
            game.split(", ")
                .map(
                    |draw| match draw.split(" ").collect::<Vec<_>>().as_slice() {
                        [x, "blue"] => Cubes::Blue(x.parse().unwrap()),
                        [x, "red"] => Cubes::Red(x.parse().unwrap()),
                        [x, "green"] => Cubes::Green(x.parse().unwrap()),
                        _ => panic!("Failed to parse color"),
                    },
                )
                .collect()
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(read_line)
        .map(|game| {
            game.iter()
                .flat_map(|draw| draw.as_slice())
                .fold([0, 0, 0], |acc, cube| match cube {
                    Cubes::Red(x) => [acc[0].max(*x), acc[1], acc[2]],
                    Cubes::Green(x) => [acc[0], acc[1].max(*x), acc[2]],
                    Cubes::Blue(x) => [acc[0], acc[1], acc[2].max(*x)],
                })
                .iter()
                .product::<u32>()
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
