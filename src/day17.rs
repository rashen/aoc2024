use itertools::Itertools;

pub fn main() {
    let input = std::fs::read_to_string("input/day17.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Clone)]
struct Program {
    a: i64,
    b: i64,
    c: i64,
    iptr: usize,
    ins: Vec<i64>,
}
impl Program {
    pub fn from_str(input: &str) -> Option<Self> {
        let mut l = input.lines();
        let a = l.next()?.split_once(": ")?.1.parse::<i64>().ok()?;
        let b = l.next()?.split_once(": ")?.1.parse::<i64>().ok()?;
        let c = l.next()?.split_once(": ")?.1.parse::<i64>().ok()?;
        l.next();
        let ins = l
            .next()?
            .split_once(": ")?
            .1
            .split(',')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        Some(Program {
            a,
            b,
            c,
            iptr: 0,
            ins,
        })
    }
    pub fn new(a: i64, b: i64, c: i64, ins: Vec<i64>) -> Self {
        Self {
            a,
            b,
            c,
            iptr: 0,
            ins,
        }
    }
}

fn literal_operand(p: &Program) -> i64 {
    p.ins[p.iptr + 1]
}
fn combo_operand(p: &Program) -> i64 {
    let val = p.ins[p.iptr + 1];
    match val {
        0..=3 => val,
        4 => p.a,
        5 => p.b,
        6 => p.c,
        _ => panic!("Invalid combo operand"),
    }
}

fn part1(input: &str) -> String {
    let mut p = Program::from_str(input).unwrap();
    let out = eval(&mut p);
    out.iter()
        .map(|i| i.to_string())
        .intersperse(",".to_string())
        .collect()
}

fn eval(p: &mut Program) -> Vec<i64> {
    let mut out = Vec::new();
    while p.iptr < p.ins.len() {
        let opcode = p.ins[p.iptr];
        match opcode {
            0 => {
                // adv
                p.a = p.a / 2_i64.pow(combo_operand(&p) as u32);
            }
            1 => {
                // bxl
                p.b = p.b ^ literal_operand(&p);
            }
            2 => {
                // bst
                p.b = combo_operand(&p).rem_euclid(8)
            }
            3 => {
                // jnz
                if p.a > 0 {
                    p.iptr = literal_operand(&p) as usize;
                    continue;
                }
            }
            4 => {
                // bxc
                p.b = p.b ^ p.c;
            }
            5 => {
                // out
                let res = combo_operand(&p).rem_euclid(8);
                out.push(res);
            }
            6 => {
                // bdv
                p.b = p.a / 2_i64.pow(combo_operand(&p) as u32);
            }
            7 => {
                // cdv
                p.c = p.a / 2_i64.pow(combo_operand(&p) as u32);
            }
            _ => panic!("invalid opcode"),
        };

        p.iptr += 2;
    }
    out
}

fn octal_to_decimal(v: &[i64]) -> i64 {
    let mut octal: i64 = v
        .iter()
        .enumerate()
        .map(|(i, n)| n * 10_i64.pow(i as u32))
        .sum();
    let mut decimal = 0;
    let mut base = 1;

    while octal > 0 {
        decimal += (octal % 10) * base;
        octal /= 10;
        base *= 8;
    }

    decimal
}

fn part2(input: &str) -> i64 {
    let p = Program::from_str(input).unwrap();

    let mut reg_a = Vec::new();

    for i in 0..p.ins.len() {
        for a in 0..8 {
            let mut p = p.clone();
            let mut test_a = reg_a.clone();
            test_a.push(a);
            test_a.reverse();
            p.a = octal_to_decimal(&test_a);
            let out = eval(&mut p);

            if out == p.ins[p.ins.len() - 1 - i..] {
                reg_a.push(a);
                break;
            }
        }
    }

    reg_a.reverse();
    octal_to_decimal(&reg_a)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_parse() {
        let p = Program::from_str(INPUT).unwrap();
        assert_eq!(p.a, 729);
        assert_eq!(p.b, 0);
        assert_eq!(p.c, 0);
        assert_eq!(p.ins, vec![0, 1, 5, 4, 3, 0]);
    }

    #[test]
    fn test_opcodes() {
        assert_eq!(3 ^ 7, 4);
        assert_eq!(0 ^ 1, 1);
        assert_eq!(2 ^ 0, 2);
        assert_eq!(7 ^ 7, 0);
    }

    #[test]
    fn test_eval() {
        {
            let mut p = Program::new(0, 0, 9, vec![2, 6]);
            let _ = eval(&mut p);
            assert_eq!(p.b, 1);
        }
        {
            let mut p = Program::new(10, 0, 0, vec![5, 0, 5, 1, 5, 4]);
            let out = eval(&mut p);
            assert_eq!(out, vec![0, 1, 2]);
        }
        {
            let mut p = Program::new(2024, 0, 0, vec![0, 1, 5, 4, 3, 0]);
            let out = eval(&mut p);
            assert_eq!(out, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
            assert_eq!(p.a, 0);
        }
        {
            let mut p = Program::new(0, 29, 0, vec![1, 7]);
            let _ = eval(&mut p);
            assert_eq!(p.b, 26);
        }
        {
            let mut p = Program::new(0, 2024, 43690, vec![4, 0]);
            let _ = eval(&mut p);
            assert_eq!(p.b, 44354);
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_octal_to_decimal() {
        assert_eq!(octal_to_decimal(&[7]), 7);
        assert_eq!(octal_to_decimal(&[5, 4]), 44);
        assert_eq!(octal_to_decimal(&[3, 4, 5, 3, 0, 0]), 117440);
        assert_eq!(octal_to_decimal(&[0, 0]), 0);
    }
}
