use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

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
pub fn part_two(input: &str) -> Option<usize> {
    use Direction::*;
    use Pipe::*;
    // We are going clockwise
    let map = parse(input);
    let pipe_loop = get_loop(&map);
    let mut inside_tiles = HashSet::new();
    let (mut x, mut y, mut direction) = find_start(&map);
    while map[y][x] != Pipe::Start {
        let pipe = map[y][x];

        // Get right side coords
        if let Some((rs_dx, rs_dy)) = match (direction, pipe) {
            // Straight
            (East, Horizontal) => Some((0, 1)),
            (West, Horizontal) => Some((0, -1)),
            (South, Vertical) => Some((-1, 0)),
            (North, Vertical) => Some((1, 0)),
            // Turns, first right side
            (East, BottomRight) => Some((0, 1)),
            (West, TopLeft) => Some((0, -1)),
            (South, BottomLeft) => Some((-1, 0)),
            (North, TopRight) => Some((1, 0)),
            // Turns, second right side
            (East, TopRight) => Some((0, 1)),
            (West, BottomLeft) => Some((0, -1)),
            (South, BottomRight) => Some((-1, 0)),
            (North, TopLeft) => Some((1, 0)),
            // Turns, outside
            (West, TopLeft) => Some((-1, -1)),
            (North, TopRight) => Some((1, -1)),
            (East, BottomRight) => Some((1, 1)),
            (South, BottomLeft) => Some((-1, 1)),
            // Turns, inside
            (North, TopLeft) => Some((1, 1)),
            (East, TopRight) => Some((-1, 1)),
            (South, BottomRight) => Some((-1, -1)),
            (West, BottomLeft) => Some((1, -1)),
            _ => None,
        } {
            let right_pos = ((x as i32 + rs_dx) as usize, (y as i32 + rs_dy) as usize);

            // Check that tile is not part of loop
            if !pipe_loop.contains(&right_pos) {
                let connected_tiles = get_connected_tiles(&pipe_loop, right_pos);
                for tile in connected_tiles {
                    inside_tiles.insert(tile);
                }
            }
        }

        (x, y, direction) = advance(x, y, map[y][x], direction);
    }
    for y in 0..142 {
        for x in 0..142 {
            if pipe_loop.contains(&(x, y)) {
                print!("X");
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
    right_pos: (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut queue = VecDeque::new();
    queue.push_back(right_pos);
    let mut connected_tiles = HashSet::new();
    connected_tiles.insert(right_pos);

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
    pipe_loop.insert((x, y));
    while map[y][x] != Pipe::Start {
        (x, y, direction) = advance(x, y, map[y][x], direction);
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
