use regex::Regex;

pub fn main() {
    let input = std::fs::read_to_string("input/day3.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let re = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();
    let mut acc = 0;
    for m in re.captures_iter(input) {
        let lhs: i32 = m
            .get(1)
            .and_then(|s| s.as_str().parse::<i32>().ok())
            .unwrap();
        let rhs: i32 = m
            .get(2)
            .and_then(|s| s.as_str().parse::<i32>().ok())
            .unwrap();
        acc += lhs * rhs;
    }
    acc
}

fn part2(input: &str) -> i32 {
    let re = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)|do\\(\\)|don\\'t\\(\\)").unwrap();
    let mut acc = 0;
    let mut enabled = true;
    for m in re.captures_iter(input) {
        match m.get(0).and_then(|m| Some(m.as_str())) {
            None => unimplemented!(),
            Some("do()") => enabled = true,
            Some("don't()") => enabled = false,
            _ => {
                if enabled {
                    let lhs: i32 = m
                        .get(1)
                        .and_then(|s| s.as_str().parse::<i32>().ok())
                        .unwrap();
                    let rhs: i32 = m
                        .get(2)
                        .and_then(|s| s.as_str().parse::<i32>().ok())
                        .unwrap();
                    acc += lhs * rhs;
                }
            }
        }
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT2), 48);
    }
}
