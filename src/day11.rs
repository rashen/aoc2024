use std::collections::BTreeMap;

pub fn main() {
    let input = std::fs::read_to_string("input/day11.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

type Stone = i64;
type Stones = Vec<Stone>;

fn parse_stones(input: &str) -> Stones {
    // let mut list = List::new();
    let mut vec = Vec::new();
    for c in input
        .split_whitespace()
        .filter_map(|c| c.parse::<Stone>().ok())
    {
        vec.push(c);
    }
    vec
}

fn blink(stones: Stones) -> Stones {
    let mut output = Vec::new();
    for s in stones.into_iter() {
        if s == 0 {
            output.push(1);
        } else if s.ilog10() % 2 == 1 {
            let num_digits = s.ilog10() + 1;
            let half = num_digits / 2;
            let first = s.div_euclid(10_i64.pow(half));
            let second = s.rem_euclid(10_i64.pow(half));
            output.push(first);
            output.push(second);
        } else {
            output.push(s * 2024);
        }
    }
    output
}

fn part1(input: &str) -> i32 {
    let mut stones = parse_stones(input);
    for _ in 0..25 {
        stones = blink(stones);
    }
    stones.len() as i32
}

fn blink_with_map(stones: BTreeMap<i64, i64>) -> BTreeMap<i64, i64> {
    let mut output = BTreeMap::new();
    for (s, num) in stones.into_iter() {
        if s == 0 {
            insert(&mut output, 1, num);
        } else if s.ilog10() % 2 == 1 {
            let num_digits = s.ilog10() + 1;
            let half = num_digits / 2;
            let first = s.div_euclid(10_i64.pow(half));
            let second = s.rem_euclid(10_i64.pow(half));
            insert(&mut output, first, num);
            insert(&mut output, second, num);
        } else {
            insert(&mut output, s * 2024, num);
        }
    }
    output
}

fn insert(m: &mut BTreeMap<i64, i64>, key: i64, val: i64) {
    if let Some(v) = m.get_mut(&key) {
        *v += val;
    } else {
        m.insert(key, val);
    }
}

fn part2(input: &str) -> i64 {
    let stones = parse_stones(input);
    let mut stones_map: BTreeMap<i64, i64> = BTreeMap::new();
    for s in stones {
        insert(&mut stones_map, s, 1);
    }
    println!("{:?}", stones_map);

    for _ in 0..75 {
        stones_map = blink_with_map(stones_map);
    }

    stones_map.into_iter().fold(0, |acc, (_, v)| acc + v)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_stones(stones: Stones) -> String {
        stones
            .into_iter()
            .map(|s| s.to_string())
            .intersperse(" ".to_string())
            .collect::<String>()
    }

    #[test]
    fn test_parse() {
        assert_eq!(print_stones(parse_stones("125 17")), "125 17");
    }

    #[test]
    fn test_blink() {
        let mut stones = parse_stones("125 17");
        stones = blink(stones);
        assert_eq!(print_stones(stones.clone()), "253000 1 7");
        stones = blink(stones);
        assert_eq!(print_stones(stones.clone()), "253 0 2024 14168");
        stones = blink(stones);
        assert_eq!(print_stones(stones.clone()), "512072 1 20 24 28676032");
        stones = blink(stones);
        assert_eq!(
            print_stones(stones.clone()),
            "512 72 2024 2 0 2 4 2867 6032"
        );
        stones = blink(stones);
        assert_eq!(
            print_stones(stones.clone()),
            "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32"
        );
        stones = blink(stones);
        assert_eq!(
            print_stones(stones),
            "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"
        );
    }
}
