use itertools::Itertools;

pub fn main() {
    let input = std::fs::read_to_string("input/day7.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let valid_operators = [Operators::Addition, Operators::Multiplication];
    input
        .lines()
        .fold(0, |acc, l| acc + eval(l, &valid_operators).unwrap_or(0))
}

#[derive(Debug, Clone, Copy)]
enum Operators {
    Addition,
    Multiplication,
    Concat,
}

fn eval(expr: &str, valid_operators: &[Operators]) -> Option<i64> {
    let (test_val, operands) = expr.split_once(':')?;
    let test_val = test_val.parse::<i64>().ok()?;
    let operands = operands
        .split(' ')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    let n_perms = operands.len() - 1;

    for operators in itertools::repeat_n(valid_operators, n_perms).multi_cartesian_product() {
        let mut acc = operands[0];
        for (operator, operand) in operators.iter().zip(operands[1..].iter()) {
            match operator {
                Operators::Addition => acc += operand,
                Operators::Multiplication => acc *= operand,
                Operators::Concat => acc = concat(acc, *operand),
            }
        }
        if acc == test_val {
            return Some(test_val);
        }
    }

    Some(0)
}

fn concat(lhs: i64, rhs: i64) -> i64 {
    let shift = rhs.ilog10() + 1;
    rhs + lhs * 10_i64.pow(shift)
}

fn part2(input: &str) -> i64 {
    let valid_operators = [
        Operators::Addition,
        Operators::Multiplication,
        Operators::Concat,
    ];
    input
        .lines()
        .fold(0, |acc, l| acc + eval(l, &valid_operators).unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3749);
    }

    #[test]
    fn test_eval() {
        let part1_operators = [Operators::Addition, Operators::Multiplication];
        assert_eq!(eval("190: 10 19", &part1_operators), Some(190));
        assert_eq!(eval("156: 15, 6", &part1_operators), Some(0));
        assert_eq!(eval("156 15 6", &part1_operators), None); // Faulty format
        let part2_operators = [
            Operators::Addition,
            Operators::Multiplication,
            Operators::Concat,
        ];
        assert_eq!(eval("156: 15 6", &part2_operators), Some(156));
        assert_eq!(eval("7290: 6 8 6 15", &part2_operators), Some(7290));
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(1, 2), 12);
        assert_eq!(concat(2, 45), 245);
        assert_eq!(concat(15, 6), 156);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 11387);
    }

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
}
