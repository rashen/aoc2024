use core::fmt;
use std::collections::HashMap;

use glam::IVec2;
use itertools::Itertools;

pub fn main() {
    let input = std::fs::read_to_string("input/day16.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let maze = parse_input(input);
    let reached = walk(&maze);
    get_cost(&reached, &maze)
}

fn get_cost(reached: &HashMap<Tile, i32>, maze: &Maze) -> i32 {
    let mut min = i32::MAX;
    for ends in reached.iter().filter(|(tile, _)| tile.pos == maze.end) {
        min = *ends.1.min(&min)
    }
    min
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
impl fmt::Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::with_capacity(self.tiles.len() + self.rows);
        let mut idx = 0;
        for _ in 0..self.rows {
            for _ in 0..self.cols {
                out.push(self.tiles[idx]);
                idx += 1;
            }
            out.push('\n');
        }
        write!(f, "{}", out)
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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
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

#[derive(Copy, Clone, PartialEq, Debug)]
struct TileToVisit {
    pos: IVec2,
    facing: Direction,
    cost: i32,
}
impl TileToVisit {
    fn new(pos: IVec2, facing: Direction, cost: i32) -> Self {
        Self { pos, facing, cost }
    }
    fn to_tile(&self) -> Tile {
        Tile {
            pos: self.pos,
            facing: self.facing,
        }
    }
}
#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq)]
struct Tile {
    pos: IVec2,
    facing: Direction,
}
impl Tile {
    fn new(pos: IVec2, facing: Direction) -> Self {
        Self { pos, facing }
    }
}

fn walk(maze: &Maze) -> HashMap<Tile, i32> {
    let mut frontier = Vec::new();
    frontier.push(TileToVisit::new(maze.start, Direction::East, 0));
    let mut reached = HashMap::new();

    while let Some(current) = frontier.pop() {
        if let Some(&c) = reached.get(&current.to_tile()) {
            if current.cost > c {
                continue;
            }
        }

        reached.insert(Tile::new(current.pos, current.facing), current.cost);
        if let Some('E') = maze.tile(current.pos) {
            break;
        }

        let fw = current.pos + current.facing.to_vec();
        if let Some(c) = maze.tile(fw) {
            if c != '#' {
                frontier.push(TileToVisit::new(fw, current.facing, current.cost + 1));
            }
        }
        let left_facing = current.facing.ccw();
        frontier.push(TileToVisit::new(
            current.pos,
            left_facing,
            current.cost + 1000,
        ));
        let right_facing = current.facing.cw();
        frontier.push(TileToVisit::new(
            current.pos,
            right_facing,
            current.cost + 1000,
        ));

        frontier.sort_by(|x, y| y.cost.cmp(&x.cost));
    }
    reached
}

fn backtrack(reached: &HashMap<Tile, i32>, maze: &Maze, lowest_cost: i32) -> Vec<Tile> {
    let mut frontier = vec![
        TileToVisit::new(maze.end, Direction::North, lowest_cost),
        TileToVisit::new(maze.end, Direction::East, lowest_cost),
        TileToVisit::new(maze.end, Direction::South, lowest_cost),
        TileToVisit::new(maze.end, Direction::West, lowest_cost),
    ];

    let mut trail = Vec::new();

    while let Some(current) = frontier.pop() {
        if trail.contains(&current.to_tile()) || maze.tile(current.pos) == Some('#') {
            continue;
        }

        if current == TileToVisit::new(maze.start, Direction::East, 0) {
            continue;
        }

        if let Some(cost) = reached.get(&current.to_tile()) {
            if *cost <= current.cost {
                trail.push(current.to_tile());

                let dir = current.facing.to_vec() * -1;
                frontier.push(TileToVisit::new(
                    current.pos + dir,
                    current.facing,
                    current.cost - 1,
                ));
                frontier.push(TileToVisit::new(
                    current.pos,
                    current.facing.cw(),
                    current.cost - 1000,
                ));
                frontier.push(TileToVisit::new(
                    current.pos,
                    current.facing.ccw(),
                    current.cost - 1000,
                ));
            }
        }
    }

    trail
}

fn part2(input: &str) -> i32 {
    let maze = parse_input(input);
    let reached = walk(&maze);

    let cost = get_cost(&reached, &maze);

    let trail = backtrack(&reached, &maze, cost);
    let trail = trail.iter().map(|t| t.pos).unique().collect::<Vec<_>>();

    // debug_print_visited(maze, &trail);

    trail.len() as i32
}

#[allow(dead_code)]
fn debug_print_visited(maze: Maze, trail: &[IVec2]) {
    for y in 0..maze.rows {
        for x in 0..maze.cols {
            let pos = IVec2::new(x as i32, y as i32);
            let c = if trail.contains(&pos) {
                'O'
            } else {
                maze.tiles[pos_to_idx(pos, maze.rows, maze.cols).unwrap()]
            };
            print!("{}", c);
        }
        println!("");
    }
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

    const INPUT2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 7036);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 45);
        assert_eq!(part2(INPUT2), 64);
    }
}
