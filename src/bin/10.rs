use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

type Direction = (i32, i32);

const NW: Direction = (-1, -1);
const NE: Direction = (1, -1);

const N: Direction = (0, -1);
const E: Direction = (1, 0);
const W: Direction = (-1, 0);
const S: Direction = (0, 1);

const SW: Direction = (-1, 1);
const SE: Direction = (1, 1);

const START: (usize, usize, Direction) = (73, 83, S); // Cheat and read manually

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pipe {
    // Start,
    Empty,
    Vertical,
    Horizontal,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub fn part_one(input: &str) -> Option<usize> {
    get_loop(&parse(input)).len().div_ceil(2).into()
}

pub fn part_two(input: &str) -> Option<usize> {
    use Pipe::*;
    let map = parse(input);
    let pipe_loop = get_loop(&map);
    let mut inside_tiles = HashSet::new();
    let (mut x, mut y, mut direction) = START;
    let start_pos = (x, y);
    loop {
        let pipe = map[y][x];

        if let Some(dirs) = match (pipe, direction) {
            (Horizontal, E) => Some([SW, S, SE]),
            (Horizontal, W) => Some([NW, N, NE]),
            (Vertical, N) => Some([NE, E, SE]),
            (Vertical, S) => Some([NW, W, SW]),
            (TopLeft, W) => Some([W, NW, N]),
            (TopRight, N) => Some([E, NE, N]),
            (BottomLeft, S) => Some([W, SW, S]),
            (BottomRight, E) => Some([E, SE, S]),
            _ => None,
        } {
            let positions: Vec<_> = dirs
                .iter()
                .map(|&dir| update_pos(x, y, dir))
                .filter(|pos| !pipe_loop.contains(pos))
                .collect();

            inside_tiles.extend(get_connected_tiles(&pipe_loop, positions));
        }

        (x, y, direction) = advance(x, y, map[y][x], direction);

        if (x, y) == start_pos {
            break;
        }
    }

    inside_tiles.len().into()
}

fn update_pos(x: usize, y: usize, (dx, dy): Direction) -> (usize, usize) {
    ((x as i32 + dx) as usize, (y as i32 + dy) as usize)
}

fn get_connected_tiles(
    pipe_loop: &HashSet<(usize, usize)>,
    positions: Vec<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut connected_tiles: HashSet<_> = positions.iter().cloned().collect();
    let mut queue: VecDeque<_> = positions.into_iter().collect();

    while let Some((x, y)) = queue.pop_front() {
        for dir in [N, W, S, E].iter().map(|&dir| update_pos(x, y, dir)) {
            if pipe_loop.contains(&dir) || !connected_tiles.insert(dir) {
                continue;
            }
            queue.push_back(dir);
        }
    }
    connected_tiles
}

fn get_loop(map: &Vec<Vec<Pipe>>) -> HashSet<(usize, usize)> {
    let mut pipe_loop = HashSet::new();
    let (mut x, mut y, mut direction) = START;
    while pipe_loop.insert((x, y)) {
        (x, y, direction) = advance(x, y, map[y][x], direction);
    }
    pipe_loop
}

fn advance(x: usize, y: usize, pipe: Pipe, direction: Direction) -> (usize, usize, Direction) {
    use Pipe::*;
    let new_dir = match (pipe, direction) {
        (Vertical, N) => direction,
        (Vertical, S) => direction,
        (Horizontal, W) => direction,
        (Horizontal, E) => direction,
        (TopLeft, N) => E,
        (TopLeft, W) => S,
        (TopRight, N) => W,
        (TopRight, E) => S,
        (BottomRight, S) => W,
        (BottomRight, E) => N,
        (BottomLeft, S) => E,
        (BottomLeft, W) => N,
        _ => panic!(),
    };

    let (x, y) = update_pos(x, y, new_dir);
    (x, y, new_dir)
}

fn parse(input: &str) -> Vec<Vec<Pipe>> {
    input
        .lines()
        .map(|line| line.chars().map(char_to_pipe).collect())
        .collect()
}

fn char_to_pipe(cell: char) -> Pipe {
    use Pipe::*;
    match cell {
        'S' => TopRight, // Let's replace with actual piece
        '|' => Vertical,
        '-' => Horizontal,
        'F' => TopLeft,
        '7' => TopRight,
        'L' => BottomLeft,
        'J' => BottomRight,
        '.' => Empty,
        _ => panic!(),
    }
}

type Position = (usize, usize);

fn part_two_math(input: &str) -> Option<u32> {
    return Some(calc_interior(&get_loop_vec(&parse(input))));
}

fn get_loop_vec(map: &Vec<Vec<Pipe>>) -> Vec<Position> {
    let mut pipe_loop = vec![];
    let start = (START.0, START.1);
    let (mut x, mut y, mut direction) = START;
    loop {
        pipe_loop.push((x, y));
        (x, y, direction) = advance(x, y, map[y][x], direction);
        if (x, y) == start {
            break;
        }
    }
    pipe_loop
}

fn calc_interior(pipe_loop: &[Position]) -> u32 {
    let mut vertices: Vec<Position> = pipe_loop.to_vec();
    vertices.push(pipe_loop[0]);

    // Shoelace formula
    let mut area: i32 = 0;
    for i in 0..(vertices.len() - 1) {
        let first = vertices[i];
        let second = vertices[i + 1];
        area += (first.0 as i32 * second.1 as i32) - (first.1 as i32 * second.0 as i32);
    }
    area = i32::abs(area) / 2;

    // Pick's theorem
    let perimeter: i32 = vertices.len() as i32 - 1;
    let interior: i32 = area - (perimeter / 2) + 1;
    interior as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
