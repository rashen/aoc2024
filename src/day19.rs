use std::collections::HashSet;

pub fn main() {
    let input = std::fs::read_to_string("input/day19.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}
impl Color {
    fn from_char(c: char) -> Option<Self> {
        let color = match c {
            'w' => Color::White,
            'u' => Color::Blue,
            'b' => Color::Black,
            'r' => Color::Red,
            'g' => Color::Green,
            _ => return None,
        };
        Some(color)
    }
}

struct Towels {
    patterns: HashSet<Vec<Color>>,
    designs: Vec<Vec<Color>>,
}

fn parse_input(input: &str) -> Option<Towels> {
    let mut lines = input.lines();
    let patterns = if let Some(l) = lines.next() {
        l.split(", ")
            .map(|s| s.chars().filter_map(Color::from_char).collect::<Vec<_>>())
            .collect::<HashSet<_>>()
    } else {
        return None;
    };

    let mut designs = Vec::new();
    while let Some(l) = lines.next() {
        let p = l.chars().filter_map(Color::from_char).collect::<Vec<_>>();
        if !p.is_empty() {
            designs.push(p);
        }
    }

    Some(Towels { patterns, designs })
}

fn part1(input: &str) -> i32 {
    todo!()
}

fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_parse() {
        let towels = parse_input(INPUT).unwrap();
        assert_eq!(
            towels.patterns,
            vec![
                vec![Color::Red],
                vec![Color::White, Color::Red],
                vec![Color::Black],
                vec![Color::Green],
                vec![Color::Black, Color::White, Color::Blue],
                vec![Color::Red, Color::Black],
                vec![Color::Green, Color::Black],
                vec![Color::Black, Color::Red]
            ]
        );
        assert_eq!(towels.designs.len(), 8);
        assert_eq!(
            towels.designs[0],
            vec![
                Color::Black,
                Color::Red,
                Color::White,
                Color::Red,
                Color::Red
            ]
        );
    }
}
