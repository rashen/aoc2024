pub fn main() {
    let input = std::fs::read_to_string("input/day2.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn is_safe(report: &[i32]) -> bool {
    let sign = (report[0] - report[1]).signum();
    if sign == 0 {
        return false;
    }
    for w in report.windows(2) {
        let diff = w[0] - w[1];
        if diff.signum() != sign {
            return false;
        }
        if diff.abs() > 3 {
            return false;
        }
    }
    true
}

fn part1(input: &str) -> i32 {
    let mut acc = 0;
    for line in input.lines() {
        let report = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();
        if is_safe(&report) {
            acc += 1;
        }
    }
    acc
}

fn is_safe_with_tolerance(report: &[i32]) -> bool {
    match is_safe(report) {
        true => return true,
        false => {
            for j in 0..report.len() {
                let report = Vec::from_iter(
                    report
                        .iter()
                        .enumerate()
                        .filter_map(|(i, val)| {
                            if i != j {
                                return Some(val);
                            }
                            None
                        })
                        .cloned(),
                );
                if is_safe(&report) {
                    return true;
                }
            }
            false
        }
    }
}

fn part2(input: &str) -> i32 {
    let mut acc = 0;
    for line in input.lines() {
        let report = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();
        if is_safe_with_tolerance(&report) {
            acc += 1;
        }
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_is_safe() {
        let descending = [7, 6, 4, 2, 1];
        let ascending = [1, 3, 6, 7, 9];
        let not_safe = [1, 2, 7, 8, 9];
        assert_eq!(is_safe(&ascending), true);
        assert_eq!(is_safe(&descending), true);
        assert_eq!(is_safe(&not_safe), false);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn test_is_safe_with_tolerance() {
        assert_eq!(is_safe_with_tolerance(&[7, 6, 4, 2, 1]), true);
        assert_eq!(is_safe_with_tolerance(&[1, 2, 7, 8, 9]), false);
        assert_eq!(is_safe_with_tolerance(&[9, 7, 6, 2, 1]), false);
        assert_eq!(is_safe_with_tolerance(&[1, 3, 2, 4, 5]), true);
        assert_eq!(is_safe_with_tolerance(&[8, 6, 4, 4, 1]), true);
        assert_eq!(is_safe_with_tolerance(&[1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 4);
    }
}
