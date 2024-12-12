use std::collections::VecDeque;

use glam::IVec2;

pub fn main() {
    let input = std::fs::read_to_string("input/day12.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
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

fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "AAAA
BBCD
BBCC
EEEC"
            ),
            140
        );
        assert_eq!(
            part1(
                "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
            ),
            772
        );
        assert_eq!(
            part1(
                "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            ),
            1930
        );
    }
}
