use glam::IVec2;

pub fn main() {
    let input = std::fs::read_to_string("input/day15.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

#[derive(Debug, PartialEq, Eq)]
struct Warehouse {
    pub walls: Vec<IVec2>,
    pub boxes: Vec<IVec2>,
    pub robot: IVec2,
}
impl Warehouse {
    pub fn new() -> Self {
        Self {
            walls: vec![],
            boxes: vec![],
            robot: IVec2::ZERO,
        }
    }
}
impl core::fmt::Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cols = self.walls.iter().map(|v| v.x).max().unwrap() as usize + 1;
        let rows = self.walls.iter().map(|v| v.y).max().unwrap() as usize + 1;

        let mut buffer = vec!['.'; rows * cols];
        for w in self.walls.iter() {
            if let Some(idx) = pos_to_idx(w, rows, cols) {
                buffer[idx] = '#';
            }
        }
        for b in self.boxes.iter() {
            if let Some(idx) = pos_to_idx(b, rows, cols) {
                buffer[idx] = 'O';
            }
        }
        if let Some(idx) = pos_to_idx(&self.robot, rows, cols) {
            buffer[idx] = '@';
        }

        let s = buffer
            .chunks(cols)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", s)
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}
type Directions = Vec<Direction>;

fn pos_to_idx(pos: &IVec2, rows: usize, cols: usize) -> Option<usize> {
    if pos.x >= cols as i32 || pos.y >= rows as i32 || pos.x < 0 || pos.y < 0 {
        return None;
    }
    Some((pos.y * (cols as i32) + pos.x) as usize)
}

fn parse_input(input: &str) -> (Warehouse, Directions) {
    let mut warehouse = Warehouse::new();
    let mut directions = Vec::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => {
                    warehouse.walls.push(IVec2::new(x as i32, y as i32));
                }
                'O' => {
                    warehouse.boxes.push(IVec2::new(x as i32, y as i32));
                }
                '@' => {
                    warehouse.robot = IVec2::new(x as i32, y as i32);
                }
                '<' => {
                    directions.push(Direction::Left);
                }
                'v' => {
                    directions.push(Direction::Down);
                }
                '^' => {
                    directions.push(Direction::Up);
                }
                '>' => {
                    directions.push(Direction::Right);
                }
                _ => {}
            }
        }
    }
    (warehouse, directions)
}

fn move_box(warehouse: &mut Warehouse, box_pos: IVec2, dir: IVec2) -> bool {
    if let Some(i) = warehouse.boxes.iter().position(|&b| b == box_pos) {
        let new_box_pos = box_pos + dir;
        if warehouse.walls.contains(&new_box_pos) {
            return false;
        }
        if move_box(warehouse, new_box_pos, dir) {
            warehouse.boxes[i] = new_box_pos;
        } else {
            return false;
        }
    }
    true
}

fn step(warehouse: &mut Warehouse, d: Direction) {
    let dir = match d {
        Direction::Up => IVec2::NEG_Y,
        Direction::Left => IVec2::NEG_X,
        Direction::Down => IVec2::Y,
        Direction::Right => IVec2::X,
    };

    let new_pos = warehouse.robot + dir;
    if warehouse.walls.contains(&new_pos) {
        return;
    }
    if move_box(warehouse, new_pos, dir) {
        warehouse.robot = new_pos;
    }
}

fn part1(input: &str) -> i32 {
    let (mut warehouse, directions) = parse_input(input);
    for d in directions.into_iter() {
        step(&mut warehouse, d);
    }

    let mut acc = 0;
    for b in warehouse.boxes.iter() {
        acc += b.x + b.y * 100;
    }
    acc
}

#[allow(unused)]
fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn test_step_once() {
        let (mut warehouse, directions) = parse_input(INPUT);
        let (step1, _) = parse_input(
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );
        step(&mut warehouse, directions[0]);
        assert_eq!(warehouse, step1);
    }

    #[test]
    fn test_step() {
        let (mut warehouse, directions) = parse_input(INPUT);
        let (last, _) = parse_input(
            "########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########",
        );
        for d in directions.into_iter() {
            step(&mut warehouse, d);
        }
        assert_eq!(
            warehouse, last,
            "Expected:\n{}\nfound:\n{}",
            warehouse, last
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2028);
    }
}
