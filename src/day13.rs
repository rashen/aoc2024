use regex::Regex;

type IVec2 = glam::I64Vec2;

pub fn main() {
    let input = std::fs::read_to_string("input/day13.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, PartialEq, Eq)]
struct ClawMachine {
    a: IVec2,
    b: IVec2,
    target: IVec2,
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    /* This is very unsafe */
    let re = Regex::new("X.+?(\\d+),\\s+Y.+?(\\d+)").unwrap();
    let mut machines = Vec::new();

    let mut lines = input.lines();
    while let Some(l) = lines.next() {
        let a = re
            .captures(l)
            .map(|c| IVec2::new(c[1].parse::<i64>().unwrap(), c[2].parse::<i64>().unwrap()))
            .unwrap();
        let l = lines.next().unwrap();
        let b = re
            .captures(l)
            .map(|c| IVec2::new(c[1].parse::<i64>().unwrap(), c[2].parse::<i64>().unwrap()))
            .unwrap();
        let l = lines.next().unwrap();
        let target = re
            .captures(l)
            .map(|c| IVec2::new(c[1].parse::<i64>().unwrap(), c[2].parse::<i64>().unwrap()))
            .unwrap();
        machines.push(ClawMachine { a, b, target });
        lines.next();
    }

    machines
}

fn part1(input: &str) -> i64 {
    let machines = parse_input(input);
    machines.iter().fold(0, |acc, m| acc + eval_machine(m))
}

fn eval_machine(machine: &ClawMachine) -> i64 {
    let max_runs = 100;
    let mut min_cost = i64::MAX;
    /* Technically this can go to 2*max_runs */
    for i in 0..max_runs {
        for j in 0..max_runs {
            if machine.a * i + machine.b * j == machine.target {
                let cost = 3 * i + j;
                if cost < min_cost {
                    min_cost = cost;
                }
            }
        }
    }
    if min_cost < i64::MAX {
        return min_cost;
    }
    0
}

fn part2(input: &str) -> i64 {
    let machines = parse_input(input)
        .iter()
        .map(|m| ClawMachine {
            a: m.a,
            b: m.b,
            target: m.target + 10000000000000,
        })
        .collect::<Vec<ClawMachine>>();

    let mut cost = 0;
    for m in machines {
        let det = (m.a.x * m.b.y) - (m.a.y * m.b.x);
        let a = (m.target.x * m.b.y - m.target.y * m.b.x) / det;
        let b = (m.a.x * m.target.y - m.a.y * m.target.x) / det;
        if a * m.a + b * m.b == m.target {
            cost += 3 * a + b;
        }
    }

    cost
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(INPUT);
        assert_eq!(
            parsed[0],
            ClawMachine {
                a: IVec2::new(94, 34),
                b: IVec2::new(22, 67),
                target: IVec2::new(8400, 5400)
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 480);
    }
}
