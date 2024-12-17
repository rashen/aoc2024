use itertools::Itertools;

pub fn main() {
    let input = std::fs::read_to_string("input/day17.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Clone)]
struct Program {
    a: i32,
    b: i32,
    c: i32,
    iptr: usize,
    ins: Vec<i32>,
}
impl Program {
    pub fn from_str(input: &str) -> Option<Self> {
        let mut l = input.lines();
        let a = l.next()?.split_once(": ")?.1.parse::<i32>().ok()?;
        let b = l.next()?.split_once(": ")?.1.parse::<i32>().ok()?;
        let c = l.next()?.split_once(": ")?.1.parse::<i32>().ok()?;
        l.next();
        let ins = l
            .next()?
            .split_once(": ")?
            .1
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        Some(Program {
            a,
            b,
            c,
            iptr: 0,
            ins,
        })
    }
    pub fn new(a: i32, b: i32, c: i32, ins: Vec<i32>) -> Self {
        Self {
            a,
            b,
            c,
            iptr: 0,
            ins,
        }
    }
}

fn literal_operand(p: &Program) -> i32 {
    p.ins[p.iptr + 1] as i32
}
fn combo_operand(p: &Program) -> i32 {
    let val = p.ins[p.iptr + 1] as i32;
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

fn eval(p: &mut Program) -> Vec<i32> {
    let mut out = Vec::new();
    while p.iptr < p.ins.len() {
        let opcode = p.ins[p.iptr];
        match opcode {
            0 => {
                // adv
                p.a = p.a / 2_i32.pow(combo_operand(&p) as u32);
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
                out.push(res)
            }
            6 => {
                // bdv
                p.b = p.a / 2_i32.pow(combo_operand(&p) as u32);
            }
            7 => {
                // cdv
                p.c = p.a / 2_i32.pow(combo_operand(&p) as u32);
            }
            _ => panic!("invalid opcode"),
        };

        p.iptr += 2;
    }
    out
}

fn part2(input: &str) -> i32 {
    let p = Program::from_str(input).unwrap();

    for i in 0.. {
        let mut p = p.clone();
        p.a = i;
        let out = eval(&mut p);
        if out == p.ins {
            return i;
        }
    }
    unreachable!()
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

    const INPUT2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT2), 117440);
    }
}
