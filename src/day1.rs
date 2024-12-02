fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let size = input.lines().count();
    let mut lhs = Vec::with_capacity(size);
    let mut rhs = Vec::with_capacity(size);
    for l in input.lines() {
        if let Some((left, right)) = l.split_once(|c: char| c.is_whitespace()) {
            lhs.push(left.trim().parse::<i32>().expect("Not an i32"));
            rhs.push(right.trim().parse::<i32>().expect("Not an i32"));
        }
    }
    (lhs, rhs)
}

fn part1(lhs: &mut [i32], rhs: &mut [i32]) -> i32 {
    lhs.sort();
    rhs.sort();

    lhs.iter().zip(rhs).fold(0_i32, |acc, (left, right)| {
        acc + (left.abs_diff(*right) as i32)
    })
}

fn part2(lhs: &mut [i32], rhs: &mut [i32]) -> i32 {
    lhs.sort();
    rhs.sort();

    let mut acc = 0;
    for x in lhs.iter() {
        acc += x * rhs.iter().filter(|&y| *y == *x).count() as i32;
    }
    acc
}

pub fn main() {
    let input = std::fs::read_to_string("input/day1.txt").expect("No input");
    let (mut lhs, mut rhs) = parse_input(&input);
    println!("Part1: {}", part1(&mut lhs, &mut rhs));
    println!("Part2: {}", part2(&mut lhs, &mut rhs));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3   4\n\
                         4   3\n\
                         2   5\n\
                         1   3\n\
                         3   9\n\
                         3   3";

    #[test]
    fn test_part1() {
        let (mut left, mut right) = parse_input(INPUT);
        assert_eq!(part1(&mut left, &mut right), 11);
    }

    #[test]
    fn test_part2() {
        let (mut left, mut right) = parse_input(INPUT);
        assert_eq!(part2(&mut left, &mut right), 31);
    }
}
