use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;
use itertools::Itertools;

pub fn main() {
    let input = std::fs::read_to_string("input/day16.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let maze = parse_input(input);
    walk(&maze)
}

#[derive(Debug)]
struct Maze {
    cols: usize,
    rows: usize,
    tiles: Vec<char>,
    start: IVec2,
    end: IVec2,
}
impl Maze {
    fn tile(&self, pos: IVec2) -> Option<char> {
        let idx = pos_to_idx(pos, self.rows, self.cols)?;
        Some(self.tiles[idx])
    }
}

fn parse_input(input: &str) -> Maze {
    let cols = input.lines().next().unwrap().chars().count();
    let rows = input.lines().count();

    let mut tiles = Vec::new();
    let mut start = None;
    let mut end = None;
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                'S' => start = Some(IVec2::new(x as i32, y as i32)),
                'E' => end = Some(IVec2::new(x as i32, y as i32)),
                _ => {}
            }
            tiles.push(c);
        }
    }
    Maze {
        cols,
        rows,
        tiles,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn pos_to_idx(pos: IVec2, rows: usize, cols: usize) -> Option<usize> {
    if pos.x >= cols as i32 || pos.y >= rows as i32 || pos.x < 0 || pos.y < 0 {
        return None;
    }
    Some((pos.y * (cols as i32) + pos.x) as usize)
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    East,
    South,
    West,
    North,
}
impl Direction {
    fn to_vec(&self) -> IVec2 {
        match self {
            Direction::East => IVec2::NEG_X,
            Direction::South => IVec2::Y,
            Direction::West => IVec2::X,
            Direction::North => IVec2::NEG_Y,
        }
    }
    fn cw(&self) -> Self {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }
    fn ccw(&self) -> Self {
        match self {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
        }
    }
}

struct TileToVisit {
    pos: IVec2,
    facing: Direction,
    cost: i32,
}
impl TileToVisit {
    fn new(pos: IVec2, facing: Direction, cost: i32) -> Self {
        Self { pos, facing, cost }
    }
}

fn walk(maze: &Maze) -> i32 {
    let mut frontier = Vec::new();
    frontier.push(TileToVisit::new(maze.start, Direction::East, 0));
    let mut reached = HashMap::new();

    while let Some(current) = frontier.pop() {
        if reached.contains_key(&current.pos) {
            continue;
        }
        if let Some('E') = maze.tile(current.pos) {
            return current.cost;
        }

        reached.insert(current.pos, current.cost);

        let fw = current.pos + current.facing.to_vec();
        if let Some(c) = maze.tile(fw) {
            if c != '#' {
                frontier.push(TileToVisit::new(fw, current.facing, current.cost + 1));
            }
        }
        let left_facing = current.facing.ccw();
        let left = current.pos + left_facing.to_vec();
        if let Some(c) = maze.tile(left) {
            if c != '#' {
                frontier.push(TileToVisit::new(left, left_facing, current.cost + 1001));
            }
        }
        let right_facing = current.facing.cw();
        let right = current.pos + right_facing.to_vec();
        if let Some(c) = maze.tile(right) {
            if c != '#' {
                frontier.push(TileToVisit::new(right, right_facing, current.cost + 1001));
            }
        }

        frontier.sort_by(|x, y| y.cost.cmp(&x.cost));
    }
    0
}

fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn test_walk() {
        let maze = parse_input(INPUT);
        assert_eq!(walk(&maze), 7036);
    }
}
