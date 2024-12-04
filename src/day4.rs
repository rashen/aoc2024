pub fn main() {
    let input = std::fs::read_to_string("input/day4.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

fn pos_to_idx(x: i32, y: i32, rows: usize, cols: usize) -> Option<usize> {
    if x >= cols as i32 || y >= rows as i32 || x < 0 || y < 0 {
        return None;
    }
    Some((y * (rows as i32) + x) as usize)
}

fn idx_to_pos(i: usize, rows: usize, cols: usize) -> Option<(i32, i32)> {
    if i >= rows * cols {
        return None;
    }
    let x = i % rows;
    let y = i / rows;
    Some((x as i32, y as i32))
}

fn xmas(input: &str, positions: &[(i32, i32)], rows: usize, cols: usize) -> bool {
    let s = positions
        .iter()
        .filter_map(|p: &(i32, i32)| {
            if let Some(idx) = pos_to_idx(p.0, p.1, rows, cols) {
                return input.chars().nth(idx);
            }
            None
        })
        .collect::<String>();
    return &s == "MAS"; // X has already been found
}

fn find_xmas(input: &str, idx: usize, rows: usize, cols: usize) -> i32 {
    let Some(pos) = idx_to_pos(idx, rows, cols) else {
        return 0;
    };
    let cmps = [
        [(pos.0 + 1, pos.1), (pos.0 + 2, pos.1), (pos.0 + 3, pos.1)],
        [(pos.0, pos.1 + 1), (pos.0, pos.1 + 2), (pos.0, pos.1 + 3)],
        [
            (pos.0 + 1, pos.1 + 1),
            (pos.0 + 2, pos.1 + 2),
            (pos.0 + 3, pos.1 + 3),
        ],
        [(pos.0 - 1, pos.1), (pos.0 - 2, pos.1), (pos.0 - 3, pos.1)],
        [(pos.0, pos.1 - 1), (pos.0, pos.1 - 2), (pos.0, pos.1 - 3)],
        [
            (pos.0 - 1, pos.1 - 1),
            (pos.0 - 2, pos.1 - 2),
            (pos.0 - 3, pos.1 - 3),
        ],
        [
            (pos.0 + 1, pos.1 - 1),
            (pos.0 + 2, pos.1 - 2),
            (pos.0 + 3, pos.1 - 3),
        ],
        [
            (pos.0 - 1, pos.1 + 1),
            (pos.0 - 2, pos.1 + 2),
            (pos.0 - 3, pos.1 + 3),
        ],
    ];

    cmps.iter().fold(0, |acc, positions| {
        if xmas(input, positions, rows, cols) {
            return acc + 1;
        }
        acc
    })
}

fn part1(input: &str) -> i32 {
    let rows = input
        .lines()
        .nth(0)
        .expect("Received empty input")
        .chars()
        .count();
    let cols = input.lines().count();
    let input = input.lines().map(|s| s.to_string()).collect::<String>();

    let mut acc = 0;
    for (idx, c) in input.chars().enumerate() {
        if c == 'X' {
            acc += find_xmas(&input, idx, rows, cols);
        }
    }
    acc
}

fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_idx_to_pos() {
        let width = 8;
        let cols = 8;
        let x = 1;
        let y = 1;
        let idx = 9;
        assert_eq!(pos_to_idx(x, y, width, cols), Some(idx));
        assert_eq!(idx_to_pos(idx, width, cols), Some((x, y)));
        assert_eq!(pos_to_idx(-1, 0, width, cols), None);
        assert_eq!(pos_to_idx(0, -1, width, cols), None);
        assert_eq!(pos_to_idx(8, 0, width, cols), None);
        assert_eq!(pos_to_idx(0, 8, width, cols), None);
        assert_eq!(idx_to_pos(64, width, cols), None);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 18);
    }
}
