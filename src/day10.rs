use glam::IVec2;
use itertools::Itertools;

pub fn main() {
    let input = std::fs::read_to_string("input/day10.txt").expect("No input");
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

fn get_rows_cols(input: &str) -> (usize, usize) {
    let rows = input
        .lines()
        .nth(0)
        .expect("Received empty input")
        .chars()
        .count();
    let cols = input.lines().count();
    (rows, cols)
}

fn find_path_endpoints(
    height: i32,
    idx: usize,
    rows: usize,
    cols: usize,
    trail_map: &str,
) -> Vec<IVec2> {
    let next_height = height + 1;
    let Some(pos) = idx_to_pos(idx, rows, cols) else {
        return vec![];
    };
    if trail_map.chars().nth(idx) == Some('9') {
        return vec![pos];
    }
    let neighbors = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

    let mut output = Vec::new();
    for n in neighbors.iter().map(|p| p + pos) {
        if let Some(i) = pos_to_idx(n, rows, cols) {
            if trail_map.chars().nth(i).and_then(|c| c.to_digit(10)) == Some(next_height as u32) {
                output.append(&mut find_path_endpoints(
                    next_height,
                    i,
                    rows,
                    cols,
                    trail_map,
                ));
            }
        }
    }
    output
}

fn part1(input: &str) -> i32 {
    let (rows, cols) = get_rows_cols(input);
    let input = input.lines().collect::<String>();

    let mut acc = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '0' {
            let path = find_path_endpoints(0, i, rows, cols, &input);
            acc += path.iter().unique().count() as i32;
        }
    }

    acc
}

fn find_paths(height: i32, idx: usize, rows: usize, cols: usize, trail_map: &str) -> i32 {
    let next_height = height + 1;
    let Some(pos) = idx_to_pos(idx, rows, cols) else {
        return 0;
    };
    if trail_map.chars().nth(idx) == Some('9') {
        return 1;
    }
    let neighbors = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

    let mut acc = 0;
    for n in neighbors.iter().map(|p| p + pos) {
        if let Some(i) = pos_to_idx(n, rows, cols) {
            if trail_map.chars().nth(i).and_then(|c| c.to_digit(10)) == Some(next_height as u32) {
                acc += find_paths(next_height, i, rows, cols, trail_map);
            }
        }
    }
    acc
}

fn part2(input: &str) -> i32 {
    let (rows, cols) = get_rows_cols(input);
    let input = input.lines().collect::<String>();

    let mut acc = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '0' {
            acc += find_paths(0, i, rows, cols, &input);
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            2,
            part1(
                "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"
            )
        );
        assert_eq!(
            4,
            part1(
                "..90..9
...1.98
...2..7
6543456
765.987
876....
987...."
            )
        );
        assert_eq!(
            36,
            part1(
                "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            3,
            part2(
                ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9...."
            )
        );
    }
}
