use glam::IVec2;
use itertools::Itertools;
use rayon::prelude::*;

pub fn main() {
    let input = std::fs::read_to_string("input/day6.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Visited {
    pub tile: IVec2,
    pub dir: Direction,
}

#[derive(Debug, Clone)]
struct World {
    pub obstacles: Vec<IVec2>,
    pub guard_pos: IVec2,
    pub guard_facing: Direction,
    pub visited: Vec<Visited>,
    pub dimension: IVec2,
}
impl World {
    pub fn new(obstacles: Vec<IVec2>, guard_pos: IVec2, dimension: IVec2) -> Self {
        World {
            obstacles,
            guard_pos,
            guard_facing: Direction::North,
            visited: vec![Visited {
                tile: guard_pos,
                dir: Direction::North,
            }],
            dimension,
        }
    }
}

fn parse_world(input: &str) -> World {
    let mut guard_pos = IVec2::ZERO;
    let mut obstacles = Vec::new();
    let mut dimension = IVec2::ZERO;
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                obstacles.push(IVec2::new(x as i32, y as i32));
            } else if c == '^' {
                guard_pos = IVec2::new(x as i32, y as i32);
            }
            dimension.x = dimension.x.max(x as i32);
        }
        dimension.y = dimension.y.max(y as i32);
    }
    World::new(obstacles, guard_pos, dimension)
}

fn dir_to_coord(dir: Direction) -> IVec2 {
    match dir {
        Direction::North => IVec2::NEG_Y,
        Direction::East => IVec2::X,
        Direction::West => IVec2::NEG_X,
        Direction::South => IVec2::Y,
    }
}

fn turn(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    Step,
    Turn,
    Done,
}

fn step(world: &mut World) -> Action {
    let next_step = world.guard_pos + dir_to_coord(world.guard_facing);
    if world.obstacles.contains(&next_step) {
        world.guard_facing = turn(world.guard_facing);
        return Action::Turn;
    }
    if next_step.x > world.dimension.x
        || next_step.y > world.dimension.y
        || next_step.x < 0
        || next_step.y < 0
    {
        return Action::Done;
    }
    world.guard_pos = next_step;
    world.visited.push(Visited {
        tile: world.guard_pos,
        dir: world.guard_facing,
    });
    Action::Step
}

fn part1(input: &str) -> i32 {
    let mut world = parse_world(input);
    while step(&mut world) != Action::Done {}
    world.visited.into_iter().map(|v| v.tile).unique().count() as i32
}

fn part2(input: &str) -> i32 {
    let mut used_world = parse_world(input);
    let world = used_world.clone();
    while step(&mut used_world) != Action::Done {}

    let acc: i32 = used_world.visited[1..]
        .par_iter()
        .fold(
            || 0_i32,
            |acc, v| {
                let mut world = world.clone();

                let obstacle = v.tile;
                world.obstacles.push(obstacle);

                loop {
                    match step(&mut world) {
                        Action::Done => return acc,
                        Action::Step => {
                            let len = world.visited.len();
                            if world.visited[0..len - 1].contains(&Visited {
                                tile: world.guard_pos,
                                dir: world.guard_facing,
                            }) {
                                return acc + 1;
                            }
                        }
                        Action::Turn => {}
                    }
                }
            },
        )
        .sum();

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_world() {
        let world = parse_world(INPUT);
        assert_eq!(world.obstacles.len(), 8);
        assert_eq!(world.dimension, IVec2::new(9, 9));
        assert_eq!(world.guard_pos, IVec2::new(4, 6));
    }

    #[test]
    fn test_step() {
        let mut world = parse_world(INPUT);
        step(&mut world);
        assert_eq!(world.guard_pos, IVec2::new(4, 5));
        step(&mut world);
        assert_eq!(world.guard_pos, IVec2::new(4, 4));
        step(&mut world);
        assert_eq!(world.guard_pos, IVec2::new(4, 3));
        step(&mut world);
        assert_eq!(world.guard_pos, IVec2::new(4, 2));
        step(&mut world);
        assert_eq!(world.guard_pos, IVec2::new(4, 1));
        step(&mut world);
        assert_eq!(world.guard_pos, IVec2::new(4, 1));
        assert_eq!(world.guard_facing, Direction::East);
        step(&mut world);
        assert_eq!(world.guard_pos, IVec2::new(5, 1));
        step(&mut world);
        assert_eq!(world.guard_pos, IVec2::new(6, 1));
        step(&mut world);
        assert_eq!(world.guard_pos, IVec2::new(7, 1));
        step(&mut world);
        assert_eq!(world.guard_pos, IVec2::new(8, 1));
        step(&mut world);
        assert_eq!(world.guard_facing, Direction::South);
        assert_eq!(world.guard_pos, IVec2::new(8, 1));
        step(&mut world);
        assert_eq!(world.guard_pos, IVec2::new(8, 2));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 41);
    }

    #[test]
    fn test_part1_multiple_obstacles() {
        let input = "..................
.....#............
.......#.#........
....#....^#.......
.........#........";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 6);
    }

    #[test]
    fn test_part2_cc() {
        let input = ".##.
#..#
....
..^.";
        assert_eq!(part2(input), 0);
    }

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
}
