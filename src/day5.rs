pub fn main() {
    let input = std::fs::read_to_string("input/day5.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Requirement {
    pub value: i32,
    pub before: i32,
}
impl Requirement {
    pub fn new(value: i32, before: i32) -> Self {
        Self { value, before }
    }
}

fn parse_requirements(input: &str) -> Vec<Requirement> {
    let mut reqs = Vec::new();
    for l in input.lines() {
        if let Some((lhs, rhs)) = l.split_once('|') {
            reqs.push(Requirement::new(
                lhs.parse::<i32>().unwrap(),
                rhs.parse::<i32>().unwrap(),
            ));
        } else {
            break;
        }
    }
    reqs
}

fn check_req(page: i32, pages: &[i32], reqs: &[Requirement]) -> bool {
    for r in reqs.iter().filter(|&r| r.value == page) {
        if pages.contains(&r.before) {
            return false;
        }
    }
    true
}

fn get_sorted_unsorted(input: &str, reqs: &[Requirement]) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut sorted = Vec::new();
    let mut unsorted = Vec::new();
    'outer: for l in input.lines() {
        if l.contains('|') {
            // Skip all requirements
            continue;
        }

        let pages = l
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        for (i, p) in pages.iter().enumerate() {
            if !check_req(*p, &pages[0..i], reqs) {
                unsorted.push(pages);
                continue 'outer;
            }
        }
        // If we get here, the sequence is valid
        sorted.push(pages);
    }
    (sorted, unsorted)
}

fn part1(input: &str) -> i32 {
    let reqs = parse_requirements(input);
    let mut acc = 0;

    let (sorted, _) = get_sorted_unsorted(input, &reqs);
    for pages in sorted.into_iter() {
        let mid = pages.len() / 2;
        if let Some(mid_val) = pages.get(mid) {
            acc += mid_val;
        }
    }
    acc
}

fn sort(pages: &[i32], reqs: &[Requirement]) -> Vec<i32> {
    // Remove any unused requirement
    let mut reqs = reqs
        .iter()
        .filter(|&r| pages.contains(&r.before))
        .cloned()
        .collect::<Vec<Requirement>>();

    let len = pages.len();
    let mut output = Vec::with_capacity(len);

    let mut remaining_pages = Vec::from(pages);
    let mut counter = 0;
    while !remaining_pages.is_empty() {
        for (i, p) in remaining_pages.iter().enumerate() {
            // reqs.retain(|p| remaining_pages.contains(&p.value));
            let local_reqs = reqs.iter().filter(|&r| r.value == *p);
            let count = local_reqs.count();
            if count == 0 {
                output.push(*p);
                remaining_pages.swap_remove(i);
                reqs.retain(|p| {
                    remaining_pages.contains(&p.value) && remaining_pages.contains(&p.before)
                });
                break;
            }
        }
        counter += 1;
        assert!(counter < 100);
    }

    output.into_iter().rev().collect::<Vec<i32>>()
}

fn part2(input: &str) -> i32 {
    let reqs = parse_requirements(input);
    let (_, unsorted) = get_sorted_unsorted(input, &reqs);
    let mut acc = 0;

    for pages in unsorted.into_iter() {
        let sorted = sort(&pages, &reqs);

        let mid = sorted.len() / 2;
        if let Some(mid_val) = sorted.get(mid) {
            acc += mid_val;
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 143);
    }

    #[test]
    fn test_parse_requirements() {
        assert_eq!(
            parse_requirements("2|3\n1|2\n3|4"),
            vec![
                Requirement::new(2, 3),
                Requirement::new(1, 2),
                Requirement::new(3, 4)
            ]
        );
    }

    #[test]
    fn test_sort() {
        let reqs = parse_requirements(INPUT);
        assert_eq!(sort(&[75, 97, 47, 61, 53], &reqs), vec![97, 75, 47, 61, 53]);
        assert_eq!(sort(&[61, 13, 29], &reqs), vec![61, 29, 13]);
        assert_eq!(sort(&[97, 13, 75, 29, 47], &reqs), vec![97, 75, 47, 29, 13]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 123);
    }

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
}
