use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

const NW: (i32, i32) = (-1, -1);
const NE: (i32, i32) = (1, -1);

const N: (i32, i32) = (0, -1);
const E: (i32, i32) = (1, 0);
const W: (i32, i32) = (-1, 0);
const S: (i32, i32) = (0, 1);

const SW: (i32, i32) = (-1, 1);
const SE: (i32, i32) = (1, 1);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pipe {
    Start,
    Empty,
    Vertical,
    Horizontal,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    East,
    South,
    West,
    North,
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse(input);
    let pipe_loop = get_loop(&map);
    Some(pipe_loop.len().div_ceil(2))
}
fn update_pos(x: usize, y: usize, dx: i32, dy: i32) -> (usize, usize) {
    ((x as i32 + dx) as usize, (y as i32 + dy) as usize)
}
pub fn part_two(input: &str) -> Option<usize> {
    use Direction::*;
    use Pipe::*;
    // We are going clockwise
    let map = parse(input);
    let pipe_loop = get_loop(&map);
    let mut inside_tiles = HashSet::new();
    let (mut x, mut y, mut direction) = find_start(&map);
    let start = (x, y);
    loop {
        let pipe = map[y][x];

        let dirs = match (pipe, direction) {
            (Horizontal, East) => [SW, S, SE],
            (Horizontal, West) => [NW, N, NE],
            (Vertical, North) => [NE, E, SE],
            (Vertical, South) => [NW, W, SW],

            (TopLeft, West) => [W, NW, N],
            (TopRight, North) => [E, NE, N],
            (BottomLeft, South) => [W, SW, S],
            (BottomRight, East) => [E, SE, S],

            (TopLeft, North) => [SE, SE, SE],
            (TopRight, East) => [SW, SW, SW],
            (BottomLeft, West) => [NE, NE, NE],
            (BottomRight, South) => [NW, NW, NW],
            _ => panic!(),
        };

        let positions: Vec<_> = dirs
            .iter()
            .map(|(dx, dy)| update_pos(x, y, *dx, *dy))
            .filter(|ele| !pipe_loop.contains(ele))
            .collect();

        let connected_tiles = get_connected_tiles(&pipe_loop, positions);
        for tile in connected_tiles {
            inside_tiles.insert(tile);
        }

        (x, y, direction) = advance(x, y, map[y][x], direction);

        if (x, y) == start {
            break;
        }
    }
    for y in 0..142 {
        for x in 0..142 {
            if pipe_loop.contains(&(x, y)) {
                print!("{}", pipe_to_char(map[y][x]));
            } else if inside_tiles.contains(&(x, y)) {
                print!("o");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    inside_tiles.len().into()
}

fn get_connected_tiles(
    pipe_loop: &HashSet<(usize, usize)>,
    positions: Vec<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut connected_tiles = HashSet::new();
    for ele in positions {
        queue.push_back(ele);
        connected_tiles.insert(ele);
    }

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let pos = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            if pipe_loop.contains(&pos) || connected_tiles.contains(&pos) {
                continue;
            } else {
                connected_tiles.insert(pos);
                queue.push_back(pos);
            }
        }
    }
    connected_tiles
}

fn find_start(_map: &Vec<Vec<Pipe>>) -> (usize, usize, Direction) {
    (73, 83, Direction::South)
}

fn get_loop(map: &Vec<Vec<Pipe>>) -> HashSet<(usize, usize)> {
    let mut pipe_loop = HashSet::new();
    let (mut x, mut y, mut direction) = find_start(&map);
    loop {
        (x, y, direction) = advance(x, y, map[y][x], direction);
        if pipe_loop.contains(&(x, y)) {
            break;
        }
        pipe_loop.insert((x, y));
    }
    pipe_loop
}

fn parse(input: &str) -> Vec<Vec<Pipe>> {
    input
        .lines()
        .map(|line| line.chars().map(char_to_pipe).collect())
        .collect()
}

fn advance(x: usize, y: usize, pipe: Pipe, direction: Direction) -> (usize, usize, Direction) {
    use Direction::*;
    use Pipe::*;
    let new_dir = match (pipe, direction) {
        (Vertical, North) => direction,
        (Vertical, South) => direction,
        (Horizontal, West) => direction,
        (Horizontal, East) => direction,
        (TopLeft, North) => East,
        (TopLeft, West) => South,
        (TopRight, North) => West,
        (TopRight, East) => South,
        (BottomRight, South) => West,
        (BottomRight, East) => North,
        (BottomLeft, South) => East,
        (BottomLeft, West) => North,
        _ => panic!(),
    };

    let (dx, dy) = match new_dir {
        North => (0, -1),
        East => (1, 0),
        South => (0, 1),
        West => (-1, 0),
    };
    let (x, y) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
    (x, y, new_dir)
}

fn char_to_pipe(cell: char) -> Pipe {
    match cell {
        'S' => Pipe::Start,
        '|' => Pipe::Vertical,
        '-' => Pipe::Horizontal,
        'F' => Pipe::TopLeft,
        '7' => Pipe::TopRight,
        'L' => Pipe::BottomLeft,
        'J' => Pipe::BottomRight,
        '.' => Pipe::Empty,
        _ => panic!(),
    }
}

fn pipe_to_char(pipe: Pipe) -> char {
    match pipe {
        Pipe::Start => 'S',
        Pipe::Vertical => '|',
        Pipe::Horizontal => '-',
        Pipe::TopLeft => 'F',
        Pipe::TopRight => '7',
        Pipe::BottomLeft => 'L',
        Pipe::BottomRight => 'J',
        Pipe::Empty => '.',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
