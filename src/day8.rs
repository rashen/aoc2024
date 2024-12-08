use glam::IVec2;
use itertools::Itertools;

pub fn main() {
    let input = std::fs::read_to_string("input/day8.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn pos_to_idx(pos: IVec2, rows: usize, cols: usize) -> Option<usize> {
    if pos.x >= cols as i32 || pos.y >= rows as i32 || pos.x < 0 || pos.y < 0 {
        return None;
    }
    Some((pos.y * (rows as i32) + pos.x) as usize)
}

fn idx_to_pos(i: usize, rows: usize, cols: usize) -> Option<IVec2> {
    if i >= rows * cols {
        return None;
    }
    let x = i % rows;
    let y = i / rows;
    Some(IVec2 {
        x: x as i32,
        y: y as i32,
    })
}

fn part1(input: &str) -> i32 {
    let cols = input.lines().next().expect("Empty input").len();
    let rows = input.lines().count();

    // Remove new lines
    let input = input.lines().collect::<String>();

    let mut nodes = Vec::new();
    for (i, first) in input.chars().enumerate() {
        if first.is_alphanumeric() {
            let first_pos = idx_to_pos(i, rows, cols).expect("Iterated to out of bounds");
            for (j, second) in input.chars().enumerate() {
                if second == first && i != j {
                    let second_pos = idx_to_pos(j, rows, cols).expect("Iterated to out of bounds");
                    let node_pos = 2 * first_pos - second_pos;
                    if pos_to_idx(node_pos, rows, cols).is_some() {
                        nodes.push(node_pos);
                    }
                }
            }
        }
    }

    nodes.iter().unique().count() as i32
}

fn part2(input: &str) -> i32 {
    let cols = input.lines().next().expect("Empty input").len();
    let rows = input.lines().count();

    // Remove new lines
    let input = input.lines().collect::<String>();

    let mut nodes = Vec::new();
    for (i, first) in input.chars().enumerate() {
        if first.is_alphanumeric() {
            let first_pos = idx_to_pos(i, rows, cols).expect("Iterated to out of bounds");
            for (j, second) in input.chars().enumerate() {
                if second == first && i != j {
                    let second_pos = idx_to_pos(j, rows, cols).expect("Iterated to out of bounds");
                    let offset = first_pos - second_pos;
                    let mut node_pos = first_pos;
                    while pos_to_idx(node_pos, rows, cols).is_some() {
                        nodes.push(node_pos);
                        node_pos += offset;
                    }
                }
            }
        }
    }

    nodes.iter().unique().count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 34);
    }

    #[test]
    fn test_part2_single_channel() {
        let input = "T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........";
        assert_eq!(part2(input), 9);
    }

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
}
