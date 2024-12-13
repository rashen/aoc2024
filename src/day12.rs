use std::collections::VecDeque;

use glam::IVec2;

pub fn main() {
    let input = std::fs::read_to_string("input/day12.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    crop: char,
    perimeter: i32,
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

fn part1(input: &str) -> i32 {
    let (rows, cols) = get_rows_cols(input);
    let tiles = parse_tiles(rows, cols, input);

    let neighbors = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];
    let mut visited = vec![false; tiles.len()];
    let mut acc = 0;

    for i in 0..tiles.len() {
        if visited[i] {
            continue;
        }

        let mut area = 0;
        let mut perimeter = 0;
        let crop = tiles[i].crop;
        let mut to_visit = VecDeque::<usize>::new();
        to_visit.push_back(i);

        while let Some(j) = to_visit.pop_front() {
            if visited[j] {
                continue;
            }
            let this_pos = idx_to_pos(j, rows, cols).unwrap();
            visited[j] = true;
            area += 1;
            perimeter += tiles[j].perimeter;
            for n in neighbors
                .iter()
                .filter_map(|n| pos_to_idx(n + this_pos, rows, cols))
                .filter(|k| tiles[*k].crop == crop)
            {
                to_visit.push_back(n);
            }
        }
        acc += area * perimeter;
    }

    acc
}

fn parse_tiles(rows: usize, cols: usize, input: &str) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = Vec::with_capacity(rows * cols);

    for c in input.lines().flat_map(|l| l.chars()) {
        tiles.push(Tile {
            crop: c,
            perimeter: 0,
        });
    }

    let neighbors = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];
    for i in 0..tiles.len() {
        let this_pos = idx_to_pos(i, rows, cols).unwrap();
        for n in neighbors.iter() {
            if let Some(j) = pos_to_idx(this_pos + n, rows, cols) {
                if tiles[j].crop != tiles[i].crop {
                    tiles[i].perimeter += 1;
                }
            } else {
                tiles[i].perimeter += 1;
            }
        }
    }
    tiles
}

#[derive(Debug, PartialEq)]
struct BorderedTile {
    pub crop: char,
    pub border: [bool; 4], // Right, Down, Left, Up
}

fn parse_bordered_tiles(rows: usize, cols: usize, input: &str) -> Vec<BorderedTile> {
    let mut tiles: Vec<BorderedTile> = Vec::with_capacity(rows * cols);

    for c in input.lines().flat_map(|l| l.chars()) {
        tiles.push(BorderedTile {
            crop: c,
            border: [false; 4],
        });
    }

    let neighbors = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];
    for i in 0..tiles.len() {
        let this_pos = idx_to_pos(i, rows, cols).unwrap();
        for (k, n) in neighbors.iter().enumerate() {
            if let Some(j) = pos_to_idx(this_pos + n, rows, cols) {
                if tiles[j].crop != tiles[i].crop {
                    tiles[i].border[k] = true;
                }
            } else {
                tiles[i].border[k] = true;
            }
        }
    }
    tiles
}

fn part2(input: &str) -> i32 {
    /* This is not a pretty solution but it works */
    let (rows, cols) = get_rows_cols(input);
    let tiles = parse_bordered_tiles(rows, cols, input);
    let neighbors = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

    let mut visited = vec![false; tiles.len()];
    let mut regions: Vec<Vec<IVec2>> = Vec::new();

    for i in 0..tiles.len() {
        if visited[i] {
            continue;
        }

        let mut region = Vec::new();
        let crop = tiles[i].crop;
        let mut to_visit = VecDeque::<usize>::new();
        to_visit.push_back(i);

        while let Some(j) = to_visit.pop_front() {
            if visited[j] {
                continue;
            }
            let this_pos = idx_to_pos(j, rows, cols).unwrap();
            visited[j] = true;
            region.push(this_pos);
            for n in neighbors
                .iter()
                .filter_map(|n| pos_to_idx(n + this_pos, rows, cols))
                .filter(|k| tiles[*k].crop == crop)
            {
                to_visit.push_back(n);
            }
        }
        regions.push(region);
    }

    let mut acc = 0;
    for region in regions.iter() {
        /* Find the range of rows and cols involved for each region. Because we check each
         * coordinate individually we create a square over the region, but not all positions are
         * within the actual region. */
        let max_col = region.iter().map(|v| v.x).max().unwrap();
        let min_col = region.iter().map(|v| v.x).min().unwrap();
        let max_row = region.iter().map(|v| v.y).max().unwrap();
        let min_row = region.iter().map(|v| v.y).min().unwrap();

        let mut sides = 0;
        /* Up and left borders */
        for k in [0, 2] {
            /* Column major search */
            for x in min_col..=max_col {
                let mut inside = false;
                for y in min_row..=max_row {
                    let pos = IVec2::new(x, y);
                    if !region.contains(&pos) {
                        inside = false;
                        continue;
                    }
                    let idx = pos_to_idx(pos, rows, cols).unwrap();
                    let has_border = tiles[idx].border[k];
                    if has_border && !inside {
                        sides += 1;
                    }
                    inside = has_border;
                }
            }
        }

        /* Down and up borders */
        for k in [1, 3] {
            /* Row major search */
            for y in min_row..=max_row {
                let mut inside = false;
                for x in min_col..=max_col {
                    let pos = IVec2::new(x, y);
                    if !region.contains(&pos) {
                        inside = false;
                        continue;
                    }
                    let idx = pos_to_idx(pos, rows, cols).unwrap();
                    let has_border = tiles[idx].border[k];
                    if has_border && !inside {
                        sides += 1;
                    }
                    inside = has_border;
                }
            }
        }

        acc += sides * region.len();
    }

    acc as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "AAAA
BBCD
BBCC
EEEC";
    const EX2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    const EX3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const EX4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const EX5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX1), 140);
        assert_eq!(part1(EX2), 772);
        assert_eq!(part1(EX3), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX1), 80);
        assert_eq!(part2(EX2), 436);
        assert_eq!(part2(EX3), 1206);
        assert_eq!(part2(EX4), 236);
        assert_eq!(part2(EX5), 368);
    }

    #[test]
    fn test_parse_bordered_tiles() {
        let tiles = parse_bordered_tiles(4, 4, EX1);
        assert_eq!(tiles.len(), 16);
        // Right, Down, Left, Up
        assert_eq!(
            tiles[4],
            BorderedTile {
                crop: 'B',
                border: [false, false, true, true]
            }
        );
        assert_eq!(
            tiles[5],
            BorderedTile {
                crop: 'B',
                border: [true, false, false, true]
            }
        );
        assert_eq!(
            tiles[0],
            BorderedTile {
                crop: 'A',
                border: [false, true, true, true]
            }
        );
        assert_eq!(
            tiles[7],
            BorderedTile {
                crop: 'D',
                border: [true, true, true, true]
            }
        );
        assert_eq!(
            tiles[11],
            BorderedTile {
                crop: 'C',
                border: [true, false, false, true]
            }
        );
        assert_eq!(
            tiles[15],
            BorderedTile {
                crop: 'C',
                border: [true, true, true, false]
            }
        );
    }
}
