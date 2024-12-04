use glam::IVec2;

pub fn main() {
    let input = std::fs::read_to_string("input/day4.txt").expect("No input");
    // Get rid of all linebreaks
    let (rows, cols) = get_rows_cols(&input);
    let input = input.lines().collect::<Vec<_>>().join("");

    println!("Part 1: {}", part1(&input, rows, cols));
    // println!("Part 2: {}", part2(&input));
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

fn get_word(input: &str, positions: &[IVec2], rows: usize, cols: usize) -> String {
    let word = positions
        .iter()
        .filter_map(|p: &IVec2| {
            if let Some(idx) = pos_to_idx(*p, rows, cols) {
                return input.chars().nth(idx);
            }
            None
        })
        .collect::<String>();
    return word;
}

fn create_neighbour_matrix(count: usize) -> Vec<Vec<IVec2>> {
    let num_directions = 8;
    let mut neighbours = Vec::with_capacity(num_directions);
    {
        // Horizontal
        let mut pos = Vec::with_capacity(count);
        let mut neg = Vec::with_capacity(count);
        for i in 1..=count {
            pos.push(IVec2::X * i as i32);
            neg.push(IVec2::NEG_X * i as i32);
        }
        neighbours.push(pos);
        neighbours.push(neg);
    }
    {
        // Vertical
        let mut pos = Vec::with_capacity(count);
        let mut neg = Vec::with_capacity(count);
        for i in 1..=count {
            pos.push(IVec2::Y * i as i32);
            neg.push(IVec2::NEG_Y * i as i32);
        }
        neighbours.push(pos);
        neighbours.push(neg);
    }
    {
        // Diagonal
        let mut first = Vec::with_capacity(count);
        let mut second = Vec::with_capacity(count);
        let mut third = Vec::with_capacity(count);
        let mut fourth = Vec::with_capacity(count);
        for i in 1..=count {
            first.push(IVec2::ONE * i as i32);
            second.push(IVec2::NEG_ONE * i as i32);
            third.push(IVec2::new(1, -1) * i as i32);
            fourth.push(IVec2::new(-1, 1) * i as i32);
        }
        neighbours.push(first);
        neighbours.push(second);
        neighbours.push(third);
        neighbours.push(fourth);
    }
    neighbours
}

fn center_on_index(pos: IVec2, neighbours: &Vec<Vec<IVec2>>) -> Vec<Vec<IVec2>> {
    let mut neighbours = neighbours.clone();
    for n in neighbours.iter_mut().flatten() {
        *n += pos;
    }
    neighbours
}

fn find_words_starting_from(
    input: &str,
    neighbours: &Vec<Vec<IVec2>>,
    idx: usize,
    word: &str,
    rows: usize,
    cols: usize,
) -> i32 {
    let Some(pos) = idx_to_pos(idx, rows, cols) else {
        return 0;
    };
    let neighbours = center_on_index(pos, neighbours);
    neighbours.iter().fold(0, |acc, positions| {
        if &get_word(input, positions, rows, cols) == word {
            return acc + 1;
        }
        acc
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

fn part1(input: &str, rows: usize, cols: usize) -> i32 {
    let neighbours = create_neighbour_matrix(3);
    let mut acc = 0;
    for (idx, c) in input.chars().enumerate() {
        if c == 'X' {
            acc += find_words_starting_from(&input, &neighbours, idx, "MAS", rows, cols);
        }
    }
    acc
}

fn part2(input: &str) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASMMSAMXMSMSAAMXSXMAAMMMSAMASMSMXXMASAMXAMMXXAMMXXAMASMSMSASXSSSAXAMASAAAMAMMMXMMMMMXMXAXMASX";

    #[test]
    fn test_idx_to_pos() {
        let width = 8;
        let cols = 8;
        assert_eq!(pos_to_idx(IVec2::ONE, width, cols), Some(9));
        assert_eq!(idx_to_pos(9, width, cols), Some(IVec2::ONE));
        assert_eq!(pos_to_idx(IVec2::new(-1, 0), width, cols), None);
        assert_eq!(pos_to_idx(IVec2::new(0, -1), width, cols), None);
        assert_eq!(pos_to_idx(IVec2::new(8, 0), width, cols), None);
        assert_eq!(pos_to_idx(IVec2::new(0, 8), width, cols), None);
        assert_eq!(idx_to_pos(64, width, cols), None);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT, 10, 10), 18);
    }
}
